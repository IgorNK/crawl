use crate::model::{Attributes, Class, SystemData};
use arc_swap::ArcSwap;
use eframe::egui::{self, Align, Layout};
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use std::sync::Arc;

pub struct ChatComponent {
    url: String,
    error: String,
    sender: Option<WsSender>,
    receiver: Option<WsReceiver>,
    events: Vec<WsEvent>,
    text_to_send: String,
}

impl Default for ChatComponent {
    fn default() -> Self {
        ChatComponent {
            url: String::new(),
            error: String::new(),
            sender: None,
            receiver: None,
            events: vec![],
            text_to_send: String::new(),
        }
    }
}

impl crate::Window for ChatComponent {
    fn name(&self) -> &'static str {
        "Chat"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use crate::View as _;
        egui::Window::new(self.name())
            .open(open)
            .default_height(500.0)
            .show(ctx, |ui| self.ui(ui, ctx));
    }
}

impl crate::View for ChatComponent {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(receiver) = &self.receiver {
            while let Some(event) = receiver.try_recv() {
                self.events.push(event);
            }
        }

        ui.label("URL: ");
        if ui.text_edit_singleline(&mut self.url).lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
        {
            self.connect(ctx.clone());
        }

        ui.separator();

        if let Some(ref mut sender) = &mut self.sender {
            ui.horizontal(|ui| {
                ui.label("Message to send:");
                if ui.text_edit_singleline(&mut self.text_to_send).lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    sender.send(WsMessage::Text(std::mem::take(&mut self.text_to_send)));
                }
            });

            ui.separator();

            ui.heading("Received events:");
            for event in &self.events {
                ui.label(format!("{:?}", event));
            }
        }

        if !self.error.is_empty() {
            egui::TopBottomPanel::bottom("error").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Error:");
                    ui.colored_label(egui::Color32::RED, &self.error);
                });
            });
        }
    }
}

impl ChatComponent {
    fn connect(&mut self, ctx: egui::Context) {
        let wakeup = move || ctx.request_repaint();
        match ewebsock::connect_with_wakeup(&self.url, wakeup) {
            Ok((ws_sender, ws_receiver)) => {
                self.sender = Some(ws_sender);
                self.receiver = Some(ws_receiver);
                self.error.clear();
            }
            Err(e) => {
                log::error!("Failed to connect to {:?}: {}", &self.url, e);
                self.error = e;
            }
        }
    }
}
