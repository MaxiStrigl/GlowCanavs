use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::quadratic;

type Point = (f64, f64);

pub fn draw_cubic_line(
    context: &CanvasRenderingContext2d,
    p0: Point,
    p1: Point,
    p2: Point,
    p3: Point
) {
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str("white"));
    context.set_line_width(2.0);
    context.move_to(p0.0, p0.1);

    context.bezier_curve_to(p1.0, p1.1, p2.0, p2.1, p3.0, p3.1);

    context.stroke();
}

pub fn draw_smooth_line(context: &CanvasRenderingContext2d, points: &Vec<(f64, f64)>) {
    if points.len() < 4 {
        quadratic::draw_smooth_line(context, points);
    } else {
        let len = points.len();
        let p0 = *points.get(len - 4).expect("Expected Point 1 but is not");
        let p1 = *points.get(len - 3).expect("Expected Point 1 but is not");
        let p2 = *points.get(len - 2).expect("Expected Point 2 but is not");
        let p3 = *points.get(len - 1).expect("Expected Point 3 but is not");

        draw_cubic_line(context, p0, p1, p2, p3);
    }
}
