use std::convert::Infallible;
use std::fs::{read, read_to_string};
use hyper::{Body, Method, Request, Response, StatusCode};

pub async fn handle(
    req: Request<Body>
) -> Result<Response<Body>, Infallible> {
    if let Some(response) = main_entry_handler(&req) {
        return Ok(response)
    }

    Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("404 not found")).unwrap())
}

fn main_entry_handler(req: &Request<Body>) -> Option<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let path = std::env::current_dir().unwrap();
            println!("The current directory is {}", path.display());
            let html = read_to_string("index.html").expect("Error reading html file");
            Some(Response::new(Body::from(html))) 
        },
        (&Method::GET, "/frontend-js") => {
            let js = read_to_string("frontend.js").expect("Error reading js file");
            Some(Response::builder().header("Content-Type", "application/javascript").body(Body::from(js)).unwrap())
        },
        (&Method::GET, "/frontend_bg.wasm") => {
            let wasm = read("frontend_bg.wasm").expect("Error reading wasm file");
            Some(Response::builder().header("Content-Type", "application/wasm").body(Body::from(wasm)).unwrap())
        },
        _ => None
    }
}