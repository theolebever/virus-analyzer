use bollard_next::image::CreateImageOptions;
use bollard_next::Docker;

use futures::stream::StreamExt;

pub async fn run_container(image_name: String) {
    let docker = Docker::connect_with_local_defaults();

    let options = CreateImageOptions {
        from_image: image_name,
        ..Default::default()
    };

    docker
        .expect("REASON")
        .create_image(Some(options), None, None)
        .for_each(|result| match result {
            Ok(info) => {
                println!("Progress: {:?}", info);
                futures::future::ready(())
            }
            Err(e) => {
                eprintln!("Failed to pull image: {}", e);
                futures::future::ready(())
            }
        })  
        .await;
}
