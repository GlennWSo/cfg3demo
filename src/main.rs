// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use cfg3demo::{product::Product, render};
    let product = Product::assy_dummy();
    render::render(product.await).await;
}

#[cfg(target_arch = "wasm32")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), wasm_bindgen::JsValue> {
    use cfg3demo::start;
    start().await
}
