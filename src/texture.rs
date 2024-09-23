use std::collections::HashMap;
use std::path::Path;
use gl::types::{GLenum, GLuint};
use image::GenericImageView;

pub struct Texture {
    texture_list: HashMap<String, GLuint> // file name: texture id
}

impl Texture {
    
    pub fn new() -> Self {
        Texture{ texture_list: HashMap::new() }
    }
    pub fn load_texture(&mut self, file_path: &str) -> GLuint {
        let img = image::open(&Path::new(file_path)).expect(format!("Failed to load texture: {}", file_path).as_str());
        let data = img.flipv().to_rgba8();
        let (width, height) = img.dimensions();
        let byte_array = data.into_raw();

        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                byte_array.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        self.texture_list.insert(file_path.to_string(), texture_id);
        texture_id
    }

    pub fn activate_texture(&self, texture_level: GLenum, texture_id: GLuint) { // TEXTURE0,1,2..15, GLuint
        unsafe {
            gl::ActiveTexture(texture_level);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }
    }

    pub fn delete_texture(&self, texture_id: GLuint) {
        unsafe {
            gl::DeleteTextures(1, texture_id as *const GLuint);
        }
    }
}