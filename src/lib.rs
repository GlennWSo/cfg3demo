mod gmain;

use log::{error, info};

// Entry for wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();

    info!("Logging works!");

    let client = reqwest::Client::new();
    let res = client.get("https://www.rust-lang.org").send().await;

    match res {
        Ok(v) => info!("got resonse: {:#?}", v),
        Err(e) => error!("{}", e),
    }

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    gmain::run().await;
    Ok(())
}
