use gl::types::*;
use glfw::*;
use scop_lib::*;
use std::{env, mem, ptr, str};

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
            if face.vt[i] != 0 {
                let tmp = face.vt[i] - 1;
                vertices.extend_from_slice(&[data.text_vert[tmp].x, data.text_vert[tmp].y]);
            } else {
                vertices.extend_from_slice(&[0.0, 0.0]);
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
            8 * mem::size_of::<GLfloat>() as GLsizei, // NOW 8 floats per vertex
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // Color attribute (location = 1)
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * mem::size_of::<GLfloat>() as GLsizei,
            (6 * mem::size_of::<GLfloat>()) as *const _,
        );
        gl::EnableVertexAttribArray(2);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    (vao, vbo)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        eprintln!("Error!");
        eprintln!("Wrong Number of arguments!");
        eprintln!(
            "Program Should have 1 arguments that is a .obj file and a optional texture file(bmp with 24 bit)!"
        );
    } else {
        let mut data = parsing_data(&args[1]).unwrap();
        let (mut glfw, mut window, events) = init_window();

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        let mut shader_manager = ShaderManager::new();
        let texture = Texture::from_bmp("leek.bmp").expect("Failed to load texture");

        unsafe {
            (data.vao, data.vbo) = load_vao_vbo(&data);
            data.vertex_count = (data.faces.len() * 3) as i32;

            shader_manager.use_color(); // Always use mixed shader
            let model_loc = gl::GetUniformLocation(
                shader_manager.color_shader,
                b"projection\0".as_ptr() as *const i8,
            );
            let hardcoded_rotation: [f32; 16] = [
                0.707, 0.0, 0.707, 0.0, // Column 1
                0.0, 1.0, 0.0, 0.0, // Column 2
                -0.707, 0.0, 0.707, 0.0, // Column 3
                0.0, 0.0, 0.0, 1.0, // Column 4
            ];
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, hardcoded_rotation.as_ptr());
            gl::Enable(gl::DEPTH_TEST);
        }
        println!("wtf");
        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event, &mut data, &mut shader_manager);
            }

            unsafe {
                // Update angles only
                update_model(&mut data);

                // Update texture transition
                if data.transitioning {
                    data.texture_mix += data.transition_direction * 0.02;

                    if data.texture_mix >= 1.0 {
                        data.texture_mix = 1.0;
                        data.transitioning = false;
                    } else if data.texture_mix <= 0.0 {
                        data.texture_mix = 0.0;
                        data.transitioning = false;
                    }
                }

                // Clear screen
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // Use shader
                shader_manager.use_color(); // Always use mixed shader

                // Set transformation matrices
                let (width, height) = window.get_size();
                set_matrices(
                    shader_manager.active_shader,
                    &data,
                    width as u32,
                    height as u32,
                );

                // Set texture mix uniform
                // let mix_loc = gl::GetUniformLocation(
                //     shader_manager.active_shader,
                //     b"textureMix\0".as_ptr() as *const i8,
                // );
                // gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, hardcoded_rotation.as_ptr());
                // gl::Uniform1f(mix_loc, data.texture_mix);

                // Bind texture and draw
                texture.bind();
                gl::BindVertexArray(data.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, data.vertex_count);
            }

            window.swap_buffers();
        }
    }
}

unsafe fn set_matrices(shader_program: GLuint, data: &Data, window_width: u32, window_height: u32) {
    // Model matrix:  Translation * Rotation
    let translation = Mat4::translation(data.pos_x, data.pos_y, data.pos_z);
    let rotation =
        Mat4::rotation_x(data.ang_x) * Mat4::rotation_y(data.ang_y) * Mat4::rotation_z(data.ang_z);
    let model = translation * rotation;

    // View matrix: Move camera back
    let view = Mat4::translation(0.0, 0.0, -3.0);

    // Projection matrix: Perspective
    let aspect = window_width as f32 / window_height as f32;
    let projection = Mat4::perspective(45.0, aspect, 0.1, 100.0);

    // Send matrices to shader
    unsafe {
        let model_loc = gl::GetUniformLocation(shader_program, b"model\0".as_ptr() as *const i8);
        let view_loc = gl::GetUniformLocation(shader_program, b"view\0".as_ptr() as *const i8);
        let proj_loc =
            gl::GetUniformLocation(shader_program, b"projection\0".as_ptr() as *const i8);

        gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
        gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
        gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, projection.as_ptr());
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

        // if angle_x != 0.0 || angle_y != 0.0 || angle_z != 0.0 {
        // data.set_rotate(angle_x, angle_y, angle_z);
        //     let vertices: Vec<f32> = load_vertices(data);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, data.vbo);
        //     gl::BufferSubData(
        //         gl::ARRAY_BUFFER,
        //         0,
        //         (vertices.len() * mem::size_of::<f32>()) as isize,
        //         vertices.as_ptr() as *const _,
        //     );
        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // }
    }
}

pub fn handle_window_event(
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    data: &mut Data,
    shader_manager: &mut ShaderManager,
) {
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
        glfw::WindowEvent::Key(Key::N, _, Action::Press, _) => {
            shader_manager.use_next();
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
