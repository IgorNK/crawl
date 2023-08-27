use std::collections::HashMap;

use crate::{node::Node, zoom_pan::ZoomPanState};
use eframe::egui;

pub struct ZoomPanContainer {
    id: egui::Id,
    nodes: HashMap<egui::Id, Node>,
}

impl ZoomPanContainer {
    pub fn new<I: Into<egui::Id>>(id: I) -> Self {
        Self {
            id: id.into(),
            nodes: HashMap::new(),
        }
    }
}

impl ZoomPanContainer {
    fn show_nodes(&mut self, ui: &mut egui::Ui, zoom_pan_state: &ZoomPanState) {
        let mut dragged_node = None;
        for node in self.nodes.values_mut() {
            let node_state = node.show(ui, zoom_pan_state);
            if node_state.dragged {
                dragged_node = Some(node_state);
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let mut zoom_pan_state =
            ZoomPanState::load(ui, self.id).unwrap_or_else(|| ZoomPanState::new(self.id, ui));

        zoom_pan_state.prepare(ui.style());
        zoom_pan_state.screen_rect = ui.available_rect_before_wrap();

        let mut response = ui.interact(
            zoom_pan_state.screen_rect,
            self.id.with("graph"),
            egui::Sense::drag(),
        );

        zoom_pan_state.show_zoomed(ui, |ui| self.show_nodes(ui, &zoom_pan_state));

        if let Some(pos) = ui.ctx().pointer_latest_pos() {
            let zoom = ui.input(|i| i.scroll_delta.y);
            if zoom != 0. && zoom_pan_state.screen_rect.contains(pos) {
                let zoom = (zoom * 0.002).exp();
                zoom_pan_state.zoom_to_screen(pos, zoom);
            }
        }

        zoom_pan_state.store(ui, self.id);
    }

    pub fn add_node(&mut self, node: Node) -> egui::Id {
        let id = node.id.clone();
        self.nodes.insert(id, node);
        id
    }

    pub fn remove_node(&mut self, node_id: egui::Id) {
        self.nodes.remove(&node_id);
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.values_mut()
    }

    pub fn node(&self, node_id: egui::Id) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    pub fn node_mut(&mut self, node_id: egui::Id) -> Option<&mut Node> {
        self.nodes.get_mut(&node_id)
    }
}
