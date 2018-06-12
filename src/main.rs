/**
 * When writing select * from table/G to a file ex output.txt you might want to parse this file to output.csv
 * This is what this funciton will do
 * My first rust project
 * example: ./mysql-csv output.txt output.csv
 */
use std::env;
use std::process;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    let arguments = lib::parse_arguments(&args);

    match lib::run(arguments) {
        Ok(()) => println!("Success"),
        Err(e) => {
            println!("{} error: {} ", program, e);
            process::exit(0);
        }
    } 

    // if let Err(e) = lib::run(arguments) {
    //     println!("{} error: {} ", program, e);
    //     process::exit(0);
    // }
}
