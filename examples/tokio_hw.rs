extern crate tokio;
extern crate futures;
extern crate tokio_core;

use futures::Stream;
use futures::future::*;
use futures::Poll;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

struct F;

impl Future for F {
    type Item = i32;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(futures::Async::Ready(1))
    }
}

impl F {
    fn new() -> F{
    F{}
    }
}

fn main() {
    let mut core = Core::new().unwrap();

    let future_of_1 = ok::<u32, u32>(1);
    let future_of_2 = ok::<u32, u32>(2);

    core.run(future_of_1).unwrap();

    println!("{:?}", core.run(future_of_2).unwrap());
    println!("{:?}", core.run(F).unwrap());
    println!("{:?}", core.run(F));
}