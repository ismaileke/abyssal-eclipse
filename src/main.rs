use crate::camera::Camera;
use crate::object::{create_program, IBO, VAO, VBO};
use crate::shape_data::ShapeData;
use crate::texture::Texture;
use crate::transform::Transform;
use crate::win_sdl::WinSDL;
use gl::types::{GLboolean, GLchar, GLenum, GLint, GLsizei, GLuint};
use nalgebra_glm::*;
use sdl2::event::{Event, WindowEvent};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr::null;
use crate::bounding_box::{BoundingBox, Player};
//use std::env;

mod win_sdl;
mod object;
mod camera;
mod transform;
mod texture;
mod shape_data;
mod bounding_box;

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 900;

/*const GRID_SIZE: usize = 100;
const SCALE: f64 = 0.05;     // Gürültü ölçeği (dağları daha küçük yapmak için)
const HEIGHT_MULTIPLIER: f32 = 2.0;*/

fn main() {
    //println!("Current working directory: {:?}", env::current_dir());
    let mut win_sdl = WinSDL::new(WIDTH, HEIGHT).unwrap();
    unsafe { gl::Viewport(0, 0, WIDTH as GLsizei, HEIGHT as GLsizei); }


    let mut texture: Texture = Texture::new();
    let image_array: Vec<GLuint> = vec![
        texture.load_texture("./src/textures/gold_ore.png"),
        texture.load_texture("./src/textures/gold_block.png"),
        texture.load_texture("./src/textures/dirt.png"),
        texture.load_texture("./src/textures/glass.png"),
        texture.load_texture("./src/textures/netherrack.png"),
        texture.load_texture("./src/textures/yellow_wool.png"),
        texture.load_texture("./src/textures/granite.png"),
        texture.load_texture("./src/textures/brown_wool.png"),
        texture.load_texture("./src/textures/blue_terracotta.png"),
        texture.load_texture("./src/textures/blue_wool.png"),
        texture.load_texture("./src/textures/jungle_planks.png"),
        texture.load_texture("./src/textures/iron_ore.png"),
        texture.load_texture("./src/textures/red_sand.png"),
        texture.load_texture("./src/textures/red_nether_bricks.png"),
        texture.load_texture("./src/textures/redstone_block.png"),
        texture.load_texture("./src/textures/warped_wart_block.png")

    ];
    let skybox_texture: GLuint = texture.load_cube_map_texture(vec!["./src/textures/right.jpg".to_string(), "./src/textures/left.jpg".to_string(), "./src/textures/top.jpg".to_string(), "./src/textures/bottom.jpg".to_string(), "./src/textures/front.jpg".to_string(), "./src/textures/back.jpg".to_string()]);



    //--------------------------------------------------------------------------------              Skybox
    let mut skybox_program = create_program("./src/shaders/skybox_vertex.glsl", "./src/shaders/skybox_fragment.glsl").unwrap();
    skybox_program.use_program();

    let vbo = VBO::generate();
    vbo.set(&ShapeData::get_cube_vertices());

    let vao = VAO::generate();
    vao.set(false);

    let ibo = IBO::generate();
    ibo.set(&ShapeData::get_cube_indices());

    skybox_program.add_uniform("u_matrix_projection");
    skybox_program.add_uniform("u_matrix_camera");
    skybox_program.add_uniform("u_matrix_transform");
    //--------------------------------------------------------------------------------




    //--------------------------------------------------------------------------------              Crosshair
    let mut crosshair_program = create_program("./src/shaders/crosshair_vertex.glsl", "./src/shaders/crosshair_fragment.glsl").unwrap();
    crosshair_program.use_program();

    let crosshair_vbo = VBO::generate();
    crosshair_vbo.set(&ShapeData::get_crosshair_vertices());

    let crosshair_vao = VAO::generate();
    crosshair_vao.set(true); // edit this func later
    //--------------------------------------------------------------------------------




    //--------------------------------------------------------------------------------
    let mut program = create_program("./src/shaders/main_vertex.glsl", "./src/shaders/main_fragment.glsl").unwrap();
    program.use_program();

    //let (rand_vertices, rand_indices) = create_perlin_noise_grid_with_tex_coords(GRID_SIZE);

    let normal_vbo = VBO::generate();
    normal_vbo.set(&ShapeData::get_cube_vertices());
    //normal_vbo.set(&rand_vertices);

    let normal_vao = VAO::generate();
    normal_vao.set(false);

    let normal_ibo = IBO::generate();
    normal_ibo.set(&ShapeData::get_cube_indices());
    //normal_ibo.set(&rand_indices);

    program.add_uniform("u_matrix_projection");
    program.add_uniform("u_matrix_camera");
    program.add_uniform("u_matrix_transform");
    program.add_uniform("custom_texture");
    //--------------------------------------------------------------------------------




    // BOUNDING BOX...
    //let mut player = Player::new(0.0, 0.0, 0.0, 0.5, 1.0, 0.5);
    let mut blocks = vec![];
    /////////////////





    let mut camera = Camera::new(vec3(0.0, 0.0, 0.0), 4.0, 0.7);
    camera.set_projection(120.0, 0.1, 100.0);

    let mut transform = Transform::new();
    transform.update();
    let mut matrix_transform = transform.get_matrix();



    let mut last_frame_time= win_sdl.sdl.timer().unwrap().ticks();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        //gl::Enable(gl::CULL_FACE);
        //gl::Enable(gl::LINE_SMOOTH);
        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(gl_debug_callback), null());
    }

    'running: loop {

        let current_frame_time = win_sdl.sdl.timer().unwrap().ticks();

        let delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;

        last_frame_time = current_frame_time;



        camera.inputs(&win_sdl, delta_time);
        camera.update_camera_look_at();


        let mut player = Player::new(camera.get_camera_position().x, camera.get_camera_position().y, camera.get_camera_position().z, 0.5, 1.0, 0.5);
        player.update(&blocks, delta_time);


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




            // SKYBOX
            gl::Disable(gl::CULL_FACE);
            vao.bind();
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
            gl::Enable(gl::CULL_FACE);




            // CROSSHAIR
            crosshair_program.use_program();
            crosshair_vao.bind();
            gl::LineWidth(3.0);
            gl::DrawArrays(gl::LINES, 0, 2);  // First two points (vertical)
            gl::DrawArrays(gl::LINES, 2, 2);  // Last two points (horizontal)
            gl::LineWidth(1.0);





            // BLOCKS
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
            program.set_texture("custom_texture", 0);

            normal_vao.bind();
            /*texture.activate_texture(gl::TEXTURE0, image_array[0]);
            gl::DrawElements(gl::TRIANGLES, rand_indices.len() as GLint, gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);*/


            for x in 0..16 {
                for z in 0..16 {
                    for y in 0..1 {

                        let block = BoundingBox::new(x as f32, y as f32, z as f32, 0.5, 0.5, 0.5);

                        if !blocks.contains(&block) {
                            blocks.push(block);
                        }

                        transform.set_position(vec3(x as f32, y as f32, z as f32));
                        transform.set_scale(vec3(0.5, 0.5, 0.5));
                        transform.set_euler_angles(vec3(0.0, 0.0, 0.0));
                        transform.update();
                        matrix_transform = transform.get_matrix();

                        program.set_mat4("u_matrix_transform", &matrix_transform);

                        texture.activate_texture(gl::TEXTURE0, image_array[z]);
                        gl::DrawElements(gl::TRIANGLES, ShapeData::get_cube_indices().len() as GLint, gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);

                    }
                }
            }
            /*let player_bounding_box = player.bounding_box.clone();
            if blocks.iter().any(|block| player_bounding_box.intersects(block)) {
                // Çarpışma var, gerekli işlemleri yap
                // Örneğin:
                // - Oyuncuyu geri konumlandır
                // - Hızını sıfırla
                // - Çarpışma efekti oynat
            } else {
                // Çarpışma yok, normal harekete devam et
            }*/




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

