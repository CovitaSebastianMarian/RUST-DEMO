use crate::seb::seb::{ToCStr, create_shader_from};
use nalgebra_glm as glm;

pub struct BlackHole {
    vao: u32,
    indices_len: i32,
    ssbo: u32,
    shader_id: u32,
    pub texture_id_1: u32,
    pub texture_id_2: u32,
}

impl BlackHole {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        let mut tvbo = 0;
        let mut ssbo = 0;
        let mut shader_id = 0;

        let vert: [f32; 12] = [
            -1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, -1.0,
        ];

        let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];

        let tcoords: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];

        let mut data: Vec<f32> = Vec::new();
        for i in 0..1000 {
            let v = i as f32 / 1000.0;
            data.push(v);
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vert.len() * std::mem::size_of::<f32>()) as isize,
                vert.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut tvbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, tvbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (tcoords.len() * std::mem::size_of::<f32>()) as isize,
                tcoords.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);

            // gl::GenBuffers(1, &mut ssbo);
            // gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);

            // gl::BufferData(
            //     gl::SHADER_STORAGE_BUFFER,
            //     (data.len() * std::mem::size_of::<f32>()) as isize,
            //     data.as_ptr() as *const _,
            //     gl::DYNAMIC_DRAW,
            // );

            // gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, ssbo);
            // gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let vshader = std::fs::read_to_string("./assets/blackhole/vertex.glsl").unwrap();
        let fshader = std::fs::read_to_string("./assets/blackhole/fragment2.glsl").unwrap();
        shader_id = create_shader_from(vshader.as_str(), fshader.as_str());

        Self {
            vao,
            indices_len: indices.len() as i32,
            ssbo,
            shader_id,
            texture_id_1: 0,
            texture_id_2: 0,
        }
    }
    pub fn load_texture(&mut self, path: &str) -> Result<u32, String> {
        let mut texture_id = 0;
        let result = stb_image::image::load(path);
        let (width, height, data, nr_channels) = match result {
            stb_image::image::LoadResult::ImageU8(img) => {
                let channels = img.data.len() / (img.width as usize * img.height as usize);
                (
                    img.width as i32,
                    img.height as i32,
                    img.data,
                    channels as i32,
                )
            }
            stb_image::image::LoadResult::Error(e) => {
                return Err(format!("Failed to load image: {}", e));
            }
            _ => return Err("Unsupported image format".to_string()),
        };

        let img_format = match nr_channels {
            3 => gl::RGB,
            4 => gl::RGBA,
            _ => return Err("Unsupported image format".to_string()),
        };
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                img_format as i32,
                width,
                height,
                0,
                img_format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::ActiveTexture(0);
        }
        Ok(texture_id)
    }
    pub fn draw(
        &self,
        projection: glm::Mat4,
        view: glm::Mat4,
        window_width: u32,
        window_height: u32,
        time: f32,
    ) {
        let mut model = glm::Mat4::identity();
        model = glm::scale(&model, &glm::vec3(10.0, 10.0, 10.0));

        unsafe {
            gl::UseProgram(self.shader_id);

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_id, "projection".c_str().as_ptr()),
                1,
                gl::FALSE,
                projection.as_ptr(),
            );

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_id, "view".c_str().as_ptr()),
                1,
                gl::FALSE,
                view.as_ptr(),
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_id, "model".c_str().as_ptr()),
                1,
                gl::FALSE,
                model.as_ptr(),
            );

            let size_x = 200.0;
            let size_y = 200.0;
            let black_hole_x = 100.0;
            let black_hole_y = 100.0;
            let black_hole_radius = 50.0;

            gl::Uniform1f(
                gl::GetUniformLocation(self.shader_id, "iTime".c_str().as_ptr()),
                time,
            );
            gl::Uniform2f(
                gl::GetUniformLocation(self.shader_id, "iResolution".c_str().as_ptr()),
                size_x,
                size_y,
            );
            // gl::Uniform2f(
            //     gl::GetUniformLocation(self.shader_id, "iBlackHolePos".c_str().as_ptr()),
            //      black_hole_x / size_x,
            //      black_hole_y / size_y,
            // );
            // gl::Uniform1f(
            //     gl::GetUniformLocation(self.shader_id, "iBlackHoleMass".c_str().as_ptr()),
            //     10f32,
            // );
            // gl::Uniform1f(
            //     gl::GetUniformLocation(self.shader_id, "iBlackHoleRadius".c_str().as_ptr()),
            //     black_hole_radius / size_x,
            // );

            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id_1);
            gl::Uniform1i(
                gl::GetUniformLocation(self.shader_id, "iChannel0".c_str().as_ptr()),
                0,
            );
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id_2);
            gl::Uniform1i(
                gl::GetUniformLocation(self.shader_id, "iChannel1".c_str().as_ptr()),
                1,
            );

            gl::DrawElements(
                gl::TRIANGLES,
                self.indices_len,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}
