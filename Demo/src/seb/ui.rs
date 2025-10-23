use std::collections::HashMap;

use gl::types::*;
use nalgebra_glm as glm;
use rusttype::Font;
use rusttype::Scale;

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

fn load_text_to_texture(
    font: &rusttype::Font,
    text: &str,
    scale: rusttype::Scale,
    flip_y: bool,
) -> (GLuint, u32, u32) {
    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(text, scale, rusttype::point(0.0, v_metrics.ascent))
        .collect();

    let width = glyphs
        .iter()
        .rev()
        .find_map(|g| g.pixel_bounding_box().map(|b| b.max.x as f32))
        .unwrap_or(0.0) as u32;

    let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

    let mut bitmap = vec![0u8; (width * height) as usize];

    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = (x as i32 + bb.min.x) as u32;
                let py = (y as i32 + bb.min.y) as u32;
                if px < width && py < height {
                    bitmap[(py * width + px) as usize] = (v * 255.0) as u8;
                }
            });
        }
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

    let texture_id = create_texture_from_bitmap(&bitmap, width, height);
    (texture_id, width, height)
}

use crate::seb::seb::ToCStr;
use crate::seb::seb::create_shader_from;

pub enum Origin {
    Center,
    TopLeft,
}

pub struct TextFont {
    font: Font<'static>,
    scale: Scale,
    flip_y: bool,
    keys: HashMap<char, (u32, u32, u32)>,
}
impl Drop for TextFont {
    fn drop(&mut self) {
        unsafe {
            for p in &self.keys {
                gl::DeleteTextures(1, &p.1.0);
            }
        }
    }
}
impl TextFont {
    pub fn from(font_path: &str, scale: f32, flip_y: bool) -> Self {
        let font_data = std::fs::read(font_path).expect("Eroare la citirea fontului");
        let font = rusttype::Font::try_from_vec(font_data).expect("Font invalid");

        let scale = rusttype::Scale::uniform(scale);

        let mut keys: HashMap<char, (u32, u32, u32)> = HashMap::new();
        Self {
            font: font,
            scale: scale,
            flip_y: flip_y,
            keys: keys,
        }
    }
    pub fn get_char_texture(&mut self, key: char) -> (u32, u32, u32) {
        let p = match self.keys.get(&key) {
            Some(p) => *p,
            None => {
                let glyph = self.font.glyph(key).scaled(self.scale);
                let advance = glyph.h_metrics().advance_width * 1.5f32;

                let (tid, w, h) = if key.is_whitespace() {
                    (0, advance as u32, 0)
                } else {
                    load_text_to_texture(&self.font, &key.to_string(), self.scale, self.flip_y)
                };

                self.keys.insert(key, (tid, w, h));
                (tid, w, h)
            }
        };
        p
    }
    pub fn get_text_texture(&self, text: &str) -> (u32, u32, u32) {
        load_text_to_texture(&self.font, &text, self.scale, self.flip_y)
    }
}

pub struct Border {
    pub position: glm::Vec2,
    pub size: glm::Vec2,
}
impl Border {
    pub fn from(pos: glm::Vec2, size: glm::Vec2) -> Self {
        Self {
            position: pos,
            size: size,
        }
    }
}

