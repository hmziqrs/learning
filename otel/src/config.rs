#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub quickwit_otlp_endpoint: String,
    pub quickwit_http_endpoint: String,
    pub prometheus_endpoint: String,
    pub service_host: String,
    pub service_port: u16,
}

impl Config {
    pub fn new() -> Self {
        Config {
            database_url: "postgres://postgres:password@localhost:5432/todos".to_string(),
            redis_url: "redis://127.0.0.1:6379".to_string(),
            quickwit_otlp_endpoint: "http://127.0.0.1:7281".to_string(),
            quickwit_http_endpoint: "http://127.0.0.1:7280".to_string(),
            prometheus_endpoint: "http://127.0.0.1:9090".to_string(),
            service_host: "0.0.0.0".to_string(),
            service_port: 3000,
        }
    }
}