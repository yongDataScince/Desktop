pub mod enviroment;
pub mod errors;

use tokio::net::TcpListener;
use enviroment::*;

#[tokio::main]
async fn main() {
    let env = Enviroment::get_env().unwrap();

    let listener = TcpListener::bind(format!("{}:{}", env.host, env.port)).await.unwrap();
}
