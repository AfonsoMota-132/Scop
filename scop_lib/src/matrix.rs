#[derive(Copy, Clone, Debug)]
pub struct Mat4 {
    pub data: [f32; 16],
}

impl Mat4 {
    // Identity matrix
    pub fn identity() -> Self {
        Mat4 {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    // Rotation around X axis
    pub fn rotation_x(angle_deg: f32) -> Self {
        let rad = angle_deg.to_radians();
        let c = rad.cos();
        let s = rad.sin();

        Mat4 {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    // Rotation around Y axis
    pub fn rotation_y(angle_deg: f32) -> Self {
        let rad = angle_deg.to_radians();
        let c = rad.cos();
        let s = rad.sin();

        Mat4 {
            data: [
                c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    // Rotation around Z axis
    pub fn rotation_z(angle_deg: f32) -> Self {
        let rad = angle_deg.to_radians();
        let c = rad.cos();
        let s = rad.sin();

        Mat4 {
            data: [
                c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    // Translation matrix
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Mat4 {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
            ],
        }
    }

    // Scale matrix
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Mat4 {
            data: [
                x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    // Perspective projection
    pub fn perspective(fov_deg: f32, aspect: f32, near: f32, far: f32) -> Self {
        let fov_rad = fov_deg.to_radians();
        let f = 1.0 / (fov_rad / 2.0).tan();

        Mat4 {
            data: [
                f / aspect,
                0.0,
                0.0,
                0.0,
                0.0,
                f,
                0.0,
                0.0,
                0.0,
                0.0,
                (far + near) / (near - far),
                -1.0,
                0.0,
                0.0,
                (2.0 * far * near) / (near - far),
                0.0,
            ],
        }
    }

    // Matrix multiplication
    pub fn multiply(&self, other: &Mat4) -> Mat4 {
        let mut result = [0.0f32; 16];

        for i in 0..4 {
            for j in 0..4 {
                result[i * 4 + j] = self.data[i * 4 + 0] * other.data[0 * 4 + j]
                    + self.data[i * 4 + 1] * other.data[1 * 4 + j]
                    + self.data[i * 4 + 2] * other.data[2 * 4 + j]
                    + self.data[i * 4 + 3] * other.data[3 * 4 + j];
            }
        }

        Mat4 { data: result }
    }

    // Get pointer for OpenGL
    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }
}

// Operator overloading for easier multiplication
impl std::ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        self.multiply(&other)
    }
}
