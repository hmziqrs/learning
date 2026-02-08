pub mod builtin;

use crate::models::template::{
    RequestTemplate, TemplateApplicationResult, TemplateCategory,
};
use crate::models::request::{KeyValuePair, BodyType};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Manager for request templates
pub struct TemplateManager {
    builtin_templates: Vec<RequestTemplate>,
    custom_templates: Vec<RequestTemplate>,
    workspace_dir: PathBuf,
}

impl TemplateManager {
    /// Create a new template manager with builtin templates
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            builtin_templates: builtin::get_builtin_templates(),
            custom_templates: Vec::new(),
            workspace_dir,
        }
    }

    /// Load custom templates from workspace
    pub fn load_custom_templates(&mut self) -> Result<(), TemplateError> {
        let templates_path = self.workspace_dir.join("templates.json");
        if templates_path.exists() {
            let data = std::fs::read_to_string(&templates_path)?;
            self.custom_templates = serde_json::from_str(&data)?;
        }
        Ok(())
    }

    /// Save custom templates to workspace
    pub fn save_custom_templates(&self) -> Result<(), TemplateError> {
        let templates_path = self.workspace_dir.join("templates.json");
        let json = serde_json::to_string_pretty(&self.custom_templates)?;
        std::fs::write(templates_path, json)?;
        Ok(())
    }

    /// Get all templates (builtin + custom)
    pub fn get_all_templates(&self) -> Vec<&RequestTemplate> {
        self.builtin_templates
            .iter()
            .chain(self.custom_templates.iter())
            .collect()
    }

    /// Get all templates grouped by category
    pub fn get_templates_by_category(&self) -> HashMap<TemplateCategory, Vec<&RequestTemplate>> {
        let mut grouped: HashMap<TemplateCategory, Vec<&RequestTemplate>> = HashMap::new();
        for template in self.get_all_templates() {
            grouped
                .entry(template.category.clone())
                .or_default()
                .push(template);
        }
        grouped
    }

    /// Get template by ID
    pub fn get_template(&self, id: &Uuid) -> Option<&RequestTemplate> {
        self.builtin_templates
            .iter()
            .chain(self.custom_templates.iter())
            .find(|t| &t.id == id)
    }

    /// Get template by name
    pub fn get_template_by_name(&self, name: &str) -> Option<&RequestTemplate> {
        self.get_all_templates()
            .into_iter()
            .find(|t| t.name == name)
    }

    /// Add a custom template
    pub fn add_custom_template(&mut self, template: RequestTemplate) -> Result<(), TemplateError> {
        if template.is_builtin {
            return Err(TemplateError::CannotModifyBuiltin);
        }
        self.custom_templates.push(template);
        Ok(())
    }

    /// Delete a custom template
    pub fn delete_custom_template(&mut self, id: &Uuid) -> Result<(), TemplateError> {
        if self.builtin_templates.iter().any(|t| &t.id == id) {
            return Err(TemplateError::CannotModifyBuiltin);
        }
        self.custom_templates.retain(|t| &t.id != id);
        Ok(())
    }

    /// Create a request from a template with variable substitutions
    pub fn create_from_template(
        &self,
        template_id: &Uuid,
        variables: &HashMap<String, String>,
        name: String,
    ) -> Result<TemplateApplicationResult, TemplateError> {
        let template = self.get_template(template_id)
            .ok_or(TemplateError::TemplateNotFound(*template_id))?;

        let mut missing_variables = Vec::new();

        // Check for required variables without values
        for var in &template.variables {
            if var.required && !variables.contains_key(&var.name) {
                if var.default_value.is_none() {
                    missing_variables.push(var.name.clone());
                }
            }
        }

        // Substitute variables in URL
        let url = Self::substitute_variables(&template.url_template, variables);

        // Substitute variables in headers
        let headers: Vec<KeyValuePair> = template
            .headers
            .iter()
            .map(|h| KeyValuePair {
                key: h.key.clone(),
                value: Self::substitute_variables(&h.value, variables),
                enabled: h.enabled,
                description: h.description.clone(),
            })
            .collect();

        // Substitute variables in query params
        let query_params: Vec<KeyValuePair> = template
            .query_params
            .iter()
            .map(|p| KeyValuePair {
                key: p.key.clone(),
                value: Self::substitute_variables(&p.value, variables),
                enabled: p.enabled,
                description: p.description.clone(),
            })
            .collect();

        // Convert body template to actual body
        let body = match &template.body_type {
            crate::models::template::BodyTemplateType::None => BodyType::None,
            crate::models::template::BodyTemplateType::Raw { content, content_type } => {
                BodyType::Raw {
                    content: Self::substitute_variables(content, variables),
                    content_type: content_type.clone(),
                }
            }
            crate::models::template::BodyTemplateType::FormUrlEncoded(fields) => {
                BodyType::FormUrlEncoded(
                    fields
                        .iter()
                        .map(|f| KeyValuePair {
                            key: f.key.clone(),
                            value: Self::substitute_variables(&f.value, variables),
                            enabled: f.enabled,
                            description: f.description.clone(),
                        })
                        .collect(),
                )
            }
        };

        let request = crate::models::request::RequestDefinition {
            id: Uuid::new_v4(),
            name,
            method: template.method.clone(),
            url,
            headers,
            query_params,
            body,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        Ok(TemplateApplicationResult {
            request,
            missing_variables,
        })
    }

    /// Substitute variables in a string using {{var}} syntax
    fn substitute_variables(template: &str, variables: &HashMap<String, String>) -> String {
        let mut result = template.to_string();

        // First, use provided variables
        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }

    /// Validate that all required variables have values
    pub fn validate_template_variables(
        &self,
        template_id: &Uuid,
        variables: &HashMap<String, String>,
    ) -> Result<(), TemplateError> {
        let template = self.get_template(template_id)
            .ok_or(TemplateError::TemplateNotFound(*template_id))?;

        let missing: Vec<String> = template
            .variables
            .iter()
            .filter(|v| {
                v.required
                    && !variables.contains_key(&v.name)
                    && v.default_value.is_none()
            })
            .map(|v| v.name.clone())
            .collect();

        if !missing.is_empty() {
            Err(TemplateError::MissingVariables(missing))
        } else {
            Ok(())
        }
    }
}

