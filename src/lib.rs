mod utils;
use std::path::Path;

use cairo_lang_compiler::{compile_cairo_project_with_input_string, CompilerConfig};
//use cairo_lang_runner::run_with_input_program_string;
use cairo_lang_starknet::contract_class::starknet_wasm_compile_with_input_string;
use rust_embed::RustEmbed;

use wasm_bindgen::prelude::*;

#[derive(RustEmbed)]
#[folder = "cairo_files"]
struct Asset;

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
    let sierra_program = compile_cairo_project_with_input_string(Path::new("./test123.cairo"), &cairo_program, CompilerConfig {
        replace_ids: replace_ids,
        ..CompilerConfig::default()
    });
    let sierra_program_str = match sierra_program {
        Ok(sierra_program) => {
            log("sierra_program is Ok");
            sierra_program.to_string()
        }
        Err(e) => {
            log("sierra_program is Err");
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(sierra_program_str)
}

/*#[wasm_bindgen(js_name = runCairoProgram)]
pub fn run_cairo_program(cairo_program: String, available_gas: Option<usize>, print_full_memory: bool, use_dbg_print_hint: bool) -> Result<String, JsError> {

    let cairo_program_result = run_with_input_program_string(&cairo_program, available_gas, print_full_memory, use_dbg_print_hint);
    let cairo_program_result_str = match cairo_program_result {
        Ok(cairo_program_result_str) => {
            log("cairo_program_result is Ok");
            cairo_program_result_str
        }
        Err(e) => {
            log("cairo_program_result is Err");
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(cairo_program_result_str)
}*/

#[wasm_bindgen(js_name = compileStarknetContract)]
pub fn compile_starknet_contract(starknet_contract: String, replace_ids: bool) -> Result<String, JsError> {
    let sierra_contract = starknet_wasm_compile_with_input_string(&starknet_contract, replace_ids, None, None, None);
    let sierra_contract_str = match sierra_contract {
        Ok(sierra_program) => {
            log("sierra_contract is Ok");
            sierra_program.to_string()
        }
        Err(e) => {
            log("sierra_contract is Err");
            log(e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(sierra_contract_str)
}