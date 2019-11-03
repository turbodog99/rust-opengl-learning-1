#[macro_use]
extern crate failure;

extern crate gl;
extern crate sdl2;
#[macro_use]
extern crate render_gl_derive;
extern crate nalgebra;

pub mod render_gl;
pub mod resources;

use failure::err_msg;
use render_gl::Color;
use resources::Resources;
use std::path::Path;

use nalgebra as na;
mod triangle;

const INIT_WINDOW_WIDTH: u32 = 900;
const INIT_WINDOW_HEIGHT: u32 = 700;

const BACKGROUND_COLOR: Color = Color {
    red: 0.3,
    green: 0.3,
    blue: 0.8,
    alpha: 1.0,
};

fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", INIT_WINDOW_WIDTH, INIT_WINDOW_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    unsafe {
        gl.Viewport(0, 0, INIT_WINDOW_WIDTH as i32, INIT_WINDOW_HEIGHT as i32);
    }

    let identity_matrix: na::Matrix4<f32> = na::Matrix4::identity();
    let translate_vector: na::Vector3<f32> = na::Vector3::new(0.2, 0.0, 0.0);
    let transform_matrix: na::Matrix4<f32> = identity_matrix.append_translation(&translate_vector);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            // handle user input here
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(width, height),
                    ..
                } => unsafe {
                    gl.Viewport(0, 0, width as i32, height as i32);
                },
                _ => {}
            }
        }

        unsafe {
            gl.ClearColor(
                BACKGROUND_COLOR.red,
                BACKGROUND_COLOR.green,
                BACKGROUND_COLOR.blue,
                BACKGROUND_COLOR.alpha,
            );
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        let triangle = triangle::Triangle::new(&res, &gl)?;
        triangle.render(&gl, transform_matrix);

        window.gl_swap_window();
    }

    Ok(())
}

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            let _ = writeln!(&mut result, "   Which caused the following issue:");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, " This happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}
