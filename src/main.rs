use crate::object::{create_program, IBO, VAO, VBO};
use crate::win_sdl::WinSDL;
use gl::types::GLint;
use sdl2::event::Event;
//use std::env;

mod win_sdl;
mod object;

fn main() {
    //println!("Current working directory: {:?}", env::current_dir());
    let mut win_sdl = WinSDL::new(1800, 900).unwrap();
    unsafe { gl::Viewport(0, 0, 1800, 900); }

    let program = create_program("./src/main_vertex.glsl", "./src/main_fragment.glsl").unwrap();
    program.use_program();

    let vertices: Vec<f32> = vec![
        0.0, 0.8,
        -0.5, -0.8,
        0.5, -0.8
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2
    ];

    let vbo = VBO::generate();
    vbo.set(&vertices);

    let vao = VAO::generate();
    vao.set();

    let ibo = IBO::generate();
    ibo.set(&indices);

    'running: loop {
        for event in win_sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(1.0, 0.9, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(gl::TRIANGLES, indices.len() as GLint, gl::UNSIGNED_INT, 0 as *const _)
        }

        win_sdl.window.gl_swap_window();
    }
}