/// Errors that can occur when working with templates
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    TemplateNotFound(Uuid),

    #[error("Missing required variables: {0:?}")]
    MissingVariables(Vec<String>),

    #[error("Cannot modify builtin template")]
    CannotModifyBuiltin,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::request::HttpMethod;
    use tempfile::TempDir;

    #[test]
    fn test_get_all_templates() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let templates = manager.get_all_templates();
        // Should have all builtin templates
        assert!(templates.len() >= 8);
    }

    #[test]
    fn test_get_template_by_id() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let templates = manager.get_all_templates();
        let first_template = templates.first().unwrap();
        let found = manager.get_template(&first_template.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, first_template.name);
    }

    #[test]
    fn test_get_template_by_name() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let template = manager.get_template_by_name("GET with Auth");
        assert!(template.is_some());
        assert_eq!(template.unwrap().method, HttpMethod::GET);
    }

    #[test]
    fn test_get_templates_by_category() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let grouped = manager.get_templates_by_category();
        assert!(grouped.contains_key(&TemplateCategory::Basic));
        assert!(grouped.contains_key(&TemplateCategory::Authentication));
        assert!(grouped.contains_key(&TemplateCategory::Api));
    }

    #[test]
    fn test_substitute_variables() {
        let result = TemplateManager::substitute_variables(
            "Hello {{name}}, your token is {{token}}",
            &HashMap::from([
                ("name".to_string(), "Alice".to_string()),
                ("token".to_string(), "abc123".to_string()),
            ]),
        );
        assert_eq!(result, "Hello Alice, your token is abc123");
    }

    #[test]
    fn test_substitute_variables_partial() {
        let result = TemplateManager::substitute_variables(
            "Hello {{name}}, your token is {{token}}",
            &HashMap::from([("name".to_string(), "Alice".to_string())]),
        );
        assert_eq!(result, "Hello Alice, your token is {{token}}");
    }

    #[test]
    fn test_create_from_template() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let template = manager.get_template_by_name("GET with Auth").unwrap();

        let variables = HashMap::from([
            ("base_url".to_string(), "https://api.example.com".to_string()),
            ("path".to_string(), "users".to_string()),
            ("access_token".to_string(), "token123".to_string()),
        ]);

        let result = manager
            .create_from_template(&template.id, &variables, "My Request".to_string())
            .unwrap();

        assert_eq!(result.request.name, "My Request");
        assert_eq!(result.request.url, "https://api.example.com/users");
        assert!(result
            .request
            .headers
            .iter()
            .any(|h| h.value == "Bearer token123"));
        assert!(result.missing_variables.is_empty());
    }

    #[test]
    fn test_create_from_template_with_missing_vars() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let template = manager.get_template_by_name("GET with Auth").unwrap();

        // Missing base_url and path which are required
        let variables = HashMap::from([(
            "access_token".to_string(),
            "token123".to_string(),
        )]);

        let result = manager
            .create_from_template(&template.id, &variables, "My Request".to_string())
            .unwrap();

        assert_eq!(result.missing_variables.len(), 2);
        assert!(result.missing_variables.contains(&"base_url".to_string()));
        assert!(result.missing_variables.contains(&"path".to_string()));
    }

    #[test]
    fn test_validate_template_variables() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let template = manager.get_template_by_name("GET with Auth").unwrap();

        let variables = HashMap::from([
            ("base_url".to_string(), "https://api.example.com".to_string()),
            ("path".to_string(), "users".to_string()),
            ("access_token".to_string(), "token123".to_string()),
        ]);

        assert!(manager
            .validate_template_variables(&template.id, &variables)
            .is_ok());
    }

    #[test]
    fn test_validate_template_variables_missing() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let template = manager.get_template_by_name("GET with Auth").unwrap();

        let variables = HashMap::new();

        let result = manager.validate_template_variables(&template.id, &variables);
        assert!(result.is_err());
        match result.unwrap_err() {
            TemplateError::MissingVariables(vars) => {
                assert!(vars.contains(&"base_url".to_string()));
                assert!(vars.contains(&"path".to_string()));
            }
            _ => panic!("Expected MissingVariables error"),
        }
    }

    #[test]
    fn test_add_custom_template() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = TemplateManager::new(temp_dir.path().to_path_buf());

        let custom_template = RequestTemplate::new(
            "Custom Template",
            "A custom template",
            TemplateCategory::Custom,
            HttpMethod::GET,
            "{{url}}",
        );

        manager.add_custom_template(custom_template).unwrap();
        assert_eq!(manager.custom_templates.len(), 1);
    }

    #[test]
    fn test_delete_custom_template() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = TemplateManager::new(temp_dir.path().to_path_buf());

        let custom_template = RequestTemplate::new(
            "Custom Template",
            "A custom template",
            TemplateCategory::Custom,
            HttpMethod::GET,
            "{{url}}",
        );

        manager.add_custom_template(custom_template.clone()).unwrap();
        assert_eq!(manager.custom_templates.len(), 1);

        manager.delete_custom_template(&custom_template.id).unwrap();
        assert_eq!(manager.custom_templates.len(), 0);
    }

    #[test]
    fn test_cannot_modify_builtin_template() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = TemplateManager::new(temp_dir.path().to_path_buf());
        let template = manager.get_template_by_name("GET with Auth").unwrap();
        let template_id = template.id;

        let result = manager.delete_custom_template(&template_id);
        assert!(matches!(result, Err(TemplateError::CannotModifyBuiltin)));
    }

    #[test]
    fn test_save_and_load_custom_templates() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = TemplateManager::new(temp_dir.path().to_path_buf());

        let custom_template = RequestTemplate::new(
            "Custom Template",
            "A custom template",
            TemplateCategory::Custom,
            HttpMethod::GET,
            "{{url}}",
        );

        manager.add_custom_template(custom_template.clone()).unwrap();
        manager.save_custom_templates().unwrap();

        let mut manager2 = TemplateManager::new(temp_dir.path().to_path_buf());
        manager2.load_custom_templates().unwrap();
        assert_eq!(manager2.custom_templates.len(), 1);
        assert_eq!(manager2.custom_templates[0].name, "Custom Template");
    }
}
