use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::line::draw_line;

fn draw_quadratic_line(
    context: &CanvasRenderingContext2d,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
) {

    let xc = (p1.0 + p2.0) / 2.0;
    let yc = (p1.1 + p2.1) / 2.0;

    context.begin_path();
    context.move_to(p0.0, p0.1);
    context.set_stroke_style(&JsValue::from_str("white"));
    context.quadratic_curve_to(p1.0, p1.1, xc, yc);
    context.stroke();
}

pub fn draw_smooth_line(context: &CanvasRenderingContext2d, points: &Vec<(f64, f64)>) {
    if points.len() < 3 {
        let len = points.len();
        let from = points.get(len - 2);
        let to = points.get(len - 1);

        draw_line(context, *from.expect("HI"), *to.expect("Bye"));
    } else {
        let len = points.len();
        let p0 = *points.get(len - 3).expect("Expected Point 1 but is not");
        let p1 = *points.get(len - 2).expect("Expected Point 2 but is not");
        let p2 = *points.get(len - 1).expect("Expected Point 3 but is not");

        draw_quadratic_line(context, p0, p1, p2)
    }
}
