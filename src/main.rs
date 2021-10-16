use std::convert::Infallible;

use futures::TryStreamExt as _;
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode};

enum EchoOpt {
    None,
    Uppercase,
    Reverse,
}

#[tokio::main]
async fn main() {
    // binding to 127.0.0.1:3000
    let addr = ([127, 0, 0, 1], 3000).into();

    // create Service to enable connection
    let make_svc = make_service_fn(|_conn| async {
        // Converts function into 'Service'
        Ok::<_, Infallible>(service_fn(req_manager))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn req_manager(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("index".into())),
        (&Method::GET, "/helloworld") => hello_world().await,
        (&Method::POST, "/echo") => echo(req, EchoOpt::None).await,
        (&Method::POST, "/echo/uppercase") => echo(req, EchoOpt::Uppercase).await,
        (&Method::POST, "/echo/reverse") => echo(req, EchoOpt::Reverse).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not found".into())
            .unwrap()),
    }
}

async fn hello_world() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new("Hello,World".into()))
}

async fn echo(req: Request<Body>, modi: EchoOpt) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    match modi {
        EchoOpt::None => *response.body_mut() = req.into_body(),
        EchoOpt::Uppercase => {
            let mapping = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            *response.body_mut() = Body::wrap_stream(mapping);
        }
        EchoOpt::Reverse => {
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let reversed = full_body.iter().rev().cloned().collect::<Vec<u8>>();
            *response.body_mut() = reversed.into();
        }
    }

    Ok(response)
}
