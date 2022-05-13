pub mod arguments;
pub mod application;

use sp_lib::util::common_app::app_main;
use crate::application::Application;

fn main() {
    app_main::<Application>()
}
