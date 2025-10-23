use std::ffi::c_float;

use crate::seb::gui::gui::Clip;
use crate::seb::seb::{ToCStr, create_shader_from};
use gl::types::*;
use nalgebra_glm as glm;
use rusttype::Font;
use rusttype::Scale;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CharPlacement {
    ch: char,
    x: f32,
    w: f32,
}
pub struct TextFont {
    font: Font<'static>,
    scale: Scale,
    texture_id: u32,
    width: u32,
    height: u32,
    chars_placements: Vec<CharPlacement>,
    pub spacing: f32,
}
impl Drop for TextFont {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.texture_id);
        }
    }
}
impl TextFont {
    pub fn new(font_path: &str, scale: f32) -> Self {
        let font_data = std::fs::read(font_path).expect("Eroare la incarcarea fontului!");
        let font = rusttype::Font::try_from_vec(font_data).expect("Font invalid");

        let scale = rusttype::Scale::uniform(scale);

        Self {
            font,
            scale,
            texture_id: 0,
            width: 0,
            height: 0,
            chars_placements: Vec::new(),
            spacing: 1.0,
        }
    }
    pub fn init_chars_texture(&mut self, flip_y: bool) {
        //" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        //pun un spatiu intre caractere ca sa ma asigur ca textura sa nu include pixeli si de la alte caractere
        let mut text = String::new();
        for i in 0..=94 {
            let c = (i + 32) as u8 as char;
            text.push(c);
            text.push(' ');
        }
        let text = &text.as_str();

        let v_metrics = self.font.v_metrics(self.scale);

        let glyphs: Vec<_> = self
            .font
            .layout(text, self.scale, rusttype::point(0.0, v_metrics.ascent))
            .collect();

        let width = glyphs
            .iter()
            .rev()
            .find_map(|g| g.pixel_bounding_box().map(|b| b.max.x as f32))
            .unwrap_or(0.0) as u32;

        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        let mut bitmap = vec![0u8; (width * height) as usize];

        let mut placements = Vec::new();
        let mut take = true;
        for (ch, glyph) in text.chars().zip(glyphs.iter()) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                if take {
                    placements.push(CharPlacement {
                        ch,
                        x: bb.min.x as f32,
                        w: (bb.max.x - bb.min.x) as f32,
                    });
                }

                glyph.draw(|x, y, v| {
                    let px = (x as i32 + bb.min.x) as u32;
                    let py = (y as i32 + bb.min.y) as u32;
                    if px < width && py < height {
                        bitmap[(py * width + px) as usize] = (v * 255.0) as u8;
                    }
                });
            } else {
                // caracter invizibil (ex: spațiu)
                let advance = glyph.unpositioned().h_metrics().advance_width;
                if take {
                    placements.push(CharPlacement {
                        ch,
                        x: glyph.position().x,
                        w: advance,
                    });
                }
            }
            take = !take;
        }

        // Flip vertical
        if flip_y {
            let row_len = width as usize;
            for y in 0..(height / 2) {
                let top_start = y as usize * row_len;
                let bottom_start = (height - 1 - y) as usize * row_len;

                for x in 0..row_len {
                    bitmap.swap(top_start + x, bottom_start + x);
                }
            }
        }

        self.texture_id = Self::create_texture_from_bitmap(&bitmap, width, height);
        self.width = width;
        self.height = height;
        self.chars_placements = placements;
    }
    fn create_texture_from_bitmap(bitmap: &[u8], width: u32, height: u32) -> GLuint {
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                bitmap.as_ptr() as *const _,
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        texture_id
    }
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[derive(Clone, Debug)]
pub struct TextBoxD {
    pub text: String,
    pub position: glm::Vec2,
    pub color: glm::Vec4,
    pub clip: Option<Clip>,
    pub z_index: f32,
}
impl TextBoxD {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            position: glm::Vec2::zeros(),
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
            clip: None,
            z_index: 0.0,
        }
    }
    pub fn get_size(&self, font: &TextFont) -> glm::Vec2 {
        let mut w: f32 = 0f32;
        let mut h: f32 = 0f32;

        h = font.get_size().1 as f32;

        for c in self.text.chars() {
            let mut index: i32 = (c as u8 as i32) - 32;
            if index > 94 || index < 0 {
                index = 94;
            }
            w += font.chars_placements[index as usize].w + font.spacing;
        }

        glm::vec2(w, h)
    }
    fn get_model(&self) -> glm::Mat4 {
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(self.position.x, self.position.y, 0.0));

        //model = glm::rotate_z(&model, self.angle.to_radians());
        model
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct InstanceData {
    atlas_coords: glm::Vec4,
    clip: glm::Vec4,
    color: glm::Vec4,
    z_index: glm::Vec4,
}
const CHUNK_MAX_CHARACTERS: usize = 100;
pub struct TextBoxRenderer {
    window_width: u32,
    window_height: u32,
    shader: u32,
    vao: u32,
    vbo: u32,
    ebo: u32,
    tvbo: u32,
    ivbo: u32,
    indices_len: usize,
}
impl Drop for TextBoxRenderer {
    fn drop(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
            }
            if self.tvbo != 0 {
                gl::DeleteBuffers(1, &self.tvbo);
            }
            if self.ebo != 0 {
                gl::DeleteBuffers(1, &self.ebo);
            }
            if self.ivbo != 0 {
                gl::DeleteBuffers(1, &self.ivbo);
            }
            if self.shader != 0 {
                gl::DeleteProgram(self.shader);
            }
        }
    }
}
impl TextBoxRenderer {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        let mut tvbo = 0;
        let mut ivbo = 0;

