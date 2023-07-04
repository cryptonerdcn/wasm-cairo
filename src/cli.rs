use anyhow::Error;
use std::path::Path;
use clap::Parser;

use cairo_lang_compiler::{compile_cairo_project_with_input_string, CompilerConfig};
use cairo_lang_runner::run_with_input_program_string;
use cairo_lang_starknet::contract_class::starknet_wasm_compile_with_input_string;

use std::sync::Arc;

use starknet_rs::{
    accounts::{Account, SingleOwnerAccount},
    core::{
        chain_id,
        types::{contract::SierraClass, BlockId, BlockTag, FieldElement},
    },
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};
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
    #[arg(long)]
    class_hash: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        // match declareContract
        // 1. contract_json: String
        // 2. class_hash: String
        "dedeclareContract" => {
            let contract_json = args.input_program_string.unwrap();
            let class_hash = args.class_hash.unwrap();
            declare_v1(contract_json, class_hash).await;
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

async fn declare_v1(contract_json: String, class_hash: String) {
    // Sierra class artifact. Output of the `starknet-compile` command
    let contract_artifact: SierraClass =
        serde_json::from_str(&contract_json).unwrap();

    // Class hash of the compiled CASM class from the `starknet-sierra-compile` command
    let compiled_class_hash =
        FieldElement::from_hex_be(&class_hash).unwrap();

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address = FieldElement::from_hex_be("02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa").unwrap();

    let mut account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    // We need to flatten the ABI into a string first
    let flattened_class = contract_artifact.flatten().unwrap();

    let result = account
        .declare(Arc::new(flattened_class), compiled_class_hash)
        .send()
        .await
        .unwrap();

    dbg!(result);
}