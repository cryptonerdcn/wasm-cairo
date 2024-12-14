mod utils;
use std::path::Path;

use cairo_lang_compiler::{
    wasm_cairo_interface::compile_cairo_project_with_input_string, CompilerConfig,
};
use cairo_lang_runner::wasm_cairo_interface::{run_with_input_program_string, run_with_sierra_json_string};
use cairo_lang_starknet::wasm_cairo_interface::starknet_wasm_compile_with_input_string;
use cairo_lang_test_runner::wasm_cairo_interface::run_tests_with_input_string_parsed;

use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, ArrayBuffer};

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

#[wasm_bindgen]
pub fn greet(s: &str) -> String {
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

#[wasm_bindgen(js_name = runSierraProgram)]
pub fn run_sierra_program(
    sierra_program: String,
    available_gas: Option<usize>,
    print_full_memory: bool,
    run_profiler: bool,
    use_dbg_print_hint: bool,
) -> Result<String, JsError> {
    let gas = match available_gas {
        Some(0) => None,
        other => other
    };
    let sierra_program_result = run_with_sierra_json_string(&sierra_program, gas, print_full_memory, run_profiler, use_dbg_print_hint);
    let result_str = match sierra_program_result {
        Ok(result) => result,
        Err(e) => e.to_string()
    };
    Ok(result_str)
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
) -> Result<String, JsError> {
    let sierra_contract = starknet_wasm_compile_with_input_string(
        &starknet_contract,
        allow_warnings,
        replace_ids,
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

#[wasm_bindgen(js_name = compileCairoProgramFromFile)]
pub fn compile_cairo_program_from_file(array_buffer: ArrayBuffer, replace_ids: bool) -> Result<String, JsError> {
    let uint8_array = Uint8Array::new(&array_buffer);
    let len = uint8_array.length() as usize;
    let mut bytes = vec![0; len];
    uint8_array.copy_to(&mut bytes);
    
    match String::from_utf8(bytes) {
        Ok(content) => {
            log("File content converted to string successfully");
            compile_cairo_program(content, replace_ids)
        },
        Err(e) => {
            let error_msg = format!("Failed to decode content: {}", e);
            log(&error_msg);
            Err(JsError::new(&error_msg))
        }
    }
}

#[wasm_bindgen(js_name = testFileRead)]
pub fn test_file_read(array_buffer: ArrayBuffer) -> Result<String, JsError> {
    let uint8_array = Uint8Array::new(&array_buffer);
    let len = uint8_array.length() as usize;
    let mut bytes = vec![0; len];
    uint8_array.copy_to(&mut bytes);
    
    match String::from_utf8(bytes) {
        Ok(content) => Ok(format!("Content length: {}", content.len())),
        Err(e) => Err(JsError::new(&format!("Failed to decode content: {}", e)))
    }
}

#[wasm_bindgen(js_name = runCairoProgramFromFile)]
pub fn run_cairo_program_from_file(
    array_buffer: ArrayBuffer,
    available_gas: Option<usize>,
    allow_warnings: bool,
    print_full_memory: bool,
    run_profiler: bool,
    use_dbg_print_hint: bool,
) -> Result<String, JsError> {
    let uint8_array = Uint8Array::new(&array_buffer);
    let len = uint8_array.length() as usize;
    let mut bytes = vec![0; len];
    uint8_array.copy_to(&mut bytes);
    
    match String::from_utf8(bytes) {
        Ok(content) => {
            run_cairo_program(
                content,
                available_gas,
                allow_warnings,
                print_full_memory,
                run_profiler,
                use_dbg_print_hint,
            )
        },
        Err(e) => Err(JsError::new(&format!("Failed to decode content: {}", e)))
    }
}

#[wasm_bindgen(js_name = runSierraProgramFromFile)]
pub fn run_sierra_program_from_file(
    array_buffer: ArrayBuffer,
    available_gas: Option<usize>,
    print_full_memory: bool,
    run_profiler: bool,
    use_dbg_print_hint: bool,
) -> Result<String, JsError> {
    let uint8_array = Uint8Array::new(&array_buffer);
    let len = uint8_array.length() as usize;
    let mut bytes = vec![0; len];
    uint8_array.copy_to(&mut bytes);
    
    match String::from_utf8(bytes) {
        Ok(content) => run_sierra_program(content, available_gas, print_full_memory, run_profiler, use_dbg_print_hint),
        Err(e) => Err(JsError::new(&format!("Failed to decode content: {}", e)))
    }
}

#[wasm_bindgen(js_name = runTestsFromFile)]
pub fn run_tests_from_file(
    array_buffer: ArrayBuffer,
    allow_warnings: bool,
    filter: String,
    include_ignored: bool,
    ignored: bool,
    starknet: bool,
    run_profiler: String,
    gas_disabled: bool,
    print_resource_usage: bool,
) -> Result<String, JsError> {
    let uint8_array = Uint8Array::new(&array_buffer);
    let len = uint8_array.length() as usize;
    let mut bytes = vec![0; len];
    uint8_array.copy_to(&mut bytes);
    
    match String::from_utf8(bytes) {
        Ok(content) => {
            run_tests(
                content,
                allow_warnings,
                filter,
                include_ignored,
                ignored,
                starknet,
                run_profiler,
                gas_disabled,
                print_resource_usage,
            )
        },
        Err(e) => Err(JsError::new(&format!("Failed to decode content: {}", e)))
    }
}

#[wasm_bindgen(js_name = compileStarknetContractFromFile)]
pub fn compile_starknet_contract_from_file(
    array_buffer: ArrayBuffer,
    allow_warnings: bool,
    replace_ids: bool,
) -> Result<String, JsError> {
    let uint8_array = Uint8Array::new(&array_buffer);
    let len = uint8_array.length() as usize;
    let mut bytes = vec![0; len];
    uint8_array.copy_to(&mut bytes);
    
    match String::from_utf8(bytes) {
        Ok(content) => {
            compile_starknet_contract(
                content,
                allow_warnings,
                replace_ids,
            )
        },
        Err(e) => Err(JsError::new(&format!("Failed to decode content: {}", e)))
    }
}
