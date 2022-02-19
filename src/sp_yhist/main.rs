pub mod arguments;
pub mod application;

use std::process;
use crate::application::Application;

fn main() {
    let mut app = Application::new();
    if let Err(err) = app.run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
