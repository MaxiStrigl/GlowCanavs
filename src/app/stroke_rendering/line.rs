use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub fn draw_line(context: CanvasRenderingContext2d, from: (f64, f64), to: (f64, f64)) {
    context.begin_path();
    context.set_line_cap("round");
    context.set_stroke_style(&JsValue::from_str("white"));
    context.set_line_width(2.0);
    context.move_to(from.0, from.1);
    context.line_to(to.0, to.1);
    context.stroke();
}

