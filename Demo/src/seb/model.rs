

use crate::seb::{
    gltfmodel::GLTFModel,
    seb::{ToCStr, create_shader_from},
};
use nalgebra_glm as glm;

pub struct Model {
    pub gltf_model: GLTFModel,
    pub shader_id: u32,
}
impl Model {
    pub fn new(file: &str) -> Self {
        let mut model = GLTFModel::new();
        model.load(file).unwrap();
        println!("{}", model.animations.len());
        Self {
            gltf_model: model,
            shader_id: 0,
        }
    }
    pub fn apply_animation(&mut self, index: usize, time: f32) {
        if index >= self.gltf_model.animations.len() {
            return;
        }
        self.gltf_model.animations[index].apply_animation(&mut self.gltf_model.meshes, time);
    }
    fn init_meshes(&mut self) {
        for mesh in &mut self.gltf_model.meshes {
            unsafe {
                gl::GenVertexArrays(1, &mut mesh.vao);
                gl::BindVertexArray(mesh.vao);

                gl::GenBuffers(1, &mut mesh.ebo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
                    mesh.indices.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );

                gl::GenBuffers(1, &mut mesh.vbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (mesh.position_coords.len() * std::mem::size_of::<f32>()) as isize,
                    mesh.position_coords.as_ptr() as *const _,
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

                gl::GenBuffers(1, &mut mesh.nbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, mesh.nbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (mesh.normal_coords.len() * std::mem::size_of::<f32>()) as isize,
                    mesh.normal_coords.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    3 * std::mem::size_of::<f32>() as i32,
                    std::ptr::null(),
                );
                gl::EnableVertexAttribArray(1);

                gl::GenBuffers(1, &mut mesh.tbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, mesh.tbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (mesh.texture_coords.len() * std::mem::size_of::<f32>()) as isize,
                    mesh.texture_coords.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
                gl::VertexAttribPointer(
                    2,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    2 * std::mem::size_of::<f32>() as i32,
                    std::ptr::null(),
                );
                gl::EnableVertexAttribArray(2);

                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindVertexArray(0);
            }
        }
    }
    fn init_shaders(&mut self) {
        let vshader = std::fs::read_to_string("./assets/model/shaders/vertex.glsl").unwrap();
        let fshader = std::fs::read_to_string("./assets/model/shaders/fragment.glsl").unwrap();
        self.shader_id = create_shader_from(&vshader, &fshader);
    }
    pub fn init(&mut self) {
        self.init_shaders();
        self.init_meshes();
    }
    pub fn draw_for_shadow(&self, light: &Light) {
        unsafe {
            for mesh in &self.gltf_model.meshes {
                light.bind_shadow_model(mesh.translation * mesh.rotation * mesh.scale);
                gl::BindVertexArray(mesh.vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                gl::BindVertexArray(0);
            }
        }
    }
    pub fn draw(&self, projection: glm::Mat4, view: glm::Mat4, eye: glm::Vec3, light: Light) {
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
                gl::GetUniformLocation(self.shader_id, "lightSpaceMatrix".c_str().as_ptr()),
                1,
                gl::FALSE,
                light.light_space_matrix.as_ptr(),
            );
            gl::Uniform3fv(
                gl::GetUniformLocation(self.shader_id, "viewPos".c_str().as_ptr()),
                1,
                eye.as_ptr(),
            );
            gl::Uniform3fv(
                gl::GetUniformLocation(self.shader_id, "lightPos".c_str().as_ptr()),
                1,
                light.light_pos.as_ptr(),
            );
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, light.depth_texture);
            gl::Uniform1i(
                gl::GetUniformLocation(self.shader_id, "shadowMap".c_str().as_ptr() as *const _),
                0,
            );
            for mesh in &self.gltf_model.meshes {
                let model = mesh.translation * mesh.rotation * mesh.scale;
                gl::UniformMatrix4fv(
                    gl::GetUniformLocation(self.shader_id, "model".c_str().as_ptr()),
                    1,
                    gl::FALSE,
                    model.as_ptr(),
                );

                if let Some(mat) = &mesh.material {
                    gl::Uniform3fv(
                        gl::GetUniformLocation(self.shader_id, "baseColor".c_str().as_ptr()),
                        1,
                        mat.base_color_factor.as_ptr(),
                    );
                    gl::Uniform1f(
                        gl::GetUniformLocation(self.shader_id, "metallic".c_str().as_ptr()),
                        mat.metallic_factor,
                    );
                    gl::Uniform1f(
                        gl::GetUniformLocation(self.shader_id, "roughness".c_str().as_ptr()),
                        mat.roughness_factor,
                    );
                    if let Some(index) = mat.base_color_texture {
                        gl::ActiveTexture(gl::TEXTURE1); // activezi slotul 0
                        gl::BindTexture(gl::TEXTURE_2D, self.gltf_model.textures_map[&index]); // faci bind la textura ta în slotul 0
                        gl::Uniform1i(
                            gl::GetUniformLocation(
                                self.shader_id,
                                "baseColorTexture".c_str().as_ptr() as *const i8,
                            ),
                            1,
                        );
                        gl::Uniform1i(
                            gl::GetUniformLocation(
                                self.shader_id,
                                "useBaseColorTexture".c_str().as_ptr() as *const i8,
                            ),
                            1,
                        );
                    } else {
                        gl::Uniform1i(
                            gl::GetUniformLocation(
                                self.shader_id,
                                "useBaseColorTexture".c_str().as_ptr() as *const i8,
                            ),
                            0,
                        );
                    }
                }
                gl::BindVertexArray(mesh.vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                gl::BindVertexArray(0);
            }
        }
    }
}

use gl::types::*;

#[derive(Clone, Copy)]
pub struct Light {
    pub fbo: u32,
    pub depth_texture: u32,
    pub width: i32,
    pub height: i32,

