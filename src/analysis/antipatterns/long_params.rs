use serde_json::Value;

const MAX_PARAMS: usize = 5;

pub fn check(data: &Value, file_name: &str) -> Vec<String> {
    let mut violations = vec![];

    if let Some(functions) = data.get("functions").and_then(|v| v.as_array()) {
        for func in functions {
            if let Some(v) = check_one(func, file_name) {
                violations.push(v);
            }
        }
    }

    if let Some(classes) = data.get("classes").and_then(|v| v.as_array()) {
        for class in classes {
            if let Some(methods) = class.get("methods").and_then(|v| v.as_array()) {
                for method in methods {
                    if let Some(v) = check_one(method, file_name) {
                        violations.push(v);
                    }
                }
            }
        }
    }

    violations
}

fn check_one(func: &Value, file_name: &str) -> Option<String> {
    let name = func.get("name").and_then(|v| v.as_str())?;
    let params = func.get("parameters").and_then(|v| v.as_array())?;

    let count = params.iter()
        .filter(|p| {
            p.get("name")
                .and_then(|v| v.as_str())
                .map(|n| n != "self")
                .unwrap_or(true)
        })
        .count();

    if count > MAX_PARAMS {
        Some(format!(
            "[LONG PARAMS]      {:<25} {:<30} ({} parameters)",
            name, file_name, count
        ))
    } else {
        None
    }
}