        let vertices: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let tcoord: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let indices: [u32; 6] = [0, 2, 1, 0, 3, 2];

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
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut tvbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, tvbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (tcoord.len() * std::mem::size_of::<f32>()) as isize,
                tcoord.as_ptr() as *const _,
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

            gl::GenBuffers(1, &mut ivbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, ivbo);

            let stride = std::mem::size_of::<InstanceData>();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (CHUNK_MAX_CHARACTERS * stride) as isize, //100 de caractere max
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );


            for i in 0..4 {
                gl::VertexAttribPointer(
                    2 + i,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    (i as usize * std::mem::size_of::<[f32; 4]>()) as *const _,
                );
                gl::EnableVertexAttribArray(2 + i);
                gl::VertexAttribDivisor(2 + i, 1);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let mut shader_id: u32 = 0;
        let vshader = r#"
            #version 330 core
            layout(location = 0) in vec2 position;
            layout(location = 1) in vec2 TexCoord;
            layout(location = 2) in vec4 atlas_coords;
            layout(location = 3) in vec4 clip;
            layout(location = 4) in vec4 tcolor;
            layout(location = 5) in vec4 zindex;

            uniform mat4 ortho;
            uniform float atlas_width;
            uniform float atlas_height;
            out vec2 tcoord;
            out vec2 world_pos;
            out vec4 color;
            out vec2 clip_size;
            out vec2 clip_pos;
            void main() {
                float xpos = atlas_coords.x;
                float ypos = atlas_coords.y;
                float x = atlas_coords.z;
                float w = atlas_coords.w;
                float z_index = zindex.x;

                color = tcolor;
                clip_pos = clip.xy;
                clip_size = clip.zw;
                vec2 pos = vec2(xpos, ypos) + position * vec2(w, atlas_height);
                world_pos = pos;
                gl_Position = ortho * vec4(pos, z_index, 1.0);
                float u0 = x / atlas_width;
                float u1 = (x + w) / atlas_width;
                tcoord = vec2(mix(u0, u1, TexCoord.x), TexCoord.y);
            }
        "#;
        let fshader = r#"
            #version 330 core
            out vec4 fragColor;
            in vec2 tcoord;
            in vec2 world_pos;
            in vec4 color;
            in vec2 clip_size;
            in vec2 clip_pos;
            uniform sampler2D texture0;
            void main() {
                if (world_pos.x < clip_pos.x || world_pos.x > clip_pos.x + clip_size.x ||
                    world_pos.y < clip_pos.y || world_pos.y > clip_pos.y + clip_size.y) {
                    discard;
                }
                
                float alpha = texture(texture0, tcoord).r;
                fragColor = vec4(color.x, color.y, color.z, alpha * color.w);
            }
        "#;
        shader_id = create_shader_from(vshader, fshader);

        Self {
            window_width: 0,
            window_height: 0,
            shader: shader_id,
            vao,
            vbo,
            ebo,
            tvbo,
            ivbo,
            indices_len: indices.len(),
        }
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
    }
    fn upload_instances(&self, font: &TextFont, texts: &[TextBoxD]) -> Vec<InstanceData> {
        let mut instances: Vec<InstanceData> = Vec::new();

        for text in texts {
            let mut ant: f32 = 0f32;
            let (cpos, csize) = if let Some(clip) = &text.clip {
                (clip.position, clip.size)
            } else {
                (
                    glm::Vec2::zeros(),
                    glm::vec2(self.window_width as f32, self.window_height as f32),
                )
            };

            for c in text.text.chars() {
                let mut index = (c as u8 as isize) - 32;
                if index > 94 || index < 0 {
                    index = 94;
                }
                let cp = font.chars_placements[index as usize];

                instances.push(InstanceData {
                    atlas_coords: glm::vec4(ant + text.position.x, 0.0 + text.position.y, cp.x, cp.w),
                    clip: glm::vec4(cpos.x, cpos.y, csize.x, csize.y),
                    color: text.color,
                    z_index: glm::vec4(text.z_index, 0.0, 0.0, 0.0),
                });
                ant += cp.w + font.spacing;
            }
        }
        instances
    }
    pub fn draw(&self, font: &TextFont, texts: &mut [TextBoxD]) {
        texts.sort_by(|a, b| {
            a.z_index
                .partial_cmp(&b.z_index)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut instances = self.upload_instances(font, texts);

        let ortho = glm::ortho(
            0.0,
            self.window_width as f32,
            self.window_height as f32,
            0.0,
            -1.0,
            1.0,
        );

        unsafe {
            gl::UseProgram(self.shader);

            gl::Uniform1f(
                gl::GetUniformLocation(self.shader, "atlas_width".c_str().as_ptr()),
                font.width as f32,
            );
            gl::Uniform1f(
                gl::GetUniformLocation(self.shader, "atlas_height".c_str().as_ptr()),
                font.height as f32,
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, font.texture_id);
            gl::Uniform1i(
                gl::GetUniformLocation(self.shader, "texture0".c_str().as_ptr()),
                0,
            );

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "ortho".c_str().as_ptr()),
                1,
                gl::FALSE,
                ortho.as_ptr(),
            );

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.ivbo);
            for chunk in instances.chunks(CHUNK_MAX_CHARACTERS) {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (chunk.len() * std::mem::size_of::<InstanceData>()) as isize,
                    chunk.as_ptr() as *const _,
                );

                gl::DrawElementsInstanced(
                    gl::TRIANGLES,
                    self.indices_len as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                    chunk.len() as i32,
                );
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::UseProgram(0);
        }
    }
}
