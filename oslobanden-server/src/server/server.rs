use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::Server;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

use crate::config::Config;
use crate::server::handler::handle;

pub fn run_server(config: Config) -> tokio::task::JoinHandle<()> {
    let server_port = config.server_port;

    tokio::spawn(async move {        
        let make_service = make_service_fn(move |_conn: &AddrStream| {
            let service = service_fn(move |req| {
                handle(req)
            });

            async move { Ok::<_, Infallible>(service) }
        });

        let server = Server::bind(
            &SocketAddr::from(([127, 0, 0, 1], server_port))
        ).serve(make_service);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    })
}