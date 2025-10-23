

use gltf::Document;
use nalgebra as na;
use nalgebra_glm as glm;
use std::collections::HashMap;

// #[derive(Clone, Debug)]
// pub struct Material {
//     pub name: Option<String>,
//     pub base_color_factor: [f32; 4],       // RGBA
//     pub base_color_texture: Option<usize>, // calea sau ID-ul texturii
//     pub metallic_factor: f32,
//     pub roughness_factor: f32,
// }
// impl Material {
//     pub fn new() -> Self {
//         Self {
//             name: None,
//             base_color_factor: [0f32; 4],
//             base_color_texture: None,
//             metallic_factor: 0f32,
//             roughness_factor: 0f32,
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Mesh {
//     pub name: String,
//     pub position_coords: Vec<f32>,
//     pub texture_coords: Vec<f32>,
//     pub normal_coords: Vec<f32>,
//     pub indices: Vec<u32>,
//     pub material: Option<Material>,
//     pub vao: u32,
//     pub vbo: u32,
//     pub nbo: u32,
//     pub tbo: u32,
//     pub ebo: u32,
//     pub matrix: glm::Mat4,
// }
// impl Drop for Mesh {
//     fn drop(&mut self) {
//         unsafe {
//             if self.vao != 0 {
//                 gl::DeleteVertexArrays(1, &self.vao);
//                 self.vao = 0;
//             }
//             if self.vbo != 0 {
//                 gl::DeleteBuffers(1, &self.vbo);
//                 self.vbo = 0;
//             }
//             if self.nbo != 0 {
//                 gl::DeleteBuffers(1, &self.nbo);
//                 self.nbo = 0;
//             }
//             if self.tbo != 0 {
//                 gl::DeleteBuffers(1, &self.tbo);
//                 self.tbo = 0;
//             }
//             if self.ebo != 0 {
//                 gl::DeleteBuffers(1, &self.ebo);
//                 self.ebo = 0;
//             }
//         }
//     }
// }

// pub struct AnimationChannel {
//     pub node_index: usize,
//     pub path: String, // "translation", "rotation", "scale"
//     pub times: Vec<f32>,
//     pub values: Vec<[f32; 4]>, // poate fi [f32; 3] pentru translation/scale, [f32; 4] pentru rotation
// }

// pub struct Animation {
//     pub name: Option<String>,
//     pub channels: Vec<AnimationChannel>,
// }

// pub struct GLTFModel {
//     pub meshes: Vec<Mesh>,
//     pub textures_map: HashMap<usize, u32>,
//     pub animations: Vec<Animation>,
// }
// impl Drop for GLTFModel {
//     fn drop(&mut self) {
//         unsafe {
//             for &texture_id in self.textures_map.values() {
//                 if texture_id != 0 {
//                     gl::DeleteTextures(1, &texture_id);
//                 }
//             }
//             self.textures_map.clear();
//         }
//     }
// }

// impl GLTFModel {
//     pub fn new() -> Self {
//         Self {
//             meshes: Vec::new(),
//             textures_map: HashMap::new(),
//             animations: Vec::new(),
//         }
//     }
//     pub fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
//         let (document, buffers, images) = gltf::import(path)?;

//         self.load_textures(&document, &images);
//         self.load_animations(&document, &buffers);

//         for scene in document.scenes() {
//             for node in scene.nodes() {
//                 self.process_node(&node, &buffers);
//             }
//         }

//         Ok(())
//     }
//     fn load_animations(&mut self, document: &Document, buffers: &Vec<gltf::buffer::Data>) {
//         for animation in document.animations() {
//             let mut channels = Vec::new();

//             for channel in animation.channels() {
//                 let target = channel.target();
//                 let node_index = target.node().index();
//                 let property = match target.property() {
//                     gltf::animation::Property::Translation => "translation",
//                     gltf::animation::Property::Rotation => "rotation",
//                     gltf::animation::Property::Scale => "scale",
//                     _ => continue,
//                 }
//                 .to_string();

//                 let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));

//                 let times: Vec<f32> = reader
//                     .read_inputs()
//                     .expect("Failed to read animation times")
//                     .collect();

//                 let outputs = reader
//                     .read_outputs()
//                     .expect("Missing outputs for animation");

//                 let values: Vec<[f32; 4]> = match outputs {
//                     gltf::animation::util::ReadOutputs::Translations(iter) => {
//                         iter.map(|v| [v[0], v[1], v[2], 0.0]).collect()
//                     }

//                     gltf::animation::util::ReadOutputs::Scales(iter) => {
//                         iter.map(|v| [v[0], v[1], v[2], 0.0]).collect()
//                     }

//                     gltf::animation::util::ReadOutputs::Rotations(
//                         gltf::animation::util::Rotations::F32(iter),
//                     ) => iter.map(|v| [v[0], v[1], v[2], v[3]]).collect(),

//                     _ => continue, // Ignoră tipuri nefolosite
//                 };

//                 channels.push(AnimationChannel {
//                     node_index,
//                     path: property,
//                     times,
//                     values,
//                 });
//             }

//             self.animations.push(Animation {
//                 name: animation.name().map(|s| s.to_string()),
//                 channels,
//             });
//         }
//     }
//     fn load_textures(&mut self, document: &Document, images: &Vec<Data>) {
//         for texture in document.textures() {
//             let tex_index = texture.index();
//             let image = &images[tex_index];
//             let pixels = &image.pixels;
//             let width = image.width;
//             let height = image.height;

//             let format = match image.format {
//                 gltf::image::Format::R8G8B8A8 => gl::RGBA,
//                 gltf::image::Format::R8G8B8 => gl::RGB,
//                 _ => {
//                     eprintln!("Format nesuportat la textura {}", tex_index);
//                     continue;
//                 }
//             };
//             println!("name: {}", texture.name().unwrap_or("nimic"));

//             let mut texture_id: u32 = 0;
//             unsafe {
//                 gl::GenTextures(1, &mut texture_id);
//                 gl::BindTexture(gl::TEXTURE_2D, texture_id);

//                 let sampler = texture.sampler();
//                 let wrap_s = sampler.wrap_s();
//                 let wrap_t = sampler.wrap_t();
//                 let min_filter = sampler.min_filter();
//                 let mag_filter = sampler.mag_filter();

//                 let gl_wrap_s = match wrap_s {
//                     gltf::texture::WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
//                     gltf::texture::WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
//                     gltf::texture::WrappingMode::Repeat => gl::REPEAT,
//                 };
//                 let gl_wrap_t = match wrap_t {
//                     gltf::texture::WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
//                     gltf::texture::WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
//                     gltf::texture::WrappingMode::Repeat => gl::REPEAT,
//                 };

//                 let gl_min_filter = match min_filter {
//                     Some(gltf::texture::MinFilter::Nearest) => gl::NEAREST,
//                     Some(gltf::texture::MinFilter::Linear) => gl::LINEAR,
//                     Some(gltf::texture::MinFilter::NearestMipmapNearest) => {
//                         gl::NEAREST_MIPMAP_NEAREST
//                     }
//                     Some(gltf::texture::MinFilter::LinearMipmapNearest) => {
//                         gl::LINEAR_MIPMAP_NEAREST
//                     }
//                     Some(gltf::texture::MinFilter::NearestMipmapLinear) => {
//                         gl::NEAREST_MIPMAP_LINEAR
//                     }
//                     Some(gltf::texture::MinFilter::LinearMipmapLinear) => gl::LINEAR_MIPMAP_LINEAR,
//                     None => gl::LINEAR, // valoare implicită
//                 };

//                 let gl_mag_filter = match mag_filter {
//                     Some(gltf::texture::MagFilter::Nearest) => gl::NEAREST,
//                     Some(gltf::texture::MagFilter::Linear) => gl::LINEAR,
//                     None => gl::LINEAR, // valoare implicită
//                 };

//                 gl::BindTexture(gl::TEXTURE_2D, texture_id);
//                 gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl_wrap_s as i32);
//                 gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl_wrap_t as i32);
//                 gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl_min_filter as i32);
//                 gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl_mag_filter as i32);

//                 gl::TexImage2D(
//                     gl::TEXTURE_2D,
//                     0,
//                     format as i32,
//                     width as i32,
//                     height as i32,
//                     0,
//                     format,
//                     gl::UNSIGNED_BYTE,
//                     pixels.as_ptr() as *const _,
//                 );

//                 gl::GenerateMipmap(gl::TEXTURE_2D);
//             }

//             self.textures_map.insert(tex_index, texture_id);
//         }
//     }
//     fn process_node(&mut self, node: &gltf::Node, buffers: &[gltf::buffer::Data]) {
//         let mut mmat: glm::Mat4 = match node.transform() {
//             gltf::scene::Transform::Decomposed {
//                 translation,
//                 rotation,
//                 scale,
//             } => {
//                 let t =
//                     glm::translation(&glm::vec3(translation[0], translation[1], translation[2]));
//                 let r = glm::quat_to_mat4(&glm::quat(
//                     rotation[0],
//                     rotation[1],
//                     rotation[2],
//                     rotation[3],
//                 ));
//                 let s = glm::scaling(&glm::vec3(scale[0], scale[1], scale[2]));
//                 t * r * s
//             }
//             gltf::scene::Transform::Matrix { matrix } => matrix.into(),
//         };

//         if let Some(mesh) = node.mesh() {
//             for primitive in mesh.primitives() {
//                 let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

//                 let positions = reader
//                     .read_positions()
//                     .map(|i| i.flatten().collect())
//                     .unwrap_or_default();

//                 let texcoords = reader
//                     .read_tex_coords(0)
//                     .map(|i| i.into_f32().flatten().collect())
//                     .unwrap_or_default();

//                 let normals = reader
//                     .read_normals()
//                     .map(|i| i.flatten().collect())
//                     .unwrap_or_default();

//                 let indices = reader
//                     .read_indices()
//                     .map(|i| i.into_u32().collect())
//                     .unwrap_or_default();

//                 let mat = primitive.material().pbr_metallic_roughness();

//                 let base_color_texture = primitive
//                     .material()
//                     .pbr_metallic_roughness()
//                     .base_color_texture()
//                     .map(|info| info.texture().index());

//                 let material_data = Material {
//                     name: primitive.material().name().map(|s| s.to_string()),
//                     base_color_factor: mat.base_color_factor(),
//                     base_color_texture: base_color_texture,
//                     metallic_factor: mat.metallic_factor(),
//                     roughness_factor: mat.roughness_factor(),
//                 };

//                 self.meshes.push(Mesh {
//                     name: mesh.name().unwrap_or("unnamed").to_string(),
//                     position_coords: positions,
//                     texture_coords: texcoords,
//                     normal_coords: normals,
//                     indices,
//                     material: Some(material_data),
//                     vao: 0,
//                     vbo: 0,
//                     nbo: 0,
//                     tbo: 0,
//                     ebo: 0,
//                     matrix: mmat,
//                 });
//             }
//         }

//         // Recursiv pentru noduri copil
//         for child in node.children() {
//             self.process_node(&child, buffers);
//         }
//     }
// }

#[derive(Clone, Debug)]
pub struct Material {
    pub name: Option<String>,
    pub base_color_factor: [f32; 4],       // RGBA
    pub base_color_texture: Option<usize>, // calea sau ID-ul texturii
    pub metallic_factor: f32,
    pub roughness_factor: f32,
}
impl Material {
    pub fn new() -> Self {
        Self {
            name: None,
            base_color_factor: [0f32; 4],
            base_color_texture: None,
            metallic_factor: 0f32,
            roughness_factor: 0f32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub index: usize,
    pub name: String,
    pub position_coords: Vec<f32>,
    pub texture_coords: Vec<f32>,
    pub normal_coords: Vec<f32>,
    pub indices: Vec<u32>,
    pub material: Option<Material>,
    pub vao: u32,
    pub vbo: u32,
    pub nbo: u32,
    pub tbo: u32,
    pub ebo: u32,
    pub translation: glm::Mat4,
    pub rotation: glm::Mat4,
    pub scale: glm::Mat4,
}
impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
                self.vao = 0;
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
                self.vbo = 0;
            }
            if self.nbo != 0 {
                gl::DeleteBuffers(1, &self.nbo);
                self.nbo = 0;
            }
            if self.tbo != 0 {
                gl::DeleteBuffers(1, &self.tbo);
                self.tbo = 0;
            }
            if self.ebo != 0 {
                gl::DeleteBuffers(1, &self.ebo);
                self.ebo = 0;
            }
        }
    }
}



