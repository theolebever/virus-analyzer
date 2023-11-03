use bollard_next::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard_next::image::CreateImageOptions;
use bollard_next::image::ListImagesOptions;
use bollard_next::Docker;
use futures::stream::StreamExt;
use uuid::Uuid;

pub async fn launch_analysis(list_images: Vec<String>, malware_path: String) {
    let docker_result = Docker::connect_with_local_defaults();
    let docker = docker_result.expect("Failed to connect to Docker");

    let mut handles = Vec::new();  // Vector to hold the thread handles

    for image_name in list_images {
        // Clone the values to avoid moving them into the closure
        let malware_path_clone = malware_path.clone();
        let docker_clone = docker.clone();
        let handle = std::thread::spawn(move || {  // Save the thread handle
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                download_container(image_name.clone(), docker_clone.clone()).await;
                run_container(image_name, malware_path_clone, docker_clone).await;
            });
        });
        handles.push(handle);  // Add the handle to the vector
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}


pub async fn download_container(image_name: String, docker : Docker){

    let images_result = docker.list_images(None::<ListImagesOptions<String>>).await;
    let images = images_result.expect("Failed to list images");

    if !images
        .iter()
        .any(|image| image.repo_tags.contains(&image_name))
    {
        println!("[+] {} image not already pulled, pulling...", image_name);
        let options = CreateImageOptions {
            from_image: image_name.clone(),
            ..Default::default()
        };

        docker
            .create_image(Some(options), None, None)
            .for_each(|result| match result {
                Ok(_) => {
                    //Pass
                    futures::future::ready(())
                }
                Err(e) => {
                    eprintln!("Failed to pull image: {}", e);
                    futures::future::ready(())

                }
            })
            .await;
        println!("[+] {} image pulled", image_name);
    } else {
        println!("[+] {} image already downloaded", image_name);
    }
}

pub async fn run_container(image_name: String, malware_path: String, docker: Docker) {
    // Define host and container directories
    let container_dir = "/malware";

    // Get the file name from the malware_path
    let file_name = std::path::Path::new(&malware_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_else(|| panic!("Invalid file name: {}", malware_path));

    // Get the path of the folder containing the malware without the file name
    let folder_path = std::path::Path::new(&malware_path)
        .parent()
        .and_then(|name| name.to_str())
        .unwrap_or_else(|| panic!("Invalid folder path: {}", malware_path));

    // Define the configuration for the container
    let config = Config {
        image: Some(image_name.clone()),
        cmd: Some(vec![file_name.to_string()]),
        host_config: Some(bollard_next::models::HostConfig {
            binds: Some(vec![
                format!("{}:{}", folder_path, container_dir),
            ]),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Generate a random UUID and convert it to a string
    let random_container_name = Uuid::new_v4().to_string();

    // Create the container
    let container_options = CreateContainerOptions {
        name: random_container_name,
        ..Default::default()
    };

    let container_result = docker
        .create_container(Some(container_options), config)
        .await;
    let container = container_result.expect("Failed to create container");

    // Start the container
    let start_result = docker
        .start_container(&container.id, None::<StartContainerOptions<String>>)
        .await;
    start_result.expect("Failed to start container");

    println!("[+] {} container started", image_name);

    // Specify the type parameter T as String
    let options = bollard_next::container::LogsOptions::<String> {
        stdout: true,
        stderr: true,
        follow: true,
        ..Default::default()
    };

    // Specify the type parameter T as String
    let logs = docker.logs::<String>(&container.id, Some(options));

    tokio::pin!(logs);  // Pin the stream so that it can be used with the next method

    while let Some(log_result) = logs.next().await {
        match log_result {
            Ok(bollard_next::container::LogOutput::StdOut { message }) |
            Ok(bollard_next::container::LogOutput::StdErr { message }) => {
                let output_str = String::from_utf8_lossy(&message);
                println!("{}", output_str);
            },
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to read logs: {}", e);
                break;
            }
        }
    }

    // Delete the container once it has stopped
    let delete_result = docker
        .remove_container(&container.id, None::<bollard_next::container::RemoveContainerOptions>)
        .await;

    delete_result.expect("Failed to delete container");
    println!("[+] {} container stopped and deleted", image_name);
}

