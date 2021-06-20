use std::convert::Infallible;
use std::env::consts::OS;
use std::fs::{read, read_to_string};
use hyper::{Body, Method, Request, Response, StatusCode};

pub async fn handle(
    req: Request<Body>
) -> Result<Response<Body>, Infallible> {
    if let Some(response) = main_entry_handler(&req) {
        return Ok(response)
    }

    if req.uri().path().starts_with("/img") {
        return Ok(img_handler(&req))
    }

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/test") => {
            Ok(Response::new(Body::from("OK")))
        },
        _ => Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("404 not found")).unwrap())
    }
}

fn main_entry_handler(req: &Request<Body>) -> Option<Response<Body>> {
    let index_path = if OS == "linux" {
        "index.html"
    } else {
        "../oslobanden-site/index.html"
    };

    let css_path = if OS == "linux" {
        "index.css"
    } else {
        "../oslobanden-site/index.css"
    };

    let package_path = if OS == "linux" {
        "package.js"
    } else {
        "../oslobanden-site/pkg/package.js"
    };

    let package_wasm_path = if OS == "linux" {
        "package_bg.wasm"
    } else {
        "../oslobanden-site/pkg/package_bg.wasm"
    };
    
    match (req.method(), req.uri().path()) {

        (&Method::GET, "/") => {
            let path = std::env::current_dir().unwrap();
            println!("The current directory is {}", path.display());
            let html = read_to_string(index_path).expect("Error reading html file");
            Some(Response::new(Body::from(html)))
        },
        (&Method::GET, "/index.css") => {
            let path = std::env::current_dir().unwrap();
            println!("The current directory is {}", path.display());
            let css = read_to_string(css_path).expect("Error reading html file");
            Some(Response::new(Body::from(css))) 
        },
        (&Method::GET, "/pkg/package.js") => {
            let js = read_to_string(package_path).expect("Error reading js file");
            Some(Response::builder().header("Content-Type", "application/javascript").body(Body::from(js)).unwrap())
        },
        (&Method::GET, "/pkg/package_bg.wasm") => {
            let wasm = read(package_wasm_path).expect("Error reading wasm file");
            Some(Response::builder().header("Content-Type", "application/wasm").body(Body::from(wasm)).unwrap())
        },
        _ => None
    }
}

fn img_handler(req: &Request<Body>) -> Response<Body> {
    let img_path = if OS == "linux" {
        "./"
    } else {
        "../oslobanden-site"
    };

    let jpg = read(img_path.to_owned() + req.uri().path()).expect("Error reading jpg file");
    Response::builder().header("Content-Type", "image/jpg").body(Body::fr1om(jpg)).unwrap()
}