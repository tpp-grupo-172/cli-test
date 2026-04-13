use std::collections::{HashMap, HashSet};
use crate::analysis::FileAnalysis;

pub fn check(analyses: &[FileAnalysis]) -> Vec<String> {
    let mut violations = vec![];
    let mut seen: HashMap<String, HashSet<String>> = HashMap::new();

    for file in analyses {
        let file_name = file.path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if let Some(functions) = file.data.get("functions").and_then(|v| v.as_array()) {
            for func in functions {
                if let Some(name) = func.get("name").and_then(|v| v.as_str()) {
                    seen.entry(name.to_string()).or_default().insert(file_name.clone());
                }
            }
        }

        if let Some(classes) = file.data.get("classes").and_then(|v| v.as_array()) {
            for class in classes {
                if let Some(methods) = class.get("methods").and_then(|v| v.as_array()) {
                    for method in methods {
                        if let Some(name) = method.get("name").and_then(|v| v.as_str()) {
                            seen.entry(name.to_string()).or_default().insert(file_name.clone());
                        }
                    }
                }
            }
        }
    }

    for (name, files) in &seen {
        if files.len() > 1 {
            let mut file_list: Vec<&String> = files.iter().collect();
            file_list.sort();
            violations.push(format!(
                "[DUPLICATE NAME]   {:<25} defined in: {}",
                name,
                file_list.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }
    }

    violations
}