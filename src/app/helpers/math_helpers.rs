use leptos::web_sys::{HtmlElement, MouseEvent};

pub fn calculate_local_coordinates(event: &MouseEvent, element: &HtmlElement) -> (f64, f64) {
    let offset_left = element.offset_left();
    let offset_top = element.offset_top();

    let x = (event.client_x() - offset_left) as f64;
    let y = (event.client_y() - offset_top) as f64;

    (x, y)
}
