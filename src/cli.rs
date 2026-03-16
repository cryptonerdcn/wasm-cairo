use std::collections::HashMap;
use std::path::Path;

use anyhow::Error;
use clap::Parser;

use cairo_lang_compiler::{
    wasm_cairo_interface::{
        compile_cairo_project_with_input_string,
        compile_cairo_project_with_virtual_files_and_deps,
        DependencyInput,
    },
    CompilerConfig,
};
use cairo_lang_runner::wasm_cairo_interface::{
    run_with_input_program_string,
    run_with_virtual_project_and_deps,
};
use cairo_lang_starknet::wasm_cairo_interface::{
    starknet_wasm_compile_with_input_string,
    starknet_wasm_compile_with_virtual_files_and_deps,
};
use cairo_lang_test_runner::wasm_cairo_interface::{
    run_tests_with_input_string_parsed,
    run_tests_with_virtual_project_and_deps_parsed,
};

/// Command line args parser.
/// Exits with 0/1 if the input is formatted correctly/incorrectly.
#[derive(Parser, Debug)]
#[clap(version, verbatim_doc_comment)]
struct Args {
    #[arg(long)]
    command: String,
    /// Whether to print the memory.
    #[arg(long, default_value_t = true)]
    print_full_memory: bool,
    #[arg(long, default_value_t = true)]
    use_dbg_print_hint: bool,
    /// Input cairo program string (single-file mode)
    #[arg(long)]
    input_program_string: Option<String>,
    /// Input project JSON string (multi-file mode)
    /// Format: {"project_name": "...", "files": {"lib.cairo": "...", ...}}
    #[arg(long)]
    project_json: Option<String>,
}

/// Parse a JSON object's files field into a HashMap<String, String>.
fn parse_files_map(value: &serde_json::Value, context: &str) -> Result<HashMap<String, String>, String> {
    let files_obj = value.get("files")
        .and_then(|v| v.as_object())
        .ok_or_else(|| format!("Missing 'files' object in {}", context))?;

    let mut files = HashMap::new();
    for (path, content) in files_obj {
        let content_str = content.as_str()
            .ok_or_else(|| format!("File content for '{}' is not a string in {}", path, context))?;
        files.insert(path.clone(), content_str.to_string());
    }
    Ok(files)
}

/// Parse a JSON string into a project input (project_name + files map + dependencies).
fn parse_project_input(json_str: &str) -> Result<(String, HashMap<String, String>, HashMap<String, DependencyInput>), String> {
    let value: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    let project_name = value.get("project_name")
        .and_then(|v| v.as_str())
        .unwrap_or("project")
        .to_string();

    let files = parse_files_map(&value, "project JSON")?;

    let mut dependencies = HashMap::new();
    if let Some(deps_obj) = value.get("dependencies").and_then(|v| v.as_object()) {
        for (dep_name, dep_value) in deps_obj {
            let dep_files = parse_files_map(dep_value, &format!("dependency '{}'", dep_name))?;

            let edition = dep_value.get("edition")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let dep_deps = if let Some(arr) = dep_value.get("dependencies").and_then(|v| v.as_array()) {
                let mut dep_dep_names = Vec::new();
                for item in arr {
                    let name = item.as_str()
                        .ok_or_else(|| format!("Dependency name in '{}' dependencies array is not a string", dep_name))?;
                    dep_dep_names.push(name.to_string());
                }
                dep_dep_names
            } else {
                Vec::new()
            };

            dependencies.insert(dep_name.clone(), DependencyInput {
                files: dep_files,
                edition,
                dependencies: dep_deps,
            });
        }
    }

    Ok((project_name, files, dependencies))
}

pub fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let command = args.command;
    match command.as_ref() {
        // Single-file commands
        "compileCairoProgram" => {
            let result = compile_cairo_program(args.input_program_string.unwrap(), true);
            println!("{}", result.unwrap());
        }
        "runCairoProgram" => {
            let result = run_cairo_program(args.input_program_string.unwrap(), None, true, true, false, true);
            println!("{}", result.unwrap());
        }
        "runTests" => {
            let result = run_tests(args.input_program_string.unwrap());
            println!("{}", result.unwrap());
        }
        "compileStarknetContract" => {
            let result = compile_starknet_contract(args.input_program_string.unwrap(), true, true);
            println!("{}", result.unwrap());
        }
        // Multi-file project commands
        "compileCairoProject" => {
            let json = args.project_json.expect("--project-json is required for compileCairoProject");
            let result = compile_cairo_project(json, true);
            println!("{}", result.unwrap());
        }
        "runCairoProject" => {
            let json = args.project_json.expect("--project-json is required for runCairoProject");
            let result = run_cairo_project(json, None, true, true, false, true);
            println!("{}", result.unwrap());
        }
        "compileStarknetProject" => {
            let json = args.project_json.expect("--project-json is required for compileStarknetProject");
            let result = compile_starknet_project(json, true, true);
            println!("{}", result.unwrap());
        }
        "runProjectTests" => {
            let json = args.project_json.expect("--project-json is required for runProjectTests");
            let result = run_project_tests(json);
            println!("{}", result.unwrap());
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }

    Ok(())
}

