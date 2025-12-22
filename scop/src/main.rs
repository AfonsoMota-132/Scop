use scop_lib::parsing_data;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error!\nWrong Number of arguments!");
    } else {
        parsing_data(&args[1]).unwrap();
    }
}
