use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::quadratic;

pub fn draw_smooth_line(context: &CanvasRenderingContext2d, points: &Vec<(f64, f64)>) {
    if points.len() < 4 {
        quadratic::draw_smooth_line(&context, points);
        return;
    }

    catmull_rom_spline(context, points, 0.8);
}

fn catmull_rom_spline(
    context: &CanvasRenderingContext2d,
    points: &Vec<(f64, f64)>,
    alpha: f64,
) {
    context.set_line_cap("round");
    context.set_stroke_style(&JsValue::from_str("white"));
    context.set_line_width(2.0);

    context.begin_path();
    context.move_to(points[0].0, points[0].1);

    let len: usize = points.len();
    for i in 0..len - 3 {
        let p0 = points[i];
        let p1 = points[i + 1];
        let p2 = points[i + 2];
        let p3 = points[i + 3];


        let distance = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt().round();
        let num_segments = distance as i32;


        for t in (0..num_segments).map(|t| t as f64 / num_segments as f64) {
            let t0 = 0.0;
            let t1 = get_t(t0, p0, p1, alpha);
            let t2 = get_t(t1, p1, p2, alpha);
            let t3 = get_t(t2, p2, p3, alpha);

            let t = t1 + t * (t2 - t1);

            let a1 = interpolate(p0, p1, (t1 - t) / (t1 - t0), (t - t0) / (t1 - t0));
            let a2 = interpolate(p1, p2, (t2 - t) / (t2 - t1), (t - t1) / (t2 - t1));
            let a3 = interpolate(p2, p3, (t3 - t) / (t3 - t2), (t - t2) / (t3 - t2));

            let b1 = interpolate(a1, a2, (t2 - t) / (t2 - t0), (t - t0) / (t2 - t0));
            let b2 = interpolate(a2, a3, (t3 - t) / (t3 - t1), (t - t1) / (t3 - t1));

            let c = interpolate(b1, b2, (t2 - t) / (t2 - t1), (t - t1) / (t2 - t1));

            context.line_to(c.0, c.1);
        }
    }

    context.stroke();
}

fn get_t(prev_t: f64, p0: (f64, f64), p1: (f64, f64), alpha: f64) -> f64 {
    let distance = ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)).sqrt();
    prev_t + distance.powf(alpha)
}

fn interpolate(p0: (f64, f64), p1: (f64, f64), t1: f64, t2: f64) -> (f64, f64) {
    (p0.0 * t1 + p1.0 * t2, p0.1 * t1 + p1.1 * t2)
}
