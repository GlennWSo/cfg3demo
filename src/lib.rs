pub mod product;
pub mod render;

use log::{error, info};

// Entry for wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    use product::Product;

    console_log::init_with_level(log::Level::Debug).unwrap();

    info!("Logging works!");

    let client = reqwest::Client::new();
    let res = client
        .get("https://www.example.org/some_api/product/chair")
        .send()
        .await;

    let fallback = Product::assy_dummy;
    let product = match res {
        Ok(v) => {
            info!("got resonse: {:#?}", v);
            error! {"Failed to parse api response"}; // TODO parse
            info!("falling back to dummy product");
            fallback()
        }
        Err(e) => {
            error!("failed to get part from database with err{}", e);
            info!("falling back to dummy product");
            fallback()
        }
    };

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    render::render(product.await).await;
    Ok(())
}
