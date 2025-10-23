use nalgebra_glm as glm;

pub trait ToCStr {
    fn c_str(&self) -> std::ffi::CString;
}
impl ToCStr for str {
    fn c_str(&self) -> std::ffi::CString {
        std::ffi::CString::new(self).unwrap()
    }
}

fn check_shader_compile_status(shader: u32, shader_type: &str) {
    let mut success: i32 = 0;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len: i32 = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.set_len(len as usize);
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut _,
            );

            let error = String::from_utf8_lossy(&buffer);
            println!("{} shader compilation failed:\n{}", shader_type, error);
        }
    }
}

fn check_program_link_status(program: u32) {
    let mut success: i32 = 0;
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len: i32 = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.set_len(len as usize);
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut _,
            );

            let error = String::from_utf8_lossy(&buffer);
            println!("Shader program linking failed:\n{}", error);
        }
    }
}

pub fn create_shader_from(vshader: &str, fshader: &str) -> u32 {
    let mut shader_program = 0;
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &vshader.c_str().as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);
        check_shader_compile_status(vertex_shader, "Vertex");

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &fshader.c_str().as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);
        check_shader_compile_status(fragment_shader, "Fragment");

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        check_program_link_status(shader_program);
        gl::UseProgram(shader_program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    shader_program
}

pub struct Shader {
    pub id: u32,
    pub cnt: u32,
}
impl Drop for Shader {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteProgram(self.id);
            }
        }
    }
}
impl Shader {
    pub fn new() -> Self {
        Self { id: 0, cnt: 0 }
    }
    pub fn from_str(&mut self, vshader: &str, fshader: &str) {
        let mut shader_program = 0;
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &vshader.c_str().as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);
            check_shader_compile_status(vertex_shader, "Vertex");

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &fshader.c_str().as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);
            check_shader_compile_status(fragment_shader, "Fragment");

            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            check_program_link_status(shader_program);
            gl::UseProgram(shader_program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
        self.id = shader_program;
    }
    pub fn from_files(&mut self, vshader_file: &str, fshader_file: &str) {
        let vshader =
            std::fs::read_to_string(vshader_file).expect("Eroare la citirea din vertex file!");
        let vshader = vshader.as_str();
        let fshader =
            std::fs::read_to_string(fshader_file).expect("Eroare la citirea din fragment file!");
        let fshader = fshader.as_str();

        self.from_str(vshader, fshader);
    }
    pub fn bind(&mut self) {
        unsafe {
            gl::UseProgram(self.id);
        }
        self.cnt = 0;
    }
    pub fn set_texture(&mut self, texture_name: &str, texture_id: &u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.cnt);
            gl::BindTexture(gl::TEXTURE_2D, *texture_id);
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, texture_name.c_str().as_ptr()),
                self.cnt as i32,
            );
            self.cnt += 1;
        }
    }
    pub fn set_int(&self, name: &str, val: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.c_str().as_ptr()), val);
        }
    }
    pub fn set_float(&self, name: &str, val: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.c_str().as_ptr()), val);
        }
    }
    pub fn set_vec2(&self, name: &str, val: [f32; 2]) {
        unsafe {
            gl::Uniform2fv(
                gl::GetUniformLocation(self.id, name.c_str().as_ptr()),
                1,
                val.as_ptr(),
            );
        }
    }
    pub fn set_vec3(&self, name: &str, val: [f32; 3]) {
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, name.c_str().as_ptr()),
                1,
                val.as_ptr(),
            );
        }
    }
    pub fn set_vec4(&self, name: &str, val: [f32; 4]) {
        unsafe {
            gl::Uniform4fv(
                gl::GetUniformLocation(self.id, name.c_str().as_ptr()),
                1,
                val.as_ptr(),
            );
        }
    }
    pub fn set_mat2(&self, name: &str, val: &[f32; 4]) {
        unsafe {
            gl::UniformMatrix2fv(
                gl::GetUniformLocation(self.id, name.c_str().as_ptr()),
                1,
                gl::FALSE,
                val.as_ptr(),
            );
        }
    }
    pub fn set_mat3(&self, name: &str, val: &[f32; 9]) {
        unsafe {
            gl::UniformMatrix3fv(
                gl::GetUniformLocation(self.id, name.c_str().as_ptr()),
                1,
                gl::FALSE,
                val.as_ptr(),
            );
        }
    }
    pub fn set_mat4(&self, name: &str, val: *const f32) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, name.c_str().as_ptr()),
                1,
                gl::FALSE,
                val,
            );
        }
    }
}