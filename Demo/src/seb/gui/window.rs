use std::io::SeekFrom;

use crate::seb::{
    gui::{
        gui::Clip,
        panel::{self, Panel, PanelRenderer},
        text::{self, TextBoxD, TextBoxRenderer, TextFont},
    },
    window::Window,
};
use gl::NONE;
use glfw::MouseButton::Button1;
use nalgebra_glm as glm;



pub struct GuiRenderer {
    font: TextFont,
    pr: PanelRenderer,
    tr: TextBoxRenderer,
}
impl GuiRenderer {
    pub fn new(font_path: &str, scale: f32) -> Self {
        let mut font = TextFont::new(font_path, scale);
        font.init_chars_texture(false);
        Self {
            font,
            pr: PanelRenderer::new(),
            tr: TextBoxRenderer::new(),
        }
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.pr.set_size(width, height);
        self.tr.set_size(width, height);
    }
    pub fn draw(&mut self, gui_builder: &mut GuiBuilder, width: u32, height: u32) {
        self.set_size(width, height);
        self.pr.draw(&mut gui_builder.panels);
        self.tr.draw(&self.font, &mut gui_builder.texts);
    }
}
pub struct GuiBuilder {
    position: glm::Vec2,
    size: glm::Vec2,
    panels: Vec<Panel>,
    texts: Vec<TextBoxD>,
}
impl GuiBuilder {
    pub fn new(pos: glm::Vec2, size: glm::Vec2) -> Self {
        Self {
            position: pos,
            size: size,
            panels: Vec::new(),
            texts: Vec::new(),
        }
    }
    pub fn push_panel(&mut self, mut panel: Panel) -> Panel {
        panel.position += self.position;
        let parent_clip = Clip::from(self.position, self.size);

        match &mut panel.clip {
            Some(c) => {
                c.position += self.position;
                *c = c.intersect(&parent_clip);
            }
            None => {
                panel.clip = Some(parent_clip);
            }
        }

        self.panels.push(panel);
        panel
    }

    pub fn push_text(&mut self, mut text: TextBoxD) -> TextBoxD {
        text.position += self.position;
        let parent_clip = Clip::from(self.position, self.size);

        match &mut text.clip {
            Some(c) => {
                c.position += self.position;
                *c = c.intersect(&parent_clip);
            }
            None => {
                text.clip = Some(parent_clip);
            }
        }

        self.texts.push(text.clone());
        text
    }
    pub fn push_elements(&mut self, mut elements: (Vec<Panel>, Vec<TextBoxD>)) {
        self.panels.append(&mut elements.0);
        self.texts.append(&mut elements.1);
    }
    pub fn get_elements(&self) -> (Vec<Panel>, Vec<TextBoxD>) {
        (self.panels.clone(), self.texts.clone())
    }
    pub fn add_window<F>(&mut self, mut window_panel: Panel, f: F)
    where
        F: FnOnce(&mut GuiBuilder),
    {
        let new_p = self.push_panel(window_panel);

        let mut sub_builder = GuiBuilder::new(new_p.position, new_p.size);

        f(&mut sub_builder);

        self.push_elements(sub_builder.get_elements());
    }
    pub fn reset(&mut self) {
        self.panels.clear();
        self.texts.clear();
    }
}
