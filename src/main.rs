mod application;
mod portfolio;
mod sputil;
mod yfinance;

use application::application::Application;

fn main() {
    let mut app = Application::new();
    match app.run() {
        true => 0,
        false => 1
    };
}
