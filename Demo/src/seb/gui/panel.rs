use crate::seb::gui::gui::Clip;
use crate::seb::seb::{ToCStr, create_shader_from};
use nalgebra_glm as glm;

#[derive(Clone, Copy, Debug)]
pub struct Panel {
    pub position: glm::Vec2,
    pub angle: f32,
    pub size: glm::Vec2,
    pub color: glm::Vec4,
    pub clip: Option<Clip>,
    pub border_thickness: f32,
    pub border_color: glm::Vec4,
    pub z_index: f32,
    pub draw_as_circle: bool,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            position: glm::Vec2::zeros(),
            angle: 0f32,
            size: glm::Vec2::zeros(),
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
            clip: None,
            border_thickness: 0.0,
            border_color: glm::vec4(1.0, 1.0, 1.0, 1.0),
            z_index: 0.0,
            draw_as_circle: false,
        }
    }
    pub fn get_model(&self) -> glm::Mat4 {
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(self.position.x, self.position.y, 0.0));
        model = glm::rotate(&model, self.angle.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
        model = glm::scale(&model, &glm::vec3(self.size.x, self.size.y, 0.0));
        model
    }
    pub fn rotate_from_point(&mut self, point: glm::Vec2, angle_deg: f32) {
        // Translate invers
        let translate_to_origin =
            glm::translate(&glm::Mat4::identity(), &glm::vec3(-point.x, -point.y, 0.0));

        // Rotate
        let rotation = glm::rotate(
            &glm::Mat4::identity(),
            angle_deg.to_radians(),
            &glm::vec3(0.0, 0.0, 1.0),
        );

        // Translate înapoi
        let translate_back =
            glm::translate(&glm::Mat4::identity(), &glm::vec3(point.x, point.y, 0.0));

        // Aplicăm la poziție
        let current_pos = glm::vec4(self.position.x, self.position.y, 0.0, 1.0);
        let new_pos = translate_back * rotation * translate_to_origin * current_pos;

        self.position = glm::vec2(new_pos.x, new_pos.y);

        // Update unghiul panelului
        self.angle += angle_deg;
    }
    pub fn set_angle_from_point(&mut self, point: glm::Vec2, angle_deg: f32) {
        // Translate invers pentru punctul de rotație
        let translate_to_origin =
            glm::translate(&glm::Mat4::identity(), &glm::vec3(-point.x, -point.y, 0.0));

        // Rotate cu diferența dintre noul unghi și cel curent
        let rotation = glm::rotate(
            &glm::Mat4::identity(),
            (angle_deg - self.angle).to_radians(),
            &glm::vec3(0.0, 0.0, 1.0),
        );

        // Translate înapoi după rotație
        let translate_back =
            glm::translate(&glm::Mat4::identity(), &glm::vec3(point.x, point.y, 0.0));

        // Aplicăm transformarea asupra poziției panelului
        let current_pos = glm::vec4(self.position.x, self.position.y, 0.0, 1.0);
        let new_pos = translate_back * rotation * translate_to_origin * current_pos;
        self.position = glm::vec2(new_pos.x, new_pos.y);

        // Setăm unghiul exact
        self.angle = angle_deg;
    }
}

const CHUNK_MAX_PANELS: usize = 10;

#[repr(C)]
struct PanelInstance {
    model: [[f32; 4]; 4],     // 16 floats → mat4
    color: glm::Vec4,         // 4 floats
    border_color: glm::Vec4,  // 4 floats
    clip: glm::Vec4,          // clip_pos.xy + clip_size.xy
    size_border_z: glm::Vec4, // size.xy + border_thickness + z_index
    flgas: glm::Vec4,
}

