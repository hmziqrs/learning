#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status_code: u16,
    pub body: String,
    pub elapsed_ms: u64,
}
