/**
idea is to use fn
futures::future::ok
to produce future
*/

//extern crate tokio_core;
//extern crate futures;
use super::futures::*;
use super::tokio_core::reactor::Core;
use super::tokio::prelude::future::FutureResult;

fn make_future()-> FutureResult<u32, u32> {
    futures::future::ok::<u32, u32>(1)
}

#[cfg(test)]
mod tests {
    use super::*;


    # [test]
    fn make_future_test() {
        let mut core = Core::new().unwrap();
        println!("{:?}", core.run(make_future()).unwrap());
    }
}