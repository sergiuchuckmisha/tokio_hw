#![feature(extern_prelude)]
#![feature(proc_macro, generators)]

extern crate tokio;
//extern crate futures;
extern crate tokio_core;
#[macro_use] extern crate futures_await as futures;
#[macro_use] extern crate failure;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

mod examples;


fn main() {
    println!("Hello, world!");

    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        let connection = io::write_all(socket, "hello world\n")
            .then(|res| {
                println!("wrote message; success={:?}", res.is_ok());
                Ok(())
            });

        // Spawn a new task that processes the socket:
        tokio::spawn(connection);

        Ok(())
    })
        .map_err(|err| {
            // All tasks must have an `Error` type of `()`. This forces error
            // handling and helps avoid silencing failures.
            //
            // In our example, we are only going to log the error to STDOUT.
            println!("accept error = {:?}", err);
        });

    println!("server running on localhost:6142");
    tokio::run(server);
}
