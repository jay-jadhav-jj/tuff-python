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
mod run_command;
pub mod translation;
mod zh_builtins;
mod zh_processor;

use std::env;

fn help_text(program: &str) -> String {
    format!(
        "tuffpython - multilingual Python compiler and runner\n\
         \n\
         Commands:\n\
         \x20 {0} <c|compile> <INPUTFILE> <OUTPUTFILE>\n\
         \x20     Compile a .tpy source file to Python.\n\
         \x20 {0} <r|run> <INPUTFILE>\n\
         \x20     Compile and run a .tpy source file.\n\
         \x20 {0} <h|help>\n\
         \x20     Show this help and examples.\n\
         \n\
         Examples:\n\
         \x20 {0} compile hello.tpy hello.py\n\
         \x20 {0} run hello.tpy\n\
         \x20 {0} r hello.tpy\n\
         \n\
         Source example (hello.tpy):\n\
         \x20 #!es\n\
         \x20 imprimir(\"Hola\")\n\
         \n\
         Source must begin with a supported language directive: #!fr, #!es, #!hi, #!mr, or #!zh.\n\
         The run command translates Python exception messages when Google Translate is reachable; \
         otherwise, it warns and leaves errors in English.\n\
         \n\
         GitHub: https://github.com/jay-jadhav-jj/tuff-python",
        program
    )
}

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "tpy".to_string());
    let args = args.collect::<Vec<_>>();

    let usage = format!(
        "Usage: {0} <c|compile> <INPUTFILE> <OUTPUTFILE>\n\
         \x20      {0} <r|run> <INPUTFILE>\n\
         \x20      {0} <h|help>\n\
         Run '{0} help' for command details and examples.\n\
         Source must start with a supported language directive: #!fr, #!es, #!hi, #!mr, or #!zh.",
        program
    );

    let exit_code = match args.first().map(String::as_str) {
        Some("h" | "help") if args.len() == 1 => {
            println!("{}", help_text(&program));
            0
        }
        Some("h" | "help") => {
            eprintln!(
                "The help command takes no arguments. Run '{} help'.",
                program
            );
            2
        }
        Some("c" | "compile") if args.len() == 3 => {
            match compiler::compile_file_with_diagnostics(&args[1]) {
                Ok(result) => {
                    if let Err(err) = filehandling::write_file(&args[2], &result.code) {
                        eprintln!("{}", err);
                        1
                    } else {
                        for error in &result.alias_errors {
                            eprintln!("alias error: {}", error);
                        }
                        0
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
                    2
                }
            }
        }
        Some("r" | "run") if args.len() == 2 => {
            match run_command::run_file(&args[1], translation::translate) {
                Ok(status) => status.code().unwrap_or(1),
                Err(err) => {
                    eprintln!("{}", err.message);
                    err.exit_code
                }
            }
        }
        Some("c" | "compile" | "r" | "run") | None => {
            eprintln!("{}", usage);
            2
        }
        Some(cmd) => {
            eprintln!(
                "Unknown command '{}'. Run '{} help' for command details and examples.",
                cmd, program
            );
            2
        }
    };

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