pub struct PanelRenderer {
    shader: u32,
    vao: u32,
    ebo: u32,
    indices_len: usize,
    vbo: u32,
    ivbo: u32,
    window_width: f32,
    window_height: f32,
}
impl Drop for PanelRenderer {
    fn drop(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
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

impl PanelRenderer {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut ebo = 0;
        let mut indices: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let mut vbo = 0;
        let mut ivbo = 0;
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

            let vert: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];
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

            gl::GenBuffers(1, &mut ivbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, ivbo);

            let panel_size = std::mem::size_of::<PanelInstance>();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (CHUNK_MAX_PANELS * panel_size) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            for i in 0..9 {
                gl::VertexAttribPointer(
                    1 + i,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    panel_size as i32,
                    (i as usize * std::mem::size_of::<[f32; 4]>()) as *const _,
                );
                gl::EnableVertexAttribArray(1 + i);
                gl::VertexAttribDivisor(1 + i, 1);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        let mut shader = 0;
        let vshader = r#"
            #version 330 core
            layout(location = 0) in vec2 position;
            layout(location = 1) in mat4 model;
            layout(location = 5) in vec4 color;
            layout(location = 6) in vec4 border_color;
            layout(location = 7) in vec4 clip;
            layout(location = 8) in vec4 size_border_z;
            layout(location = 9) in vec4 flags;

            out vec4 col;
            out vec2 localPos;
            out vec2 worldPos;
            out vec2 Size;
            out float borderThickness;
            out vec4 borderColor;
            out vec2 clipPos;
            out vec2 clipSize;
            out float circle;
            uniform mat4 ortho;
            void main() {
                vec2 clip_pos = clip.xy;
                vec2 clip_size = clip.zw;
                vec2 size = size_border_z.xy;
                float border_thickness = size_border_z.z;
                float z_index = size_border_z.w;

                circle = flags.x;
                col = color;
                localPos = position * size;
                worldPos = (model * vec4(position, 0.0, 1.0)).xy;
                Size = size;
                clipPos = clip_pos;
                clipSize = clip_size;
                borderThickness = border_thickness;
                borderColor = border_color;
                gl_Position = ortho * model * vec4(position, z_index, 1.0);
            }
        "#;
        let fshader = r#"
            #version 330 core
            out vec4 fragColor;
            in vec4 col;
            in vec2 localPos;
            in vec2 worldPos;
            in vec2 Size;
            in float borderThickness;
            in vec4 borderColor;
            in vec2 clipPos;
            in vec2 clipSize;
            in float circle;
            void main() {
                if (worldPos.x < clipPos.x || worldPos.x > clipPos.x + clipSize.x ||
                    worldPos.y < clipPos.y || worldPos.y > clipPos.y + clipSize.y) {
                    discard;
                }

                if(circle == 1.0) {
                    vec2 center = Size / 2.0;
                    float a = Size.x / 2.0; // raza pe axa X
                    float b = Size.y / 2.0; // raza pe axa Y

                    // coordonate normalizate (în raport cu semi-axe)
                    float dx = (localPos.x - center.x) / a;
                    float dy = (localPos.y - center.y) / b;

                    // valoarea elipsei
                    float ellipse = dx * dx + dy * dy;

                    // în afara elipsei
                    if (ellipse > 1.0) {
                        discard;
                    }

                    // border thickness: reducem a și b cu borderThickness
                    float inner_a = max(a - borderThickness, 0.01);
                    float inner_b = max(b - borderThickness, 0.01);

                    float dx_inner = (localPos.x - center.x) / inner_a;
                    float dy_inner = (localPos.y - center.y) / inner_b;
                    float ellipse_inner = dx_inner * dx_inner + dy_inner * dy_inner;

                    // Dacă pixelul e între elipsa mare și cea mică → border
                    if (ellipse <= 1.0 && ellipse_inner >= 1.0) {
                        fragColor = borderColor;
                    } else {
                        fragColor = col;
                    }
                }
                else {
                    if (localPos.x <= borderThickness || localPos.x >= Size.x - borderThickness ||
                        localPos.y <= borderThickness || localPos.y >= Size.y - borderThickness) {
                        fragColor = borderColor;
                    } else {
                        fragColor = col;
                    }
                }
            }
        "#;
        shader = create_shader_from(vshader, fshader);
        Self {
            shader: shader,
            vao: vao,
            ebo: ebo,
            indices_len: indices.len(),
            vbo: vbo,
            ivbo: ivbo,
            window_width: 0.0,
            window_height: 0.0,
        }
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window_width = width as f32;
        self.window_height = height as f32;
    }
    pub fn draw(&self, panels: &mut [Panel]) {
        let ortho = glm::ortho(0.0, self.window_width, self.window_height, 0.0, -1.0, 1.0);

        panels.sort_by(|a, b| {
            b.z_index
                .partial_cmp(&a.z_index)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut ipanels: Vec<PanelInstance> = Vec::new();
        for p in panels {
            let (cpos, csize) = if let Some(clip) = &p.clip {
                (clip.position, clip.size)
            } else {
                (
                    glm::Vec2::zeros(),
                    glm::vec2(self.window_width as f32, self.window_height as f32),
                )
            };

            ipanels.push(PanelInstance {
                model: p.get_model().into(),
                color: p.color,
                border_color: p.border_color,
                clip: glm::vec4(cpos.x, cpos.y, csize.x, csize.y),
                size_border_z: glm::vec4(p.size.x, p.size.y, p.border_thickness, p.z_index),
                flgas: glm::vec4(p.draw_as_circle as u8 as f32, 0.0, 0.0, 0.0),
            });
        }

        unsafe {
            gl::UseProgram(self.shader);

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "ortho".c_str().as_ptr()),
                1,
                gl::FALSE,
                ortho.as_ptr(),
            );

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.ivbo);
            for chunk in ipanels.chunks(CHUNK_MAX_PANELS) {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (chunk.len() * std::mem::size_of::<PanelInstance>()) as isize,
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
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}
