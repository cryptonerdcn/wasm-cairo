use anyhow::Error;
use std::path::Path;
use clap::Parser;
use cairo_vm_new::without_std::result::Result::Ok;

use cairo_lang_compiler::{compile_cairo_project_with_input_string, CompilerConfig};
use cairo_lang_runner::run_with_input_program_string;
use cairo_lang_starknet::contract_class::starknet_wasm_compile_with_input_string;
/// Command line args parser.
/// Exits with 0/1 if the input is formatted correctly/incorrectly.
#[derive(Parser, Debug)]
#[clap(version, verbatim_doc_comment)]
struct Args {
    #[arg(long)]
    command: String,
    /// The file to compile and run.
    /// path: Option<String>,
    /// Whether to print the memory.
    #[arg(long, default_value_t = true)]
    print_full_memory: bool,
    #[arg(long, default_value_t = true)]
    use_dbg_print_hint: bool,
    /// Input cairo program string
    #[arg(long)]
    input_program_string: Option<String>,
}

pub fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let command = args.command;
    match command.as_ref() {
        "compileCairoProgram" => {
            let sierra_program_str = compile_cairo_program(args.input_program_string.unwrap(), true);
            println!("{}", sierra_program_str.unwrap());
        }
        "runCairoProgram" => {
            let cairo_program_result_str = run_cairo_program(args.input_program_string.unwrap(), None, true, true);
            println!("{}", cairo_program_result_str.unwrap());
        }
        "compileStarknetContract" => {
            let sierra_contract_str = compile_starknet_contract(args.input_program_string.unwrap(), true);
            println!("{}", sierra_contract_str.unwrap());
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
    
    Ok(())
}

fn compile_cairo_program(cairo_program: String, replace_ids: bool) -> Result<String, Error> {
    let sierra_program = compile_cairo_project_with_input_string(Path::new("./test123.cairo"), &cairo_program, CompilerConfig {
        replace_ids: replace_ids,
        ..CompilerConfig::default()
    });
    let sierra_program_str = match sierra_program {
        Ok(sierra_program) => {
            sierra_program.to_string()
        }
        Err(e) => {
            println!("{}", e.to_string());
            e.to_string()
        }
    };
    Ok(sierra_program_str)
}

fn run_cairo_program(cairo_program: String, available_gas: Option<usize>, print_full_memory: bool, use_dbg_print_hint: bool) -> Result<String, Error> {

    let cairo_program_result = run_with_input_program_string(&cairo_program, available_gas, print_full_memory, use_dbg_print_hint);
    let cairo_program_result_str = match cairo_program_result {
        Ok(cairo_program_result_str) => {
            
            cairo_program_result_str
        }
        Err(e) => {
            
            println!("{}", e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(cairo_program_result_str)
}

fn compile_starknet_contract(starknet_contract: String, replace_ids: bool) -> Result<String, Error> {
    let sierra_contract = starknet_wasm_compile_with_input_string(&starknet_contract, replace_ids, None, None, None);
    let sierra_contract_str = match sierra_contract {
        Ok(sierra_program) => {
            sierra_program.to_string()
        }
        Err(e) => {
            
            println!("{}", e.to_string().as_str());
            e.to_string()
        }
    };
    Ok(sierra_contract_str)
}