pub struct StaticTextBox {
    pub position: glm::Vec2,
    size: glm::Vec2,
    pub origin: Origin,
    texture_id: u32,
    pub color: glm::Vec4,
    pub angle: f32,
    pub border: Option<Border>,
}
impl Drop for StaticTextBox {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

impl StaticTextBox {
    pub fn from(font: &TextFont, text: &str) -> Self {
        let (tid, w, h) = font.get_text_texture(text);

        Self {
            position: glm::vec2(0.0, 0.0),
            size: glm::vec2(w as f32, h as f32),
            origin: Origin::Center,
            texture_id: tid,
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
            angle: 0.0,
            border: None,
        }
    }
    pub fn get_size(&self) -> glm::Vec2 {
        self.size
    }
    fn get_model(&self) -> glm::Mat4 {
        let (w, h) = match self.origin {
            Origin::Center => (0f32, 0f32),
            Origin::TopLeft => (self.size.x as f32 / 2f32, self.size.y as f32 / 2f32),
        };

        let mut model = glm::Mat4::identity();
        model = glm::translate(
            &model,
            &glm::vec3(self.position.x + w, self.position.y + h, 0.0),
        );
        model = glm::rotate(&model, self.angle, &glm::vec3(0.0, 0.0, 1.0));
        model = glm::scale(
            &model,
            &glm::vec3(self.size.x as f32 / 2f32, self.size.y as f32 / 2f32, 0.0),
        );
        model
    }
}

pub struct DynamicTextBox {
    pub text: String,
    pub position: glm::Vec2,
    pub origin: Origin,
    pub color: glm::Vec4,
    pub border: Option<Border>,
}

impl DynamicTextBox {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            position: glm::Vec2::zeros(),
            origin: Origin::Center,
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
            border: None,
        }
    }
    pub fn get_size(&self, font: &mut TextFont) -> glm::Vec2 {
        let mut size = glm::Vec2::zeros();
        for c in self.text.chars() {
            let p = font.get_char_texture(c);
            size.x += p.1 as f32;
            size.y = size.y.max(p.2 as f32);
        }
        size
    }
    fn get_model(&self, size: glm::Vec2, width: u32, ant: f32) -> glm::Mat4 {
        let (w, h) = match self.origin {
            Origin::Center => (-size.x as f32 / 2f32, 0f32),
            Origin::TopLeft => (0f32, size.y as f32 / 2f32),
        };

        let mut model = glm::Mat4::identity();
        model = glm::translate(
            &model,
            &glm::vec3(
                self.position.x + (width as f32) / 2f32 + ant + w,
                self.position.y + h,
                0.0,
            ),
        );
        model = glm::scale(
            &model,
            &glm::vec3(width as f32 / 2f32, size.y as f32 / 2f32, 1.0),
        );
        model
    }
}

pub struct TextBoxRender {
    shader: u32,
    vao: u32,
    ebo: u32,
    indices: [u32; 6],
    vbo: u32,
    tvbo: u32,
    window_width: f32,
    window_height: f32,
}
impl Drop for TextBoxRender {
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
            if self.shader != 0 {
                gl::DeleteProgram(self.shader);
            }
        }
    }
}
impl TextBoxRender {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut ebo = 0;
        let mut indices: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let mut vbo = 0;
        let mut tvbo = 0;
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

