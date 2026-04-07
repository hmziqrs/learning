use regex::Regex;
use std::collections::HashMap;
use std::borrow::Cow;
use crate::models::request::{RequestDefinition, BodyType, KeyValuePair};

pub struct Interpolator;

impl Interpolator {
    /// Resolve all `{{var}}` placeholders in a request definition,
    /// returning a new owned copy with concrete values.
    pub fn resolve(
        req: &RequestDefinition,
        vars: &HashMap<String, String>,
    ) -> RequestDefinition {
        let mut resolved = req.clone();
        resolved.url = Self::replace(&resolved.url, vars).into_owned();
        Self::resolve_pairs(&mut resolved.headers, vars);
        Self::resolve_pairs(&mut resolved.query_params, vars);
        resolved.body = match &resolved.body {
            BodyType::None => BodyType::None,
            BodyType::Raw { content, content_type } => BodyType::Raw {
                content: Self::replace(content, vars).into_owned(),
                content_type: content_type.clone(),
            },
            BodyType::FormUrlEncoded(pairs) => {
                let mut p = pairs.clone();
                Self::resolve_pairs(&mut p, vars);
                BodyType::FormUrlEncoded(p)
            }
        };
        resolved
    }

    /// Replace `{{var}}` placeholders with values from vars.
    /// Returns `Cow<'a, str>` — borrows input when no substitutions needed,
    /// owns an allocated String only when interpolation occurs.
    fn replace<'a>(input: &'a str, vars: &HashMap<String, String>) -> Cow<'a, str> {
        // Fast-path: no placeholders = zero allocation
        if !input.contains("{{") {
            return Cow::Borrowed(input);
        }

        // Slow-path: regex substitution
        let re = Regex::new(r"\{\{(\w+)\}\}").unwrap(); // compile once in real code
        Cow::Owned(re.replace_all(input, |caps: &regex::Captures| {
            let key = &caps[1];
            vars.get(key).cloned().unwrap_or_else(|| format!("{{{{{}}}}}", key))
        }).to_string())
    }

    fn resolve_pairs(pairs: &mut Vec<KeyValuePair>, vars: &HashMap<String, String>) {
        for pair in pairs.iter_mut() {
            pair.key = Self::replace(&pair.key, vars).into_owned();
            pair.value = Self::replace(&pair.value, vars).into_owned();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_interpolation() {
        let mut vars = HashMap::new();
        vars.insert("base_url".into(), "https://api.example.com".into());
        vars.insert("token".into(), "abc123".into());

        let result = Interpolator::replace("{{base_url}}/users?t={{token}}", &vars);
        assert_eq!(result, "https://api.example.com/users?t=abc123");
    }

    #[test]
    fn test_missing_var_preserved() {
        let vars = HashMap::new();
        let result = Interpolator::replace("{{missing}}", &vars);
        assert_eq!(result, "{{missing}}");
    }

    #[test]
    fn test_nested_variable_interpolation() {
        let mut vars = HashMap::new();
        vars.insert("base_url".into(), "https://api.example.com".into());
        vars.insert("path".into(), "v1/users".into());
        vars.insert("endpoint".into(), "profile".into());

        let result = Interpolator::replace("{{base_url}}/{{path}}/{{endpoint}}", &vars);
        assert_eq!(result, "https://api.example.com/v1/users/profile");
    }

    #[test]
    fn test_empty_string_variables() {
        let mut vars = HashMap::new();
        vars.insert("empty_var".into(), "".into());
        vars.insert("normal_var".into(), "value".into());

        let result = Interpolator::replace("start {{empty_var}} middle {{normal_var}} end", &vars);
        assert_eq!(result, "start  middle value end");
    }

    #[test]
    fn test_special_characters_in_variable_values() {
        let mut vars = HashMap::new();
        vars.insert("url".into(), "https://example.com/path?query=value&other=123".into());
        vars.insert("json".into(), r#"{"key": "value", "special": "!@#$%^&*()"}"#.into());
        vars.insert("unicode".into(), "Hello 世界 🌍".into());

        let result = Interpolator::replace(
            "{{url}}/{{json}}/{{unicode}}",
            &vars
        );
        assert_eq!(
            result,
            "https://example.com/path?query=value&other=123/{\"key\": \"value\", \"special\": \"!@#$%^&*()\"}/Hello 世界 🌍"
        );
    }

    #[test]
    fn test_multiple_occurrences_of_same_variable() {
        let mut vars = HashMap::new();
        vars.insert("api_key".into(), "secret123".into());

        let result = Interpolator::replace(
            "Authorization: Bearer {{api_key}}, api_key={{api_key}}, second={{api_key}}",
            &vars
        );
        assert_eq!(
            result,
            "Authorization: Bearer secret123, api_key=secret123, second=secret123"
        );
    }

    #[test]
    fn test_variables_with_underscores_and_numbers() {
        let mut vars = HashMap::new();
        vars.insert("api_v2_endpoint".into(), "https://api.v2.example.com".into());
        vars.insert("user_id_123".into(), "user456".into());
        vars.insert("v1_token".into(), "token_abc_123".into());
        vars.insert("test_123_var".into(), "value".into());

        let result = Interpolator::replace(
            "{{api_v2_endpoint}}/users/{{user_id_123}}/token?v={{v1_token}}&test={{test_123_var}}",
            &vars
        );
        assert_eq!(
            result,
            "https://api.v2.example.com/users/user456/token?v=token_abc_123&test=value"
        );
    }

    #[test]
    fn test_mixed_edge_cases() {
        let mut vars = HashMap::new();
        vars.insert("base".into(), "https://test.com".into());
        vars.insert("empty".into(), "".into());
        vars.insert("special".into(), "a/b?c=d&e=f".into());
        vars.insert("multiple".into(), "value".into());

        let result = Interpolator::replace(
            "{{base}}/{{empty}}/{{special}}/{{multiple}}/{{multiple}}",
            &vars
        );
        assert_eq!(
            result,
            "https://test.com//a/b?c=d&e=f/value/value"
        );
    }

    #[test]
    fn test_no_variables() {
        let vars = HashMap::new();
        let input = "This is a plain string with no variables";
        let result = Interpolator::replace(input, &vars);
        assert_eq!(result, input);
    }

    #[test]
    fn test_partial_variables() {
        let mut vars = HashMap::new();
        vars.insert("existing".into(), "value".into());

        let result = Interpolator::replace(
            "This has {{existing}} and {{missing}} variables",
            &vars
        );
        assert_eq!(result, "This has value and {{missing}} variables");
    }

    #[test]
    fn test_variables_in_different_positions() {
        let mut vars = HashMap::new();
        vars.insert("var1".into(), "value1".into());
        vars.insert("var2".into(), "value2".into());

        let result = Interpolator::replace(
            "{{var1}} start {{var2}} middle {{var1}} end {{var2}}",
            &vars
        );
        assert_eq!(result, "value1 start value2 middle value1 end value2");
    }
}
