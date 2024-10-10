use crate::app::helpers::canvas_helpers::get_context;
use crate::app::helpers::mouse_helpers::handle_mouse_event;
use crate::app::{
    components::coordinate_display::CoordinateDisplay, stroke_rendering::cubic::draw_smooth_line,
};
use leptos::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console::log_1;
use web_sys::{CanvasRenderingContext2d, ImageData, MouseEvent};

type Point = (f64, f64);
type Segment = Vec<Point>;
type Stroke = Vec<Point>;
type ContextRef = Option<CanvasRenderingContext2d>;
type OptImageData = Option<ImageData>;

fn push_on_stack(stack: &mut Segment, value: Point) {
    if stack.len() >= 4 {
        stack.remove(0);
        stack.remove(0);
        stack.remove(0);
    }
    stack.push(value);
}

fn clear_stack(stack: &mut Segment) {
    stack.clear();
}

fn save_canvas_state(context: CanvasRenderingContext2d, dimensions: (f64, f64)) -> ImageData {
    log_1(&JsValue::from_str("Save"));
    context
        .get_image_data(0.0, 0.0, dimensions.0, dimensions.1)
        .expect("No Image Data to save")
}

fn restore_canvas_state(context: CanvasRenderingContext2d, image_data: &ImageData) {
    log_1(&JsValue::from_str("HI"));
    let _ = context.put_image_data(image_data, 0.0, 0.0);
}

#[component]
pub fn Canvas() -> impl IntoView {
    let (coordinates, set_coordinates) = create_signal((0.0, 0.0));

    let (to_coordinates, set_to_coordinates) = create_signal((0.0, 0.0));

    let (is_mouse_down, set_is_mouse_down) = create_signal(false);

    let (current_segment, set_current_segment) = create_signal(Segment::new());

    let (current_stroke, set_current_stroke) = create_signal(Stroke::new());

    let (image_data, set_image_data) = create_signal(OptImageData::None);

    let _strokes = create_signal(Vec::<Stroke>::new());

    let canvas_ref = create_node_ref::<html::Canvas>();

    let (context_ref, set_context_ref) = create_signal(ContextRef::None);

    let get_dimensions = move || {
        if let Some(canvas) = canvas_ref.get() {
            (canvas.width() as f64, canvas.height() as f64)
        } else {
            (-1.0, -1.0)
        }
    };

    //MOUSE DOWN
    let handle_mouse_down = move |ev: MouseEvent| {
        set_is_mouse_down.update(|down: &mut bool| *down = true);

        handle_mouse_event(ev, |coordinate| {
            set_coordinates.update(|c| *c = coordinate);
            set_current_segment.update(|seg| seg.push(coordinate));
            set_current_stroke.update(|stroke| stroke.push(coordinate))
        });

        let context = context_ref.get();

        if context.is_none() {
            let context = get_context(canvas_ref);
            set_context_ref.set(context);
        }

        let image_d = save_canvas_state(context_ref.get().expect("No Cotext"), get_dimensions());
        set_image_data.set(Some(image_d));

        let dim = get_dimensions();
        let x: Result<ImageData, JsValue> = context_ref
            .get()
            .expect("LOL")
            .get_image_data(0.0, 0.0, dim.0, dim.1);
    };

    //MOUSE MOVE
    let handle_mouse_move = move |ev: MouseEvent| {
        if !is_mouse_down.get() {
            return;
        }


        handle_mouse_event(ev, |coordinate| {
            let (prev_x, prev_y) = coordinates.get();
            let (curr_x, curr_y) = coordinate;

            let distance = (curr_x - prev_x).powi(2) + (curr_y - prev_y).powi(2).sqrt();

            //TODO: Dyncamic threshhold
            if distance < 5.0 {
                return;
            }

            let context = context_ref.get().expect("No Cotext");

            if current_segment.get().len() >= 4 {
                let image_d = save_canvas_state(context, get_dimensions());
                set_image_data.set(Some(image_d));
            } else {
                restore_canvas_state(context, &image_data.get().expect("No Image Data"));
            }

            set_to_coordinates.update(|c| *c = coordinate);
            set_current_stroke.update(|stroke| stroke.push(coordinate));
            set_current_segment.update(|segment| push_on_stack(segment, coordinate));

            //TODO: Pass only 4 segments to the function instead of entire list-
            draw_smooth_line(
                context_ref.get().expect("Context is none"),
                &current_segment.get(),
            );

            set_coordinates.set(to_coordinates.get());
        });
    };

    //Mouse UP
    let handle_mouse_up = move |ev: MouseEvent| {
        set_is_mouse_down.update(|down: &mut bool| *down = false);

        handle_mouse_event(ev, |coordinate| {
            set_to_coordinates.update(|c| *c = coordinate);
            set_current_stroke.update(|stroke| stroke.push(coordinate));

            set_current_segment.update(|segment| push_on_stack(segment, coordinate));
        });

        draw_smooth_line(
            context_ref.get().expect("Context is none"),
            &current_segment.get(),
        );

        set_current_segment.update(|segment| clear_stack(segment));
        set_current_stroke.set(Stroke::new());
    };

    let scale_canvas = move || {
        if let Some(canvas) = canvas_ref.get() {
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            let device_pixel_ratio = web_sys::window().unwrap().device_pixel_ratio();
            let width = canvas.client_width() as f64;
            let height = canvas.client_height() as f64;

            canvas.set_width((width * device_pixel_ratio) as u32);
            canvas.set_height((height * device_pixel_ratio) as u32);

            context
                .scale(device_pixel_ratio, device_pixel_ratio)
                .unwrap();
        }
    };

    // Call scale_canvas when the component is mounted
    create_effect(move |_| {
        scale_canvas();
    });

    view! {
        <div class="container" >
            <canvas class="my_canvas" node_ref=canvas_ref on:mousedown=handle_mouse_down on:mousemove=handle_mouse_move on:mouseup=handle_mouse_up> </canvas>
            <CoordinateDisplay from=coordinates to=to_coordinates/>
        </div>
    }
}
