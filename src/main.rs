use crate::camera::Camera;
use crate::object::{create_program, IBO, VAO, VBO};
use crate::win_sdl::WinSDL;
use gl::types::{GLint, GLsizei};
use nalgebra_glm::*;
use sdl2::event::{Event, WindowEvent};
use crate::transform::Transform;
//use std::env;

mod win_sdl;
mod object;
mod camera;
mod transform;

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 900;

fn main() {
    //println!("Current working directory: {:?}", env::current_dir());
    let mut win_sdl = WinSDL::new(WIDTH, HEIGHT).unwrap();
    unsafe { gl::Viewport(0, 0, WIDTH as GLsizei, HEIGHT as GLsizei); }


    let vertices: Vec<f32> = vec![
        0.1, 0.8, 0.3,
        -0.5, -0.8, -0.4,
        0.5, -0.8, 1.0,
        0.4, 0.5, 0.3,
        -0.5, -0.4, -0.2,
        0.8, -0.8, 1.0,
        0.4, 0.8, 0.3,
        -0.5, -0.8, -0.4,
        0.5, -0.8, 1.0,
        0.4, 0.5, 0.3,
        -0.5, -0.4, -0.2,
        0.8, -0.8, 1.0,

    ];

    let indices: Vec<u32> = vec![
        0, 1, 2, 3, 4, 5, 4, 6, 7, 8, 9, 10
    ];

    // PROGRAM 1
    let mut program = create_program("./src/shaders/main_vertex.glsl", "./src/shaders/main_fragment.glsl").unwrap();
    program.use_program();

    let vbo = VBO::generate();
    vbo.set(&vertices);

    let vao = VAO::generate();
    vao.set();

    let ibo = IBO::generate();
    ibo.set(&indices);

    program.add_uniform("u_matrix_projection");
    program.add_uniform("u_matrix_camera");
    program.add_uniform("u_matrix_transform");
    //--------------------------------------------------------------------------------

    let mut camera = Camera::new(vec3(0.0, 0.0, 2.0), 4.0, 1.0);
    camera.set_projection(120.0, 0.1, 100.0);

    let mut transform = Transform::new();
    transform.update();
    let mut matrix_transform = transform.get_matrix();



    let mut last_frame_time= win_sdl.sdl.timer().unwrap().ticks();

    unsafe { gl::Enable(gl::DEPTH); }

    'running: loop {
        let current_frame_time = win_sdl.sdl.timer().unwrap().ticks();

        let delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;

        last_frame_time = current_frame_time;

        for event in win_sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        unsafe { gl::Viewport(0, 0, width, height); }
                    }
                },
                _ => {}
            }
        }



        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            camera.inputs(&win_sdl, delta_time);
            camera.update_camera_look_at();



            program.use_program();
            program.set_mat4("u_matrix_projection", &camera.get_projection());
            program.set_mat4("u_matrix_camera", &camera.get_camera_look_at());
            program.set_mat4("u_matrix_transform", &matrix_transform);




            transform.set_position(vec3(0.0, 0.0, 0.0));
            transform.set_scale(vec3(1.0, 1.0, 1.0));
            transform.set_euler_angles(vec3(0.0, 0.0, 0.0));
            transform.update();
            matrix_transform = transform.get_matrix();

            program.set_mat4("u_matrix_transform", &matrix_transform);

            gl::DrawElements(gl::TRIANGLES, indices.len() as GLint, gl::UNSIGNED_INT, 0 as *const _);
        }

        win_sdl.window.gl_swap_window();
    }
}