    pub light_pos: glm::Vec3,
    pub light_target: glm::Vec3,
    pub light_space_matrix: glm::Mat4,

    pub shadow_shader: u32,
}

impl Light {
    pub fn new() -> Self {
        Self {
            fbo: 0,
            depth_texture: 0,
            width: 0,
            height: 0,
            light_pos: glm::vec3(0.0, 0.0, 0.0),
            light_target: glm::vec3(0.0, 0.0, 0.0),
            light_space_matrix: glm::Mat4::identity(),
            shadow_shader: 0,
        }
    }

    pub fn init_shadow(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        unsafe {
            gl::GenFramebuffers(1, &mut self.fbo);
            gl::GenTextures(1, &mut self.depth_texture);
            gl::BindTexture(gl::TEXTURE_2D, self.depth_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH_COMPONENT as GLint,
                width,
                height,
                0,
                gl::DEPTH_COMPONENT,
                gl::FLOAT,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as GLint,
            );

            let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            gl::TexParameterfv(
                gl::TEXTURE_2D,
                gl::TEXTURE_BORDER_COLOR,
                border_color.as_ptr(),
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_COMPARE_MODE,
                gl::COMPARE_REF_TO_TEXTURE as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_COMPARE_FUNC,
                gl::LEQUAL as GLint,
            );

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                self.depth_texture,
                0,
            );

            gl::DrawBuffer(gl::NONE);
            gl::ReadBuffer(gl::NONE);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        let shadow_vshader = r#"
                #version 330 core
                layout(location = 0) in vec3 aPos;
                uniform mat4 light_matrix;
                uniform mat4 model;
                void main()
                {
                    gl_Position = light_matrix * model * vec4(aPos, 1.0);
                }
                "#;
        let shadow_fshader = r#"
                #version 330 core
                void main() {}
                "#;
        self.shadow_shader = create_shader_from(shadow_vshader, shadow_fshader);
    }

    pub fn add_light(&mut self, light_pos: glm::Vec3, light_target: glm::Vec3) {
        self.light_pos = light_pos;
        self.light_target = light_target;
        self.light_space_matrix = Self::compute_light_space_matrix(&light_pos, &light_target);
    }

    fn compute_light_space_matrix(pos: &glm::Vec3, target: &glm::Vec3) -> glm::Mat4 {
        let light_projection = glm::ortho(-20.0, 20.0, -20.0, 20.0, -20.0, 20.0);
        let light_view = glm::look_at(pos, target, &glm::vec3(0.0, 0.1, 0.0));
        light_projection * light_view
    }

    pub fn bind_shadow(&self) {
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::FRONT);

