use std::process;
use stock_portfolio::application::Application;

fn main() {
    let mut app = Application::new();
    if !app.run() {
        process::exit(1);
    }
}
