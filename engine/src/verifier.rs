use crate::types::*;
use serde_json;

pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self
    }

    pub fn verify_step(
        &self,
        step: &Step,
        extracted_data: Option<&serde_json::Value>,
        dom_hash: &str,
    ) -> VerificationResult {
        let mut checks = Vec::new();

        for verification_type in &step.verification {
            let check_result = match verification_type {
                VerificationType::Schema => {
                    self.verify_schema(step, extracted_data)
                }
                VerificationType::SanityCheck => {
                    self.verify_sanity_check(extracted_data)
                }
                VerificationType::ElementPresence => {
                    self.verify_element_presence(step, dom_hash)
                }
                VerificationType::NumericRange => {
                    self.verify_numeric_range(step, extracted_data)
                }
            };
            checks.push(check_result);
        }

        let passed = checks.iter().all(|c| c.passed);

        VerificationResult { passed, checks }
    }

    fn verify_schema(
        &self,
        step: &Step,
        extracted_data: Option<&serde_json::Value>,
    ) -> CheckResult {
        if let Some(expected_schema) = &step.expected_schema {
            if let Some(data) = extracted_data {
                // Simple schema validation - in production, use a proper JSON schema validator
                if self.matches_schema(data, expected_schema) {
                    CheckResult {
                        check_type: "schema".to_string(),
                        passed: true,
                        message: Some("Schema validation passed".to_string()),
                    }
                } else {
                    CheckResult {
                        check_type: "schema".to_string(),
                        passed: false,
                        message: Some("Schema validation failed".to_string()),
                    }
                }
            } else {
                CheckResult {
                    check_type: "schema".to_string(),
                    passed: false,
                    message: Some("No data to validate".to_string()),
                }
            }
        } else {
            CheckResult {
                check_type: "schema".to_string(),
                passed: true,
                message: Some("No schema defined, skipping".to_string()),
            }
        }
    }

    fn verify_sanity_check(&self, extracted_data: Option<&serde_json::Value>) -> CheckResult {
        if let Some(data) = extracted_data {
            // Basic sanity checks
            if data.is_null() {
                return CheckResult {
                    check_type: "sanity_check".to_string(),
                    passed: false,
                    message: Some("Data is null".to_string()),
                };
            }

            if let Some(obj) = data.as_object() {
                if obj.is_empty() {
                    return CheckResult {
                        check_type: "sanity_check".to_string(),
                        passed: false,
                        message: Some("Data object is empty".to_string()),
                    };
                }
            }

            CheckResult {
                check_type: "sanity_check".to_string(),
                passed: true,
                message: Some("Sanity check passed".to_string()),
            }
        } else {
            CheckResult {
                check_type: "sanity_check".to_string(),
                passed: false,
                message: Some("No data to check".to_string()),
            }
        }
    }

    fn verify_element_presence(&self, _step: &Step, _dom_hash: &str) -> CheckResult {
        // In a real implementation, this would check if the element exists in the DOM
        // For now, we assume presence is verified by the step executor
        CheckResult {
            check_type: "element_presence".to_string(),
            passed: true,
            message: Some("Element presence verified by executor".to_string()),
        }
    }

    fn verify_numeric_range(
        &self,
        step: &Step,
        extracted_data: Option<&serde_json::Value>,
    ) -> CheckResult {
        if let Some(data) = extracted_data {
            if let Some(num) = data.as_f64() {
                // Check if parameters contain range constraints
                if let Some(params) = &step.parameters {
                    if let Some(min) = params.get("min_value").and_then(|v| v.as_f64()) {
                        if num < min {
                            return CheckResult {
                                check_type: "numeric_range".to_string(),
                                passed: false,
                                message: Some(format!("Value {} is below minimum {}", num, min)),
                            };
                        }
                    }
                    if let Some(max) = params.get("max_value").and_then(|v| v.as_f64()) {
                        if num > max {
                            return CheckResult {
                                check_type: "numeric_range".to_string(),
                                passed: false,
                                message: Some(format!("Value {} is above maximum {}", num, max)),
                            };
                        }
                    }
                }
                CheckResult {
                    check_type: "numeric_range".to_string(),
                    passed: true,
                    message: Some("Numeric range check passed".to_string()),
                }
            } else {
                CheckResult {
                    check_type: "numeric_range".to_string(),
                    passed: true,
                    message: Some("Not a numeric value, skipping range check".to_string()),
                }
            }
        } else {
            CheckResult {
                check_type: "numeric_range".to_string(),
                passed: false,
                message: Some("No data to check".to_string()),
            }
        }
    }

    fn matches_schema(&self, data: &serde_json::Value, schema: &serde_json::Value) -> bool {
        // Simplified schema matching - in production, use a proper JSON schema validator
        match (data, schema) {
            (serde_json::Value::Object(data_obj), serde_json::Value::Object(schema_obj)) => {
                // Check if all required keys from schema exist in data
                for (key, _) in schema_obj {
                    if !data_obj.contains_key(key) {
                        return false;
                    }
                }
                true
            }
            (serde_json::Value::Array(data_arr), serde_json::Value::Array(schema_arr)) => {
                // For arrays, check if structure matches
                data_arr.len() == schema_arr.len()
            }
            _ => {
                // Simple type check - both are same JSON value type
                matches!(
                    (data, schema),
                    (serde_json::Value::Null, serde_json::Value::Null) |
                    (serde_json::Value::Bool(_), serde_json::Value::Bool(_)) |
                    (serde_json::Value::Number(_), serde_json::Value::Number(_)) |
                    (serde_json::Value::String(_), serde_json::Value::String(_)) |
                    (serde_json::Value::Array(_), serde_json::Value::Array(_)) |
                    (serde_json::Value::Object(_), serde_json::Value::Object(_))
                )
            }
        }
    }
}

