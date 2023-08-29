use std::collections::HashMap;

use crate::utils::{load_image_from_bytes, load_image_from_path};
use crate::{
    node::{Node, NodeData},
    node_image::NodeImageData,
    zoom_pan::ZoomPanState,
};
use bytes::Bytes;
use eframe::egui;
use std::sync::{mpsc, Arc};

pub struct ZoomPanContainer {
    id: egui::Id,
    nodes: HashMap<egui::Id, Node>,
    rx: mpsc::Receiver<Arc<Bytes>>,
    pub tx: mpsc::Sender<Arc<Bytes>>,
}

impl ZoomPanContainer {
    pub fn new<I: Into<egui::Id>>(id: I) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            id: id.into(),
            nodes: HashMap::new(),
            rx,
            tx,
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
        // Handle pan

        zoom_pan_state.drag(response.drag_delta());

        // Handle zoom
        if let Some(pos) = ui.ctx().pointer_latest_pos() {
            let zoom = ui.input(|i| i.scroll_delta.y);
            if zoom != 0. && zoom_pan_state.screen_rect.contains(pos) {
                let zoom = (zoom * 0.002).exp();
                zoom_pan_state.zoom_to_screen(pos, zoom);
            }
        }

        zoom_pan_state.store(ui, self.id);

        self.detect_files_being_dropped(ui);
        self.wait_for_image(ui);
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

    fn detect_files_being_dropped(&mut self, ui: &mut egui::Ui) {
        let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        if !dropped_files.is_empty() {
            for file in dropped_files {
                dbg!(&file);
                if let Some(path) = &file.path {
                    if let Ok(image) = load_image_from_path(path) {
                        let texture = ui.ctx().load_texture("tex", image, Default::default());
                        let node_data = NodeImageData {
                            texture: Some(texture),
                        };
                        let node = Node::new("Image node", egui::Pos2::ZERO).with_data(node_data);
                        self.add_node(node);
                    }
                }
            }
        }
    }

    fn wait_for_image(&mut self, ui: &mut egui::Ui) {
        if let Ok(bytes) = self.rx.try_recv() {
            dbg!("try receive");
            if let Ok(image) = load_image_from_bytes(bytes) {
                let texture = ui.ctx().load_texture("tex", image, Default::default());
                let node_data = NodeImageData {
                    texture: Some(texture),
                };
                let node = Node::new("Image node", egui::Pos2::ZERO).with_data(node_data);
                self.add_node(node);
            }
        }
    }
}
