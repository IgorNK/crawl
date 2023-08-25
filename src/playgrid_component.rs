use eframe::egui;
use crate::zoom_pan::{ZoomPanState, ZoomPan};

const GRID_SIZE: f32 = 32f32;

struct GridCell {
    id: egui::Id,
    rect: egui::Rect,
    x: u32,
    y: u32,
    offset: egui::Vec2,
}

impl GridCell {
    fn new(x: u32, y: u32) -> Self {
        GridCell {
            id: egui::Id::new("GridCell")
                .with(format!("x{}", x))
                .with(format!("y{}", y)),
            rect: egui::Rect::from_min_size(
                egui::Pos2 {
                    x: x as f32 * GRID_SIZE,
                    y: y as f32 * GRID_SIZE,
                },
                egui::Vec2 {
                    x: GRID_SIZE,
                    y: GRID_SIZE,
                },
            ),
            x,
            y,
            offset: egui::vec2(0f32, 0f32),
        }
    }

    fn set_offset(&mut self, amnt: egui::Vec2) {
        self.offset = amnt;
    }
}

impl crate::View for GridCell {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());
        let rect = self.rect.translate(self.offset);
        let response = ui.interact(rect.clone(), self.id.clone(), egui::Sense::hover());
        let background = ui.painter().add(egui::Shape::Noop); // Put something later
        let style = if is_being_dragged && response.hovered() {
            ui.visuals().widgets.active
        } else {
            ui.visuals().widgets.inactive
        };

        let fill = style.bg_fill;
        let stroke = style.bg_stroke;

        ui.painter().set(
            background,
            egui::epaint::RectShape {
                rect,
                rounding: style.rounding,
                fill,
                stroke,
            },
        );
    }
}

pub struct PlayGridComponent {
    id: egui::Id,
    cells: Vec<GridCell>,
    zoom_pan_state: ZoomPanState,
}

impl PlayGridComponent {
    pub fn new(size: egui::Vec2) -> Self {
        let id = egui::Id::new("Play grid");
        PlayGridComponent {
            id,
            cells: PlayGridComponent::create_cells(size),
            zoom_pan_state: ZoomPanState::new(id),
        }
    }

    fn create_cells(size: egui::Vec2) -> Vec<GridCell> {
        let amount_x = size.x / GRID_SIZE as f32;
        let amount_y = size.y / GRID_SIZE as f32;
        let mut cells = vec![];
        for i in 0..amount_x as u32 {
            for j in 0..amount_y as u32 {
                cells.push(GridCell::new(i, j));
            }
        }
        cells
    }
}

impl crate::View for PlayGridComponent {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Frame::none().fill(egui::Color32::RED).show(ui, |ui| {
            let (id, rect) = ui.allocate_space(egui::vec2(2048.0, 2048.0));
            for cell in &mut self.cells {
                cell.ui(ui, ctx);
                cell.set_offset(rect.min.to_vec2());
            }
        });
    }
}

impl ZoomPan for PlayGridComponent {
  fn zoom_pan_state(&mut self) -> &mut ZoomPanState {
    &mut self.zoom_pan_state
  }

  fn id(&self) -> &egui::Id {
    &self.id
  }
}
