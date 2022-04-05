use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;


//define a server struct(object)

pub struct Server<'a> {
     socket_addr: &'a str,  //reference type
}

//implement methods for struct above

impl<'a> Server<'a> {

     //create new server
     pub fn new(socket_addr: &'a str) -> Self {
          Server { socket_addr }
     }

     //run server
     pub fn run(&self) {
          let connection = TcpListener::bind(self.socket_addr).unwrap(); //bind to address and listen connection
          println!("Running on {}",self.socket_addr);

          for stream in connection.incoming(){ //use a for loop to read incoming connection into a vector
               let mut stream = stream.unwrap();
               println!("connection established");

               let mut read_buffer = [0; 200];
               stream.read(&mut read_buffer).unwrap();
               let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
               Router::route(req,&mut stream);
          }
     }
}
