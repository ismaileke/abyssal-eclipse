use crate::camera::Camera;
use crate::object::{create_program, IBO, VAO, VBO};
use crate::win_sdl::WinSDL;
use gl::types::{GLint, GLsizei};
use nalgebra_glm::*;
use sdl2::event::{Event, WindowEvent};
use crate::texture::Texture;
use crate::transform::Transform;
//use std::env;

mod win_sdl;
mod object;
mod camera;
mod transform;
mod texture;

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 900;

fn main() {
    //println!("Current working directory: {:?}", env::current_dir());
    let mut win_sdl = WinSDL::new(WIDTH, HEIGHT).unwrap();
    unsafe { gl::Viewport(0, 0, WIDTH as GLsizei, HEIGHT as GLsizei); }


    let mut texture = Texture::new();

    let brick_texture_id = texture.load_texture("./src/textures/brick.jpg");



    //--------------------------------------------------------------------------------
    let length = 1.0;
    let vertices: Vec<f32> = vec![
        -length, -length, -length, 1.0, 0.0,  // 0
        length, -length, -length, 1.0, 0.0,   // 1
        length, -length, length, 1.0, 0.0,    // 2
        -length, -length, length, 1.0, 0.0,   // 3
        -length, length, -length, 1.0, 0.0,   // 4
        length, length, -length, 1.0, 0.0,    // 5
        length, length, length, 1.0, 0.0,     // 6
        -length, length, length, 1.0, 0.0,    // 7

    ];

    let indices: Vec<u32> = vec![
        4, 0, 3, 3, 7, 4,   // left
        7, 3, 2, 2, 6, 7,   // front
        6, 2, 1, 1, 5, 6,   // right
        5, 1, 0, 0, 4, 5,   // back
        4, 7, 6, 6, 5, 4,   // top
        3, 0, 1, 1, 2, 3,   // bottom
    ];

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
    program.add_uniform("custom_texture");
    //--------------------------------------------------------------------------------








    let mut camera = Camera::new(vec3(0.0, 0.0, 2.0), 4.0, 1.0);
    camera.set_projection(120.0, 0.1, 100.0);

    let mut transform = Transform::new();
    transform.update();
    let mut matrix_transform = transform.get_matrix();



    let mut last_frame_time= win_sdl.sdl.timer().unwrap().ticks();

    unsafe {
        gl::Enable(gl::DEPTH);
        gl::Enable(gl::CULL_FACE);
    }

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
            program.set_texture("custom_texture", 0);


            for x in 0..16 {
                for z in 0..2 {
                    for y in 0..2 {
                        transform.set_position(vec3(x as f32, y as f32, z as f32));
                        transform.set_scale(vec3(1.0, 1.0, 1.0));
                        transform.set_euler_angles(vec3(0.0, 0.0, 0.0));
                        transform.update();
                        matrix_transform = transform.get_matrix();

                        program.set_mat4("u_matrix_transform", &matrix_transform);

                        gl::DrawElements(gl::TRIANGLES, indices.len() as GLint, gl::UNSIGNED_INT, 0 as *const _);
                        texture.activate_texture(gl::TEXTURE0, brick_texture_id);
                    }
                }
            }


        }

        win_sdl.window.gl_swap_window();
    }
}
