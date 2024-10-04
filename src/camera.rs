use crate::win_sdl::WinSDL;
use crate::{HEIGHT, WIDTH};
use nalgebra_glm::{angle, cross, identity, look_at, normalize, perspective, rotate, rotate_vec3, scale, translate, vec3, Mat4, Vec3};
use sdl2::keyboard::Scancode;

pub struct Camera {
    camera_position: Vec3,
    camera_up: Vec3,
    orientation: Vec3,
    speed: f32,
    sensitivity: f32,
    first_click: bool,
    matrix_projection: Mat4,
    matrix_camera: Mat4
}
impl Camera {
    pub fn new(camera_position: Vec3, speed: f32, sensitivity: f32) -> Self {
        let identity_matrix = identity::<f32, 4>();
        Camera{
            camera_position,
            camera_up: vec3(0.0, 1.0, 0.0),
            orientation: vec3(0.0, 0.0, -1.0),
            speed,
            sensitivity,
            first_click: true,
            matrix_projection: identity_matrix,
            matrix_camera: identity_matrix,
        }
    }

    fn set_transform(&self, object_position: Vec3, rotate_angle: f32, rotate_direction: Vec3, object_scale: Vec3/*, is_cube: bool*/) -> Mat4 {
        let identity_matrix = identity::<f32, 4>();

        let matrix_translation = translate(&identity_matrix, &object_position);

        let matrix_rotation = rotate(&identity_matrix, rotate_angle.to_radians(), &rotate_direction);

        let matrix_scale = scale(&identity_matrix, &object_scale);

        matrix_translation * matrix_rotation * matrix_scale
    }

    pub fn set_projection(&mut self, fov_degree: f32, near_plane: f32, far_plane: f32) {
        self.matrix_projection = perspective((WIDTH / HEIGHT) as f32, fov_degree.to_radians(), near_plane, far_plane);
    }

    pub fn set_camera_position(&mut self, camera_position: Vec3) {
        self.camera_position = camera_position;
    }

    pub fn update_camera_look_at(&mut self) {
        self.matrix_camera = look_at(&self.camera_position, &(self.camera_position + self.orientation), &self.camera_up);
    }

    pub fn get_projection(&mut self) -> Mat4 {
        self.matrix_projection
    }

    pub fn get_camera_look_at(&mut self) -> Mat4 {
        self.matrix_camera
    }

    pub fn get_camera_position(&mut self) -> Vec3 {
        self.camera_position
    }

    pub fn inputs(&mut self, win_sdl: &WinSDL, delta_time: f32) {
        let keyboard_state = win_sdl.event_pump.keyboard_state();

        // W key (Forward)
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            self.camera_position += self.orientation * self.speed * delta_time;
        }
        // A key (Left)
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            self.camera_position -= normalize(&cross(&self.orientation, &self.camera_up)) * self.speed * delta_time;
        }
        // S key (Backward)
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            self.camera_position -= self.orientation * self.speed * delta_time;
        }
        // D key (Right)
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            self.camera_position += normalize(&cross(&self.orientation, &self.camera_up)) * self.speed * delta_time;
        }
        // SPACE key (Up)
        if keyboard_state.is_scancode_pressed(Scancode::Space) {
            self.camera_position += self.camera_up * self.speed * delta_time;
        }
        // LEFT CONTROL key (Down)
        if keyboard_state.is_scancode_pressed(Scancode::LCtrl) {
            self.camera_position -= self.camera_up * self.speed * delta_time;
        }
        // LEFT SHIFT key (Boost speed)
        if keyboard_state.is_scancode_pressed(Scancode::LShift) {
            self.speed = 20.0;
        } else {
            self.speed = 10.0;
        }

        // M key (Wireframe mode toggle)
        if keyboard_state.is_scancode_pressed(Scancode::M) {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        } else {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }

        win_sdl.sdl.mouse().show_cursor(false);

        if self.first_click {
            win_sdl.sdl.mouse().warp_mouse_in_window(&win_sdl.window, (WIDTH / 2) as i32, (HEIGHT / 2) as i32);
            self.first_click = false;
        }

        let mouse = win_sdl.event_pump.relative_mouse_state();
        let mouse_x = mouse.x();
        let mouse_y = mouse.y();

        let rot_x = self.sensitivity * (mouse_y as f32);
        let rot_y = self.sensitivity * (mouse_x as f32);

        let new_orientation = rotate_vec3(&self.orientation, -rot_x.to_radians(), &normalize(&cross(&self.orientation, &self.camera_up)));

        if (angle(&new_orientation, &self.camera_up) - 90f32.to_radians()).abs() <= 85f32.to_radians() {
            self.orientation = new_orientation;
        }

        self.orientation = rotate_vec3(&self.orientation, -rot_y.to_radians(), &self.camera_up);
    }
}