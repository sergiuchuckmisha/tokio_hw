/**
idea is to combine future with duration
*/
use super::*;
use super::tokio_core::reactor::Core;
use super::tokio::prelude::*;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct F_with_notification {
    state: i32,
}

impl Future for F_with_notification {
    type Item = i32;
    type Error = ();

    /**idea is to return NotReady few times and only then return Ready*/
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if self.state > 0 {
            self.state -= 1;
            //https://users.rust-lang.org/t/waiting-short-periods-of-time-with-tokio-core-reactor-timeout/15378
            futures::task::current().notify();
            return Ok(futures::Async::NotReady);
        }
        Ok(futures::Async::Ready(self.state))
    }
}

impl F_with_notification {
    fn new(initial_state: i32) -> F_with_notification {
        F_with_notification { state: initial_state }
    }
}

#[derive(Debug)]
struct F_without_notification {
    state: i32,
}

impl Future for F_without_notification {
    type Item = i32;
    type Error = ();

    /**idea is to return NotReady few times and only then return Ready*/
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if self.state > 0 {
            self.state -= 1;
            //https://users.rust-lang.org/t/waiting-short-periods-of-time-with-tokio-core-reactor-timeout/15378
//            futures::task::current().notify();
            return Ok(futures::Async::NotReady);
        }
        Ok(futures::Async::Ready(self.state))
    }
}

impl F_without_notification {
    fn new(initial_state: i32) -> F_without_notification {
        F_without_notification { state: initial_state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_future_without_notification_test() {
        let mut core = Core::new().unwrap();
        println!("{:?}", core.run(F_without_notification::new(0)));
        println!("{:?}", F_without_notification::new(0).wait());
        assert_eq!(Ok(0), core.run(F_without_notification::new(0)));
    }

    #[test]
    fn make_future_with_notification_test() {
        let mut core = Core::new().unwrap();
        println!("{:?}", F_with_notification::new(1).wait());
        println!("{:?}", core.run(F_with_notification::new(1)));
        println!("{:?}", F_with_notification::new(1).wait());
        assert_eq!(Ok(0), core.run(F_with_notification::new(1)));
    }

    #[test]
    fn make_future_deadline_test() {
        let mut core = Core::new().unwrap();

        let future_with_notification_with_deadline = F_with_notification::new(2).deadline(Instant::now() + Duration::from_millis(100));
        println!("{:?}", core.run(future_with_notification_with_deadline));

        let future_without_notification_with_deadline = F_without_notification::new(1).deadline(Instant::now() + Duration::from_millis(100));
        println!("{:?}", core.run(future_without_notification_with_deadline));

        let future_without_notification_with_deadline = F_without_notification::new(1).deadline(Instant::now() + Duration::from_millis(100));
        println!("{:?}", future_without_notification_with_deadline.wait());

        let future_without_notification_with_deadline = F_without_notification::new(2).deadline(Instant::now() + Duration::from_millis(100));
        println!("{:?}", core.run(future_without_notification_with_deadline));

        let future_without_notification_with_deadline = F_without_notification::new(2).deadline(Instant::now() + Duration::from_millis(100));
        println!("{:?}", future_without_notification_with_deadline.wait());
    }
}