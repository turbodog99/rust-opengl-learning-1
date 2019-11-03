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
use resources::Resources;
use std::path::Path;

use nalgebra as na;

mod triangle;

const INIT_WINDOW_WIDTH: u32 = 900;
const INIT_WINDOW_HEIGHT: u32 = 700;

fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    let mut viewport =
        render_gl::Viewport::for_window(INIT_WINDOW_WIDTH as i32, INIT_WINDOW_HEIGHT as i32);

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

    let background_color = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    background_color.set_used(&gl);

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    viewport.set_used(&gl);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            // handle user input here
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                }
                _ => {}
            }
        }

        background_color.clear(&gl);

        let triangle = triangle::Triangle::new(&res, &gl)?;
        triangle.render(&gl);

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
