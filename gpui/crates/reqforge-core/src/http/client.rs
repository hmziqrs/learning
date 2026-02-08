use reqwest::Client;
use crate::models::request::{RequestDefinition, BodyType, RawContentType};
use crate::models::response::HttpResponse;
use crate::validation::ValidationError;
use std::time::Instant;
use std::time::Duration;

pub struct HttpEngine {
    client: Client,
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    UrlParse(String),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}

impl HttpEngine {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(false)   // make configurable later
            // Prevent requests from hanging indefinitely (can be made configurable later)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client");
        Self { client }
    }

    /// Execute a fully-resolved RequestDefinition.
    /// Variables must already be interpolated before calling this.
    ///
    /// This method validates the request before sending it, checking:
    /// - URL format and scheme
    /// - Header names and values
    /// - Body content matching the content-type
    /// - Body compatibility with the HTTP method
    pub async fn execute(&self, req: &RequestDefinition) -> Result<HttpResponse, HttpError> {
        // Validate the request before executing
        req.validate()?;

        let method = match req.method {
            crate::models::request::HttpMethod::GET => reqwest::Method::GET,
            crate::models::request::HttpMethod::POST => reqwest::Method::POST,
            crate::models::request::HttpMethod::PUT => reqwest::Method::PUT,
            crate::models::request::HttpMethod::PATCH => reqwest::Method::PATCH,
            crate::models::request::HttpMethod::DELETE => reqwest::Method::DELETE,
            crate::models::request::HttpMethod::HEAD => reqwest::Method::HEAD,
            crate::models::request::HttpMethod::OPTIONS => reqwest::Method::OPTIONS,
        };

        let mut builder = self.client.request(method, &req.url);

        // Query params
        let query_params: Vec<(&str, &str)> = req.query_params
            .iter()
            .filter(|p| p.enabled)
            .map(|p| (p.key.as_str(), p.value.as_str()))
            .collect();
        if !query_params.is_empty() {
            builder = builder.query(&query_params);
        }

        // Headers
        for h in req.headers.iter().filter(|h| h.enabled) {
            builder = builder.header(&h.key, &h.value);
        }

        // Body
        builder = match &req.body {
            BodyType::None => builder,
            BodyType::Raw { content, content_type } => {
                let mime = match content_type {
                    RawContentType::Json => "application/json",
                    RawContentType::Xml  => "application/xml",
                    RawContentType::Text => "text/plain",
                    RawContentType::Html => "text/html",
                };
                builder.header("Content-Type", mime).body(content.clone())
            }
            BodyType::FormUrlEncoded(pairs) => {
                let form: Vec<(&str, &str)> = pairs.iter()
                    .filter(|p| p.enabled)
                    .map(|p| (p.key.as_str(), p.value.as_str()))
                    .collect();
                if form.is_empty() { builder } else { builder.form(&form) }
            }
        };

        let start = Instant::now();
        let response = builder.send().await?;
        let elapsed = start.elapsed();

        let status = response.status().as_u16();
        let status_text = response.status().canonical_reason().unwrap_or("").to_string();
        let headers = response.headers().iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response.bytes().await?;
        let size = body.len();
        let body_text = std::str::from_utf8(body.as_ref()).ok().map(ToOwned::to_owned);
        let bytes = body.to_vec();

        Ok(HttpResponse {
            status,
            status_text,
            headers,
            body: bytes,
            body_text,
            size_bytes: size,
            elapsed,
        })
    }
}
