use crate::models::template::{
    RequestTemplate, TemplateCategory, TemplateVariable, BodyTemplateType,
};
use crate::models::request::{HttpMethod, RawContentType};

/// Get all built-in templates
pub fn get_builtin_templates() -> Vec<RequestTemplate> {
    vec![
        get_with_auth_template(),
        post_json_template(),
        put_update_template(),
        delete_template(),
        oauth2_password_flow_template(),
        get_with_pagination_template(),
        post_form_template(),
        graphql_query_template(),
    ]
}

/// GET request with Authorization header
fn get_with_auth_template() -> RequestTemplate {
    RequestTemplate::builtin(
        "GET with Auth",
        "A GET request with an Authorization header for authenticated API calls",
        TemplateCategory::Authentication,
        HttpMethod::GET,
        "{{base_url}}/{{path}}",
    )
    .with_header("Authorization", "Bearer {{access_token}}")
    .with_header("Accept", "application/json")
    .with_variable(TemplateVariable::new("base_url", "The base URL of the API"))
    .with_variable(TemplateVariable::new("path", "The API endpoint path"))
    .with_variable(
        TemplateVariable::new("access_token", "The access token for authentication")
            .with_default("your_token_here"),
    )
}

/// POST request with JSON body
fn post_json_template() -> RequestTemplate {
    RequestTemplate::builtin(
        "POST JSON",
        "A POST request with a JSON body for creating resources",
        TemplateCategory::Api,
        HttpMethod::POST,
        "{{base_url}}/{{path}}",
    )
    .with_header("Content-Type", "application/json")
    .with_header("Accept", "application/json")
    .with_body(BodyTemplateType::Raw {
        content: "{\n  \"key\": \"{{value}}\"\n}".to_string(),
        content_type: RawContentType::Json,
    })
    .with_variable(TemplateVariable::new("base_url", "The base URL of the API"))
    .with_variable(TemplateVariable::new("path", "The API endpoint path"))
    .with_variable(TemplateVariable::new("key", "The JSON key name"))
    .with_variable(TemplateVariable::new("value", "The JSON value"))
}

/// PUT request for updating resources
fn put_update_template() -> RequestTemplate {
    RequestTemplate::builtin(
        "PUT Update",
        "A PUT request for updating existing resources",
        TemplateCategory::Api,
        HttpMethod::PUT,
        "{{base_url}}/{{path}}/{{resource_id}}",
    )
    .with_header("Content-Type", "application/json")
    .with_header("Accept", "application/json")
    .with_body(BodyTemplateType::Raw {
        content: "{\n  \"id\": \"{{resource_id}}\",\n  \"field\": \"{{updated_value}}\"\n}".to_string(),
        content_type: RawContentType::Json,
    })
    .with_variable(TemplateVariable::new("base_url", "The base URL of the API"))
    .with_variable(TemplateVariable::new("path", "The API endpoint path"))
    .with_variable(TemplateVariable::new("resource_id", "The ID of the resource to update"))
    .with_variable(TemplateVariable::new("updated_value", "The new value for the field"))
}

/// DELETE request
fn delete_template() -> RequestTemplate {
    RequestTemplate::builtin(
        "DELETE Resource",
        "A DELETE request for removing resources",
        TemplateCategory::Api,
        HttpMethod::DELETE,
        "{{base_url}}/{{path}}/{{resource_id}}",
    )
    .with_header("Authorization", "Bearer {{access_token}}")
    .with_header("Accept", "application/json")
    .with_variable(TemplateVariable::new("base_url", "The base URL of the API"))
    .with_variable(TemplateVariable::new("path", "The API endpoint path"))
    .with_variable(TemplateVariable::new("resource_id", "The ID of the resource to delete"))
    .with_variable(
        TemplateVariable::new("access_token", "The access token for authentication")
            .with_default("your_token_here"),
    )
}

/// OAuth2 Password Flow template
fn oauth2_password_flow_template() -> RequestTemplate {
    RequestTemplate::builtin(
        "OAuth2 Password Flow",
        "OAuth2 authentication using password grant type",
        TemplateCategory::Authentication,
        HttpMethod::POST,
        "{{auth_url}}/oauth/token",
    )
    .with_header("Content-Type", "application/x-www-form-urlencoded")
    .with_header("Accept", "application/json")
    .with_body(BodyTemplateType::FormUrlEncoded(vec![
        crate::models::request::KeyValuePair {
            key: "grant_type".to_string(),
            value: "password".to_string(),
            enabled: true,
            description: Some("OAuth2 grant type".to_string()),
        },
        crate::models::request::KeyValuePair {
            key: "client_id".to_string(),
            value: "{{client_id}}".to_string(),
            enabled: true,
            description: Some("OAuth2 client ID".to_string()),
        },
        crate::models::request::KeyValuePair {
            key: "client_secret".to_string(),
            value: "{{client_secret}}".to_string(),
            enabled: true,
            description: Some("OAuth2 client secret".to_string()),
        },
        crate::models::request::KeyValuePair {
            key: "username".to_string(),
            value: "{{username}}".to_string(),
            enabled: true,
            description: Some("User username".to_string()),
        },
        crate::models::request::KeyValuePair {
            key: "password".to_string(),
            value: "{{password}}".to_string(),
            enabled: true,
            description: Some("User password".to_string()),
        },
    ]))
    .with_variable(TemplateVariable::new("auth_url", "The authentication server URL"))
    .with_variable(TemplateVariable::new("client_id", "OAuth2 client ID"))
    .with_variable(TemplateVariable::new("client_secret", "OAuth2 client secret"))
    .with_variable(TemplateVariable::new("username", "Username"))
    .with_variable(TemplateVariable::new("password", "Password"))
}

