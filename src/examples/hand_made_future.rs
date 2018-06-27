/**
idea is to implement Future manually for some struct
*/
use super::*;
use super::tokio_core::reactor::Core;

const N: i32 = 1;

struct F {}

impl Future for F {
    type Item = i32;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            Ok(futures::Async::Ready(N))
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    # [test]
    fn make_future_test() {
        let mut core = Core::new().unwrap();
        println!("{:?}", core.run(F{}));
        assert_eq!(Ok(N), core.run(F{}));
    }

    # [test]
    fn make_future_wait_test() {
        println!("{:?}", F{}.wait());
        assert_eq!(Ok(N), F{}.wait());
    }
}