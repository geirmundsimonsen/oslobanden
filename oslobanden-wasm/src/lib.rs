mod mainpage;
mod dom;
mod fetch;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    crate::utils::set_panic_hook();

    crate::mainpage::mainpage().await?;

    Ok(())
}