#[derive(Clone)]
pub struct AnimationChannel {
    pub node_index: usize,
    pub path: String, // "translation", "rotation", "scale"
    pub times: Vec<f32>,
    pub values: Vec<[f32; 4]>, // poate fi [f32; 3] pentru translation/scale, [f32; 4] pentru rotation
}

#[derive(Clone)]
pub struct Animation {
    pub name: Option<String>,
    pub channels: Vec<AnimationChannel>,
}

impl Animation {
    fn find_keyframe_pair(times: &[f32], time: f32) -> Option<(usize, usize)> {
        if times.is_empty() || time < times[0] || time > *times.last()? {
            return None;
        }
        for i in 0..times.len() - 1 {
            if time >= times[i] && time <= times[i + 1] {
                return Some((i, i + 1));
            }
        }
        None
    }
    pub fn apply_animation(&mut self, meshes: &mut Vec<Mesh>, time: f32) {
        for mesh in meshes.iter_mut() {
            // Variabile pentru transformările interpolate
            let mut translation = glm::identity();
            let mut rotation = glm::identity();
            let mut scale = glm::identity();

            // Pentru fiecare canal din animație care afectează acest mesh
            for channel in self.channels.iter().filter(|c| c.node_index == mesh.index) {
                // Găsim două keyframe-uri între care e timpul curent
                let (prev, next) = match Self::find_keyframe_pair(&channel.times, time) {
                    Some(pair) => pair,
                    None => continue,
                };

                let t0 = channel.times[prev];
                let t1 = channel.times[next];
                let a = (time - t0) / (t1 - t0);

                let v0 = channel.values[prev];
                let v1 = channel.values[next];

                match channel.path.as_str() {
                    "translation" => {
                        let p0 = glm::vec3(v0[0], v0[1], v0[2]);
                        let p1 = glm::vec3(v1[0], v1[1], v1[2]);
                        let interp = glm::mix(&p0, &p1, a);
                        translation = glm::translate(&glm::identity(), &interp);
                    }
                    "rotation" => {
                        let q0 = na::UnitQuaternion::from_quaternion(na::Quaternion::new(
                            v0[3], v0[0], v0[1], v0[2],
                        ));
                        let q1 = na::UnitQuaternion::from_quaternion(na::Quaternion::new(
                            v1[3], v1[0], v1[1], v1[2],
                        ));

                        let interp = q0.slerp(&q1, a);

                        rotation = interp.to_homogeneous();
                    }
                    "scale" => {
                        let s0 = glm::vec3(v0[0], v0[1], v0[2]);
                        let s1 = glm::vec3(v1[0], v1[1], v1[2]);
                        let interp = glm::mix(&s0, &s1, a);
                        scale = glm::scaling(&interp);
                    }
                    _ => {}
                }
            }
            // Aplicăm transformările în ordinea scale -> rotation -> translation
            // Poți schimba ordinea în funcție de ce vrei să obții
            if translation != glm::Mat4::identity() {
                mesh.translation = translation;
            }
            if rotation != glm::Mat4::identity() {
                mesh.rotation = rotation;
            }
            if scale != glm::Mat4::identity() {
                mesh.scale = scale;
            }
        }
    }
}

