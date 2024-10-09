use leptos::web_sys::{MouseEvent, HtmlElement, console::log_1};
use wasm_bindgen::{JsCast, JsValue};
use crate::app::helpers::math_helpers::calculate_local_coordinates;

pub fn handle_mouse_event<F>(ev: MouseEvent, update_coordinates: F)
where
    F: Fn((f64, f64)),
{
    if let Some(div) = ev.target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
        let (x, y) = calculate_local_coordinates(&ev, &div);

        update_coordinates((x, y));
    } else {
        log_1(&JsValue::from_str("Failed to get element"));
    }
}
