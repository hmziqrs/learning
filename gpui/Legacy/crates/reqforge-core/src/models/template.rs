use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::request::{HttpMethod, KeyValuePair, RawContentType};

/// Category for organizing templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TemplateCategory {
    Basic,
    Authentication,
    Api,
    Testing,
    Custom,
}

impl std::fmt::Display for TemplateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateCategory::Basic => write!(f, "Basic"),
            TemplateCategory::Authentication => write!(f, "Authentication"),
            TemplateCategory::Api => write!(f, "API"),
            TemplateCategory::Testing => write!(f, "Testing"),
            TemplateCategory::Custom => write!(f, "Custom"),
        }
    }
}

/// Definition of a template variable that can be substituted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
}

impl TemplateVariable {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            default_value: None,
            required: true,
        }
    }

    pub fn with_default(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self.required = false;
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

/// A reusable request template with placeholder variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub method: HttpMethod,
    pub url_template: String,
    pub headers: Vec<KeyValuePair>,
    pub query_params: Vec<KeyValuePair>,
    pub body_type: BodyTemplateType,
    pub variables: Vec<TemplateVariable>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_builtin: bool,
}

/// Template body type with support for placeholder content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyTemplateType {
    None,
    Raw { content: String, content_type: RawContentType },
    FormUrlEncoded(Vec<KeyValuePair>),
}

impl RequestTemplate {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        category: TemplateCategory,
        method: HttpMethod,
        url_template: impl Into<String>,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            category,
            method,
            url_template: url_template.into(),
            headers: Vec::new(),
            query_params: Vec::new(),
            body_type: BodyTemplateType::None,
            variables: Vec::new(),
            created_at: now,
            is_builtin: false,
        }
    }

    pub fn builtin(
        name: impl Into<String>,
        description: impl Into<String>,
        category: TemplateCategory,
        method: HttpMethod,
        url_template: impl Into<String>,
    ) -> Self {
        let mut template = Self::new(name, description, category, method, url_template);
        template.is_builtin = true;
        template
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(KeyValuePair {
            key: key.into(),
            value: value.into(),
            enabled: true,
            description: None,
        });
        self
    }

    pub fn with_query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push(KeyValuePair {
            key: key.into(),
            value: value.into(),
            enabled: true,
            description: None,
        });
        self
    }

    pub fn with_body(mut self, body: BodyTemplateType) -> Self {
        self.body_type = body;
        self
    }

    pub fn with_variable(mut self, variable: TemplateVariable) -> Self {
        self.variables.push(variable);
        self
    }
}

/// Result of creating a request from a template
#[derive(Debug, Clone)]
pub struct TemplateApplicationResult {
    pub request: super::request::RequestDefinition,
    pub missing_variables: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_variable_builder() {
        let var = TemplateVariable::new("api_key", "The API key to use")
            .with_default("default_key")
            .optional();

        assert_eq!(var.name, "api_key");
        assert_eq!(var.description, "The API key to use");
        assert_eq!(var.default_value, Some("default_key".to_string()));
        assert!(!var.required);
    }

    #[test]
    fn test_request_template_builder() {
        let template = RequestTemplate::new(
            "Test Template",
            "A test template",
            TemplateCategory::Basic,
            HttpMethod::GET,
            "{{base_url}}/test",
        )
        .with_header("Authorization", "Bearer {{token}}")
        .with_query_param("limit", "{{limit}}")
        .with_variable(TemplateVariable::new("base_url", "Base URL"))
        .with_variable(TemplateVariable::new("token", "Auth token").with_default("my_token"))
        .with_variable(TemplateVariable::new("limit", "Limit results"));

        assert_eq!(template.name, "Test Template");
        assert_eq!(template.headers.len(), 1);
        assert_eq!(template.query_params.len(), 1);
        assert_eq!(template.variables.len(), 3);
        assert!(!template.is_builtin);
    }

    #[test]
    fn test_builtin_template() {
        let template = RequestTemplate::builtin(
            "Builtin Template",
            "A builtin template",
            TemplateCategory::Basic,
            HttpMethod::POST,
            "{{base_url}}/api/create",
        );

        assert!(template.is_builtin);
    }

    #[test]
    fn test_category_display() {
        assert_eq!(TemplateCategory::Basic.to_string(), "Basic");
        assert_eq!(TemplateCategory::Authentication.to_string(), "Authentication");
        assert_eq!(TemplateCategory::Api.to_string(), "API");
        assert_eq!(TemplateCategory::Testing.to_string(), "Testing");
        assert_eq!(TemplateCategory::Custom.to_string(), "Custom");
    }
}
