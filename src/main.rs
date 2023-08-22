#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use mock_db::SystemData;
use std::sync::Arc;
use tokio::runtime::Builder;

// Example of use:
// assert_eq!(**STORE.load(), "hello");
// STORE.swap(Arc::new("world"));
// assert_eq!(**STORE.load(), "hello");

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let rt = Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .expect("Unable to create Runtime");

    let _enter = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            // loop {}
            loop {
                std::future::pending::<()>().await;
            }
        })
    });

    let native_options = eframe::NativeOptions {
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(crawl::TemplateApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions {
        follow_system_theme: false,
        ..Default::default()
    };
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(crawl::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
