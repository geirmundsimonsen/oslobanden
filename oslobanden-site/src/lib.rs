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

fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
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

fn view(_model: &Model) -> Node<Msg> {
    div![
        C!["main-container"],
        div![
            C!["main-image-gradient"],
            img![
                C!["main-image"],
                attrs!{At::Src => "img/oslobanden-main.jpg"},
                attrs!{At::Alt => "Fotograf: Kristian Leraand"}
            ],
        ],
        div![
            C!["header"],
            "OSLOBANDEN",
        ],
        div![
            C!["secondary-header"],
            "OG DENS BEJUBLEDE BINGOJUKEBOKS",
        ],
        div![
            C!["paragraph-text"],
            "Slippes løs på Oslos gater fra 6. til 11. juli. Nøyaktig tid og sted følger. Tillatelser innhentes.",
        ],
        div![
            C!["paragraph-text"],
            "Oslobanden er et knippe spille-avhengige musikere som på ingen måte trenger å ty til gatene for å tjene til livets opphold - nei, det hersker et genuint ønske om å overøse et forbipasserende publikum med musikalsk glede.",

        ],
        div![
            C!["paragraph-text"],
            "Oslobanden spiller låter fra alle epoker, med relativt uanstrengt bravur, på alle tenkelige instrumenter, og i absolutt all slags vær. Oslobanden har også utviklet et unikt, patentert system der publikum selv kan bestemme hva slags låter som spilles.",
        ],
        div![
            C!["musikere-header"],
            "Musikere:"
        ],
        div![
            C!["musiker-flex"],
            div![
                C!["musiker"],
                "Hugo Herrman",
            ],
            div![
                C!["instrument"],
                "trommer, sax, vokal",
            ],
        ],
        div![
            C!["musiker-flex"],
            div![
                C!["musiker"],
                "Jonathan S. Bjørnseth",
            ],
            div![
                C!["instrument"],
                "bass, vokal",
            ],
        ],
        div![
            C!["musiker-flex"],
            div![
                C!["musiker"],
                "Anja G. Bajer",
            ],
            div![
                C!["instrument"],
                "fiolin",
            ],
        ],
        div![
            C!["musiker-flex"],
            div![
                C!["musiker"],
                "Jacob B. Hvattum",
            ],
            div![
                C!["instrument"],
                "cello, vokal",
            ],
        ],
        div![
            C!["musiker-flex"],
            div![
                C!["musiker"],
                "Hauk J. Røsten",
            ],
            div![
                C!["instrument"],
                "gitarer, vokal",
            ],
        ],
        div![
            C!["musiker-flex"],
            div![
                C!["musiker"],
                "Geirmund T. Simonsen",
            ],
            div![
                C!["instrument"],
                "tangenter",
            ],
        ],
        div![
            C!["paragraph-text"],
            "Kontakt oss: kontakt@oslobanden.no.",
        ],
        div![
            C!["paragraph-text-mini"],
            "Fotograf: Kristian Leraand",
        ],
    ]

    /*div![
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
    ]*/

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