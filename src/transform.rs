use nalgebra_glm::{identity, rotate, scale, translate, vec3, Mat4, Vec3};

pub struct Transform {
    object_transform: Mat4,
    object_position: Vec3,
    object_scale: Vec3,
    object_euler_angles: Vec3
}

impl Transform {
    pub fn new() -> Self {
        Transform{
            object_transform: identity(),
            object_position: vec3(0.0, 0.0, 0.0),
            object_scale: vec3(1.0, 1.0, 1.0),
            object_euler_angles: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        self.object_transform
    }

    pub fn get_position(&self) -> Vec3 {
        self.object_position
    }

    pub fn get_scale(&self) -> Vec3 {
        self.object_scale
    }

    pub fn get_euler_angles(&self) -> Vec3 {
        self.object_euler_angles
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.object_position = position
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.object_scale = scale
    }

    pub fn set_euler_angles(&mut self, euler_angles: Vec3) {
        self.object_euler_angles = euler_angles
    }

    pub fn update(&mut self) {
        let identity_matrix = identity::<f32, 4>();

        let mtx_translate = translate(&identity_matrix, &self.object_position);
        let mtx_scale = scale(&identity_matrix, &self.object_scale);

        let mtx_rot_x = rotate(&identity_matrix, self.object_euler_angles.x.to_radians(), &vec3(1.0, 0.0, 0.0));
        let mtx_rot_y = rotate(&identity_matrix, self.object_euler_angles.y.to_radians(), &vec3(0.0, 1.0, 0.0));
        let mtx_rot_z = rotate(&identity_matrix, self.object_euler_angles.z.to_radians(), &vec3(0.0, 0.0, 1.0));

        self.object_transform = mtx_translate * (mtx_rot_x * mtx_rot_y * mtx_rot_z) * mtx_scale;
    }
}