mod long_function;
mod long_params;
mod duplicate_functions;
mod god_class;

use std::path::Path;
use crate::config::Config;
use crate::analysis::analyze_project;

pub fn run(path: &Path, config: &Config) {
    let analyses = analyze_project(path);
    let mut found = false;

    for file in &analyses {
        let file_name = file.path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let antipatterns: Vec<String> = [
            long_function::check(&file.data, &file_name, &config.long_function),
            long_params::check(&file.data, &file_name, &config.long_params),
        ]
        .into_iter()
        .flatten()
        .collect();

        for antipattern in &antipatterns {
            println!("{}", antipattern);
            found = true;
        }
    }

    for v in duplicate_functions::check(&analyses, &config.duplicate_functions) {
        println!("{}", v);
        found = true;
    }

    for v in god_class::check(&analyses, &config.god_class) {
        println!("{}", v);
        found = true;
    }

    if !found {
        println!("No antipatterns detected.");
        std::process::exit(0);
    }

    std::process::exit(1);
}