// ========== Single-file functions ==========

fn compile_cairo_program(cairo_program: String, replace_ids: bool) -> Result<String, Error> {
    let sierra_program = compile_cairo_project_with_input_string(
        Path::new("./astro.cairo"),
        &cairo_program,
        CompilerConfig {
            replace_ids,
            ..CompilerConfig::default()
        },
    );
    match sierra_program {
        Ok(p) => Ok(p.to_string()),
        Err(e) => Ok(e.to_string()),
    }
}

fn run_cairo_program(
    cairo_program: String,
    available_gas: Option<usize>,
    allow_warnings: bool,
    print_full_memory: bool,
    run_profiler: bool,
    use_dbg_print_hint: bool,
) -> Result<String, Error> {
    match run_with_input_program_string(
        &cairo_program, available_gas, allow_warnings, print_full_memory, run_profiler, use_dbg_print_hint,
    ) {
        Ok(s) => Ok(s),
        Err(e) => Ok(e.to_string()),
    }
}

fn compile_starknet_contract(
    starknet_contract: String,
    allow_warnings: bool,
    replace_ids: bool,
) -> Result<String, Error> {
    match starknet_wasm_compile_with_input_string(&starknet_contract, allow_warnings, replace_ids, false, None, None, None) {
        Ok(s) => Ok(s),
        Err(e) => Ok(e.to_string()),
    }
}

fn run_tests(input_string: String) -> Result<String, Error> {
    match run_tests_with_input_string_parsed(
        &input_string, false, "".to_string(), false, false, false, "".to_string(), false, false,
    ) {
        Ok(s) => Ok(s),
        Err(e) => Ok(e.to_string()),
    }
}

// ========== Multi-file project functions ==========

fn compile_cairo_project(project_json: String, replace_ids: bool) -> Result<String, Error> {
    let (project_name, files, dependencies) = parse_project_input(&project_json)
        .map_err(|e| anyhow::anyhow!(e))?;
    match compile_cairo_project_with_virtual_files_and_deps(
        &project_name, &files, &dependencies, CompilerConfig { replace_ids, ..CompilerConfig::default() },
    ) {
        Ok(p) => Ok(p.to_string()),
        Err(e) => Ok(e.to_string()),
    }
}

fn run_cairo_project(
    project_json: String,
    available_gas: Option<usize>,
    allow_warnings: bool,
    print_full_memory: bool,
    run_profiler: bool,
    use_dbg_print_hint: bool,
) -> Result<String, Error> {
    let (project_name, files, dependencies) = parse_project_input(&project_json)
        .map_err(|e| anyhow::anyhow!(e))?;
    match run_with_virtual_project_and_deps(
        &project_name, &files, &dependencies, available_gas, allow_warnings, print_full_memory, run_profiler, use_dbg_print_hint,
    ) {
        Ok(s) => Ok(s),
        Err(e) => Ok(e.to_string()),
    }
}

fn compile_starknet_project(
    project_json: String,
    allow_warnings: bool,
    replace_ids: bool,
) -> Result<String, Error> {
    let (project_name, files, dependencies) = parse_project_input(&project_json)
        .map_err(|e| anyhow::anyhow!(e))?;
    match starknet_wasm_compile_with_virtual_files_and_deps(
        &project_name, &files, &dependencies, allow_warnings, replace_ids, false, None, None, None,
    ) {
        Ok(s) => Ok(s),
        Err(e) => Ok(e.to_string()),
    }
}

fn run_project_tests(project_json: String) -> Result<String, Error> {
    let (project_name, files, dependencies) = parse_project_input(&project_json)
        .map_err(|e| anyhow::anyhow!(e))?;
    match run_tests_with_virtual_project_and_deps_parsed(
        &project_name, &files, &dependencies, false, "".to_string(), false, false, false, false, false,
    ) {
        Ok(s) => Ok(s),
        Err(e) => Ok(e.to_string()),
    }
}