use sdl2::video::{GLContext, GLProfile, SwapInterval, Window};
use sdl2::{EventPump, Sdl};
use std::os::raw::c_void;

pub struct WinSDL {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub gl: (),
    pub event_pump: EventPump
}

impl WinSDL {
    pub fn new(width: u32, height: u32) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(4, 3);

        gl_attr.set_multisample_samples(4); // 4x MSAA

        let window = video_subsystem.window("Abyssal Eclipse", width, height).resizable().opengl().build().unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const c_void
        });

        window.subsystem().gl_set_swap_interval(SwapInterval::VSync).unwrap();

        let event_pump = sdl.event_pump().unwrap();
        Ok(WinSDL{
            sdl,
            window,
            gl_context,
            gl,
            event_pump,
        })
    }
}