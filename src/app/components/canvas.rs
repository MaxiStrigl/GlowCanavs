use crate::app::helpers::canvas_helpers::*;
use crate::app::helpers::mouse_helpers::handle_mouse_event;
use crate::app::stroke_rendering::catmull_rom;
use crate::app::{stroke_rendering::cubic::draw_smooth_line, stroke_rendering::segment::Segment};
use js_sys::Math::{log, log2};
use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::{CanvasRenderingContext2d, ImageData, MouseEvent};

type ContextRef = Option<CanvasRenderingContext2d>;
type OptImageData = Option<ImageData>;
type Stroke = Vec<(f64, f64)>;


fn rerender_canvas(context: &CanvasRenderingContext2d, strokes: &Vec<Vec<(f64, f64)>>) {
    context.reset();

    for stroke in strokes {
        catmull_rom::draw_smooth_line(&context, &stroke);
    }
}

#[component]
pub fn Canvas() -> impl IntoView {
    let (is_mouse_down, set_is_mouse_down) = create_signal(false);

    let (current_segment, set_current_segment) = create_signal(Segment::new(4));

    let (image_data, set_image_data) = create_signal(OptImageData::None);

    let (context_ref, set_context_ref) = create_signal(ContextRef::None);

    let (points, set_points) = create_signal(Vec::<(f64, f64)>::new());

    let canvas_ref = create_node_ref::<html::Canvas>();

    let mut strokes: Vec<Stroke> = Vec::new();

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
            set_current_segment.update(|seg| seg.push(coordinate));
            set_points.update(|seg| {
                seg.clear();
                seg.push(coordinate)
            });
        });

        let context = if context_ref.get().is_none() {
            let context = get_context(&canvas_ref);
            set_context_ref.set(context);
            context_ref.get().expect("Context is None")
        } else {
            context_ref.get().expect("Context is None")
        };

        let image_data = save_canvas_state(&context, get_dimensions());
        set_image_data.set(Some(image_data));
    };

    //MOUSE MOVE
    let handle_mouse_move = move |ev: MouseEvent| {
        if !is_mouse_down.get() {
            return;
        }

        let context = context_ref.get().expect("Context is None");

        let (prev_x, prev_y) = current_segment.get().peek();

        handle_mouse_event(ev, |coordinate| {
            set_current_segment.update(|segment| segment.push(coordinate));
        });

        let (curr_x, curr_y) = current_segment.get().peek();

        let distance = (curr_x - prev_x).powi(2) + (curr_y - prev_y).powi(2).sqrt();

        //TODO: Dyncamic threshhold
        if distance < 10.0 {
            set_current_segment.update(|segment| segment.pop());
            return;
        }

        set_points.update(|seg| seg.push(current_segment.get().peek()));

        if current_segment.get().len() == 2 {
            // let image_d = save_canvas_state(&context, get_dimensions());
            // set_image_data.set(Some(image_d));
        } else {
            // restore_canvas_state(&context, &image_data.get().expect("No Image Data"));
        }

        draw_smooth_line(&context, &current_segment.get().get_points());
    };

    //Mouse UP
    let handle_mouse_up = move |ev: MouseEvent| {
        set_is_mouse_down.update(|down: &mut bool| *down = false);

        let context = context_ref.get().expect("Context is None");

        handle_mouse_event(ev, |coordinate| {
            set_current_segment.update(|segment| segment.push(coordinate));
            set_points.update(|seg| seg.push(coordinate));
        });

        draw_smooth_line(&context, &current_segment.get().get_points());
        restore_canvas_state(&context, &image_data.get().expect("No Image Data"));

        catmull_rom::draw_smooth_line(&context, &points.get());

        strokes.push(points.get());

        set_points.update(|list| list.clear());

        rerender_canvas(&context, &strokes);

        set_current_segment.update(|segment| segment.clear());
    };

    // Call scale_canvas when the component is mounted
    create_effect(move |_| {
        scale_canvas(&canvas_ref);
    });

    window_event_listener(ev::resize, move |_| {
        scale_canvas(&canvas_ref);
    });


    view! {
        <div class="container" >
            <canvas class="my_canvas" node_ref=canvas_ref on:mousedown=handle_mouse_down on:mousemove=handle_mouse_move on:mouseup=handle_mouse_up> </canvas>
        </div>
    }
}
