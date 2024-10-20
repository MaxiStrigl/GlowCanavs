mod components;
mod enums;
mod helpers;
mod stroke_rendering;

use components::bottombar::BottomBar;
use components::canvas::Canvas;
use components::popover::Popover;
use enums::drawing_mode::Mode;
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
    let (current_mode, set_current_mode) = create_signal(Mode::Pen);

    provide_context(current_mode);
    provide_context(set_current_mode);

    view! {
        <main class="container">
            <Canvas/>
            <div class="ui-container">
                <Popover/>
                <BottomBar/>
            </div>
        </main>
    }
}
