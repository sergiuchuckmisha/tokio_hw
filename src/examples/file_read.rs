

use super::*;
use super::tokio_core::reactor::Core;
use super::futures::prelude::*;
use super::failure::Error;
use std::path;

#[async]
fn read_file () -> Result<(), Error> {
    let file = path::Path::new("./README.md");

    let file = await!(tokio::fs::file::File::open(file))?;
    println!("file_fs: {:?}", &file);

    // TODO: so this super weird syntax over here where it owns, then returns the
    // owned value is because we can't do borrows with async/await.
    // This is isn't great - but not having this would be *so much worse*. So
    // yeah, we just have to accept the reality we live in. I like it.
    let (file, data) = await!(tokio::io::read_to_end(file, vec![]))?;
    println!("\n{:#}", String::from_utf8(data)?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn read_file_async_test_() {
        let mut core = Core::new().unwrap();
        core.run(read_file());
    }

    # [test]
    fn read_file_async_test() {
        let mut eloop = tokio::runtime::Runtime::new().unwrap();
        println!("here1");
        eloop.spawn(read_file().map_err(|err| println!("{:?}", err)));
        eloop.shutdown_on_idle().wait().unwrap();
//        read_file().wait();
        println!("here2");
    }

        // todo
        /// looks like bug
        /// https://github.com/tokio-rs/tokio/issues/386#issuecomment-394141537
    # [test]
    fn open_file_test_() {
        let mut core = Core::new().unwrap();
        println!("{:?}", core.run(tokio::fs::File::open("README.md")));
    }

    # [test]
    fn open_file_test() {
        let file_path = "README.md";
        let open_file_result = tokio::fs::File::open(file_path).wait();

        println!("{:?}", open_file_result);
//        assert_eq!(Ok(N), F{}.wait());
    }
}