            gl::Viewport(0, 0, self.width, self.height);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(self.shadow_shader);
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shadow_shader, "light_matrix".c_str().as_ptr()),
                1,
                gl::FALSE,
                self.light_space_matrix.as_ptr(),
            );
        }
    }

    pub fn bind_shadow_model(&self, model: glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shadow_shader, "model".c_str().as_ptr()),
                1,
                gl::FALSE,
                model.as_ptr(),
            );
        }
    }
    pub fn unbind_shadow(&self) {
        unsafe {
            gl::CullFace(gl::BACK);
            //gl::Disable(gl::CULL_FACE);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}
pub struct Map {
    map: Vec<Vec<f32>>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    shader_id: u32,
    texture_id: u32,
    vao: u32,
    ebo: u32,
    vbo: u32,
    scale: f32,
}

impl Map {
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
        }
    }
    pub fn from_height_map(&mut self, path: &str, height_scale: f32) -> Result<(), String> {
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
                let index = ((y * width + x) * nr_channels) as usize;
                let gray = (data[index] as f32 / 255.0) * height_scale; // doar canalul roșu pentru grayscale
                map[y as usize][x as usize] = gray;
            }
        }
        self.map = map;

        Ok(())
    }

    pub fn generate_terrain(&mut self) {
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

        let width = self.width();
        let height = self.height();

        let mut vertices = Vec::with_capacity(width * height);
        let mut indices = Vec::new();
        let mut normals = vec![[0.0f32; 3]; width * height];

        // 1. Generează toți vertecșii (poziție, normală 0, tex_coords)
        for z in 0..height {
            for x in 0..width {
                let y = self.get(x, z).unwrap_or(0.0);

                vertices.push(Vertex {
                    position: [x as f32, y, z as f32],
                    normal: [0.0, 0.0, 0.0], // Temporar, o vom calcula mai jos
                    tex_coords: [x as f32 / width as f32, z as f32 / height as f32],
                });
            }
        }

        // 2. Construiește triunghiurile și calculează normalele
        for z in 0..(height - 1) {
            for x in 0..(width - 1) {
                let top_left = (z * width + x) as usize;
                let top_right = (z * width + (x + 1)) as usize;
                let bottom_left = ((z + 1) * width + x) as usize;
                let bottom_right = ((z + 1) * width + (x + 1)) as usize;

                // Triunghi 1: top_left, bottom_left, top_right
                {
                    let normal = compute_normal(
                        &vertices[top_left].position,
                        &vertices[bottom_left].position,
                        &vertices[top_right].position,
                    );
                    for &i in &[top_left, bottom_left, top_right] {
                        normals[i][0] += normal[0];
                        normals[i][1] += normal[1];
                        normals[i][2] += normal[2];
                    }

                    indices.push(top_left as u32);
                    indices.push(bottom_left as u32);
                    indices.push(top_right as u32);
                }

                // Triunghi 2: top_right, bottom_left, bottom_right
                {
                    let normal = compute_normal(
                        &vertices[top_right].position,
                        &vertices[bottom_left].position,
                        &vertices[bottom_right].position,
                    );
                    for &i in &[top_right, bottom_left, bottom_right] {
                        normals[i][0] += normal[0];
                        normals[i][1] += normal[1];
                        normals[i][2] += normal[2];
                    }

                    indices.push(top_right as u32);
                    indices.push(bottom_left as u32);
                    indices.push(bottom_right as u32);
                }
            }
        }

        // 3. Normalizează normalele la fiecare vertex
        for (i, vertex) in vertices.iter_mut().enumerate() {
            let n = normals[i];
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if len > 0.0 {
                vertex.normal = [n[0] / len, n[1] / len, n[2] / len];
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
        //model = glm::translate(&model, &glm::vec3(0.0, 0.0, 0.0));
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
    pub fn get_y(&self, x: f32, z: f32) -> f32 {
        // Convertim coordonatele din lume în coordonate de hartă (cu scalare 10x)
        let fx = x / self.scale;
        let fz = z / self.scale;

        // Coordonatele întregi ale celulei în care ne aflăm
        let x0 = fx.floor() as usize;
        let z0 = fz.floor() as usize;
        let x1 = x0 + 1;
        let z1 = z0 + 1;

        // Factorii de interpolare (0.0 - 1.0)
        let tx = fx - x0 as f32;
        let tz = fz - z0 as f32;

        // Obținem înălțimile pentru cele 4 colțuri ale celulei
        let y00 = self.get(x0, z0).unwrap_or(0.0) * self.scale;
        let y10 = self.get(x1, z0).unwrap_or(0.0) * self.scale;
        let y01 = self.get(x0, z1).unwrap_or(0.0) * self.scale;
        let y11 = self.get(x1, z1).unwrap_or(0.0) * self.scale;

        // Interpolare biliniară
        let y0 = y00 * (1.0 - tx) + y10 * tx; // Interpolare pe latura de jos
        let y1 = y01 * (1.0 - tx) + y11 * tx; // Interpolare pe latura de sus
        let y = y0 * (1.0 - tz) + y1 * tz; // Interpolare între laturi

        y // Adăugăm un mic offset pentru a evita clipping-ul
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
