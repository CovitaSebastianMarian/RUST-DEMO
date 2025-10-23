use glfw::Context;
use glfw::{GlfwReceiver, fail_on_errors};


#[derive(PartialEq, Clone, Copy)]
pub enum MouseScroll {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct MouseButton {
    pub button: glfw::MouseButton,
    pub action: glfw::Action,
    pub modifiers: glfw::Modifiers,
}

pub struct Mouse {
    pub x: f64,
    pub y: f64,
    pub buttons: Vec<MouseButton>,
    pub scroll: Vec<MouseScroll>,
}
impl Mouse {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            buttons: Vec::new(),
            scroll: Vec::new(),
        }
    }
    pub fn reset(&mut self) {
        self.buttons.clear();
        self.scroll.clear();
    }
}
#[derive(PartialEq, Clone, Copy)]
pub struct KeyButton {
    pub key: glfw::Key,
    pub action: glfw::Action,
    pub modifiers: glfw::Modifiers,
    pub scancode: i32,
}

pub struct Keyboard {
    pub keys: Vec<KeyButton>,
    pub char_keys: Vec<char>,
}
impl Keyboard {
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            char_keys: Vec::new(),
        }
    }
    pub fn reset(&mut self) {
        self.keys.clear();
        self.char_keys.clear();
    }
    pub fn find_key(&self, key: glfw::Key) -> Option<KeyButton> {
        self.keys.iter().find(|x| x.key == key).copied()
    }
}
pub struct Window {
    pub width: u32,
    pub height: u32,
    glfw: Option<glfw::Glfw>,
    pub glfw_window: Option<glfw::PWindow>,
    pub glfw_events: Option<GlfwReceiver<(f64, glfw::WindowEvent)>>,
    resize: bool,
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub drag_and_drop_files: Vec<std::path::PathBuf>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            glfw: None,
            glfw_window: None,
            glfw_events: None,
            resize: false,
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
            drag_and_drop_files: Vec::new(),
        }
    }
    pub fn create(&mut self, width: u32, height: u32, title: &str) {
        self.width = width;
        self.height = height;

        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        //glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
        //glfw.window_hint(glfw::WindowHint::Decorated(false));
        glfw.window_hint(glfw::WindowHint::Resizable(true));
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Eroare la crearea ferestrei");

        window.make_current();
        window.focus();
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_char_polling(true);
        window.set_drag_and_drop_polling(true);

        gl::load_with(|s| {
            window
                .get_proc_address(s)
                .map(|p| p as *const _)
                .unwrap_or(std::ptr::null())
        });

        unsafe {
            gl::Viewport(0, 0, self.width as i32, self.height as i32);

            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        self.glfw = Some(glfw);
        self.glfw_window = Some(window);
        self.glfw_events = Some(events);
    }
    pub fn is_open(&self) -> bool {
        if let Some(ref window) = self.glfw_window {
            !window.should_close()
        } else {
            false
        }
    }
    pub fn close(&mut self) {
        if let Some(ref mut window) = self.glfw_window {
            window.set_should_close(true);
        }
    }
    
    pub fn get_key(&self, key: glfw::Key) -> Option<glfw::Action> {
        if let Some(ref window) = self.glfw_window {
            Some(window.get_key(key))
        }
        else {
            None
        }
    }
    pub fn get_mouse_button(&self, button: glfw::MouseButton) -> Option<glfw::Action> {
        if let Some(ref window) = self.glfw_window {
            Some(window.get_mouse_button(button))
        }
        else {
            None
        }
    }
    pub fn is_resized(&self) -> bool {
        self.resize
    }
    pub fn poll_events(&mut self) {
        if let Some(ref mut glfw) = self.glfw {
            glfw.poll_events();
        } else {
            return;
        }
        self.resize = false;
        self.mouse.reset();
        self.keyboard.reset();
        self.drag_and_drop_files.clear();
        if let Some(ref events) = self.glfw_events {
            for (_, event) in glfw::flush_messages(events) {
                match event {
                    glfw::WindowEvent::FramebufferSize(w, h) => {
                        self.width = w as u32;
                        self.height = h as u32;
                        self.resize = true;
                        unsafe {
                            gl::Viewport(0, 0, w, h);
                        }
                    }
                    glfw::WindowEvent::CursorPos(x, y) => {
                        self.mouse.x = x;
                        self.mouse.y = y;
                    }
                    glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                        self.mouse.buttons.push(MouseButton {
                            button,
                            action,
                            modifiers,
                        });
                    }
                    glfw::WindowEvent::Scroll(x, y) => {
                        if y > 0.0 {
                            self.mouse.scroll.push(MouseScroll::Up);
                        } else if y < 0.0 {
                            self.mouse.scroll.push(MouseScroll::Down);
                        }

                        if x > 0.0 {
                            self.mouse.scroll.push(MouseScroll::Right);
                        } else if x < 0.0 {
                            self.mouse.scroll.push(MouseScroll::Left);
                        }
                    }
                    glfw::WindowEvent::Char(x) => {
                        self.keyboard.char_keys.push(x);
                    }
                    glfw::WindowEvent::FileDrop(v) => {
                        self.drag_and_drop_files = v;
                    }
                    glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                        self.keyboard.keys.push(KeyButton {
                            key,
                            action,
                            modifiers,
                            scancode,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        if let Some(win) = &self.glfw_window {
            win.get_framebuffer_size()
        } else {
            (0, 0)
        }
    }
    pub fn set_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }
    pub fn swap_buffers(&mut self) {
        if let Some(ref mut window) = self.glfw_window {
            window.swap_buffers();
        }
    }
}
