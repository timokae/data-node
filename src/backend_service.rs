use crate::logger;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("lol"))),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub async fn start_server(port: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr_str = format!("127.0.0.1:{}", port);
    let addr = addr_str.parse()?;

    logger::log(&format!("Server started on {}", addr));

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });
    let server = Server::bind(&addr).serve(service);

    let _ = server.await;

    Ok(())
}
