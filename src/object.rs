use gl::types::{GLchar, GLenum, GLint, GLsizeiptr, GLuint};
use std::ffi::{CStr, CString};
use std::fs::read_to_string;
use std::ptr::{null, null_mut};

pub struct Program {
    id: GLuint
}

impl Program {
    fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()); }
        }

        unsafe { gl::LinkProgram(id); }

        let mut success: GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success); }

        if success == 0 { // An error occurred
            let mut len: GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len); }

            let error: CString = create_whitespace_cstring_with_len(len as usize);

            unsafe { gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar); }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Program{ id })
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id); }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}


pub struct Shader {
    id: GLuint
}

impl Shader {
    pub fn from_source(source: &CStr, shader_type: GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success); }

        if success == 0 { // An error occurred
            let mut len: GLint = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); }

            let error: CString = create_whitespace_cstring_with_len(len as usize);

            unsafe { gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar); }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader{ id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}


pub fn create_program(vertex_path: &str, fragment_path: &str) -> Result<Program, &'static str> {
    let vertex_shader_source = read_to_string(vertex_path).map_err(|_| "Failed to read vertex shader file")?;
    let fragment_shader_source = read_to_string(fragment_path).map_err(|_| "Failed to read fragment shader file")?;

    let vertex_shader = Shader::from_source(&CString::new(vertex_shader_source).unwrap(), gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::from_source(&CString::new(fragment_shader_source).unwrap(), gl::FRAGMENT_SHADER).unwrap();

    let program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

    Ok(program)
}


pub struct VBO {
    pub id: GLuint
}

impl VBO {
    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }

        VBO{ id }
    }

    pub fn set(&self, vertices: &Vec<f32>) {
        self.bind();
        self.data(vertices);
    }

    fn data(&self, vertices: &Vec<f32>) {
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * size_of::<f32>()) as GLsizeiptr, vertices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}


pub struct IBO {
    pub id: GLuint
}

impl IBO {
    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }

        IBO{ id }
    }

    pub fn set(&self, indices: &Vec<u32>) {
        self.bind();
        self.data(indices);
    }

    fn data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * size_of::<u32>()) as GLsizeiptr, indices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

impl Drop for IBO {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}



pub struct VAO {
    pub id: GLuint
}

impl VAO {
    pub fn generate() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id); }

        VAO { id }
    }

    pub fn set(&self) {
        self.bind();
        self.setup();
    }

    fn setup(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (2 * size_of::<f32>()) as GLint, null());
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id); }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}