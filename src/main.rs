mod alias;
mod builtin_translations;
mod compiler;
mod es_builtins;
mod es_processor;
mod filehandling;
mod fr_builtins;
mod fr_processor;
mod hi_builtins;
mod hi_processor;
mod language_names;
mod lexer;
mod mr_builtins;
mod mr_processor;
mod processor;
mod zh_builtins;
mod zh_processor;

use std::env;

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "tpy".to_string());
    let args = args.collect::<Vec<_>>();

    let usage = format!(
        "Usage: {} <c|compile> <INPUTFILE> <OUTPUTFILE>\n\
         Source must start with a supported language directive: #!fr, #!es, #!hi, #!mr, or #!zh.",
        program
    );

    if args.len() != 3 {
        eprintln!("{}", usage);
        std::process::exit(2);
    }

    if args[0] != "c" && args[0] != "compile" {
        eprintln!("Unknown compile action '{}'.", args[0]);
        eprintln!("{}", usage);
        std::process::exit(2);
    }

    match compiler::compile_file_with_diagnostics(&args[1]) {
        Ok(result) => {
            if let Err(err) = filehandling::write_file(&args[2], &result.code) {
                eprintln!("{}", err);
                std::process::exit(1);
            }

            for error in &result.alias_errors {
                eprintln!("alias error: {}", error);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(2);
        }
    }
}
