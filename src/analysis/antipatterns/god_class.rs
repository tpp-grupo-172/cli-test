use serde_json::Value;
use std::collections::HashSet;
use crate::analysis::FileAnalysis;

pub struct GodClassResult {
    pub file_name: String,
    pub class_name: String,
    pub score: f32,
    pub method_count: usize,
    pub distinct_imports: usize,
    pub total_lines: usize,
}

const METHOD_COUNT_NORM: f32 = 8.0;
const DISTINCT_IMPORTS_NORM: f32 = 4.0;
const TOTAL_LINES_NORM: f32 = 150.0;
const FLAG_THRESHOLD: f32 = 0.5;

const GOD_NAMES: &[&str] = &[
    "manager", "coordinator", "handler", "controller",
    "processor", "helper", "utils", "utility", "service"
];

// weights must sum to 1.0
const W_METHOD_COUNT: f32 = 0.50;
const W_DISTINCT_IMPORTS: f32 = 0.15;
const W_TOTAL_LINES: f32 = 0.30;
const W_NAME: f32 = 0.05;

pub fn check(analyses: &[FileAnalysis]) -> Vec<String> {
    let mut violations = vec![];

    for file in analyses {
        let file_name = file.path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if let Some(classes) = file.data.get("classes").and_then(|v| v.as_array()) {
            for class in classes {
                if let Some(result) = check_one(class, &file_name) {
                    violations.push(format!(
                        "[GOD CLASS]        {:<25} defined in: {:<30} (score: {:.2}, methods: {}, imports: {}, lines: {})",
                        result.class_name,
                        result.file_name,
                        result.score,
                        result.method_count,
                        result.distinct_imports,
                        result.total_lines,
                    ));
                }
            }
        }
    }

    violations
}

fn check_one(class: &Value, file_name: &str) -> Option<GodClassResult> {
    let class_name = class.get("name").and_then(|v| v.as_str())?.to_string();
    let methods = class.get("methods").and_then(|v| v.as_array())?;

    let method_count = methods.len();
    let distinct_imports = count_distinct_imports(methods);
    let total_lines = sum_method_lines(methods);

    let score = compute_score(
        method_count,
        distinct_imports,
        total_lines,
        &class_name,
    );

    if score >= FLAG_THRESHOLD {
        Some(GodClassResult {
            file_name: file_name.to_string(),
            class_name,
            score,
            method_count,
            distinct_imports,
            total_lines,
        })
    } else {
        None
    }
}

fn compute_score(
    method_count: usize,
    distinct_imports: usize,
    total_lines: usize,
    class_name: &str,
) -> f32 {
    let method_score = (method_count as f32 / METHOD_COUNT_NORM).min(1.0);
    let imports_score = (distinct_imports as f32 / DISTINCT_IMPORTS_NORM).min(1.0);
    let lines_score = (total_lines as f32 / TOTAL_LINES_NORM).min(1.0);
    let name_score = if has_god_name(class_name) { 1.0 } else { 0.0 };

    W_METHOD_COUNT * method_score
        + W_DISTINCT_IMPORTS * imports_score
        + W_TOTAL_LINES * lines_score
        + W_NAME * name_score
}

fn count_distinct_imports(methods: &[Value]) -> usize {
    let mut seen = HashSet::new();
    for method in methods {
        if let Some(calls) = method.get("function_calls").and_then(|v| v.as_array()) {
            for call in calls {
                if let Some(import_name) = call.get("import_name").and_then(|v| v.as_str()) {
                    if !import_name.is_empty() {
                        seen.insert(import_name.to_string());
                    }
                }
            }
        }
    }
    seen.len()
}

fn sum_method_lines(methods: &[Value]) -> usize {
    methods.iter().map(|m| {
        let start = m.get("line").and_then(|v| v.as_u64()).unwrap_or(0);
        let end = m.get("end_line").and_then(|v| v.as_u64()).unwrap_or(0);
        end.saturating_sub(start) as usize
    }).sum()
}

fn has_god_name(class_name: &str) -> bool {
    let lower = class_name.to_lowercase();
    GOD_NAMES.iter().any(|n| lower.contains(n))
}