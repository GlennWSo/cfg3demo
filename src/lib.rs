mod render;

use log::{error, info};

// Entry for wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    use render::Component;

    console_log::init_with_level(log::Level::Debug).unwrap();

    info!("Logging works!");

    let client = reqwest::Client::new();
    let res = client
        .get("https://www.example.org/some_api/product/chair")
        .send()
        .await;

    let part = match res {
        Ok(v) => {
            info!("got resonse: {:#?}", v);
            error! {"Failed to parse api response"}; // TODO parse
            info!("falling back to dummy product");
            Component::placeholder()
        }
        Err(e) => {
            error!("failed to get part from database with err{}", e);
            info!("falling back to dummy product");
            Component::placeholder()
        }
    };

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    render::render(part).await;
    Ok(())
}
