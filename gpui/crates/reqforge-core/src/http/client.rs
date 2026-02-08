use reqwest::Client;
use crate::models::request::{RequestDefinition, BodyType, RawContentType};
use crate::models::response::HttpResponse;
use crate::validation::ValidationError;
use std::time::Instant;
use std::collections::HashMap;

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

        let method: reqwest::Method = req.method.to_string().parse().unwrap();

        let mut builder = self.client.request(method, &req.url);

        // Query params
        let query_params: HashMap<String, String> = req.query_params
            .iter()
            .filter(|p| p.enabled)
            .map(|p| (p.key.clone(), p.value.clone()))
            .collect();
        builder = builder.query(&query_params);

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
                let form: HashMap<String, String> = pairs.iter()
                    .filter(|p| p.enabled)
                    .map(|p| (p.key.clone(), p.value.clone()))
                    .collect();
                builder.form(&form)
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

        let bytes = response.bytes().await?.to_vec();
        let size = bytes.len();
        let body_text = String::from_utf8(bytes.clone()).ok();

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
