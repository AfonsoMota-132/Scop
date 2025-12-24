use crate::Vect3;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Data {
    pub geo_vert: Vec<Vect3>,
    pub text_vert: Vec<Vect3>,
    pub vert_norm: Vec<Vect3>,
    pub faces: Vec<[Vect3; 3]>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            geo_vert: Vec::new(),
            text_vert: Vec::new(),
            vert_norm: Vec::new(),
            faces: Vec::new(),
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
    let mut data = Data::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let arr: Vec<&str> = line.split(' ').collect();
            if arr[0] == "v" {
                parse_geo_vert(&mut data, &arr);
            } else if arr[0] == "vt" {
                parse_text_vert(&mut data, &arr);
            } else if arr[0] == "vn" {
                parse_vert_norm(&mut data, &arr);
            } else if arr[0] == "f" {
                parse_faces(&mut data, &arr);
            }
        }
    }
    Ok(data)
}

pub fn parse_geo_vert(data: &mut Data, arr: &Vec<&str>) {
    let x: f32 = arr[1].parse().unwrap();
    let y: f32 = arr[2].parse().unwrap();
    let z: f32 = arr[3].parse().unwrap();
    data.geo_vert.push(Vect3::new([x, y, z]));
}

pub fn parse_text_vert(data: &mut Data, arr: &Vec<&str>) {
    let u: f32 = arr[1].parse().unwrap();
    let (mut v, mut w): (f32, f32) = (0.0, 0.0);
    if arr.len() >= 3 {
        v = arr[2].parse().unwrap();
    }
    if arr.len() >= 4 {
        w = arr[3].parse().unwrap();
    }
    data.text_vert.push(Vect3::new([u, v, w]));
}

pub fn parse_vert_norm(data: &mut Data, arr: &Vec<&str>) {
    let x: f32 = arr[1].parse().unwrap();
    let y: f32 = arr[2].parse().unwrap();
    let z: f32 = arr[3].parse().unwrap();
    data.vert_norm.push(Vect3::new([x, y, z]));
}

pub fn parse_face_point(str: &str) -> Vect3 {
    let mut v: f32 = 0.0;
    let mut vt: f32 = 0.0;
    let mut vn: f32 = 0.0;
    let tmp: Vec<&str> = str.split('/').collect();
    if tmp.len() >= 1 {
        v = tmp[0].parse().unwrap();
    }
    if tmp.len() >= 2 && !tmp[1].is_empty() {
        vt = tmp[1].parse().unwrap();
    }
    if tmp.len() >= 3 && !tmp[1].is_empty() {
        vn = tmp[2].parse().unwrap();
    }
    Vect3::new([v, vt, vn])
}

pub fn parse_faces(data: &mut Data, arr: &Vec<&str>) {
    let mut faces: Vec<Vect3> = Vec::new();
    for i in 1..arr.len() {
        faces.push(parse_face_point(arr[i]));
    }
    if faces.len() < 3 {
        return;
    }
    for i in 1..(faces.len() - 1) {
        data.faces.push([faces[0], faces[i], faces[i + 1]]);
    }
}
