use crate::error::Error;
use crate::http::method::HttpMethod;
use crate::http::request::{RequestBody, Request};
use crate::http::response::Response;

pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

    pub async fn send(&self, url: String, method: HttpMethod) -> Result<Response, Error> {
        let request = Request::new(url, method);
        self.execute(request).await
    }

    pub async fn execute(&self, request: Request) -> Result<Response, Error> {
        let start = std::time::Instant::now();

        let mut builder = match request.method {
            HttpMethod::Get => self.inner.get(&request.url),
            HttpMethod::Post => self.inner.post(&request.url),
            HttpMethod::Put => self.inner.put(&request.url),
            HttpMethod::Delete => self.inner.delete(&request.url),
            HttpMethod::Patch => self.inner.patch(&request.url),
        };

        for (key, value) in &request.headers {
            builder = builder.header(key.as_str(), value.as_str());
        }

        if let Some(body) = request.body {
            builder = match body {
                RequestBody::Raw(text) => builder.body(text),
                RequestBody::Json(value) => builder.json(&value),
                RequestBody::Form(pairs) => builder.form(&pairs),
            };
        }

        let response = builder.send().await?;

        let status_code = response.status().as_u16();
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        let body = response.text().await?;
        let elapsed_ms = start.elapsed().as_millis() as u64;

        Ok(Response {
            status_code,
            body,
            elapsed_ms,
            headers,
        })
    }
}
