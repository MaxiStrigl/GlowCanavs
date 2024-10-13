mod components;
mod helpers;
mod stroke_rendering;

use components::canvas::Canvas;
use components::bottombar::BottomBar;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {

    view! {
        <main class="container">
            <Canvas/>
            <BottomBar/>
        </main>
    }
}
