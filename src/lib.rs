#[macro_use]
extern crate lazy_static;

mod tohebrew;

use seed::{prelude::*, *};
// use crate::tohebrew::transl;
// use seed::util::get_value;

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// `Model` describes our app state.
#[derive(Default)]
pub struct Model {
    content : String,
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    TextChanged(String),
    Reset,
    ToClip,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TextChanged(text) => {
            
            model.content = text;
        },
        Msg::Reset => {
            model.content = String::from("");
            resetFocus();
        },
        Msg::ToClip => {
            copyTextToClipboard(&tohebrew::transl(&model.content));
            resetFocus();
        }, 
    }
}

#[wasm_bindgen]
extern {
    fn copyTextToClipboard(s: &str);
}

#[wasm_bindgen]
extern {
    fn resetFocus();
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        id!("coeur"),
        "Type on latin keyboard :",
        button![
            "Copy to Clipboard",
            ev(Ev::Click, |_| Msg::ToClip),
        ],
        "    ",
        button![
            C!("reset"),
            "Reset",
            ev(Ev::Click, |_| Msg::Reset),
        ],
        br![],
        
        input![
            id!["entrée"],
            attrs!{
                // At::AutoFocus => String::from("true"), DOES NOT WORK
                At::Type => "text",
                At::Value => model.content;
                At::Placeholder => "Start typing ..."
            },
            style!{
                St::Width => px(700),
                St::MarginTop => px(10),
            },
            input_ev(Ev::Input, Msg::TextChanged)
        ],
        p![
            "Get hebrew text :"
        ],
        p![
            id!("sortie"),
            &tohebrew::transl(&model.content),
        ],
    ]
} 
// https://github.com/seed-rs/seed/blob/131c53194f7362f056692859a12c21ab20747471/examples/counter_advanced/src/lib.rs
// https://github.com/seed-rs/seed/blob/131c53194f7362f056692859a12c21ab20747471/examples/websocket/src/client.rs

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
    resetFocus(); // not guaranteed to work (document not sure to be loaded at that time), but whatever :-)
}

