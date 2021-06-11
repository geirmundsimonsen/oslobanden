use wasm_bindgen::prelude::*;
use weblib::utils::get_body;
use weblib::dom::Tag::{Div, H1};
use weblib::dom::{Background, Color, FontFamily, FontSize, MinWidth, TextAlign};
use weblib::dom::set_body_style;

const MIN_WIDTH_PX: u32 = 800;
// http://detectmobilebrowsers.com/ for a script that test for mobile devices - in case

pub async fn mainpage() -> Result<(), JsValue> {
    let page = Div.node().style(MinWidth::Px(MIN_WIDTH_PX))
        .child(
            H1.node()
            .style(FontFamily::Val("Luckiest Guy".to_string(), "cursive".to_string()))
            .style(FontSize::Str(variable_size_simple(96)))
            .style(TextAlign::Center)
            .style(Color::Str("#fff3b5".to_string()))
            .content("Oslobandens<br/>~ bejublede ~<br/>bingojukeboks"));

    set_body_style(vec![Box::new(Background::Str("#b35008".to_string()))]);
    page.realize(&get_body());

    Ok(())
}

fn variable_size_rem(sz_min: f32, sz_max: f32, w_min: f32, w_max: f32) -> String {
    format!("clamp({sz_min}rem, calc({sz_min}rem + ({sz_max} - {sz_min}) * ((100vw - {w_min}rem) / ({w_max} - {w_min}))), {sz_max}rem)", sz_min = sz_min, sz_max = sz_max, w_min = w_min, w_max = w_max)
}

fn variable_size_simple(min_size_px: u32) -> String {
    let scale_factor = 1.5;
    let min_size_rem = min_size_px as f32 / 16.0;
    let min_width = MIN_WIDTH_PX as f32 / 16.0;
    let max_width = min_width * scale_factor; 
    variable_size_rem(min_size_rem, min_size_rem * scale_factor, min_width, max_width)
}