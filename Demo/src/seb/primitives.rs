use crate::seb::seb::{ToCStr, create_shader_from};
use nalgebra_glm as glm;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: glm::Vec3,
    pub end: glm::Vec3,
    pub color: glm::Vec4,
}
impl Line {
    pub fn new() -> Self {
        Self {
            start: glm::Vec3::zeros(),
            end: glm::Vec3::zeros(),
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        }
    }
    pub fn from(start: glm::Vec3, end: glm::Vec3) -> Self {
        Self {
            start,
            end,
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        }
    }
}

const CHUNK_MAX_LINES: usize = 1000;
pub struct LineRenderer {
    vao: u32,
    ivbo: u32,
    shader: u32,
}
impl LineRenderer {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut shader = 0;
        let mut ivbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut ivbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, ivbo);

            let line_size = std::mem::size_of::<Line>();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (CHUNK_MAX_LINES * line_size) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                line_size as i32,
                (0 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribDivisor(0, 1);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                line_size as i32,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribDivisor(1, 1);

            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                line_size as i32,
                (6 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribDivisor(2, 1);

            gl::BindVertexArray(0);

            let vshader = r#"
                    #version 330 core
                    layout(location = 0) in vec3 uStart;
                    layout(location = 1) in vec3 uEnd;
                    layout(location = 2) in vec4 col;

                    uniform mat4 projection;
                    uniform mat4 view;
                    out vec4 color;
                    void main() {
                        color = col;
                        vec3 pos = (gl_VertexID % 2 == 0) ? uStart : uEnd;
                        gl_Position = projection * view * vec4(pos, 1.0);
                    }
                "#;
            let fshader = r#"
                    #version 330 core
                    out vec4 fragColor;
                    in vec4 color;
                    void main() {
                        fragColor = color;
                    }
                "#;
            shader = create_shader_from(vshader, fshader);
        }

        Self {
            vao: vao,
            ivbo: ivbo,
            shader: shader,
        }
    }
    pub fn draw(&self, projection: glm::Mat4, view: glm::Mat4, lines: &mut [Line]) {
        unsafe {
            gl::UseProgram(self.shader);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "projection".c_str().as_ptr()),
                1,
                gl::FALSE,
                projection.as_ptr(),
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "view".c_str().as_ptr()),
                1,
                gl::FALSE,
                view.as_ptr(),
            );

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.ivbo);
            for chunk in lines.chunks(CHUNK_MAX_LINES) {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (chunk.len() * std::mem::size_of::<Line>()) as isize,
                    chunk.as_ptr() as *const _,
                );

                gl::DrawArraysInstanced(gl::LINES, 0, 2, chunk.len() as i32);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, self.ivbo);
            gl::BindVertexArray(0);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub direction: glm::Vec3,
    pub position: glm::Vec3,
    pub color: glm::Vec4,
}
impl Vector {
    pub fn new() -> Self {
        Self {
            direction: glm::Vec3::zeros(),
            position: glm::Vec3::zeros(),
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        }
    }
    pub fn from(pos: glm::Vec3, dir: glm::Vec3) -> Self {
        Self {
            direction: dir,
            position: pos,
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        }
    }
    pub fn get_arrow_position(&self) -> glm::Vec3 {
        self.position + self.direction
    }
}
const CHUNK_MAX_VECTORS: usize = 100;
struct VectorInstance {
    model: [[f32; 4]; 4],
    color: glm::Vec4,
}
pub struct VectorRenderer {
    vao: u32,
    ivbo: u32,
    indices_len: i32,
    shader: u32,
}
impl VectorRenderer {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ivbo = 0;
        let mut ebo = 0;
        let mut shader = 0;

        let mut vertices: Vec<f32> = Vec::new();
        //up
        vertices.push(0.0);
        vertices.push(1.0);
        vertices.push(0.0);
        //midle
        vertices.push(0.0);
        vertices.push(0.8);
        vertices.push(0.0);
        //down
        vertices.push(0.0);
        vertices.push(0.0);
        vertices.push(0.0);

        let mut indices: Vec<u32> = Vec::new();
        indices.push(1);
        indices.push(2);
        indices.push(0);

        let segments = 32;
        for i in 0..segments {
            let theta = (i as f32 / segments as f32) * std::f32::consts::TAU;
            vertices.push(theta.cos() * 0.04);
            vertices.push(0.8);
            vertices.push(theta.sin() * 0.04);

            let next = if i + 1 < segments { i + 3 + 1 } else { 3 };
            indices.push(0);
            indices.push(next);
            indices.push(i + 3);

            indices.push(next);
            indices.push(1);
            indices.push(i + 3);
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            // Trimitem datele în GPU
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut ivbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, ivbo);

