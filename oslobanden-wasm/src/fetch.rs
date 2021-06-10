use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;

pub async fn fetch_json<T>(path: &str) -> Result<T, JsValue> where T: for<'de> serde::Deserialize<'de> {
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);

    let request = web_sys::Request::new_with_str_and_init(&path, &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<web_sys::Response>());
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    Ok(json.into_serde().unwrap())
}