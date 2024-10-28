use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;

#[component]
pub fn ColorDisplay(color: String) -> impl IntoView {
    view! {
       <div class="color-display" on:click=|_| log_1(&JsValue::from_str("WOW"))>
            <div class="color-circle" style:background-color=color.clone()/>
            <div>{color}</div>
       </div>
    }
}
