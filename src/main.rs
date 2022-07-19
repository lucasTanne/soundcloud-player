use std::{env, process};
use rcli_lib::parser as parser;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    parser::Parser::new(&args).unwrap_or_else(| err | {
        println!("{}", err);
        process::exit(1);
    });
}
