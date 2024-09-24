use crate::camera::Camera;
use crate::object::{create_program, IBO, VAO, VBO};
use crate::texture::Texture;
use crate::transform::Transform;
use crate::win_sdl::WinSDL;
use gl::types::{GLboolean, GLchar, GLenum, GLint, GLsizei, GLuint};
use nalgebra_glm::*;
use sdl2::event::{Event, WindowEvent};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr::null;
use crate::shape_data::ShapeData;
//use std::env;

mod win_sdl;
mod object;
mod camera;
mod transform;
mod texture;
mod shape_data;

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 900;

fn main() {
    //println!("Current working directory: {:?}", env::current_dir());
    let mut win_sdl = WinSDL::new(WIDTH, HEIGHT).unwrap();
    unsafe { gl::Viewport(0, 0, WIDTH as GLsizei, HEIGHT as GLsizei); }


    let mut texture = Texture::new();
    let bricks = texture.load_texture("./src/textures/brick.jpg");
    let skybox_texture = texture.load_cube_map_texture(vec!["./src/textures/right.jpg".to_string(), "./src/textures/left.jpg".to_string(), "./src/textures/top.jpg".to_string(), "./src/textures/bottom.jpg".to_string(), "./src/textures/front.jpg".to_string(), "./src/textures/back.jpg".to_string()]);



    //--------------------------------------------------------------------------------              Skybox
    let mut skybox_program = create_program("./src/shaders/skybox_vertex.glsl", "./src/shaders/skybox_fragment.glsl").unwrap();
    skybox_program.use_program();

    let vbo = VBO::generate();
    vbo.set(&ShapeData::get_cube_vertices());

    let vao = VAO::generate();
    vao.set();

    let ibo = IBO::generate();
    ibo.set(&ShapeData::get_cube_indices());

    skybox_program.add_uniform("u_matrix_projection");
    skybox_program.add_uniform("u_matrix_camera");
    skybox_program.add_uniform("u_matrix_transform");
    //--------------------------------------------------------------------------------

    //--------------------------------------------------------------------------------
    let mut program = create_program("./src/shaders/main_vertex.glsl", "./src/shaders/main_fragment.glsl").unwrap();
    program.use_program();

    let vbo = VBO::generate();
    vbo.set(&ShapeData::get_cube_vertices());

    let vao = VAO::generate();
    vao.set();

    let ibo = IBO::generate();
    ibo.set(&ShapeData::get_cube_indices());

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
        gl::Enable(gl::DEPTH_TEST);
        //gl::Enable(gl::CULL_FACE);
        //gl::CullFace(gl::FRONT_FACE);
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(gl_debug_callback), null());
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


            skybox_program.use_program();
            skybox_program.set_mat4("u_matrix_projection", &camera.get_projection());
            skybox_program.set_mat4("u_matrix_camera", &camera.get_camera_look_at());
            skybox_program.set_mat4("u_matrix_transform", &matrix_transform);

            transform.set_position(camera.get_camera_position());
            transform.set_scale(vec3(2.0, 2.0, 2.0));
            transform.set_euler_angles(vec3(0.0, 0.0, 0.0));
            transform.update();
            matrix_transform = transform.get_matrix();
            skybox_program.set_mat4("u_matrix_transform", &matrix_transform);

            gl::DepthMask(GLboolean::from(false));
            texture.activate_cube_map_texture(skybox_texture);
            gl::DrawElements(gl::TRIANGLES, ShapeData::get_cube_indices().len() as GLint, gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
            gl::DepthMask(GLboolean::from(true));



            program.use_program();
            program.set_mat4("u_matrix_projection", &camera.get_projection());
            program.set_mat4("u_matrix_camera", &camera.get_camera_look_at());
            program.set_mat4("u_matrix_transform", &matrix_transform);
            program.set_texture("custom_texture", 0);


            for x in 0..16 {
                for z in 0..16 {
                    for y in 0..2 {
                        transform.set_position(vec3(x as f32, y as f32, z as f32));
                        transform.set_scale(vec3(0.5, 0.5, 0.5));
                        transform.set_euler_angles(vec3(0.0, 0.0, 0.0));
                        transform.update();
                        matrix_transform = transform.get_matrix();

                        program.set_mat4("u_matrix_transform", &matrix_transform);

                        texture.activate_texture(gl::TEXTURE0, bricks);
                        gl::DrawElements(gl::TRIANGLES, ShapeData::get_cube_indices().len() as GLint, gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);

                    }
                }
            }


        }

        win_sdl.window.gl_swap_window();
    }
}

extern "system" fn gl_debug_callback(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    let message = unsafe { CStr::from_ptr(message).to_string_lossy().into_owned() };
    println!("GL CALLBACK: source = {}, type = {}, id = {}, severity = {}, message = {}", source, type_, id, severity, message);
}
