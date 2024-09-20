use gl::COLOR_BUFFER_BIT;
use sdl2::event::Event;
use crate::win_sdl::WinSDL;

mod win_sdl;

fn main() {
    println!("Hello, world!");
    let mut win_sdl = WinSDL::new(960, 540).unwrap();

    'running: loop {
        for event in win_sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(1.0, 0.2, 1.0, 1.0);
            gl::Clear(COLOR_BUFFER_BIT);
        }

        win_sdl.window.gl_swap_window();
    }
}
