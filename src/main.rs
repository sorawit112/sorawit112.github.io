use leptos::mount::mount_to_body;
use my_pages::components::app::*;

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