            let vector_size = std::mem::size_of::<VectorInstance>();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (CHUNK_MAX_VECTORS * vector_size) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            for i in 0..5 {
                gl::VertexAttribPointer(
                    1 + i,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    vector_size as i32,
                    (i as usize * std::mem::size_of::<[f32; 4]>()) as *const _,
                );
                gl::EnableVertexAttribArray(1 + i);
                gl::VertexAttribDivisor(1 + i, 1);
            }

            gl::BindVertexArray(0);

            let vshader = r#"
                    #version 330 core
                    layout(location = 0) in vec3 position;
                    layout(location = 1) in mat4 model;
                    layout(location = 5) in vec4 color;

                    uniform mat4 view;
                    uniform mat4 projection;
                    out vec4 col;
                    void main() {
                        col = color;
                        gl_Position = projection * view * model * vec4(position, 1.0);
                    }
                "#;
            let fshader = r#"
                    #version 330 core
                    out vec4 fragColor;
                    in vec4 col;
                    void main() {
                        fragColor = col;
                    }
                "#;
            shader = create_shader_from(vshader, fshader);
        }

        Self {
            vao: vao,
            ivbo: ivbo,
            indices_len: indices.len() as i32,
            shader: shader,
        }
    }
    fn get_vector_model(vec: &Vector) -> glm::Mat4 {
        let start = vec.position;
        let direction = vec.direction;
        let length = glm::length(&direction);

        if length < 1e-6 {
            // Vector aproape zero, return identity translate la start
            return glm::translate(&glm::Mat4::identity(), &start);
        }

        // Normalize direction for rotation
        let target_norm = direction / length;
        let up = glm::vec3(0.0, 1.0, 0.0);

        // Axis și unghi pentru rotație
        let axis = glm::cross(&up, &target_norm);
        let dot = glm::dot(&up, &target_norm);

        let rotation = if glm::length(&axis) < 1e-6 {
            // Vector paralel cu up
            if dot > 0.0 {
                glm::Mat4::identity()
            } else {
                // Invers pe Y
                glm::rotate(
                    &glm::Mat4::identity(),
                    std::f32::consts::PI,
                    &glm::vec3(1.0, 0.0, 0.0),
                )
            }
        } else {
            glm::rotate(&glm::Mat4::identity(), dot.acos(), &axis)
        };

        // Scale pe Y pentru lungime
        let scale = glm::scaling(&glm::vec3(1.0, length, 1.0));

        // Translate baza la vec.position
        let translation = glm::translate(&glm::Mat4::identity(), &start);

        // Ordinea: translate → rotate → scale
        translation * rotation * scale
    }
    pub fn draw(&self, projection: glm::Mat4, view: glm::Mat4, vectors: &mut [Vector]) {
        let mut ivectors: Vec<VectorInstance> = Vec::new();

        for v in vectors {
            ivectors.push(VectorInstance {
                model: Self::get_vector_model(&v).into(),
                color: v.color,
            });
        }

        unsafe {
            gl::UseProgram(self.shader);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "projection".c_str().as_ptr()),
                1,
                gl::FALSE,
                projection.as_ptr(),
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "view".c_str().as_ptr()),
                1,
                gl::FALSE,
                view.as_ptr(),
            );

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.ivbo);
            for chunk in ivectors.chunks(CHUNK_MAX_VECTORS) {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (chunk.len() * std::mem::size_of::<VectorInstance>()) as isize,
                    chunk.as_ptr() as *const _,
                );

                gl::DrawElementsInstanced(
                    gl::LINES,
                    2,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                    chunk.len() as i32,
                );
                gl::DrawElementsInstanced(
                    gl::TRIANGLES,
                    self.indices_len,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                    chunk.len() as i32,
                );
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub position: glm::Vec3,
    pub scale: glm::Vec3,
    pub x_angle: f32,
    pub y_angle: f32,
    pub z_angle: f32,
    pub color: glm::Vec4,
}
impl Rectangle {
    pub fn new() -> Self {
        Self {
            position: glm::Vec3::zeros(),
            scale: glm::vec3(1.0, 1.0, 1.0),
            x_angle: 0f32,
            y_angle: 0f32,
            z_angle: 0f32,
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        }
    }
    pub fn get_model(&self) -> glm::Mat4 {
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &self.position);
        model = glm::rotate_z(&model, self.z_angle.to_radians());
        model = glm::rotate_y(&model, self.y_angle.to_radians());
        model = glm::rotate_x(&model, self.x_angle.to_radians());
        model = glm::scale(&model, &self.scale);
        model
    }
    pub fn rotate_from_point(&mut self, point: glm::Vec3, rotation_angles: glm::Vec3) {
        let translated_pos = self.position - point;

        let rotation_matrix =
            glm::rotation(rotation_angles.z.to_radians(), &glm::vec3(0.0, 0.0, 1.0))
                * glm::rotation(rotation_angles.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0))
                * glm::rotation(rotation_angles.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));

        let rotated_pos = rotation_matrix.transform_vector(&translated_pos);

        self.position = rotated_pos + point;

        self.x_angle += rotation_angles.x;
        self.y_angle += rotation_angles.y;
        self.z_angle += rotation_angles.z;
    }
}

