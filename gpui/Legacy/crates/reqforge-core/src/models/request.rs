use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::validation::{validate_request, ValidationResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BodyType {
    #[default]
    None,
    Raw { content: String, content_type: RawContentType },
    FormUrlEncoded(Vec<KeyValuePair>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RawContentType {
    #[default]
    Json,
    Xml,
    Text,
    Html,
}

/// The core, persistable request definition.
/// All string fields may contain `{{variable}}` placeholders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDefinition {
    pub id: Uuid,
    pub name: String,
    pub method: HttpMethod,
    pub url: String,                         // e.g. "{{base_url}}/api/users"
    pub headers: Vec<KeyValuePair>,
    pub query_params: Vec<KeyValuePair>,
    pub body: BodyType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl RequestDefinition {
    pub fn new(name: impl Into<String>, method: HttpMethod, url: impl Into<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            method,
            url: url.into(),
            headers: Vec::new(),
            query_params: Vec::new(),
            body: BodyType::None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate this request definition.
    ///
    /// This method performs comprehensive validation on the request:
    /// - URL format and scheme
    /// - Header names and values
    /// - Body content matching the content-type
    /// - Body compatibility with the HTTP method
    ///
    /// # Examples
    ///
    /// ```
    /// use reqforge_core::models::request::{RequestDefinition, HttpMethod};
    ///
    /// let req = RequestDefinition::new("Test", HttpMethod::GET, "https://api.example.com");
    /// assert!(req.validate().is_ok());
    /// ```
    pub fn validate(&self) -> ValidationResult<()> {
        validate_request(self)
    }
}
