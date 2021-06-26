#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model { Model {} }

struct Model {}

enum Msg {}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

fn view(_: &Model) -> Node<Msg> {
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
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}