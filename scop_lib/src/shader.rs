use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

const COLOR_VERTEX_SHADER: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec3 vertexColor;

    void main() {
        gl_Position = projection * view * model * vec4(aPos, 1.0);
        vertexColor = aColor;
    }
"#;

const COLOR_FRAGMENT_SHADER: &str = r#"
    #version 330 core
    in vec3 vertexColor;
    out vec4 FragColor;

    void main() {
        FragColor = vec4(vertexColor, 1.0);
    }
"#;

const TEXTURE_VERTEX_SHADER: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    layout (location = 2) in vec2 aTexCoord;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec3 vertexColor;
    out vec2 TexCoord;

    void main() {
        gl_Position = projection * view * model * vec4(aPos, 1.0);
        vertexColor = aColor;
        TexCoord = aTexCoord;
    }
"#;

const TEXTURE_FRAGMENT_SHADER: &str = r#"
    #version 330 core
    in vec3 vertexColor;
    in vec2 TexCoord;
    out vec4 FragColor;

    uniform sampler2D ourTexture;

    void main() {
        vec4 texColor = texture(ourTexture, TexCoord);
        FragColor = texColor * vec4(vertexColor, 1.0);
    }
"#;

// If you have a MIXED shader, here it is too:
const MIXED_VERTEX_SHADER: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    layout (location = 2) in vec2 aTexCoord;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec3 vertexColor;
    out vec2 TexCoord;

    void main() {
        gl_Position = projection * view * model * vec4(aPos, 1.0);
        vertexColor = aColor;
        TexCoord = aTexCoord;
    }
"#;

const MIXED_FRAGMENT_SHADER: &str = r#"
    #version 330 core
    in vec3 vertexColor;
    in vec2 TexCoord;
    out vec4 FragColor;

    uniform sampler2D ourTexture;
    uniform float textureMix;

    void main() {
        vec4 texColor = texture(ourTexture, TexCoord);
        vec4 vertColor = vec4(vertexColor, 1.0);
        FragColor = mix(vertColor, texColor, textureMix);
    }
"#;

pub unsafe fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut GLchar,
            );
            panic!("{}", str::from_utf8(&buffer).unwrap());
        }
        shader
    }
}

pub unsafe fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Check for linking errors
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut GLchar,
            );
            panic!("{}", str::from_utf8(&buffer).unwrap());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        program
    }
}

#[derive(Clone)]
pub struct ShaderManager {
    pub color_shader: GLuint,
    pub texture_shader: GLuint,
    // pub mixed_shader: GLuint,
    pub active_shader: GLuint,
}

impl ShaderManager {
    pub fn new() -> Self {
        unsafe {
            // Color-only shader
            let color_vs = compile_shader(COLOR_VERTEX_SHADER, gl::VERTEX_SHADER);
            let color_fs = compile_shader(COLOR_FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
            let color_shader = link_program(color_vs, color_fs);

            // Texture-only shader
            let tex_vs = compile_shader(TEXTURE_VERTEX_SHADER, gl::VERTEX_SHADER);
            let tex_fs = compile_shader(TEXTURE_FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
            let texture_shader = link_program(tex_vs, tex_fs);

            // Mixed shader (for Scop transition)
            // let mix_vs = compile_shader(MIXED_VERTEX_SHADER, gl::VERTEX_SHADER);
            // let mix_fs = compile_shader(MIXED_FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
            // let mixed_shader = link_program(mix_vs, mix_fs);

            Self {
                color_shader,
                texture_shader,
                // mixed_shader,
                active_shader: texture_shader,
            }
        }
    }
    pub fn use_next(&mut self) {
        if self.active_shader == self.color_shader {
            self.active_shader = self.texture_shader;
        } else {
            self.active_shader = self.color_shader;
        }
        unsafe {
            gl::UseProgram(self.active_shader);
        }
    }

    pub fn use_color(&mut self) {
        self.active_shader = self.color_shader;
        unsafe {
            gl::UseProgram(self.color_shader);
        }
    }

    pub fn use_texture(&mut self) {
        self.active_shader = self.texture_shader;
        unsafe {
            gl::UseProgram(self.texture_shader);
        }
    }

    // pub fn use_mixed(&mut self) {
    //     self.active_shader = self.mixed_shader;
    //     unsafe {
    //         gl::UseProgram(self.mixed_shader);
    //     }
    // }

    pub fn use_current(&self) {
        unsafe {
            gl::UseProgram(self.active_shader);
        }
    }
}

impl Drop for ShaderManager {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.color_shader);
            gl::DeleteProgram(self.texture_shader);
            // gl::DeleteProgram(self.mixed_shader);
        }
    }
}
