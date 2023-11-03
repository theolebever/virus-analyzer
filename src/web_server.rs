// src/web_server.rs

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

pub async fn run_server() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handler)) });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handler(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Handle requests here
    Ok(Response::new(Body::from("Hello, World!")))
}
