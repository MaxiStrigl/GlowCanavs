use leptos::web_sys::{HtmlElement, MouseEvent};
use wasm_bindgen::JsValue;
use web_sys::console::log_1;

type Point = (f64, f64);

pub fn calculate_local_coordinates(event: &MouseEvent, element: &HtmlElement) -> (f64, f64) {
    let offset_left = element.offset_left();
    let offset_top = element.offset_top();

    let x = (event.client_x() - offset_left) as f64;
    let y = (event.client_y() - offset_top) as f64;

    (x, y)
}


pub fn do_segments_intersect(p1: Point, q1: Point, p2: Point, q2: Point) -> bool {
    // 0 -> p, q,r are collinear
    // 1 -> Clockwise
    // 2 -> Counterclockwise

    fn orientation(p: Point, q: Point, r: Point) -> i32 {
        let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);

        if val.abs() < 0.00001 {
            0
        } else if val > 0.0 {
            1
        } else {
            2
        }
    }

    // Check if point `r` lies on segment `pq`
    fn on_segment(p: Point, q: Point, r: Point) -> bool {
        r.0 <= p.0.max(q.0) && r.0 >= p.0.min(q.0) && r.1 <= p.1.max(q.1) && r.1 >= p.1.min(q.1)
    }

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    //WARNING: Special case: Points are collinear and lie on each other not implemented
    false
}


