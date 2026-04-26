use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub long_function: LongFunctionConfig,
    pub long_params: LongParamsConfig,
    pub god_class: GodClassConfig,
    pub duplicate_functions: DuplicateFunctionsConfig,
}

#[derive(Deserialize)]
pub struct LongFunctionConfig {
    pub max_lines: usize,
}

#[derive(Deserialize)]
pub struct LongParamsConfig {
    pub max_params: usize,
}

#[derive(Deserialize)]
pub struct GodClassConfig {
    pub flag_threshold: f32,
    pub method_count_norm: f32,
    pub distinct_imports_norm: f32,
    pub total_lines_norm: f32,
    pub weight_method_count: f32,
    pub weight_distinct_imports: f32,
    pub weight_total_lines: f32,
    pub weight_name: f32,
    pub god_names: Vec<String>,
}

#[derive(Deserialize)]
pub struct DuplicateFunctionsConfig {
    pub ignored_names: Vec<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Could not read config file: {}", e))?;
        toml::from_str(&content)
            .map_err(|e| format!("Could not parse config file: {}", e))
    }

    pub fn load_default() -> Self {
        Self {
            long_function: LongFunctionConfig { max_lines: 30 },
            long_params: LongParamsConfig { max_params: 5 },
            god_class: GodClassConfig {
                flag_threshold: 0.5,
                method_count_norm: 8.0,
                distinct_imports_norm: 4.0,
                total_lines_norm: 150.0,
                weight_method_count: 0.50,
                weight_distinct_imports: 0.15,
                weight_total_lines: 0.30,
                weight_name: 0.05,
                god_names: vec![
                    "manager", "coordinator", "handler", "controller",
                    "processor", "helper", "utils", "utility", "service"
                ].into_iter().map(String::from).collect(),
            },
            duplicate_functions: DuplicateFunctionsConfig {
                ignored_names: vec![
                    "__init__", "__str__", "__repr__", "__len__", "__eq__",
                    "__hash__", "__del__", "__enter__", "__exit__", "__iter__",
                    "__next__", "__call__", "constructor", "toString", "valueOf",
                    "render", "run", "main", "execute", "start", "stop",
                    "init", "setup", "teardown"
                ].into_iter().map(String::from).collect(),
            },
        }
    }
}