use crate::node::NodeData;
use eframe::egui;

pub struct NodeImageData {
    pub texture: Option<egui::TextureHandle>,
}

impl NodeData for NodeImageData {
    fn set_location(&mut self, _new_location: egui::Pos2) {}
    fn show(&mut self, ui: &mut egui::Ui) {
        if let Some(texture) = &self.texture {
            ui.add(egui::Image::new(texture, texture.size_vec2()));
        }
    }
}
