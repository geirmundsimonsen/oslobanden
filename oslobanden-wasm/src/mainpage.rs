use wasm_bindgen::prelude::*;
use crate::utils::get_body;
use crate::dom::Tag::{Div, H1};
use crate::dom::Margin;

pub async fn mainpage() -> Result<(), JsValue> {
    let page = Div.node().style(Margin::Px(32))
        .child(H1.node().content("Oslobandens (bejublede) bingojukebox"));

    page.realize(&get_body());

    Ok(())
}