/// GET request with pagination
fn get_with_pagination_template() -> RequestTemplate {
    RequestTemplate::builtin(
        "GET with Pagination",
        "A GET request with pagination query parameters",
        TemplateCategory::Api,
        HttpMethod::GET,
        "{{base_url}}/{{path}}",
    )
    .with_header("Accept", "application/json")
    .with_query_param("page", "{{page}}")
    .with_query_param("per_page", "{{per_page}}")
    .with_query_param("sort", "{{sort_field}}")
    .with_query_param("order", "{{order}}")
    .with_variable(TemplateVariable::new("base_url", "The base URL of the API"))
    .with_variable(TemplateVariable::new("path", "The API endpoint path"))
    .with_variable(TemplateVariable::new("page", "Page number").with_default("1"))
    .with_variable(TemplateVariable::new("per_page", "Items per page").with_default("10"))
    .with_variable(TemplateVariable::new("sort_field", "Field to sort by"))
    .with_variable(TemplateVariable::new("order", "Sort order").with_default("asc"))
}

/// POST request with form data
fn post_form_template() -> RequestTemplate {
    RequestTemplate::builtin(
        "POST Form Data",
        "A POST request with form-encoded data",
        TemplateCategory::Basic,
        HttpMethod::POST,
        "{{base_url}}/{{path}}",
    )
    .with_header("Content-Type", "application/x-www-form-urlencoded")
    .with_header("Accept", "application/json")
    .with_body(BodyTemplateType::FormUrlEncoded(vec![
        crate::models::request::KeyValuePair {
            key: "field1".to_string(),
            value: "{{value1}}".to_string(),
            enabled: true,
            description: Some("First form field".to_string()),
        },
        crate::models::request::KeyValuePair {
            key: "field2".to_string(),
            value: "{{value2}}".to_string(),
            enabled: true,
            description: Some("Second form field".to_string()),
        },
    ]))
    .with_variable(TemplateVariable::new("base_url", "The base URL of the API"))
    .with_variable(TemplateVariable::new("path", "The API endpoint path"))
    .with_variable(TemplateVariable::new("value1", "Value for field1"))
    .with_variable(TemplateVariable::new("value2", "Value for field2"))
}

/// GraphQL query template
fn graphql_query_template() -> RequestTemplate {
    RequestTemplate::builtin(
        "GraphQL Query",
        "A POST request with a GraphQL query",
        TemplateCategory::Api,
        HttpMethod::POST,
        "{{graphql_url}}",
    )
    .with_header("Content-Type", "application/json")
    .with_header("Accept", "application/json")
    .with_body(BodyTemplateType::Raw {
        content: "{\n  \"query\": \"{{query}}\",\n  \"variables\": {{variables}}\n}".to_string(),
        content_type: RawContentType::Json,
    })
    .with_variable(TemplateVariable::new("graphql_url", "The GraphQL endpoint URL"))
    .with_variable(TemplateVariable::new("query", "The GraphQL query"))
    .with_variable(
        TemplateVariable::new("variables", "JSON object with query variables")
            .with_default("{}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_builtin_templates() {
        let templates = get_builtin_templates();
        assert_eq!(templates.len(), 8);
    }

    #[test]
    fn test_get_with_auth_template() {
        let template = get_with_auth_template();
        assert_eq!(template.name, "GET with Auth");
        assert_eq!(template.method, HttpMethod::GET);
        assert!(template.headers.iter().any(|h| h.key == "Authorization"));
        assert_eq!(template.variables.len(), 3);
        assert!(template.is_builtin);
    }

    #[test]
    fn test_post_json_template() {
        let template = post_json_template();
        assert_eq!(template.name, "POST JSON");
        assert_eq!(template.method, HttpMethod::POST);
        assert!(matches!(
            &template.body_type,
            BodyTemplateType::Raw { content_type: RawContentType::Json, .. }
        ));
    }

    #[test]
    fn test_oauth2_template() {
        let template = oauth2_password_flow_template();
        assert_eq!(template.name, "OAuth2 Password Flow");
        assert!(matches!(
            &template.body_type,
            BodyTemplateType::FormUrlEncoded(fields) if fields.len() == 5
        ));
    }

    #[test]
    fn test_all_templates_have_required_fields() {
        let templates = get_builtin_templates();
        for template in templates {
            assert!(!template.id.is_nil());
            assert!(!template.name.is_empty());
            assert!(!template.url_template.is_empty());
            assert!(template.is_builtin);
        }
    }
}
