use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use crate::app::components::color_display::ColorDisplay;


#[component]
pub fn ColorSelector() -> impl IntoView {
    let (colors, set_colors) = create_signal(vec!["#FFFFFF", "#D9D9D9", "#000000", "#FF94F1", "#FF8080", "#B8FF97"]);

    view!{
        <div class="color-selector">
            <For each=move || colors.get() 
                key = |color| color.clone()
                children = |color| {
                view! {
                    <ColorDisplay color=color.to_string() on:click=move|_| log_1(&JsValue::from_str("HI")) />
                }
            }/>
        </div>
    }
}
