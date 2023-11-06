// src/main.rs

mod web_server;

mod handle_docker;

use iced::Application;
use iced::Settings;

mod gui;
use gui::*;

use log::{info, warn};

#[tokio::main]
async fn main() {
    // Launch the GUI thread
    let error = VirusAnalyzer::run(Settings::default());

    match error {
        Ok(_) => info!("GUI thread finished"),
        Err(_) => warn!("GUI thread panicked"),
    }

    // Optionally, wait for the server thread to finish (this will block the main thread)
    // server_thread.join().unwrap();
}
