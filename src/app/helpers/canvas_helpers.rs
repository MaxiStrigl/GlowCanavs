use leptos::{html, NodeRef};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::app::stroke_rendering::catmull_rom;

pub fn rerender_canvas(context: &CanvasRenderingContext2d,  strokes: &Vec<Vec<(f64, f64)>>, offset: (f64, f64)) {
    context.clear_rect(-offset.0, -offset.1, context.canvas().unwrap().width() as f64, context.canvas().unwrap().height() as f64);

    for stroke in strokes {
        catmull_rom::draw_smooth_line(&context, &stroke);
    }
}

pub fn get_context(canvas_ref: &NodeRef<html::Canvas>) -> Option<CanvasRenderingContext2d> {
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

pub fn scale_canvas(canvas_ref: &NodeRef<html::Canvas>) {
    if let Some(canvas) = canvas_ref.get() {
        let context = get_context(&canvas_ref).expect("No Context");


            //TODO: Maybe adjust this to the whole content 

            let device_pixel_ratio = web_sys::window().unwrap().device_pixel_ratio();
            let width = canvas.client_width() as f64;
            let height = canvas.client_height() as f64;

            let image_data = context.get_image_data(0.0, 0.0, width, height).expect("Failed to get ImageData");

            canvas.set_width((width * device_pixel_ratio) as u32);
            canvas.set_height((height * device_pixel_ratio) as u32);

            context
                .scale(device_pixel_ratio, device_pixel_ratio)
                .unwrap();

            context.put_image_data(&image_data, 0.0, 0.0).expect("Failed to put ImageData");
    }
}


pub fn save_canvas_state(context: &CanvasRenderingContext2d, dimensions: (f64, f64)) -> ImageData {
    context
        .get_image_data(0.0, 0.0, dimensions.0, dimensions.1)
        .expect("No Image Data to save")
}

pub fn restore_canvas_state(context: &CanvasRenderingContext2d, image_data: &ImageData) {
    let _ = context.put_image_data(image_data, 0.0, 0.0);
}
