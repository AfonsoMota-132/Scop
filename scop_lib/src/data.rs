use crate::{Face, KeyIn, ShaderManager, Vect3};
use gl::types::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
pub struct Data {
    pub geo_vert: Vec<Vect3>,
    pub ori_vert: Vec<Vect3>, // Original, unmodified vertices
    pub text_vert: Vec<Vect3>,
    pub vert_norm: Vec<Vect3>,
    pub faces: Vec<Face>,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub vertex_count: i32,

    // Rotation angles
    pub ang_x: f32,
    pub ang_y: f32,
    pub ang_z: f32,

    // Translation position
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    // Other stuff
    pub g_scale: Vec<[f32; 3]>,
    pub g_bool: bool,
    pub key: KeyIn,
    pub mode: usize,

    // Texture stuff
    pub texture_mix: f32,
    pub transitioning: bool,
    pub transition_direction: f32,

    // Auto-rotation
    pub auto_rotate: bool,
    pub auto_rotate_speed: f32,
}

impl Data {
    pub fn new() -> Self {
        Data {
            geo_vert: Vec::new(),
            ori_vert: Vec::new(),
            text_vert: Vec::new(),
            vert_norm: Vec::new(),
            faces: Vec::new(),
            vao: 0,
            vbo: 0,
            vertex_count: 0,
            ang_x: 0.0,
            ang_y: 0.0,
            ang_z: 0.0,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            g_scale: Vec::new(),
            g_bool: true,
            mode: 0,
            key: KeyIn::default(),
            texture_mix: 0.0,
            transitioning: false,
            transition_direction: 1.0,
            auto_rotate: true,
            auto_rotate_speed: 0.5,
        }
    }
    pub fn scale(&mut self) {
        let max_val = self
            .geo_vert
            .iter()
            .flat_map(|p| [p.x.abs(), p.y.abs()])
            .fold(0.0_f32, f32::max);

        let scale = 0.9 / max_val;

        for point in &mut self.geo_vert {
            point.x *= scale;
            point.y *= scale;
            point.z *= scale;
        }
    }
    pub fn center(&mut self) {
        let center_x: f32 =
            self.geo_vert.iter().map(|p| p.x).sum::<f32>() / self.geo_vert.len() as f32;
        let center_y: f32 =
            self.geo_vert.iter().map(|p| p.y).sum::<f32>() / self.geo_vert.len() as f32;
        let center_z: f32 =
            self.geo_vert.iter().map(|p| p.z).sum::<f32>() / self.geo_vert.len() as f32;
        for point in &mut self.geo_vert {
            point.x -= center_x;
            point.y -= center_y;
            point.z -= center_z;
        }
    }
    pub fn restore(&mut self) {
        self.ang_x = 0.0;
        self.ang_y = 0.0;
        self.ang_z = 0.0;
    }
    // pub unsafe fn set_rotate_x(&mut self, angle: f32) {
    //     let rad = angle.to_radians();

    //     let cos = rad.cos();
    //     let sin = rad.sin();

    //     for i in 0..self.geo_vert.len() {
    //         let x = self.geo_vert[i].x;
    //         let y = self.geo_vert[i].y;
    //         let z = self.geo_vert[i].z;
    //         self.geo_vert[i].x = x;
    //         self.geo_vert[i].y = y * cos - z * sin;
    //         self.geo_vert[i].z = y * sin + z * cos;
    //     }
    // }
    // pub unsafe fn set_rotate_y(&mut self, angle: f32) {
    //     let rad = angle.to_radians();

    //     let cos = rad.cos();
    //     let sin = rad.sin();

    //     for i in 0..self.geo_vert.len() {
    //         let x = self.geo_vert[i].x;
    //         let y = self.geo_vert[i].y;
    //         let z = self.geo_vert[i].z;
    //         self.geo_vert[i].x = x * cos - z * sin;
    //         self.geo_vert[i].y = y;
    //         self.geo_vert[i].z = x * sin + z * cos;
    //     }
    // }
    // pub unsafe fn set_rotate_z(&mut self, angle: f32) {
    //     let rad = angle.to_radians();

