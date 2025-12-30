use gl::types::*;

pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    pub fn from_data(width: u32, height: u32, data: &[u8]) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }

        Self { id, width, height }
    }

    // Load from BMP file
    pub fn from_bmp(path: &str) -> Result<Self, String> {
        let (width, height, data) = load_bmp(path)?;
        Ok(Self::from_data(width, height, &data))
    }
}

pub fn load_bmp(path: &str) -> Result<(u32, u32, Vec<u8>), String> {
    let data = std::fs::read(path).map_err(|e| e.to_string())?;
    if &data[0..2] != b"BM" {
        return Err("Not a BMP".to_string());
    }
    let bit_depth = u16::from_le_bytes([data[28], data[29]]);

    if bit_depth != 24 {
        panic!(
            "❌ Only 24-bit BMP supported!  This file is {}-bit",
            bit_depth
        );
    }
    let offset = u32::from_le_bytes([data[10], data[11], data[12], data[13]]);
    let width = u32::from_le_bytes([data[18], data[19], data[20], data[21]]);
    let height = u32::from_le_bytes([data[22], data[23], data[24], data[25]]);
    let bit_depth = u16::from_le_bytes([data[28], data[29]]);

    let row_size = ((width * 3 + 3) / 4) * 4;
    let mut pixels = Vec::with_capacity((width * height * 3) as usize);

    for y in (0..height).rev() {
        // Reverse to flip image
        let row_start = offset as usize + (y * row_size) as usize;
        for x in 0..width {
            let pixel_start = row_start + (x * 3) as usize;
            let b = data[pixel_start];
            let g = data[pixel_start + 1];
            let r = data[pixel_start + 2];

            // Convert BGR → RGB
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
        }
    }
    Ok((width, height, pixels))
}
