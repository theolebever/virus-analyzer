// src/main.rs

mod web_server;

use web_server::*;

mod handle_docker;
use handle_docker::*;

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Please provide a path to the malware folder with -p or --path");
    }

    // check for arguments
    match args[1].as_str() {
        "-h" | "--help" => {
            println!("Usage: ./{} -p <path_to_malware_file>", args[0]);
        },
        "-p" | "--path" => {
            let malware_path = args[2].clone();

            let _server_thread = std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(run_server());
            });
        
            // List all the containers to download here 
            let container_list = vec![
                "malice/windows-defender:latest".to_string(),
                "malice/escan:latest".to_string(),
                "malice/bitdefender:latest".to_string(),
                // "malice/avg:latest".to_string(),
                "malice/avast:latest".to_string(),
                "malice/comodo:latest".to_string(),
                "malice/fprot:latest".to_string(),
                "malice/fsecure:latest".to_string(),
                // "malice/kaspersky:latest".to_string(),
                "malice/mcafee:latest".to_string(),
                "malice/sophos:latest".to_string(),
                "malice/clamav:latest".to_string(),
                // "malice/avira:latest".to_string(),
        
            ];
        
            // Download and launch all the containers in parallel
            launch_analysis(container_list, malware_path).await;
        },
        _ => {
            eprintln!("Unknown argument");
        }
        
    }



    

    // Optionally, wait for the server thread to finish (this will block the main thread)
    // server_thread.join().unwrap();
}
