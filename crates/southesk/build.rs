// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use std::{
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
enum ApiFunctionsState {
    Before,
    In,
    After,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    update_api_functions_in_readme()
}

fn update_api_functions_in_readme() -> Result<(), Box<dyn std::error::Error>> {
    let api_code_file = "src/client/connected.rs";
    let readme_path = "../../README.md";
    println!("cargo::rerun-if-changed={api_code_file}");

    let start = "// BUILD: HIGH-LEVEL START";
    let end = "// BUILD: HIGH-LEVEL END";
    let readme_start = "<!-- BUILD: HIGH-LEVEL START -->";
    let readme_end = "<!-- BUILD: HIGH-LEVEL END -->";
    let func_prefix = "    pub async fn ";
    let mut file = BufReader::new(File::open(api_code_file)?);
    let mut read_state = ApiFunctionsState::Before;
    let mut func_names = Vec::new();
    let line = &mut String::new();
    while file.read_line(line)? > 0 {
        match read_state {
            ApiFunctionsState::Before => {
                if line.contains(start) {
                    read_state = ApiFunctionsState::In;
                }
            }
            ApiFunctionsState::In => {
                if line.contains(end) {
                    read_state = ApiFunctionsState::After;
                    continue;
                }

                if line.starts_with(func_prefix) {
                    let func_name = &line[func_prefix.len()
                        ..line
                            .find('(')
                            .expect("function should have argument list after name")];
                    func_names.push(func_name.to_owned());
                }
            }
            ApiFunctionsState::After => {
                break;
            }
        }
        line.clear();
    }
    assert_eq!(read_state, ApiFunctionsState::After);

    let old_readme = std::fs::read_to_string(readme_path)?;
    let mut new_readme = String::new();
    let mut write_state = ApiFunctionsState::Before;
    for line in old_readme.lines() {
        match write_state {
            ApiFunctionsState::Before => {
                new_readme.push_str(line);
                new_readme.push('\n');

                if line == readme_start {
                    for func_name in &func_names {
                        writeln!(
                            new_readme,
                            "* [{func_name}](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.{func_name})"
                        )?;
                    }
                    write_state = ApiFunctionsState::In;
                }
            }
            ApiFunctionsState::In => {
                if line == readme_end {
                    new_readme.push_str(line);
                    new_readme.push('\n');
                    write_state = ApiFunctionsState::After;
                }
            }
            ApiFunctionsState::After => {
                new_readme.push_str(line);
                new_readme.push('\n');
            }
        }
    }
    assert_eq!(write_state, ApiFunctionsState::After);
    if new_readme != old_readme {
        std::fs::write(readme_path, new_readme)?;
    }

    Ok(())
}
