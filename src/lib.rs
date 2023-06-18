mod utils;
use std::path::Path;

use cairo_lang_compiler::{compile_cairo_project_with_input_string, SierraProgram, CompilerConfig};
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
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-cairo!");
}

#[wasm_bindgen(js_name = compileCairoProgram)]
pub fn compile_cairo_program(cairo_program: String) -> Result<(), JsError> {
    
    /*let test_cairo = Asset::get("test.cairo").unwrap();
    let test_cairo_str = String::from_utf8(test_cairo.data.to_vec()).unwrap();
    log(test_cairo_str.as_str());*/

    let sierra_program = compile_cairo_project_with_input_string(Path::new("./test123.cairo"), &cairo_program, CompilerConfig {
        replace_ids: false,
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
    log(sierra_program_str.as_str());
    Ok(())
}
