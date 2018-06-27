
/**
idea is just read file and print it

base methods:
 - let file = path::Path::new("./README.md");
 - let file = (fs::file::File::open(file));
 - let (file, data) = await!(tokio::io::read_to_end(file, vec![]));
 - println!("\n{:#}", String::from_utf8(data));

 I want to construct chain of futures with these functions
*/

extern crate tokio;
extern crate futures;
extern crate tokio_core;

use futures::prelude::*;
use futures::future;
//use failure::Error;
use std::path;
use tokio::{fs, io, io::AsyncRead};
use tokio::runtime::current_thread::Runtime;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
//    let mut rt = Runtime::new().unwrap();
//    let file_path = "README.md";
//    rt.spawn(

//    let task = tokio::fs::File::open("README.md")
//        .map(|file| {io::read_to_end(file, vec![])})
//        .then(move |res| { println!("{:?}", res); Ok(()) })
//        .map_err(|e: ()| {println!("{:?}", e);})
//    ;


//    core.run(
//        future::lazy(move || {
//            tokio::fs::File::open("README.md")
//                .and_then(|file| {
//                    io::read_to_end(file, vec![])
//                })
//                .then(move |res| {
//                    println!("{:?}", res);
//                    Ok(())
//                })
//        })
//    );
//    rt.run();
    //rt.shutdown_on_idle();

    let (channel, data) = core.run(tokio::io::read_to_end(tokio::fs::File::open("README.md"), Vec::new())).unwrap();

    core.run(task).unwrap()
}

