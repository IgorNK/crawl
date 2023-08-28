use crate::zoom_pan::ZoomPanState;
use eframe::egui;

pub struct NodeData {
  caption: String,
}

impl NodeData {
    fn set_location(&mut self, _new_location: egui::Pos2) {}
    fn show(&mut self, ui: &mut egui::Ui) {
      ui.label(&self.caption);
    }
}

pub struct Node {
    pub id: egui::Id,
    pub caption: String,
    pub location: egui::Pos2,
    data: Box<NodeData>,
}

impl Node {
    pub fn new(caption: &str, location: egui::Pos2) -> Self {
        Self {
            id: egui::Id::new(caption),
            caption: caption.to_string(),
            location,
            data: Box::new(NodeData { caption: caption.to_string() }),
        }
    }

    pub(crate) fn show(&mut self, ui: &mut egui::Ui, zoom_pan_state: &ZoomPanState) -> NodeState {
        let node_id = self.id;
        let id = zoom_pan_state.child_id(node_id);
        let mut node_state = NodeState::load(ui, id).unwrap_or_else(NodeState::new);
        let screen_location = zoom_pan_state.pos2_area_to_screen(self.location);

        let response = egui::Area::new(id)
            .order(egui::Order::Middle)
            .current_pos(screen_location)
            .drag_bounds(egui::Rect::EVERYTHING)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(zoom_pan_state.screen_rect);
                ui.set_max_size(egui::Vec2::INFINITY);

                egui::Frame::window(ui.style())
                    .shadow(egui::epaint::Shadow::default())
                    .inner_margin(25.0f32)
                    .show(ui, |ui| {
                        self.data.show(ui);
                    });
            })
            .response;

        node_state.drag_started = false;
        if let Some(pos) = ui.ctx().pointer_latest_pos() {
            if response.drag_started() && zoom_pan_state.screen_rect.contains(pos) {
                node_state.drag_started = true;
                node_state.dragged = true;
            }
        }
        if !response.dragged() {
            node_state.dragged = false;
        }

        if node_state.dragged && response.drag_delta() != egui::Vec2::ZERO {
            let new_location =
                self.location + zoom_pan_state.vec2_screen_to_area(response.drag_delta());
            self.location = new_location;
            self.data_mut().set_location(new_location);
        }

        node_state.clone().store(ui, id);
        node_state
    }

    pub fn data(&self) -> &NodeData {
        &*self.data
    }

    pub fn data_mut(&mut self) -> &mut NodeData {
        &mut *self.data
    }
}

#[derive(Clone)]
pub(crate) struct NodeState {
    pub drag_started: bool,
    pub dragged: bool,
}

impl NodeState {
    fn load(ui: &mut egui::Ui, id: egui::Id) -> Option<NodeState> {
        ui.data(|data| data.get_temp(id))
    }

    fn store(self, ui: &mut egui::Ui, id: egui::Id) {
        ui.data_mut(|data| data.insert_temp(id, self));
    }

    fn new() -> Self {
        Self {
            drag_started: false,
            dragged: false,
        }
    }
}