/*fn create_perlin_noise_grid_with_tex_coords(grid_size: usize) -> (Vec<f32>, Vec<u32>) {
    let perlin = Perlin::new(0); // Perlin Noise oluştur
    let mut vertices: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = i as f32;
            let z = j as f32;

            // Perlin Noise kullanarak yumuşak yükseklikler
            let y = perlin.get([x as f64 * SCALE, z as f64 * SCALE]) as f32;

            // Yüksekliği artırmak ve karesini alarak "dağ" yapısı oluşturmak
            let height = y * HEIGHT_MULTIPLIER;
            let height_transformed = height * height; // Yükseklik karesi

            // Doku koordinatlarını 0.0 ile 1.0 arasında ayarla
            let s = i as f32 / (grid_size - 1) as f32;
            let t = j as f32 / (grid_size - 1) as f32;

            vertices.push(x);
            vertices.push(height_transformed); // Yükseklik değeri burada
            vertices.push(z);
            vertices.push(s); // Doku koordinat s
            vertices.push(t); // Doku koordinat t
        }
    }

    // Index verileri (triangle strip)
    for i in 0..(grid_size - 1) {
        for j in 0..(grid_size - 1) {
            let top_left = (i * grid_size + j) as u32;
            let top_right = (i * grid_size + j + 1) as u32;
            let bottom_left = ((i + 1) * grid_size + j) as u32;
            let bottom_right = ((i + 1) * grid_size + j + 1) as u32;

            indices.push(top_left);
            indices.push(bottom_left);
            indices.push(top_right);

            indices.push(top_right);
            indices.push(bottom_left);
            indices.push(bottom_right);
        }
    }

    (vertices, indices)
}*/