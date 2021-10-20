use std::{env, process::exit};

mod compiler;

pub struct CompilerArguments {
    filename: String,
    out_filename: String,
}

#[warn(non_snake_case)]
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut compiler_args = CompilerArguments {
        filename: String::from(""),
        out_filename: String::from("")
    };

    //Parse Arguments
    
    for argument in args {
        if argument.starts_with("file=") {
            let mut _filename: String = argument.replace("file=", "");

            compiler_args.filename = _filename;
        } else if argument.starts_with("out=") {
            let mut _out_filename: String = argument.replace("out=", "");

            compiler_args.out_filename = _out_filename;
        }
    }

    if compiler_args.filename.trim().is_empty() {
        println!("Error: No file specified!");
        exit(-1);
    }

    if compiler_args.out_filename.trim().is_empty() {
        compiler_args.out_filename = format!("{}{}", compiler_args.filename, ".c");
    }

    //Log information
    println!("Compiling: {}\nOutput Location: {}", compiler_args.filename, compiler_args.out_filename);

    compiler::compiler::compile(compiler_args);
}