            let vert: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0];
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vert.len() * std::mem::size_of::<f32>()) as isize,
                vert.as_ptr() as *const _,
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

            let tcoord: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];

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

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let mut shader_program = 0;
        unsafe {
            let vshader = r#"
            #version 330 core
            layout(location = 0) in vec2 position;
            layout(location = 1) in vec2 TexCoord;
            out vec2 tcoord;
            uniform mat4 model;
            out vec2 world_pos;
            uniform mat4 ortho;
            void main() {
                tcoord = TexCoord;
                world_pos = (model * vec4(position, 0.0, 1.0)).xy;
                gl_Position = ortho * model * vec4(position, 0.0, 1.0);
            }
        "#;
            let fshader = r#"
            #version 330 core
            out vec4 fragColor;
            uniform sampler2D texture0;
            in vec2 tcoord;
            uniform vec4 color;
            in vec2 world_pos;
            uniform vec2 border_size;
            uniform vec2 border_pos;
            void main() {
                if (world_pos.x < border_pos.x || world_pos.x > border_pos.x + border_size.x ||
                    world_pos.y < border_pos.y || world_pos.y > border_pos.y + border_size.y) {
                    discard;
                }
                
        
                float alpha = texture(texture0, tcoord).r;
                // if (alpha < 0.05) {
                //     discard;
                // }
                // alpha = pow(alpha, 0.45);
                fragColor = vec4(color.x, color.y, color.z, alpha * color.w);
            }
        "#;
            shader_program = create_shader_from(vshader, fshader);
        }

        Self {
            shader: shader_program,
            vao: vao,
            ebo: ebo,
            indices: indices,
            vbo: vbo,
            tvbo: tvbo,
            window_width: 0.0,
            window_height: 0.0,
        }
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window_width = width as f32;
        self.window_height = height as f32;
    }
    pub fn draw_static_text(&self, textbox: &StaticTextBox) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);

            gl::UseProgram(self.shader);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, textbox.texture_id);

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "model".c_str().as_ptr()),
                1,
                gl::FALSE,
                textbox.get_model().as_ptr(),
            );
            let ortho = glm::ortho(0.0, self.window_width, self.window_height, 0.0, 0.0, 0.1);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "ortho".c_str().as_ptr()),
                1,
                gl::FALSE,
                ortho.as_ptr(),
            );
            gl::Uniform4fv(
                gl::GetUniformLocation(self.shader, "color".c_str().as_ptr()),
                1,
                textbox.color.as_ptr(),
            );

            let (bpos, bsize) = if let Some(border) = &textbox.border {
                (border.position, border.size)
            } else {
                (
                    glm::vec2(0.0, 0.0),
                    glm::vec2(self.window_width, self.window_height),
                )
            };

            gl::Uniform2fv(
                gl::GetUniformLocation(self.shader, "border_pos".c_str().as_ptr()),
                1,
                bpos.as_ptr(),
            );
            gl::Uniform2fv(
                gl::GetUniformLocation(self.shader, "border_size".c_str().as_ptr()),
                1,
                bsize.as_ptr(),
            );

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::Enable(gl::DEPTH_TEST);
        }
    }
    pub fn draw_dynamic_text(&self, font: &mut TextFont, textbox: &DynamicTextBox) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);

            gl::UseProgram(self.shader);

            let ortho = glm::ortho(0.0, self.window_width, self.window_height, 0.0, 0.0, 0.1);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "ortho".c_str().as_ptr()),
                1,
                gl::FALSE,
                ortho.as_ptr(),
            );
            gl::Uniform4fv(
                gl::GetUniformLocation(self.shader, "color".c_str().as_ptr()),
                1,
                textbox.color.as_ptr(),
            );
            let (bpos, bsize) = if let Some(border) = &textbox.border {
                (border.position, border.size)
            } else {
                (
                    glm::vec2(0.0, 0.0),
                    glm::vec2(self.window_width, self.window_height),
                )
            };

            gl::Uniform2fv(
                gl::GetUniformLocation(self.shader, "border_pos".c_str().as_ptr()),
                1,
                bpos.as_ptr(),
            );
            gl::Uniform2fv(
                gl::GetUniformLocation(self.shader, "border_size".c_str().as_ptr()),
                1,
                bsize.as_ptr(),
            );

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            let mut ant = 0;
            for c in textbox.text.chars() {
                let p = font.get_char_texture(c);

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, p.0);

                gl::UniformMatrix4fv(
                    gl::GetUniformLocation(self.shader, "model".c_str().as_ptr()),
                    1,
                    gl::FALSE,
                    textbox
                        .get_model(textbox.get_size(font), p.1, ant as f32)
                        .as_ptr(),
                );
                ant += p.1;
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }

            gl::Enable(gl::DEPTH_TEST);
        }
    }
}

pub struct Panel {
    pub position: glm::Vec2,
    pub size: glm::Vec2,
    texture_id: u32,
    use_texture: i32,
    flip_texture: i32,
    pub color: glm::Vec4,
    pub angle: f32,
    pub origin: Origin,
}
impl Drop for Panel {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

impl Panel {
    pub fn new() -> Self {
        Self {
            position: glm::Vec2::zeros(),
            size: glm::Vec2::zeros(),
            texture_id: 0,
            use_texture: 0,
            flip_texture: 0,
            color: glm::Vec4::zeros(),
            angle: 0.0,
            origin: Origin::Center,
        }
    }
    pub fn set_texture(&mut self, texture_id: u32, flip: bool) {
        self.flip_texture = if flip { 1 } else { 0 };
        self.use_texture = 1;
        self.texture_id = texture_id;
    }
    pub fn load_texture(&mut self, path: &str, flip: bool) -> Result<(), String> {
        self.flip_texture = if flip { 1 } else { 0 };
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
        self.use_texture = 1;
        unsafe {
            gl::GenTextures(1, &mut self.texture_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

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
        Ok(())
    }
    pub fn get_model(&self) -> glm::Mat4 {
        let (w, h) = match self.origin {
            Origin::Center => (0f32, 0f32),
            Origin::TopLeft => (self.size.x as f32 / 2f32, self.size.y as f32 / 2f32),
        };

        let mut model = glm::Mat4::identity();
        model = glm::translate(
            &model,
            &glm::vec3(self.position.x + w, self.position.y + h, 0.0),
        );
        model = glm::rotate(&model, self.angle, &glm::vec3(0.0, 0.0, 1.0));
        model = glm::scale(
            &model,
            &glm::vec3(self.size.x / 2f32, self.size.y / 2f32, 0.0),
        );
        model
    }
}

pub struct PanelRender {
    shader: u32,
    vao: u32,
    ebo: u32,
    indices: [u32; 6],
    vbo: u32,
    tvbo: u32,
    window_width: f32,
    window_height: f32,
}
impl Drop for PanelRender {
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
            if self.shader != 0 {
                gl::DeleteProgram(self.shader);
            }
        }
    }
}

impl PanelRender {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut ebo = 0;
        let mut indices: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let mut vbo = 0;
        let mut tvbo = 0;
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