pub struct GLTFModel {
    pub meshes: Vec<Mesh>,
    pub textures_map: HashMap<usize, u32>,
    pub animations: Vec<Animation>,
}
impl Drop for GLTFModel {
    fn drop(&mut self) {
        unsafe {
            for &texture_id in self.textures_map.values() {
                if texture_id != 0 {
                    gl::DeleteTextures(1, &texture_id);
                }
            }
            self.textures_map.clear();
        }
    }
}

impl GLTFModel {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            textures_map: HashMap::new(),
            animations: Vec::new(),
        }
    }
    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (document, buffers, images) = gltf::import(path)?;

        self.load_textures(&document, &images);

        for scene in document.scenes() {
            for node in scene.nodes() {
                self.process_node(&node, &buffers);
            }
        }

        self.load_animations(&document, &buffers);

        Ok(())
    }
    fn load_animations(&mut self, document: &Document, buffers: &Vec<gltf::buffer::Data>) {
        for animation in document.animations() {
            let mut channels = Vec::new();

            for channel in animation.channels() {
                let target = channel.target();
                let node_index = target.node().index();
                let property = match target.property() {
                    gltf::animation::Property::Translation => "translation",
                    gltf::animation::Property::Rotation => "rotation",
                    gltf::animation::Property::Scale => "scale",
                    _ => continue,
                }
                .to_string();

                let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));

                let times: Vec<f32> = reader
                    .read_inputs()
                    .expect("Failed to read animation times")
                    .collect();

                let outputs = reader
                    .read_outputs()
                    .expect("Missing outputs for animation");

                let values: Vec<[f32; 4]> = match outputs {
                    gltf::animation::util::ReadOutputs::Translations(iter) => {
                        iter.map(|v| [v[0], v[1], v[2], 0.0]).collect()
                    }

                    gltf::animation::util::ReadOutputs::Scales(iter) => {
                        iter.map(|v| [v[0], v[1], v[2], 0.0]).collect()
                    }

                    gltf::animation::util::ReadOutputs::Rotations(
                        gltf::animation::util::Rotations::F32(iter),
                    ) => iter.map(|v| [v[0], v[1], v[2], v[3]]).collect(),

                    _ => continue, // Ignoră tipuri nefolosite
                };

                channels.push(AnimationChannel {
                    node_index,
                    path: property,
                    times,
                    values,
                });
            }

            self.animations.push(Animation {
                name: animation.name().map(|s| s.to_string()),
                channels,
            });
        }
    }
    fn load_textures(&mut self, document: &Document, images: &Vec<gltf::image::Data>) {
        for texture in document.textures() {
            let tex_index = texture.index();
            let image = &images[tex_index];
            let pixels = &image.pixels;
            let width = image.width;
            let height = image.height;

            let format = match image.format {
                gltf::image::Format::R8G8B8A8 => gl::RGBA,
                gltf::image::Format::R8G8B8 => gl::RGB,
                _ => {
                    eprintln!("Format nesuportat la textura {}", tex_index);
                    continue;
                }
            };

            let mut texture_id: u32 = 0;
            unsafe {
                gl::GenTextures(1, &mut texture_id);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);

                let sampler = texture.sampler();
                let wrap_s = sampler.wrap_s();
                let wrap_t = sampler.wrap_t();
                let min_filter = sampler.min_filter();
                let mag_filter = sampler.mag_filter();

                let gl_wrap_s = match wrap_s {
                    gltf::texture::WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
                    gltf::texture::WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
                    gltf::texture::WrappingMode::Repeat => gl::REPEAT,
                };
                let gl_wrap_t = match wrap_t {
                    gltf::texture::WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
                    gltf::texture::WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
                    gltf::texture::WrappingMode::Repeat => gl::REPEAT,
                };

                let gl_min_filter = match min_filter {
                    Some(gltf::texture::MinFilter::Nearest) => gl::NEAREST,
                    Some(gltf::texture::MinFilter::Linear) => gl::LINEAR,
                    Some(gltf::texture::MinFilter::NearestMipmapNearest) => {
                        gl::NEAREST_MIPMAP_NEAREST
                    }
                    Some(gltf::texture::MinFilter::LinearMipmapNearest) => {
                        gl::LINEAR_MIPMAP_NEAREST
                    }
                    Some(gltf::texture::MinFilter::NearestMipmapLinear) => {
                        gl::NEAREST_MIPMAP_LINEAR
                    }
                    Some(gltf::texture::MinFilter::LinearMipmapLinear) => gl::LINEAR_MIPMAP_LINEAR,
                    None => gl::LINEAR, // valoare implicită
                };

                let gl_mag_filter = match mag_filter {
                    Some(gltf::texture::MagFilter::Nearest) => gl::NEAREST,
                    Some(gltf::texture::MagFilter::Linear) => gl::LINEAR,
                    None => gl::LINEAR, // valoare implicită
                };

                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl_wrap_s as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl_wrap_t as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl_min_filter as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl_mag_filter as i32);

                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    format as i32,
                    width as i32,
                    height as i32,
                    0,
                    format,
                    gl::UNSIGNED_BYTE,
                    pixels.as_ptr() as *const _,
                );

                gl::GenerateMipmap(gl::TEXTURE_2D);
            }

            self.textures_map.insert(tex_index, texture_id);
        }
    }
    fn process_node(&mut self, node: &gltf::Node, buffers: &[gltf::buffer::Data]) {
        let (translation, rotation, scale) = match node.transform() {
            gltf::scene::Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => {
                let t =
                    glm::translation(&glm::vec3(translation[0], translation[1], translation[2]));
                let r = glm::quat_to_mat4(&glm::quat(
                    rotation[0],
                    rotation[1],
                    rotation[2],
                    rotation[3],
                ));
                let s = glm::scaling(&glm::vec3(scale[0], scale[1], scale[2]));
                (t, r, s)
            }
            gltf::scene::Transform::Matrix { matrix } => {
                let mut matrix: glm::Mat4 = matrix.into();
                // 1. Extrage translația
                let translation_vec = glm::vec3(matrix[(0, 3)], matrix[(1, 3)], matrix[(2, 3)]);
                let translation = glm::translate(&glm::identity(), &translation_vec);

                // 2. Extrage scale-ul
                let scale_x = glm::vec3(matrix[(0, 0)], matrix[(1, 0)], matrix[(2, 0)]).magnitude();
                let scale_y = glm::vec3(matrix[(0, 1)], matrix[(1, 1)], matrix[(2, 1)]).magnitude();
                let scale_z = glm::vec3(matrix[(0, 2)], matrix[(1, 2)], matrix[(2, 2)]).magnitude();
                let scale_vec = glm::vec3(scale_x, scale_y, scale_z);
                let scale = glm::scaling(&scale_vec);

                // 3. Extrage rotația (partea 3x3 fără scale)
                let mut rotation_matrix = glm::mat3(
                    matrix[(0, 0)] / scale_x,
                    matrix[(0, 1)] / scale_y,
                    matrix[(0, 2)] / scale_z,
                    matrix[(1, 0)] / scale_x,
                    matrix[(1, 1)] / scale_y,
                    matrix[(1, 2)] / scale_z,
                    matrix[(2, 0)] / scale_x,
                    matrix[(2, 1)] / scale_y,
                    matrix[(2, 2)] / scale_z,
                );
                let rotation = glm::mat4(
                    rotation_matrix[(0, 0)],
                    rotation_matrix[(0, 1)],
                    rotation_matrix[(0, 2)],
                    0.0,
                    rotation_matrix[(1, 0)],
                    rotation_matrix[(1, 1)],
                    rotation_matrix[(1, 2)],
                    0.0,
                    rotation_matrix[(2, 0)],
                    rotation_matrix[(2, 1)],
                    rotation_matrix[(2, 2)],
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                );
                (translation, rotation, scale)
            }
        };

        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let positions = reader
                    .read_positions()
                    .map(|i| i.flatten().collect())
                    .unwrap_or_default();

                let texcoords = reader
                    .read_tex_coords(0)
                    .map(|i| i.into_f32().flatten().collect())
                    .unwrap_or_default();

                let normals = reader
                    .read_normals()
                    .map(|i| i.flatten().collect())
                    .unwrap_or_default();

                let indices = reader
                    .read_indices()
                    .map(|i| i.into_u32().collect())
                    .unwrap_or_default();

                let mat = primitive.material().pbr_metallic_roughness();

                let base_color_texture = primitive
                    .material()
                    .pbr_metallic_roughness()
                    .base_color_texture()
                    .map(|info| info.texture().index());

                let material_data = Material {
                    name: primitive.material().name().map(|s| s.to_string()),
                    base_color_factor: mat.base_color_factor(),
                    base_color_texture: base_color_texture,
                    metallic_factor: mat.metallic_factor(),
                    roughness_factor: mat.roughness_factor(),
                };

                self.meshes.push(Mesh {
                    index: node.index(),
                    name: mesh.name().unwrap_or("unnamed").to_string(),
                    position_coords: positions,
                    texture_coords: texcoords,
                    normal_coords: normals,
                    indices,
                    material: Some(material_data),
                    vao: 0,
                    vbo: 0,
                    nbo: 0,
                    tbo: 0,
                    ebo: 0,
                    translation: translation,
                    rotation: rotation,
                    scale: scale,
                });
            }
        }

        // Recursiv pentru noduri copil
        for child in node.children() {
            self.process_node(&child, buffers);
        }
    }
}
