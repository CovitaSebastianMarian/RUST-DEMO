use core::f32;

use crate::seb::{
    seb::{ToCStr, create_shader_from},
};
use nalgebra_glm as glm;

#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}
pub struct Planet {
    map: Vec<Vec<f32>>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    shader_id: u32,
    texture_id: u32,
    vao: u32,
    ebo: u32,
    vbo: u32,
    pub scale: f32,
    pub position: glm::Vec3,
    pub x_angle: f32,
    pub y_angle: f32,
    pub z_angle: f32,
}

impl Planet {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
            shader_id: 0,
            texture_id: 0,
            vao: 0,
            ebo: 0,
            vbo: 0,
            scale: 1.0,
            position: glm::Vec3::zeros(),
            x_angle: 0.0,
            y_angle: 0.0,
            z_angle: 0.0,
        }
    }
    pub fn from_map(&mut self, path: &str, height_scale: f32, scale: f32) -> Result<(), String> {
        self.scale = scale;
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

        if nr_channels < 1 {
            return Err("Image does not contain color channels".to_string());
        }

        // Creează matricea de înălțimi
        let mut map = vec![vec![0.0f32; width as usize]; height as usize];

        let (mut a, mut b) = (0, 0);
        for y in 0..height {
            for x in 0..width {
                // let index = ((y * width + x) * nr_channels) as usize;
                // let gray = (data[index] as f32 / 255.0) * height_scale; // doar canalul roșu pentru grayscale
                // map[y as usize][x as usize] = gray;

                let index = ((y * width + x) * nr_channels) as usize;
                if index + 2 >= data.len() {
                    continue; // skip dacă depășește
                }
                let r = data[index] as f32 / 255.0;
                let g = data[index + 1] as f32 / 255.0;
                let b = data[index + 2] as f32 / 255.0;

                let height_value = {
                    // Calculăm o valoare bazată pe predominanța culorilor
                    let blue_dominance = b - (r + g) * 0.5;
                    let green_dominance = g - (r + b) * 0.5;
                    let red_dominance = r - (g + b) * 0.5;

                    // Normalizăm și ajustăm pentru intervalele dorite
                    let water_level = blue_dominance.clamp(0.0, 1.0) * -0.05; // -1..0
                    let land_level = green_dominance.clamp(0.0, 1.0) * 0.005; // 0..0.5
                    let mountain_level = red_dominance.clamp(0.0, 1.0) * 0.05; // 0..1

                    // Combinăm influența fiecărui tip de teren
                    water_level + land_level + mountain_level
                };
                map[y as usize][x as usize] = height_value * height_scale;
            }
        }
        self.map = map;

        Ok(())
    }
    /*
    pub fn get_surrounding_vertices(&self, px: f32, py: f32, pz: f32) -> Option<[Vertex; 4]> {
        let width = self.width();
        let height = self.height();

        // Transformă poziția în coordonate sferice
        let r = (px * px + py * py + pz * pz).sqrt();
        if r < 1e-6 {
            return None;
        }

        let phi = (py / r).acos(); // [0, π]
        let theta = pz.atan2(px); // [-π, π]
        let theta = if theta < 0.0 {
            theta + 2.0 * std::f32::consts::PI
        } else {
            theta
        };

        // Transformă în coordonate de grid
        let z_float = phi / std::f32::consts::PI * (height - 1) as f32;
        let x_float = theta / (2.0 * std::f32::consts::PI) * width as f32;

        let x0 = x_float.floor() as usize;
        let x1 = (x0 + 1) % width; // wrap-around pe longitudine
        let z0 = z_float.floor() as usize;
        let z1 = if z0 + 1 < height { z0 + 1 } else { z0 }; // clamp la pol

        // Funcție helper să ia un vertex
        let mut get_vertex = |x: usize, z: usize| -> Vertex {
            let theta = (x as f32 / width as f32) * 2.0 * std::f32::consts::PI;
            let phi = (z as f32 / (height - 1) as f32) * std::f32::consts::PI;

            let height_value = self.get(x % width, z).unwrap_or(0.0);
            let r_local = 1.0 + height_value;

            let vx = r_local * phi.sin() * theta.cos();
            let vy = r_local * phi.cos();
            let vz = r_local * phi.sin() * theta.sin();

            Vertex {
                position: [vx, vy, vz],
                normal: [0.0, 0.0, 0.0],
                tex_coords: [x as f32 / width as f32, z as f32 / (height - 1) as f32],
            }
        };

        Some([
            get_vertex(x0, z0),
            get_vertex(x1, z0),
            get_vertex(x0, z1),
            get_vertex(x1, z1),
        ])
    }
    */

    pub fn get_position_on_sphere(&self, position: glm::Vec3, h: f32) -> glm::Vec3 {
        let width = self.width();
        let height = self.height();

        // Transformă în coordonate sferice
        let r = glm::length(&position);
        if r < 1e-6 {
            return glm::vec3(0.0, 0.0, 0.0);
        }

        let phi = (position.y / r).acos(); // [0, π]
        let mut theta = position.z.atan2(position.x); // [-π, π]
        if theta < 0.0 {
            theta += 2.0 * std::f32::consts::PI;
        }

        // Poziția relativă în grid
        let z_float = phi / std::f32::consts::PI * (height - 1) as f32;
        let x_float = theta / (2.0 * std::f32::consts::PI) * width as f32;

        let x0 = x_float.floor() as usize;
        let x1 = (x0 + 1) % width;
        let z0 = z_float.floor() as usize;
        let z1 = if z0 + 1 < height { z0 + 1 } else { z0 };

        let tx = x_float - x0 as f32;
        let tz = z_float - z0 as f32;

        // Helper să ia poziția vertexului
        let get_radius = |x: usize, z: usize| -> f32 {
            let h = self.get(x % width, z).unwrap_or(0.0);
            1.0 + h
        };

        let r00 = get_radius(x0, z0);
        let r10 = get_radius(x1, z0);
        let r01 = get_radius(x0, z1);
        let r11 = get_radius(x1, z1);

        // Interpolare biliniară pe rază, nu pe poziție
        let r0 = r00 * (1.0 - tx) + r10 * tx;
        let r1 = r01 * (1.0 - tx) + r11 * tx;
        let r_interp = r0 * (1.0 - tz) + r1 * tz;

        // Poziția corectă = direcția radială * raza interpolată
        let radial = glm::normalize(&position);
        let mut pos = radial * r_interp * self.scale;

        let len = glm::length(&(pos - self.position)) + h;
        let dir = glm::normalize(&(pos - self.position));
        dir * len
    }
    pub fn generate_terrain(&mut self) {
        let width = self.width();
        let height = self.height();
        let radius = 1.0;

        // Adăugăm +1 la width pentru a închide cercul
        let vertex_count = (width + 1) * height;
        let mut vertices = Vec::with_capacity(vertex_count);
        let mut indices = Vec::new();
        let mut normals = vec![[0.0f32; 3]; vertex_count];

        // 1. Generare vertecși (cu coloana suplimentară pentru închidere)
        for z in 0..height {
            let phi = (z as f32 / (height - 1) as f32) * std::f32::consts::PI;
            for x in 0..=width {
                // <= Observă <= pentru a include coloana suplimentară
                let theta = (x as f32 / width as f32) * 2.0 * std::f32::consts::PI;

                // Folosim x % width pentru a repeta ultimul rând de vertecși
                let height_value = self.get(x % width, z).unwrap_or(0.0);
                let r_local = radius + height_value;

                let px = r_local * phi.sin() * theta.cos();
                let py = r_local * phi.cos();
                let pz = r_local * phi.sin() * theta.sin();

                vertices.push(Vertex {
                    position: [px, py, pz],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [
                        x as f32 / width as f32, // Tex coord nu trebuie să meargă peste 1.0
                        z as f32 / (height - 1) as f32,
                    ],
                });
            }
        }

        fn compute_normal(p1: &[f32; 3], p2: &[f32; 3], p3: &[f32; 3]) -> [f32; 3] {
            let u = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
            let v = [p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]];

            // Cross product u × v
            [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ]
        }

        // 2. Generare indecși și calcul normale
        for z in 0..(height - 1) {
            for x in 0..width {
                let top_left = z * (width + 1) + x;
                let top_right = top_left + 1;
                let bottom_left = (z + 1) * (width + 1) + x;
                let bottom_right = bottom_left + 1;

                // Triunghi 1 (top-left, bottom-left, top-right)
                let n1 = compute_normal(
                    &vertices[top_left].position,
                    &vertices[bottom_left].position,
                    &vertices[top_right].position,
                );
                for &i in &[top_left, bottom_left, top_right] {
                    normals[i][0] += n1[0];
                    normals[i][1] += n1[1];
                    normals[i][2] += n1[2];
                }
                indices.extend_from_slice(&[top_left as u32, top_right as u32, bottom_left as u32]);

                // Triunghi 2 (top-right, bottom-left, bottom-right)
                let n2 = compute_normal(
                    &vertices[top_right].position,
                    &vertices[bottom_left].position,
                    &vertices[bottom_right].position,
                );
                for &i in &[top_right, bottom_left, bottom_right] {
                    normals[i][0] += n2[0];
                    normals[i][1] += n2[1];
                    normals[i][2] += n2[2];
                }
                indices.extend_from_slice(&[
                    top_right as u32,
                    bottom_right as u32,
                    bottom_left as u32,
                ]);
            }
        }

        // 3. Normalizare normale
        for (i, vertex) in vertices.iter_mut().enumerate() {
            let n = &normals[i];
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if len > 0.0 {
                vertex.normal = [-n[0] / len, -n[1] / len, -n[2] / len];
            } else {
                vertex.normal = [0.0, 1.0, 0.0];
            }
        }

        self.vertices = vertices;
        self.indices = indices;
    }

    pub fn init(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            // EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>() * 8) as isize,
                self.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Attribute 0 - position (vec3)
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(), // offset 0
            );
            gl::EnableVertexAttribArray(0);

            // Attribute 1 - normal (vec3)
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as i32,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            // Attribute 2 - tex_coords (vec2)
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as i32,
                (6 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);

            // Done
            gl::BindVertexArray(0);
        }

        let vshader = r#"
            #version 330 core
            layout(location = 0) in vec3 position;
            layout(location = 1) in vec3 normal;
            layout(location = 2) in vec2 texcoord;

            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 projection;

            out vec3 frag_pos;
            out vec3 frag_normal;
            out vec2 tcoord;

            void main() {
                tcoord = texcoord;
                vec4 world_pos = model * vec4(position, 1.0);
                frag_pos = world_pos.xyz;

                // Transformăm normala în spațiul lumii (dacă modelul nu e uniform, se folosește inverse transpose)
                frag_normal = mat3(transpose(inverse(model))) * normal;

                gl_Position = projection * view * world_pos;
            }
        "#;
        let fshader = r#"
            #version 330 core

            in vec3 frag_pos;       // Poziția fragmentului în lume
            in vec3 frag_normal;    // Normala fragmentului
            in vec2 tcoord;         // Coordonate UV (nefolosite aici)

            out vec4 fragColor;

            uniform vec3 cam_pos;   // Poziția camerei
            uniform sampler2D texture0;
            void main() {

                // -------------------- CONFIG --------------------
                vec3 object_color     = texture(texture0, tcoord).rgb;
                vec3 light_color      = vec3(1.0);            // Lumină albă
                vec3 light_pos        = cam_pos;             // Lumină dinspre cameră

                float ambient_strength  = 0.2;   // Intensitate lumină ambientală
                float specular_strength = 0.5;   // Intensitate lumină speculară
                float shininess         = 32.0;  // Concentrarea highlight-ului

                // ------------------ ILUMINARE -------------------
                // 1. Ambientă
                vec3 ambient = ambient_strength * light_color;

                // 2. Difuză (Lambert)
                vec3 normal    = normalize(frag_normal);
                vec3 light_dir = normalize(light_pos - frag_pos);

                float diff     = max(dot(normal, light_dir), 0.0);
                vec3 diffuse   = diff * light_color;

                // 3. Speculară (Phong)
                vec3 view_dir    = normalize(cam_pos - frag_pos);
                vec3 reflect_dir = reflect(-light_dir, normal);

                float spec       = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
                vec3 specular    = specular_strength * spec * light_color;

                // --------------- COMBINARE FINALĂ ---------------
                vec3 lighting    = ambient + diffuse + specular;
                vec3 final_color = lighting * object_color;

                fragColor = vec4(final_color, 1.0);
            }
        "#;
        self.shader_id = create_shader_from(vshader, fshader);
    }
    pub fn load_texture(&mut self, path: &str) -> Result<(), String> {
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
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
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
                img_format as i32, //img_format as i32,
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
    pub fn draw(&mut self, projection: glm::Mat4, view: glm::Mat4, cam_pos: glm::Vec3) {
        let mut model: glm::Mat4 = glm::Mat4::identity();
        model = glm::translate(&model, &self.position);
        model = glm::rotate_z(&model, self.z_angle.to_radians());
        model = glm::rotate_y(&model, self.y_angle.to_radians());
        model = glm::rotate_x(&model, self.x_angle.to_radians());
        model = glm::scale(&model, &glm::vec3(self.scale, self.scale, self.scale));
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::UseProgram(self.shader_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
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
            gl::Uniform3fv(
                gl::GetUniformLocation(self.shader_id, "cam_pos".c_str().as_ptr()),
                1,
                cam_pos.as_ptr(),
            );
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::UseProgram(0);
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<f32> {
        self.map.get(y)?.get(x).copied()
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }
}
