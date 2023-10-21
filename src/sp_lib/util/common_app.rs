use std::process;
use crate::util::error::Error;

pub type RunResult = Result<(), Error>;

pub trait AppTrait {
    fn new() -> Self;
    fn run(&mut self) -> RunResult;
}

pub fn app_main<App: AppTrait>() {
    let mut app = App::new();
    if let Err(err) = app.run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
