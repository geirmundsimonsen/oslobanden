mod config;
mod server;

use crate::config::get_config;
use crate::server::run_server;

#[tokio::main]
async fn main() {
    let config = get_config();

    let server_handle = run_server(config.clone());
    
    server_handle.await.expect("Server thread has crashed.");
}