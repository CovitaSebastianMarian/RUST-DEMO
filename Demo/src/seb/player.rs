use crate::Window;

use nalgebra as na;
use nalgebra_glm as glm;

pub struct Player {
    pub projection: glm::Mat4,
    pub view: glm::Mat4,
    pub cam_pos: glm::Vec3,
    pub cam_center: glm::Vec3,
    pub cam_up: glm::Vec3,
    yaw: f32,
    pitch: f32,
    last_mouse_x: f64,
    last_mouse_y: f64,
    first_mouse: bool,
    mouse_lock: bool,
}
impl Player {
    pub fn new(y: f32) -> Self {
        Self {
            projection: glm::Mat4::identity(),
            view: glm::Mat4::identity(),
            cam_pos: glm::vec3(0.0, y, 0.0),
            cam_center: glm::vec3(0.0, y, 1.0),
            cam_up: glm::vec3(0.0, 0.1, 0.0),
            yaw: 90.0, // privim înainte pe Z
            pitch: 0.0,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
            first_mouse: true,
            mouse_lock: false,
        }
    }
    pub fn bind(&mut self, window: &mut Window, speed: f32) {
        if let Some(key) = window.keyboard.find_key(glfw::Key::Tab)
            && key.action == glfw::Action::Press
        {
            if self.mouse_lock {
                window
                    .glfw_window
                    .as_mut()
                    .unwrap()
                    .set_cursor_mode(glfw::CursorMode::Normal);
                self.mouse_lock = false;
            } else {
                window
                    .glfw_window
                    .as_mut()
                    .unwrap()
                    .set_cursor_mode(glfw::CursorMode::Disabled);
                self.mouse_lock = true;
            }
        }

        let mut up: glm::Vec3 = glm::normalize(&self.cam_up);
        let mut dir = glm::normalize(&(self.cam_center - self.cam_pos));
        //dir.y = 0.0; // asta e daca vreau sa nu mai ma misc in sus
        let right = glm::normalize(&glm::cross(&dir, &up));
        if window.get_key(glfw::Key::W).unwrap() == glfw::Action::Press {
            self.cam_pos += dir * speed;
            self.cam_center += dir * speed;
        }
        if window.get_key(glfw::Key::S).unwrap() == glfw::Action::Press {
            self.cam_pos -= dir * speed;
            self.cam_center -= dir * speed;
        }
        if window.get_key(glfw::Key::A).unwrap() == glfw::Action::Press {
            self.cam_pos -= right * speed;
            self.cam_center -= right * speed;
        }
        if window.get_key(glfw::Key::D).unwrap() == glfw::Action::Press {
            self.cam_pos += right * speed;
            self.cam_center += right * speed;
        }
        if self.mouse_lock {
            let xmouse: f64 = window.mouse.x;
            let ymouse: f64 = window.mouse.y;

            let sensitivity = 0.1;

            if self.first_mouse {
                self.last_mouse_x = xmouse;
                self.last_mouse_y = ymouse;
                self.first_mouse = false;
            }

            let xoffset = ((xmouse - self.last_mouse_x) as f32) * sensitivity;
            let yoffset = ((self.last_mouse_y - ymouse) as f32) * sensitivity;

            self.last_mouse_x = xmouse;
            self.last_mouse_y = ymouse;

            self.yaw += xoffset;
            self.pitch += yoffset;

            // limitează pitch-ul
            self.pitch = self.pitch.clamp(-89.0, 89.0);

            // calculează noua direcție
            let yaw_rad = self.yaw.to_radians();
            let pitch_rad = self.pitch.to_radians();

            let direction = glm::vec3(
                yaw_rad.cos() * pitch_rad.cos(),
                pitch_rad.sin(),
                yaw_rad.sin() * pitch_rad.cos(),
            );
            let dir_normalized = glm::normalize(&direction);

            //aici vreau sa rotesc dir_normalized ca sa fie la 90 de grade cu up
            //let dir_ortho = glm::normalize(&(dir_normalized - up * glm::dot(&dir_normalized, &up)));

            self.cam_center = self.cam_pos + dir_normalized;
        }
        let w = window.width as f32;
        let h = window.height as f32;

        self.projection = glm::perspective_fov(70f32.to_radians(), w, h, 0.01, 100.0);
        self.view = glm::look_at(&self.cam_pos, &self.cam_center, &up);
    }
    pub fn bind2(&mut self, window: &mut Window, speed: f32) {
        if let Some(key) = window.keyboard.find_key(glfw::Key::Tab)
            && key.action == glfw::Action::Press
        {
            if self.mouse_lock {
                window
                    .glfw_window
                    .as_mut()
                    .unwrap()
                    .set_cursor_mode(glfw::CursorMode::Normal);
                self.mouse_lock = false;
            } else {
                window
                    .glfw_window
                    .as_mut()
                    .unwrap()
                    .set_cursor_mode(glfw::CursorMode::Disabled);
                self.mouse_lock = true;
            }
        }

        // Calculăm vectorii de bază folosind cam_up actualizat
        let up: glm::Vec3 = glm::normalize(&self.cam_up);
        let mut dir = glm::normalize(&(self.cam_center - self.cam_pos));
        let right = glm::normalize(&glm::cross(&dir, &up));

        // Recalculăm dir să fie perpendicular pe up (pentru consistență)
        dir = glm::normalize(&glm::cross(&up, &right));

        if window.get_key(glfw::Key::W).unwrap() == glfw::Action::Press {
            self.cam_pos += dir * speed;
            self.cam_center += dir * speed;
        }
        if window.get_key(glfw::Key::S).unwrap() == glfw::Action::Press {
            self.cam_pos -= dir * speed;
            self.cam_center -= dir * speed;
        }
        if window.get_key(glfw::Key::A).unwrap() == glfw::Action::Press {
            self.cam_pos -= right * speed;
            self.cam_center -= right * speed;
        }
        if window.get_key(glfw::Key::D).unwrap() == glfw::Action::Press {
            self.cam_pos += right * speed;
            self.cam_center += right * speed;
        }

        if self.mouse_lock {
            let xmouse: f64 = window.mouse.x;
            let ymouse: f64 = window.mouse.y;

            let sensitivity = 0.002; // radiani per pixel

            if self.first_mouse {
                self.last_mouse_x = xmouse;
                self.last_mouse_y = ymouse;
                self.first_mouse = false;
            }

            let xoffset = (xmouse - self.last_mouse_x) as f32 * sensitivity;
            let yoffset = (self.last_mouse_y - ymouse) as f32 * sensitivity;

            self.last_mouse_x = xmouse;
            self.last_mouse_y = ymouse;

            // Actualizează yaw și pitch cumulative
            self.yaw += xoffset;
            self.pitch += yoffset;

            // Clamp pitch între -89° și 89°
            let pitch_limit = 89.0_f32.to_radians();
            self.pitch = self.pitch.clamp(-pitch_limit, pitch_limit);

            // Vectorul forward inițial (în spațiul camerei)
            let forward = glm::vec3(0.0, 0.0, -1.0);

            // Aplicăm yaw (rotație în jurul up)
            let yaw_rot = glm::rotation(-self.yaw, &up);
            let dir_after_yaw = (yaw_rot * glm::vec4(forward.x, forward.y, forward.z, 0.0)).xyz();

            // Calculăm right-ul actualizat după yaw
            let right_updated = glm::normalize(&glm::cross(&dir_after_yaw, &up));

            // Aplicăm pitch (rotație în jurul right)
            let pitch_rot = glm::rotation(self.pitch, &right_updated);
            let final_dir = (pitch_rot
                * glm::vec4(dir_after_yaw.x, dir_after_yaw.y, dir_after_yaw.z, 0.0))
            .xyz();

            self.cam_center = self.cam_pos + glm::normalize(&final_dir);

            self.cam_up = glm::normalize(&glm::cross(&right_updated, &glm::normalize(&final_dir)));
        }

        let w = window.width as f32;
        let h = window.height as f32;

        self.projection = glm::perspective_fov(70f32.to_radians(), w, h, 0.01, 100.0);
        self.view = glm::look_at(
            &self.cam_pos,
            &self.cam_center,
            &glm::normalize(&self.cam_up),
        );
    }
    pub fn bind_sphere(
        &mut self,
        window: &mut crate::seb::window::Window,
        speed: f32,
        planet_center: glm::Vec3,
        planet_radius: f32,
    ) {
        // --- Toggle mouse lock
        if let Some(key) = window.keyboard.find_key(glfw::Key::Tab)
            && key.action == glfw::Action::Press
        {
            if self.mouse_lock {
                window
                    .glfw_window
                    .as_mut()
                    .unwrap()
                    .set_cursor_mode(glfw::CursorMode::Normal);
                self.mouse_lock = false;
            } else {
                window
                    .glfw_window
                    .as_mut()
                    .unwrap()
                    .set_cursor_mode(glfw::CursorMode::Disabled);
                self.mouse_lock = true;
            }
        }

        // --- Sistemul local
        let mut up = glm::normalize(&(self.cam_pos - planet_center));
        let mut forward = self.cam_center - self.cam_pos;

        if glm::length(&forward) < 1e-6 {
            // fallback dacă forward = 0
            let mut tmp = glm::cross(&up, &glm::vec3(0.0, 1.0, 0.0));
            if glm::length(&tmp) < 1e-6 {
                tmp = glm::cross(&up, &glm::vec3(1.0, 0.0, 0.0));
            }
            forward = glm::normalize(&tmp);
        } else {
            forward = glm::normalize(&forward);
        }
        let mut right = glm::normalize(&glm::cross(&forward, &up));

        let mut target_dist = glm::length(&(self.cam_center - self.cam_pos));
        if target_dist < 1e-4 {
            target_dist = 1.0;
        }
        let mut altitude = glm::length(&(self.cam_pos - planet_center));
        let min_altitude = planet_radius + 0.01;
        if altitude < min_altitude {
            altitude = min_altitude;
        }

        // --- Mouse look
        if self.mouse_lock {
            let (xmouse, ymouse) = (window.mouse.x, window.mouse.y);
            let sensitivity = 0.1;

            if self.first_mouse {
                self.last_mouse_x = xmouse;
                self.last_mouse_y = ymouse;
                self.first_mouse = false;
            }

            let xoffset = ((xmouse - self.last_mouse_x) as f32) * sensitivity;
            let yoffset = ((self.last_mouse_y - ymouse) as f32) * sensitivity;

            self.last_mouse_x = xmouse;
            self.last_mouse_y = ymouse;

            // yaw
            if xoffset.abs() > 0.0 {
                forward = glm::rotate_vec3(&forward, (-xoffset).to_radians(), &up);
                forward = glm::normalize(&forward);
                right = glm::normalize(&glm::cross(&forward, &up));
            }

            // pitch
            if yoffset.abs() > 0.0 {
                let candidate = glm::rotate_vec3(&forward, yoffset.to_radians(), &right);
                let dot_up = glm::dot(&candidate, &up).clamp(-1.0, 1.0);
                let ang = dot_up.acos();
                let min_ang = 5.0_f32.to_radians();
                let max_ang = (180.0f32 - 5.0).to_radians();
                if ang > min_ang && ang < max_ang {
                    forward = glm::normalize(&candidate);
                    right = glm::normalize(&glm::cross(&forward, &up));
                }
            }
        }

        let forward_tangent = {
            let f = forward - up * glm::dot(&forward, &up);
            if glm::length(&f) > 1e-6 {
                glm::normalize(&f)
            } else {
                forward
            }
        };
        let right_tangent = right;

        let mut delta = glm::vec3(0.0, 0.0, 0.0);
        if window.get_key(glfw::Key::W).unwrap() == glfw::Action::Press {
            delta += forward_tangent * speed;
        }
        if window.get_key(glfw::Key::S).unwrap() == glfw::Action::Press {
            delta -= forward_tangent * speed;
        }
        if window.get_key(glfw::Key::A).unwrap() == glfw::Action::Press {
            delta -= right_tangent * speed;
        }
        if window.get_key(glfw::Key::D).unwrap() == glfw::Action::Press {
            delta += right_tangent * speed;
        }

        if glm::length(&delta) > 0.0 {
            let old_up = up;
            let tentative = self.cam_pos + delta;
            let new_up = glm::normalize(&(tentative - planet_center));

            self.cam_pos = planet_center + new_up * altitude;

            // Parallel transport
            let axis = glm::cross(&old_up, &new_up);
            let axis_len = glm::length(&axis);
            if axis_len > 1e-6 {
                let axis_n = axis / axis_len;
                let angle = glm::dot(&old_up, &new_up).clamp(-1.0, 1.0).acos();
                forward = glm::rotate_vec3(&forward, angle, &axis_n);
                forward = glm::normalize(&forward);
            }

            up = new_up;
            right = glm::normalize(&glm::cross(&forward, &up));
        }

        forward = glm::normalize(&forward);
        //right   = glm::normalize(&glm::cross(&forward, &up));
        up = glm::normalize(&glm::cross(&right, &forward));

        self.cam_center = self.cam_pos + forward * target_dist;
        self.cam_up = up;

        let w = window.width as f32;
        let h = window.height as f32;
        self.projection = glm::perspective_fov(70f32.to_radians(), w, h, 0.001, 1000.0);
        self.view = glm::look_at(&self.cam_pos, &self.cam_center, &self.cam_up);
    }
    pub fn add_height(&mut self, planet_pos: glm::Vec3, h: f32) {
        let forward = glm::normalize(&(self.cam_center - self.cam_pos));

        let dist = glm::length(&(self.cam_center - self.cam_pos));

        let len = glm::length(&(self.cam_pos - planet_pos)) + h;
        let dir = glm::normalize(&(self.cam_pos - planet_pos));
        self.cam_pos = dir * len;

        self.cam_center = self.cam_pos + forward * dist;
    }
    pub fn set_height(&mut self, ground_pos: glm::Vec3, planet_pos: glm::Vec3) {
        let radial = glm::normalize(&(self.cam_pos - planet_pos));

        let r_ground = glm::length(&(ground_pos - planet_pos)); // f32

        let target_r = r_ground; // tot f32

        let forward = glm::normalize(&(self.cam_center - self.cam_pos));

        let dist = glm::length(&(self.cam_center - self.cam_pos));

        self.cam_pos = planet_pos + radial * target_r;

        self.cam_center = self.cam_pos + forward * dist;
    }
}