            let vert: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0];
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vert.len() * std::mem::size_of::<f32>()) as isize,
                vert.as_ptr() as *const _,
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

            let tcoord: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];
            //let tcoord: [f32; 8] = [0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0];

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

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        let mut shader = 0;
        let vshader = r#"
            #version 330 core
            layout(location = 0) in vec2 position;
            layout(location = 1) in vec2 TexCoord;
            out vec2 tcoord;
            uniform mat4 model;
            uniform mat4 ortho;
            void main() {
                tcoord = TexCoord;
                gl_Position = ortho * model * vec4(position, 0.0, 1.0);
            }
        "#;
        let fshader = r#"
            #version 330 core
            out vec4 fragColor;
            uniform sampler2D texture0;
            in vec2 tcoord;
            uniform vec4 color;
            uniform int use_texture;
            uniform int flip_texture;
            void main() {
                if(use_texture == 1) {
                    vec2 uv = tcoord;
                    if(flip_texture == 1) {
                        uv.y = 1.0 - uv.y;
                    }
                    fragColor = texture(texture0, uv) * color;
                }
                else {
                    fragColor = color;
                }
            }
        "#;
        shader = create_shader_from(vshader, fshader);
        Self {
            shader: shader,
            vao: vao,
            ebo: ebo,
            indices: indices,
            vbo: vbo,
            tvbo: tvbo,
            window_width: 0.0,
            window_height: 0.0,
        }
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window_width = width as f32;
        self.window_height = height as f32;
    }
    pub fn draw(&self, panel: &Panel) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);

            gl::UseProgram(self.shader);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, panel.texture_id);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "model".c_str().as_ptr()),
                1,
                gl::FALSE,
                panel.get_model().as_ptr(),
            );
            let ortho = glm::ortho(0.0, self.window_width, self.window_height, 0.0, 0.0, 0.1);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "ortho".c_str().as_ptr()),
                1,
                gl::FALSE,
                ortho.as_ptr(),
            );
            gl::Uniform4fv(
                gl::GetUniformLocation(self.shader, "color".c_str().as_ptr()),
                1,
                panel.color.as_ptr(),
            );
            gl::Uniform1i(
                gl::GetUniformLocation(self.shader, "use_texture".c_str().as_ptr()),
                panel.use_texture,
            );
            gl::Uniform1i(
                gl::GetUniformLocation(self.shader, "flip_texture".c_str().as_ptr()),
                panel.flip_texture,
            );
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::Enable(gl::DEPTH_TEST);
        }
    }
}

// pub struct FrameBuffer {
//     pub framebuffer: u32,
//     pub texture_colorbuffer: u32,
//     pub renderbuffer: u32,
// }
// impl Drop for FrameBuffer {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteFramebuffers(1, &self.framebuffer);
//             gl::DeleteTextures(1, &self.texture_colorbuffer);
//             gl::DeleteRenderbuffers(1, &self.renderbuffer);
//         }
//     }
// }
// impl FrameBuffer {
//     pub fn new(width: i32, height: i32) -> Self {
//         let mut framebuffer = 0;
//         let mut texture_colorbuffer = 0;
//         let mut rbo: u32 = 0;

//         unsafe {
//             gl::GenFramebuffers(1, &mut framebuffer);
//             gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

