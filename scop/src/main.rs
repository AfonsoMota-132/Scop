use gl::*;
use glfw::*;
use scop_lib::parsing_data;
use std::env;

pub fn init_window() -> (Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(720, 720, "Scop", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();
    window.set_key_polling(true);
    (glfw, window, events)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error!");
        eprintln!("Wrong Number of arguments!");
        eprintln!("Program Should have 1 argument that is a .obj file!");
    } else {
        let mut data = parsing_data(&args[1]).unwrap();
        for point in data.geo_vert {
            println!("x: {}\ty: {}\tz: {}", point.x, point.y, point.z);
        }
        let (mut glfw, mut window, events) = init_window();
        // let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        while !window.should_close() {
            unsafe {
                gl::ClearColor(0.3, 0.3, 0.5, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            window.swap_buffers();
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    }
                    _ => {}
                }
            }
        }
    }
}
