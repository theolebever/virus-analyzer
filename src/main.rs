// src/main.rs

mod web_server;
use web_server::run_server;

mod handle_docker;
use handle_docker::run_container;

fn main() {
    let server_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(run_server());
    });

    // ... any other code you want to run in the main thread ...
    let container_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(run_container("nginx".to_string()));
    });

    // Optionally, wait for the server thread to finish (this will block the main thread)
    //server_thread.join().unwrap();

    container_thread.join().unwrap();
}
