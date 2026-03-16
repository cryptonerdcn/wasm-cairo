mod utils;
use std::collections::HashMap;
use std::path::Path;

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

use cairo_lang_starknet_classes::compiler_version::{current_compiler_version_id, current_sierra_version_id};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
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
/// Expected JSON format:
/// ```json
/// {
///   "project_name": "my_project",
///   "files": {
///     "lib.cairo": "mod utils;\nfn main() { ... }",
///     "utils.cairo": "fn helper() -> felt252 { 42 }"
///   },
///   "dependencies": {
///     "openzeppelin_token": {
///       "files": { "lib.cairo": "...", "erc20.cairo": "..." },
///       "edition": "2024_07",
///       "dependencies": ["openzeppelin_access"]
///     },
///     "openzeppelin_access": {
///       "files": { "lib.cairo": "..." }
///     }
///   }
/// }
/// ```
/// The `dependencies` field is optional. When absent, an empty HashMap is returned.
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

// ========== Version info ==========

#[wasm_bindgen(js_name = getCairoVersion)]
pub fn get_cairo_version() -> String {
    let compiler = current_compiler_version_id();
    let sierra = current_sierra_version_id();
    format!(
        "{{\"cairo\":\"{}\",\"sierra\":\"{}\"}}",
        compiler, sierra
    )
}

// ========== Single-file APIs (existing) ==========

#[wasm_bindgen]
pub fn greet(s: &str) -> String {
    utils::set_panic_hook();
    return format!("Hello {}!", s);
}

