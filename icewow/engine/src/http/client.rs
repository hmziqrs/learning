use crate::error::Error;
use crate::http::method::HttpMethod;
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
        let start = std::time::Instant::now();

        let response = match method {
            HttpMethod::Get => self.inner.get(&url),
            HttpMethod::Post => self.inner.post(&url),
            HttpMethod::Put => self.inner.put(&url),
            HttpMethod::Delete => self.inner.delete(&url),
            HttpMethod::Patch => self.inner.patch(&url),
        }
        .send()
        .await?;

        let status_code = response.status().as_u16();
        let body = response.text().await?;
        let elapsed_ms = start.elapsed().as_millis() as u64;

        Ok(Response {
            status_code,
            body,
            elapsed_ms,
        })
    }
}