//             gl::GenTextures(1, &mut texture_colorbuffer);
//             gl::BindTexture(gl::TEXTURE_2D, texture_colorbuffer);
//             gl::TexImage2D(
//                 gl::TEXTURE_2D,
//                 0,
//                 gl::RGB as i32,
//                 width,
//                 height,
//                 0,
//                 gl::RGB,
//                 gl::UNSIGNED_BYTE,
//                 std::ptr::null(),
//             );
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

//             gl::FramebufferTexture2D(
//                 gl::FRAMEBUFFER,
//                 gl::COLOR_ATTACHMENT0,
//                 gl::TEXTURE_2D,
//                 texture_colorbuffer,
//                 0,
//             );

//             gl::GenRenderbuffers(1, &mut rbo);
//             gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
//             gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height);
//             gl::FramebufferRenderbuffer(
//                 gl::FRAMEBUFFER,
//                 gl::DEPTH_STENCIL_ATTACHMENT,
//                 gl::RENDERBUFFER,
//                 rbo,
//             );

//             if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
//                 println!("Framebuffer is not complete!");
//             }

//             gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
//         }

//         Self {
//             framebuffer: framebuffer,
//             texture_colorbuffer: texture_colorbuffer,
//             renderbuffer: rbo,
//         }
//     }
//     pub fn bind(&self) {
//         unsafe {
//             gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
//         }
//     }
//     pub fn unbind(&self) {
//         unsafe {
//             gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
//         }
//     }
// }

pub struct FrameBuffer {
    pub id: u32,
    pub texture_colorbuffer: u32,
    pub rbo: u32,
    pub resolve_fbo: u32,
    pub resolve_texture: u32,
    width: i32,
    height: i32,
}
impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            if self.id != 0 {
                gl::DeleteFramebuffers(1, &self.id);
            }
            if self.texture_colorbuffer != 0 {
                gl::DeleteTextures(1, &self.texture_colorbuffer);
            }
            if self.rbo != 0 {
                gl::DeleteRenderbuffers(1, &self.rbo);
            }
            if self.resolve_fbo != 0 {
                gl::DeleteFramebuffers(1, &self.resolve_fbo);
            }
            if self.resolve_texture != 0 {
                gl::DeleteTextures(1, &self.resolve_texture);
            }
        }
    }
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self {
            id: 0,
            texture_colorbuffer: 0,
            rbo: 0,
            resolve_fbo: 0,
            resolve_texture: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn create(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        unsafe {
            gl::GenFramebuffers(1, &mut self.id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);

            gl::GenTextures(1, &mut self.texture_colorbuffer);
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.texture_colorbuffer);
            gl::TexImage2DMultisample(
                gl::TEXTURE_2D_MULTISAMPLE,
                4,
                gl::RGB as u32,
                width,
                height,
                gl::TRUE,
            );
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D_MULTISAMPLE,
                self.texture_colorbuffer,
                0,
            );

            gl::GenRenderbuffers(1, &mut self.rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.rbo);
            gl::RenderbufferStorageMultisample(
                gl::RENDERBUFFER,
                4,
                gl::DEPTH24_STENCIL8,
                width,
                height,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                self.rbo,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("ERROR::FRAMEBUFFER:: Multisample framebuffer is not complete!");
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            // Resolve FBO
            gl::GenFramebuffers(1, &mut self.resolve_fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.resolve_fbo);

            gl::GenTextures(1, &mut self.resolve_texture);
            gl::BindTexture(gl::TEXTURE_2D, self.resolve_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.resolve_texture,
                0,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("ERROR::FRAMEBUFFER:: Resolve framebuffer is not complete!");
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn resolve(&self) {
        let width = self.width;
        let height = self.height;
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.id);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.resolve_fbo);
            gl::BlitFramebuffer(
                0,
                0,
                width,
                height,
                0,
                0,
                width,
                height,
                gl::COLOR_BUFFER_BIT,
                gl::NEAREST,
            );
            //gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
            gl::DeleteTextures(1, &self.texture_colorbuffer);
            gl::DeleteRenderbuffers(1, &self.rbo);
            gl::DeleteFramebuffers(1, &self.resolve_fbo);
            gl::DeleteTextures(1, &self.resolve_texture);
            self.create(width, height);
        }
    }
}
