use crate::Vertex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Data {
    pub vertices: Vec<Vertex>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            vertices: Vec::new(),
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parsing_data(file: &str) -> Result<Data, String> {
    let data = Data::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let arr: Vec<&str> = line.split(' ').collect();
            if arr[0] == "v" {
                let x: i32 = arr[1].parse().unwrap();
                let y: i32 = arr[1].parse().unwrap();
                let z: i32 = arr[1].parse().unwrap();
                println!("x: {}", x);
            }
        }
    }
    Ok(data)
}
