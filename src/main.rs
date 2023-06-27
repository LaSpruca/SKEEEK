#![feature(unboxed_closures, fn_traits)]

use components::App;
use leptos::{mount_to_body, view};
use log::Level;
use std::panic;

mod components;
mod data;
mod util;

fn main() {
    _ = console_log::init_with_level(Level::Trace);
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    mount_to_body(|cx| view! { cx, <App />})
}
