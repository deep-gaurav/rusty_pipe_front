#![recursion_limit = "1024"]

#[macro_use]
extern crate cfg_if;

extern crate anyhow;
extern crate async_trait;
extern crate base64;
extern crate http;
extern crate human_format;
extern crate log;
extern crate reqwest;
extern crate rusty_pipe;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate wasm_logger;
extern crate web_sys;
extern crate yew;
extern crate yew_router;

use wasm_bindgen::prelude::*;

mod app;
use app::App;
mod downloader;
mod route_comp;
mod search_result;
mod trending;
mod video;
mod video_player;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

// cfg_if! {
//     // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//     // allocator.
//     if #[cfg(feature = "wee_alloc")] {
//         extern crate wee_alloc;
//         #[global_allocator]
//         static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//     }
// }
// extern crate wee_alloc;
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

    wasm_logger::init(wasm_logger::Config::default());

    // yew::start_app::<App>();
    let el = yew::utils::document()
    .query_selector("#approot")
    .expect("can't get body node for rendering")
    .expect("can't unwrap body node");
    yew::App::<App>::new().mount(el);
    yew::run_loop();
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    let el = yew::utils::document()
    .query_selector("#approot")?.expect("Cant unwrap body");
    yew::App::<App>::new().mount(el);
    yew::run_loop();

    Ok(())
}