const CHUNK_MAX_RECTANGLES: usize = 100;
struct RectangleInstance {
    model: [[f32; 4]; 4],
    color: glm::Vec4,
}
pub struct RectangleRenderer {
    vao: u32,
    ivbo: u32,
    indices_len: i32,
    shader: u32,
}
impl RectangleRenderer {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ivbo = 0;
        let mut ebo = 0;
        let mut shader = 0;

        let vertices: [f32; 24] = [
            -1.0, -1.0, -1.0, // 0
            1.0, -1.0, -1.0, // 1
            1.0, 1.0, -1.0, // 2
            -1.0, 1.0, -1.0, // 3
            -1.0, -1.0, 1.0, // 4
            1.0, -1.0, 1.0, // 5
            1.0, 1.0, 1.0, // 6
            -1.0, 1.0, 1.0, // 7
        ];

        let indices: [u32; 24] = [
            0, 1, 1, 2, 2, 3, 3, 0, // fața din spate
            4, 5, 5, 6, 6, 7, 7, 4, // fața din față
            0, 4, 1, 5, 2, 6, 3, 7, // muchiile laterale
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            // Trimitem datele în GPU
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut ivbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, ivbo);

            let rectangle_size = std::mem::size_of::<RectangleInstance>();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (CHUNK_MAX_RECTANGLES * rectangle_size) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            for i in 0..5 {
                gl::VertexAttribPointer(
                    1 + i,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    rectangle_size as i32,
                    (i as usize * std::mem::size_of::<[f32; 4]>()) as *const _,
                );
                gl::EnableVertexAttribArray(1 + i);
                gl::VertexAttribDivisor(1 + i, 1);
            }

            gl::BindVertexArray(0);

            let vshader = r#"
                    #version 330 core
                    layout(location = 0) in vec3 position;
                    layout(location = 1) in mat4 model;
                    layout(location = 5) in vec4 color;

                    uniform mat4 view;
                    uniform mat4 projection;
                    out vec4 col;
                    void main() {
                        col = color;
                        gl_Position = projection * view * model * vec4(position, 1.0);
                    }
                "#;
            let fshader = r#"
                    #version 330 core
                    out vec4 fragColor;
                    in vec4 col;
                    void main() {
                        fragColor = col;
                    }
                "#;
            shader = create_shader_from(vshader, fshader);
        }

        Self {
            vao: vao,
            ivbo: ivbo,
            indices_len: indices.len() as i32,
            shader: shader,
        }
    }
    pub fn draw(&self, projection: glm::Mat4, view: glm::Mat4, rectangles: &mut [Rectangle]) {
        let mut irectangles: Vec<RectangleInstance> = Vec::new();

        for r in rectangles {
            irectangles.push(RectangleInstance {
                model: r.get_model().into(),
                color: r.color,
            });
        }

        unsafe {
            gl::UseProgram(self.shader);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "projection".c_str().as_ptr()),
                1,
                gl::FALSE,
                projection.as_ptr(),
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "view".c_str().as_ptr()),
                1,
                gl::FALSE,
                view.as_ptr(),
            );

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.ivbo);
            for chunk in irectangles.chunks(CHUNK_MAX_RECTANGLES) {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (chunk.len() * std::mem::size_of::<RectangleInstance>()) as isize,
                    chunk.as_ptr() as *const _,
                );

                gl::DrawElementsInstanced(
                    gl::LINES,
                    self.indices_len,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                    chunk.len() as i32,
                );
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub position: glm::Vec3,
    pub scale: f32,
    pub color: glm::Vec4,
}
impl Sphere {
    pub fn new() -> Self {
        Self {
            position: glm::Vec3::zeros(),
            scale: 1.0,
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        }
    }
    pub fn get_model(&self) -> glm::Mat4 {
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &self.position);
        model = glm::scale(&model, &glm::vec3(self.scale, self.scale, self.scale));
        model
    }
    pub fn rotate_from_point(&mut self, point: glm::Vec3, rotation_angles: glm::Vec3) {
        let translated_pos = self.position - point;

        let rotation_matrix =
            glm::rotation(rotation_angles.z.to_radians(), &glm::vec3(0.0, 0.0, 1.0))
                * glm::rotation(rotation_angles.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0))
                * glm::rotation(rotation_angles.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));

        let rotated_pos = rotation_matrix.transform_vector(&translated_pos);

        self.position = rotated_pos + point;
    }
}

