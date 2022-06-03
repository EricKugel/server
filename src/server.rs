use std::net::TCPListener;
use std::io::Read;

pub struct Server {
  address: String,
}

impl Server {
  pub fn new(address: String) -> Self {
    Self {address}
  }

  pub fn run(self) {
    println!("Listening on {}", self.address);
    let listener = TCPListener::bind(&self.address).unwrap();
    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Request recieved: {}", String::from_utf8(buffer));
            },
            Error(e) => println!("Failed to read from connection: {}", e);
          }
        },
        Error(e) => println!("Failed to establish connection: {}", e);
      }
    }
  }
}