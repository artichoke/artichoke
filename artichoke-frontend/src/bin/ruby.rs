use artichoke_frontend::ruby::{self, Error};

fn main() {
    match ruby::entrypoint() {
        Ok(_) => {}
        Err(Error::Artichoke(err)) => eprintln!("{}", err),
        Err(Error::Fail(err)) => eprintln!("{}", err),
    }
}
