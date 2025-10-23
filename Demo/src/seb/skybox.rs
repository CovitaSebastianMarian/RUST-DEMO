
use core::f32;

use crate::seb::{
    seb::{ToCStr, create_shader_from},
};
use nalgebra_glm as glm;
use stb_image;

pub struct Skybox {
    shader_id: u32,
    texture_id: u32,
    vao: u32,
    vbo: u32,
}

impl Skybox {
    pub fn new(faces: [&str; 6]) -> Self {
        let vertex_shader = r#"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            out vec3 TexCoords;

            uniform mat4 projection;
            uniform mat4 view;

            void main() {
                TexCoords = aPos;
                gl_Position = projection * view * vec4(aPos, 1.0);
                gl_Position = gl_Position.xyww;
            }
        "#;

        let fragment_shader = r#"
            #version 330 core
            out vec4 FragColor;
            in vec3 TexCoords;

            uniform samplerCube skybox;

            void main() {
                FragColor = texture(skybox, TexCoords) * 1.5;
            }
        "#;

        let shader_id = create_shader_from(vertex_shader, fragment_shader);

        // Cube vertices
        let vertices: [f32; 108] = [
            -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0,
        ];

        // Generate VAO & VBO
        let (mut vao, mut vbo) = (0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, std::ptr::null());
        }

        // Load textures for each face
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

            for (i, path) in faces.iter().enumerate() {
                let img = match stb_image::image::load(path) {
                    stb_image::image::LoadResult::ImageU8(img) => img,
                    _ => panic!("Skybox must be an RGB image"),
                };

                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    gl::RGBA as i32,
                    img.width as i32,
                    img.height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    img.data.as_ptr() as *const _,
                );
            }

            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
        }

        Self {
            shader_id,
            texture_id,
            vao,
            vbo,
        }
    }

    pub fn draw(&self, projection: glm::Mat4, view: glm::Mat4) {
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
            gl::UseProgram(self.shader_id);

            let view_no_translation = glm::mat4_to_mat3(&view);
            let view = glm::mat3_to_mat4(&view_no_translation);

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_id, "view".c_str().as_ptr()),
                1,
                gl::FALSE,
                view.as_ptr(),
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_id, "projection".c_str().as_ptr()),
                1,
                gl::FALSE,
                projection.as_ptr(),
            );

            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
            gl::DepthFunc(gl::LESS);
        }
    }
}
