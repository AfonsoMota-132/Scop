use gl::types::*;
use gl::*;
use glfw::*;
use scop_lib::*;
use std::ffi::CString;
use std::{env, mem, ptr, str};

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 vertexColor;

    void main() {
        gl_Position = vec4(aPos. x, aPos.y, -0.5, 1.0);  // Scale down and move back
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

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(720, 720, "Scop", glfw::WindowMode::Windowed)
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
            let face = face[i][0] - 1;
            vertices.extend_from_slice(&[
                data.geo_vert[face].x,
                data.geo_vert[face].y,
                data.geo_vert[face].z,
            ]);
            vertices.push(0.6);
            vertices.push(0.5);
            vertices.push(0.2);
        }
    }
    vertices
}
unsafe fn load_vao_vbo(data: &Data) -> (GLuint, GLuint) {
    let vertices: Vec<f32> = load_vertices(data);

    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;

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

    (vao, vbo)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error!");
        eprintln!("Wrong Number of arguments!");
        eprintln!("Program Should have 1 argument that is a .obj file!");
    } else {
        let mut data = parsing_data(&args[1]).unwrap();
        // for point in data.geo_vert {
        //     println!("x: {}\ty: {}\tz: {}", point.x, point.y, point.z);
        // }
        let (mut glfw, mut window, events) = init_window();
        // let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        unsafe {
            // Compile shaders
            let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
            let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
            let shader_program = link_program(vertex_shader, fragment_shader);
            (data.vao, data.vbo) = load_vao_vbo(&data);
            gl::Disable(gl::CULL_FACE); // Don't cull back faces
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); // Wireframe mode (optional, to see structure)
            while !window.should_close() {
                // Process events
                glfw.poll_events();
                for (_, event) in glfw::flush_messages(&events) {
                    handle_window_event(&mut window, event, &mut data);
                }

                // Render
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // Draw triangle
                gl::UseProgram(shader_program);
                gl::BindVertexArray(data.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, (data.faces.len() as i32) * 3);

                window.swap_buffers();
            }
            gl::DeleteVertexArrays(1, &data.vao);
            gl::DeleteBuffers(1, &data.vbo);
            gl::DeleteProgram(shader_program);
        }
    }
}

pub fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, data: &mut Data) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(Key::Right, _, _, _) => {
            data.rotate_x(0.5);
            data.set_rotate_x();
            unsafe {
                (data.vao, data.vbo) = load_vao_vbo(&data);
            }
            // println!("Rigt Arrow");
        }
        glfw::WindowEvent::Key(Key::Left, _, _, _) => {
            data.rotate_x(-0.5);
            data.set_rotate_x();
            unsafe {
                (data.vao, data.vbo) = load_vao_vbo(&data);
            }
            // println!("Rigt Arrow");
        }
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },

        _ => {}
    }
}
