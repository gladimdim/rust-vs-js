#[macro_use]
extern crate seed;
use seed::prelude::*;

use seed::{spawn_local, Method, Request};

use futures::Future;
mod model;

#[derive(Clone)]
struct Model {
    response: Option<model::Response>,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self { response: None }
    }
}
fn get_data(state: seed::App<Msg, Model>) -> impl Future<Item = (), Error = JsValue> {
    let url = "/data.json";

    Request::new(url)
        .method(Method::Get)
        .fetch_json()
        .map(move |json| state.update(Msg::SetResponse(json)))
}
// Update

#[derive(Clone)]
enum Msg {
    SetResponse(model::Response),
    FetchData(seed::App<Msg, Model>),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::FetchData(state) => {
            spawn_local(get_data(state));
            model
        }
        Msg::SetResponse(response) => Model {
            response: Some(response),
            ..model
        },
    }
}

fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style! {
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

    div![
        outer_style,
        div![
            (match model.response {
                None => vec![button![
                    simple_ev("click", Msg::FetchData(state)),
                    "Fetch Data"
                ]],
                Some(response) => {
                    let result: Vec<El<Msg>> = response
                        .items
                        .into_iter()
                        .map(|node: model::ModelNode| div![node.long_description])
                        .collect();
                    result
                }
            }),
        ],
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None, None);
}
