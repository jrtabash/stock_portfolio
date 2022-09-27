pub mod application;
pub mod arguments;

use crate::application::Application;
use sp_lib::util::common_app::app_main;

fn main() {
    app_main::<Application>()
}
