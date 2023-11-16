// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("running wasm32 target as binary")
}
