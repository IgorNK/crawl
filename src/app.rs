use crate::api::{self, ResponseData};
use crate::todos::{Todo, TodoList};
use crate::window_manager::{self, Windows};
use crate::chat_component::ChatComponent;
use crate::View;
use std::sync::mpsc::{self, Receiver, Sender};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    windows: Windows,

    #[serde(skip)]
    chat: ChatComponent,

    #[serde(skip)]
    tx: Sender<ResponseData>,
    #[serde(skip)]
    rx: Receiver<ResponseData>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            windows: Windows::default(),
            chat: ChatComponent::default(),
            tx,
            rx,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // #[cfg(target_arch = "wasm32")]
        log::warn!("We're in app baby!");
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { windows, chat, tx, rx } = self;

        if let Ok(result) = rx.try_recv() {
            match result {
                ResponseData::GetResponse(result) => {
                    if let Ok(result) = result {
                        //todos.todos = result;
                    }
                }
                ResponseData::PostResponse(result) => {
                    if let Ok(_result) = result {
                        // *todo_title = String::new();
                        // *todo_content = String::new();
                        // #[cfg(target_arch = "wasm32")]
                        // api::get_todos_web(tx.clone());
                        // #[cfg(not(target_arch = "wasm32"))]
                        // api::get_todos(tx.clone());
                    }
                }
            }
        }

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });

                //if ui.button("fetch").clicked() {
                //    #[cfg(target_arch = "wasm32")]
                //    api::get_todos_web(tx.clone());
                //    #[cfg(not(target_arch = "wasm32"))]
                //    api::get_todos(tx.clone());
                //}
                let mut new_character_open = windows.open.contains("New Character");
                ui.toggle_value(&mut new_character_open, "New Character");
                window_manager::set_open(&mut windows.open, "New Character", new_character_open);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            windows.windows(ctx);
            chat.ui(ui, ctx);
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