    //     let cos = rad.cos();
    //     let sin = rad.sin();

    //     for i in 0..self.geo_vert.len() {
    //         let x = self.geo_vert[i].x;
    //         let y = self.geo_vert[i].y;
    //         let z = self.geo_vert[i].z;
    //         self.geo_vert[i].x = x * cos - y * sin;
    //         self.geo_vert[i].y = x * sin + y * cos;
    //         self.geo_vert[i].z = z;
    //     }
    // }
    // pub unsafe fn set_rotate(&mut self, angle_x: f32, angle_y: f32, angle_z: f32) {
    //     unsafe {
    //         self.set_rotate_x(angle_x);
    //         self.set_rotate_y(angle_y);
    //         self.set_rotate_z(angle_z);
    //     }
    // }
}

pub fn rotate(angle: &mut f32, inc: f32) {
    *angle += inc;
    *angle = *angle % 360.0;
    if *angle <= 0.0 {
        *angle += 360.0;
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
            if arr[0] == "v" && arr.len() >= 4 {
                parse_geo_vert(&mut data, &arr);
            } else if arr[0] == "vt" && arr.len() >= 2 {
                parse_text_vert(&mut data, &arr);
            } else if arr[0] == "vn" && arr.len() >= 4 {
                parse_vert_norm(&mut data, &arr);
            } else if arr[0] == "f" && arr.len() >= 4 {
                parse_faces(&mut data, &arr);
            }
        }
    }
    data.center();
    data.scale();
    data.ori_vert = data.geo_vert.clone();
    Ok(data)
}

fn parse_geo_vert(data: &mut Data, arr: &Vec<&str>) {
    data.geo_vert.push(Vect3::new(
        arr[1].parse().unwrap(),
        arr[2].parse().unwrap(),
        arr[3].parse().unwrap(),
    ));
}

fn parse_text_vert(data: &mut Data, arr: &Vec<&str>) {
    let u: f32 = arr[1].parse().unwrap();
    let (mut v, mut w): (f32, f32) = (0.0, 0.0);
    if arr.len() >= 3 {
        v = arr[2].parse().unwrap();
    }
    if arr.len() >= 4 {
        w = arr[3].parse().unwrap();
    }
    data.text_vert.push(Vect3::new(u, v, w));
}

fn parse_vert_norm(data: &mut Data, arr: &Vec<&str>) {
    data.vert_norm.push(Vect3::new(
        arr[1].parse().unwrap(),
        arr[2].parse().unwrap(),
        arr[3].parse().unwrap(),
    ));
}

fn parse_face_point(str: &str) -> [usize; 3] {
    let mut v: usize = 0;
    let mut vt: usize = 0;
    let mut vn: usize = 0;
    let tmp: Vec<&str> = str.split('/').collect();
    if tmp.len() >= 1 {
        v = tmp[0].parse().unwrap();
    }
    if tmp.len() >= 2 && !tmp[1].is_empty() {
        vt = tmp[1].parse().unwrap();
    }
    if tmp.len() >= 3 && !tmp[2].is_empty() {
        vn = tmp[2].parse().unwrap();
    }
    [v, vt, vn]
}

fn parse_faces(data: &mut Data, arr: &Vec<&str>) {
    let mut faces: Vec<[usize; 3]> = Vec::new();
    for i in 1..arr.len() {
        faces.push(parse_face_point(arr[i]));
    }
    if faces.len() < 3 {
        return;
    }
    for i in 1..(faces.len() - 1) {
        data.faces.push(Face::new(
            [faces[0][0], faces[i][0], faces[i + 1][0]],
            [faces[0][1], faces[i][1], faces[i + 1][1]],
            [faces[0][2], faces[i][2], faces[i + 1][2]],
        ));
    }
}
