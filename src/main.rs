use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
  let server = Server::new("localhost:8000".to_string());
  server.run();
}