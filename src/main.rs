use leptos::prelude::*;
mod app;
mod components;

use app::App;
use dotenv::dotenv;

fn main() {
    console_error_panic_hook::set_once();
    dotenv().ok();
    mount_to_body(App);
}
