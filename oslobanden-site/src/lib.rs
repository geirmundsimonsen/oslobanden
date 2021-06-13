#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { /*counter: 0, test_response: "".to_string()*/ }
}

// keeping model, msg, update - for reference.. keep building oslobanden.no
struct Model {
    //counter: i32,
    //test_response: String,
}

enum Msg {
    //Increment,
    //TestRequest,
    //TestResponse(String)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        /*Msg::Increment => model.counter += 4,
        Msg::TestRequest => {
            orders.skip().perform_cmd(async {
                let response = fetch("test").await.expect("HTTP request failed");
                let response = response.check_status().expect("status failed").text().await.expect("deserialization failed");
                Msg::TestResponse(response)
            });
        },
        Msg::TestResponse(response) => {
            model.test_response = response;
        }*/
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        h1![
            C!["header"],
            "Oslobandens",
            br![],
            "~ bejublede ~",
            br![],
            "bingojukeboks",
            style!{St::FontFamily => "Luckiest Guy, cursive"},
            style!{St::FontSize => variable_size_simple(40)},
            style!{St::LineHeight => variable_size_simple(40)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
            style!{St::MarginTop => variable_size_simple(20)},
        ],
        p![
            "Medvirkende:",
            style!{St::MarginTop => variable_size_simple(60)},
            style!{St::FontFamily => "Londrina Solid, cursive"},
            style!{St::FontSize => variable_size_simple(30)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
        ],
        p![
            "Jonathan",
            style!{St::FontFamily => "Londrina Solid, cursive"},
            style!{St::FontSize => variable_size_simple(30)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
        ],
        p![
            "Jacob",
            style!{St::FontFamily => "Londrina Solid, cursive"},
            style!{St::FontSize => variable_size_simple(30)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
        ],
        p![
            "Hugo",
            style!{St::FontFamily => "Londrina Solid, cursive"},
            style!{St::FontSize => variable_size_simple(30)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
        ],
        p![
            "Hauk",
            style!{St::FontFamily => "Londrina Solid, cursive"},
            style!{St::FontSize => variable_size_simple(30)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
        ],
        p![
            "Geirmund",
            style!{St::FontFamily => "Londrina Solid, cursive"},
            style!{St::FontSize => variable_size_simple(30)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
        ],
        p![
            "Kontakt:",
            style!{St::MarginTop => variable_size_simple(60)},
            style!{St::FontFamily => "Londrina Solid, cursive"},
            style!{St::FontSize => variable_size_simple(30)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
        ],
        p![
            "kontakt@oslobanden.no",
            style!{St::FontFamily => "Londrina Solid, cursive"},
            style!{St::FontSize => variable_size_simple(30)},
            style!{St::TextAlign => "center"},
            style!{St::Color => "#fff3b5"},
        ],
    ]



    /*div![
        "This is a counter: ",
        C!["counter"],
        button![
            style!{St::Padding => px(20) + " " + &px(15)},
            model.counter,
            ev(Ev::Click, |_| Msg::Increment),
        ],
        button![
            style!{St::Padding => px(20) + " " + &px(15)},
            "FETCH",
            ev(Ev::Click, |_| Msg::TestRequest),
        ],
        p![
            { model.test_response.clone() }
        ]
    ]*/

}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

fn variable_size_rem(sz_min: f32, sz_max: f32, w_min: f32, w_max: f32) -> String {
    format!("clamp({sz_min}rem, calc({sz_min}rem + ({sz_max} - {sz_min}) * ((100vw - {w_min}rem) / ({w_max} - {w_min}))), {sz_max}rem)", sz_min = sz_min, sz_max = sz_max, w_min = w_min, w_max = w_max)
}

const MIN_WIDTH_PX: u32 = 360;

fn variable_size_simple(min_size_px: u32) -> String {
    let scale_factor = 3.0;
    let min_size_rem = min_size_px as f32 / 16.0;
    let min_width = MIN_WIDTH_PX as f32 / 16.0;
    let max_width = min_width * scale_factor; 
    variable_size_rem(min_size_rem, min_size_rem * scale_factor, min_width, max_width)
}