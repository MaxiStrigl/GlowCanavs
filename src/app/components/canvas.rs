use crate::app::helpers::canvas_helpers::get_context;
use crate::app::helpers::mouse_helpers::handle_mouse_event;
use crate::app::{
    components::coordinate_display::CoordinateDisplay,
    stroke_rendering::cubic::draw_smooth_line,
};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, MouseEvent};

type Point = (f64, f64);
type Stroke = Vec<Point>;
type CanvasRef = Option<CanvasRenderingContext2d>;

#[component]
pub fn Canvas() -> impl IntoView {
    let (coordinates, set_coordinates) = create_signal((0.0, 0.0));

    let (to_coordinates, set_to_coordinates) = create_signal((0.0, 0.0));

    let (is_mouse_down, set_is_mouse_down) = create_signal(false);

    let (current_stroke, set_current_stroke) = create_signal(Stroke::new());

    let _strokes = create_signal(Vec::<Stroke>::new());

    let canvas_ref = create_node_ref::<html::Canvas>();

    let (context_ref, set_context_ref): (ReadSignal<CanvasRef>, WriteSignal<CanvasRef>) =
        create_signal(None);

    let handle_mouse_down = move |ev: MouseEvent| {
        set_is_mouse_down.update(|down: &mut bool| *down = true);

        handle_mouse_event(ev, |coordinate| {
            set_coordinates.update(|c| *c = coordinate);
            set_current_stroke.update(|stroke| stroke.push(coordinate))
        });

        let context = context_ref.get();

        if context.is_none() {
            let context = get_context(canvas_ref);
            set_context_ref.set(context);
        }
    };

    let handle_mouse_move = move |ev: MouseEvent| {
        if !is_mouse_down.get() {
            return;
        }

        handle_mouse_event(ev, |coordinate| {
            let (prev_x, prev_y) = coordinates.get();
            let (curr_x, curr_y) = coordinate;

            let distance = (curr_x - prev_x).powi(2) + (curr_y - prev_y).powi(2).sqrt();

            if distance < 5.0 {
                return;
            }

            set_to_coordinates.update(|c| *c = coordinate);
            set_current_stroke.update(|stroke| stroke.push(coordinate));

            //TODO: Pass only 4 segments to the function instead of entire list-
            draw_smooth_line(
                context_ref.get().expect("Context is none"),
                &current_stroke.get(),
            );

            set_coordinates.set(to_coordinates.get());
        });
    };

    let handle_mouse_up = move |ev: MouseEvent| {
        set_is_mouse_down.update(|down: &mut bool| *down = false);

        handle_mouse_event(ev, |coordinate| {
            set_to_coordinates.update(|c| *c = coordinate);
            set_current_stroke.update(|stroke| stroke.push(coordinate))
        });

        draw_smooth_line(
            context_ref.get().expect("Context is none"),
            &current_stroke.get(),
        );

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
