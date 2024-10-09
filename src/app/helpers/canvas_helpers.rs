use leptos::{html, NodeRef};
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;


pub fn get_context(canvas_ref: NodeRef<html::Canvas>) -> Option<CanvasRenderingContext2d> {
    if let Some(canvas) = canvas_ref.get() {
        Some(
            canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap(),
        )
    } else {
        None
    }
}
