fn main() {
    let server = Server::new("localhost:8000");
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    
}