const CHUNK_MAX_SPHERES: usize = 100;
struct SphereInstance {
    model: [[f32; 4]; 4],
    color: glm::Vec4,
}
pub struct SphereRenderer {
    pub vao: u32,
    pub ivbo: u32,
    pub indices_len: i32,
    pub shader: u32,
}

impl SphereRenderer {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ivbo = 0;
        let mut ebo = 0;
        let mut shader = 0;

        let latitude_segments = 8;
        let longitude_segments = 32;
        let mut vertices: Vec<f32> = Vec::new();

        for i in 0..=latitude_segments {
            let phi = (i as f32 / latitude_segments as f32) * std::f32::consts::PI;
            for j in 0..=longitude_segments {
                let theta = (j as f32 / longitude_segments as f32) * std::f32::consts::TAU;

                let x = phi.sin() * theta.cos();
                let y = phi.cos();
                let z = phi.sin() * theta.sin();

                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
            }
        }

        let mut indices: Vec<u32> = Vec::new();
        for i in 0..latitude_segments {
            for j in 0..longitude_segments {
                let first = (i * (longitude_segments + 1) + j) as u32;
                let second = first + longitude_segments as u32 + 1;

                indices.push(first);
                indices.push(second);
                indices.push(first + 1);

                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut ivbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, ivbo);

            let sphere_size = std::mem::size_of::<SphereInstance>();

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (CHUNK_MAX_SPHERES * sphere_size) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            for i in 0..5 {
                gl::VertexAttribPointer(
                    1 + i,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    sphere_size as i32,
                    (i as usize * std::mem::size_of::<[f32; 4]>()) as *const _,
                );
                gl::EnableVertexAttribArray(1 + i);
                gl::VertexAttribDivisor(1 + i, 1);
            }

            gl::BindVertexArray(0);

            let vshader = r#"
                #version 330 core
                layout(location = 0) in vec3 position;
                layout(location = 1) in mat4 model;
                layout(location = 5) in vec4 color;
                uniform mat4 view;
                uniform mat4 projection;
                out vec4 col;
                void main() {
                    col = color;
                    gl_Position = projection * view * model * vec4(position, 1.0);
                }
            "#;

            let fshader = r#"
                #version 330 core
                out vec4 fragColor;
                in vec4 col;
                void main() {
                    fragColor = col;
                }
            "#;

            shader = create_shader_from(vshader, fshader);
        }

        Self {
            vao,
            ivbo,
            indices_len: indices.len() as i32,
            shader,
        }
    }

    pub fn draw(&self, projection: glm::Mat4, view: glm::Mat4, spheres: &mut [Sphere]) {
        let mut ispheres: Vec<SphereInstance> = Vec::new();

        for s in spheres {
            ispheres.push(SphereInstance {
                model: s.get_model().into(),
                color: s.color,
            });
        }

        unsafe {
            gl::UseProgram(self.shader);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "projection".c_str().as_ptr()),
                1,
                gl::FALSE,
                projection.as_ptr(),
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader, "view".c_str().as_ptr()),
                1,
                gl::FALSE,
                view.as_ptr(),
            );

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.ivbo);
            for chunk in ispheres.chunks(CHUNK_MAX_SPHERES) {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (chunk.len() * std::mem::size_of::<SphereInstance>()) as isize,
                    chunk.as_ptr() as *const _,
                );

                gl::DrawElementsInstanced(
                    gl::LINE_LOOP,
                    self.indices_len as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                    chunk.len() as i32,
                );
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}
