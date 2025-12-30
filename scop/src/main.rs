use gl::types::*;
use glfw::*;
use scop_lib::*;
use std::{env, mem, ptr, str};

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 vertexColor;

    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z , 1.0);  // Scale down and move back
        vertexColor = aColor;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    in vec3 vertexColor;
    out vec4 FragColor;

    void main() {
        FragColor = vec4(vertexColor, 1.0);
    }
"#;

pub fn init_window() -> (Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::DepthBits(Some(24)));
    let (mut window, events) = glfw
        .create_window(1080, 720, "Scop", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    (glfw, window, events)
}

fn load_vertices(data: &Data) -> Vec<f32> {
    let mut vertices: Vec<f32> = Vec::new();

    for face in &data.faces {
        for i in 0..3 {
            let tmp = face.v[i] - 1;
            vertices.extend_from_slice(&[
                data.geo_vert[tmp].x,
                data.geo_vert[tmp].y,
                data.geo_vert[tmp].z,
            ]);
            if data.g_bool {
                vertices.extend_from_slice(&face.g_scale);
            } else {
                vertices.extend_from_slice(&[0.6, 0.5, 0.2]);
            }
        }
    }
    vertices
}
unsafe fn load_vao_vbo(data: &Data) -> (GLuint, GLuint) {
    let vertices: Vec<f32> = load_vertices(data);

    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        // Position attribute (location = 0)
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei, // 6 floats per vertex (x,y,z,r,g,b)
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // Color attribute (location = 1)
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const _, // Offset by 3 floats
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    (vao, vbo)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error!");
        eprintln!("Wrong Number of arguments!");
        eprintln!("Program Should have 1 argument that is a .obj file!");
        eprintln!("{}\t{}\t{}", gl::FILL, gl::LINE, gl::POINT);
    } else {
        let mut data = parsing_data(&args[1]).unwrap();
        let (mut glfw, mut window, events) = init_window();
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        unsafe {
            // Compile shaders
            println!("Number of faces: {}", data.faces.len());
            println!("Number of vertices: {}", data.ori_vert.len());
            let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
            let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
            let shader_program = link_program(vertex_shader, fragment_shader);
            (data.vao, data.vbo) = load_vao_vbo(&data);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            while !window.should_close() {
                glfw.poll_events();
                for (_, event) in glfw::flush_messages(&events) {
                    handle_window_event(&mut window, event, &mut data);
                }
                update_model(&mut data);
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::UseProgram(shader_program);
                gl::BindVertexArray(data.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, (data.faces.len() as i32) * 3);

                window.swap_buffers();
            }
            gl::DeleteVertexArrays(1, &data.vao);
            gl::DeleteBuffers(1, &data.vbo);
            gl::DeleteProgram(shader_program);
            gl::DeleteProgram(vertex_shader);
        }
    }
}

pub unsafe fn update_model(data: &mut Data) {
    unsafe {
        let (mut angle_x, mut angle_y, mut angle_z) = (0.0, 0.0, 0.0);
        if data.key.up {
            angle_x -= 2.5;
        }
        if data.key.down {
            angle_x += 2.5;
        }
        if data.key.left {
            angle_y -= 2.5;
        }
        if data.key.right {
            angle_y += 2.5;
        }
        if data.key.r_left {
            angle_z -= 2.5;
        }
        if data.key.r_right {
            angle_z += 2.5;
        }

        if angle_x != 0.0 || angle_y != 0.0 || angle_z != 0.0 {
            data.set_rotate(angle_x, angle_y, angle_z);
            (data.vao, data.vbo) = load_vao_vbo(&data);
        }
    }
}

pub fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, data: &mut Data) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(
            key @ (Key::Up | Key::Down),
            _,
            press @ (Action::Press | Action::Release),
            _,
        ) => {
            if key == Key::Up {
                data.key.up = press == Action::Press;
            } else if key == Key::Down {
                data.key.down = press == Action::Press;
            }
        }
        glfw::WindowEvent::Key(
            key @ (Key::Left | Key::Right),
            _,
            press @ (Action::Press | Action::Release),
            _,
        ) => {
            if key == Key::Right {
                data.key.right = press == Action::Press;
            } else if key == Key::Left {
                data.key.left = press == Action::Press;
            }
        }
        glfw::WindowEvent::Key(
            key @ (Key::I | Key::O),
            _,
            press @ (Action::Press | Action::Release),
            _,
        ) => {
            if key == Key::O {
                data.key.r_right = press == Action::Press;
            } else if key == Key::I {
                data.key.r_left = press == Action::Press;
            }
        }
        glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => {
            data.restore();
            unsafe {
                (data.vao, data.vbo) = load_vao_vbo(&data);
            }
        }
        glfw::WindowEvent::Key(Key::M, _, Action::Press, _) => {
            const POLY_MODES: [u32; 3] = [gl::FILL, gl::LINE, gl::POINT];
            data.mode = (data.mode + 1) % POLY_MODES.len();
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, POLY_MODES[data.mode]);
            }
        }
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },

        _ => {}
    }
}
