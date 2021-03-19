use stock_portfolio::application::Application;

fn main() {
    let mut app = Application::new();
    match app.run() {
        true => 0,
        false => 1
    };
}