#[wasm_bindgen(js_name = compileCairoProgram)]
pub fn compile_cairo_program(cairo_program: String, replace_ids: bool) -> Result<String, JsError> {
    let sierra_program = compile_cairo_project_with_input_string(
        Path::new("./astro.cairo"),
        &cairo_program,
        CompilerConfig {
            replace_ids: replace_ids,
            ..CompilerConfig::default()
        },
    );
    let sierra_program_str = match sierra_program {
        Ok(sierra_program) => sierra_program.to_string(),
        Err(e) => {
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(sierra_program_str)
}

#[wasm_bindgen(js_name = runCairoProgram)]
pub fn run_cairo_program(
    cairo_program: String,
    available_gas: Option<usize>,
    allow_warnings: bool,
    print_full_memory: bool,
    run_profiler: bool,
    use_dbg_print_hint: bool,
) -> Result<String, JsError> {
    let cairo_program_result = run_with_input_program_string(
        &cairo_program,
        available_gas,
        allow_warnings,
        print_full_memory,
        run_profiler,
        use_dbg_print_hint,
    );
    let cairo_program_result_str = match cairo_program_result {
        Ok(cairo_program_result_str) => cairo_program_result_str,
        Err(e) => {
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(cairo_program_result_str)
}

#[wasm_bindgen(js_name = runTests)]
pub fn run_tests(
    cairo_program: String,
    allow_warnings: bool,
    filter: String,
    include_ignored: bool,
    ignored: bool,
    starknet: bool,
    run_profiler: String,
    gas_disabled: bool,
    print_resource_usage: bool,
) -> Result<String, JsError> {
    let test_results = run_tests_with_input_string_parsed(
        &cairo_program,
        allow_warnings,
        filter,
        include_ignored,
        ignored,
        starknet,
        run_profiler,
        gas_disabled,
        print_resource_usage,
    );
    let test_results_str = match test_results {
        Ok(test_results) => test_results.to_string(),
        Err(e) => {
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(test_results_str)
}

#[wasm_bindgen(js_name = compileStarknetContract)]
pub fn compile_starknet_contract(
    starknet_contract: String,
    allow_warnings: bool,
    replace_ids: bool,
    output_casm: bool,
) -> Result<String, JsError> {
    let sierra_contract = starknet_wasm_compile_with_input_string(
        &starknet_contract,
        allow_warnings,
        replace_ids,
        output_casm,
        None,
        None,
        None,
    );
    let sierra_contract_str = match sierra_contract {
        Ok(sierra_program) => sierra_program.to_string(),
        Err(e) => {
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(sierra_contract_str)
}

// ========== Multi-file project APIs (new) ==========

/// Compile a multi-file Cairo project to Sierra.
/// Input: JSON string with `project_name`, `files` map, and optional `dependencies`.
#[wasm_bindgen(js_name = compileCairoProject)]
pub fn compile_cairo_project(project_json: String, replace_ids: bool) -> Result<String, JsError> {
    let (project_name, files, dependencies) = parse_project_input(&project_json)
        .map_err(|e| JsError::new(&e))?;

    let sierra_program = compile_cairo_project_with_virtual_files_and_deps(
        &project_name,
        &files,
        &dependencies,
        CompilerConfig {
            replace_ids,
            ..CompilerConfig::default()
        },
    );
    let sierra_program_str = match sierra_program {
        Ok(sierra_program) => sierra_program.to_string(),
        Err(e) => {
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(sierra_program_str)
}

/// Run a multi-file Cairo project.
/// Input: JSON string with `project_name`, `files` map, and optional `dependencies`.
#[wasm_bindgen(js_name = runCairoProject)]
pub fn run_cairo_project(
    project_json: String,
    available_gas: Option<usize>,
    allow_warnings: bool,
    print_full_memory: bool,
    run_profiler: bool,
    use_dbg_print_hint: bool,
) -> Result<String, JsError> {
    let (project_name, files, dependencies) = parse_project_input(&project_json)
        .map_err(|e| JsError::new(&e))?;

    let result = run_with_virtual_project_and_deps(
        &project_name,
        &files,
        &dependencies,
        available_gas,
        allow_warnings,
        print_full_memory,
        run_profiler,
        use_dbg_print_hint,
    );
    let result_str = match result {
        Ok(s) => s,
        Err(e) => {
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(result_str)
}

/// Compile a multi-file Starknet contract project.
/// Input: JSON string with `project_name`, `files` map, and optional `dependencies`.
/// When `output_casm` is true, returns JSON with both `sierra` and `casm` fields.
/// When false, returns Sierra ContractClass JSON only (backward compatible).
#[wasm_bindgen(js_name = compileStarknetProject)]
pub fn compile_starknet_project(
    project_json: String,
    allow_warnings: bool,
    replace_ids: bool,
    output_casm: bool,
) -> Result<String, JsError> {
    let (project_name, files, dependencies) = parse_project_input(&project_json)
        .map_err(|e| JsError::new(&e))?;

    let result = starknet_wasm_compile_with_virtual_files_and_deps(
        &project_name,
        &files,
        &dependencies,
        allow_warnings,
        replace_ids,
        output_casm,
        None,
        None,
        None,
    );
    let result_str = match result {
        Ok(s) => s,
        Err(e) => {
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(result_str)
}

/// Run tests in a multi-file Cairo project.
/// Input: JSON string with `project_name`, `files` map, and optional `dependencies`.
#[wasm_bindgen(js_name = runProjectTests)]
pub fn run_project_tests(
    project_json: String,
    allow_warnings: bool,
    filter: String,
    include_ignored: bool,
    ignored: bool,
    starknet: bool,
    gas_disabled: bool,
    print_resource_usage: bool,
) -> Result<String, JsError> {
    let (project_name, files, dependencies) = parse_project_input(&project_json)
        .map_err(|e| JsError::new(&e))?;

    let result = run_tests_with_virtual_project_and_deps_parsed(
        &project_name,
        &files,
        &dependencies,
        allow_warnings,
        filter,
        include_ignored,
        ignored,
        starknet,
        gas_disabled,
        print_resource_usage,
    );
    let result_str = match result {
        Ok(s) => s,
        Err(e) => {
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(result_str)
}
