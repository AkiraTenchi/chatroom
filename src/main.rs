use std::convert::Infallible;

use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode};

#[tokio::main]
async fn main() {
    // binding to 127.0.0.1:3000
    let addr = ([127, 0, 0, 1], 3000).into();

    // create Service to enable connection
    let make_svc = make_service_fn(|_conn| async {
        // Converts function into 'Service'
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello,World".into()))
}

// async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {}
