use scop_lib::parsing_data;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    parsing_data(&args[1]).unwrap();
}
