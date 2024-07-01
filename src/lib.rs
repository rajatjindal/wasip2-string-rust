#[allow(warnings)]
mod bindings;

use crate::bindings::exports::wasi::cli::run::Guest;
use crate::bindings::wasi::cli::environment::{hello, justhello};

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        println!("just hello {}", justhello());
        println!("hello with result {:?}", hello());
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
