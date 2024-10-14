mod components;
mod helpers;
mod stroke_rendering;
mod enums;

use components::canvas::Canvas;
use components::bottombar::BottomBar;
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
            <BottomBar/>
       </main>
    }
}
