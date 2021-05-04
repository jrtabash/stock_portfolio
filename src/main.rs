use std::process;
use stock_portfolio::application::Application;

fn main() {
    let mut app = Application::new();
    if let Err(err) = app.run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
