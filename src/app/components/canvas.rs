use crate::app::enums::drawing_mode::Mode;
use crate::app::enums::mouse_button::MouseButton;
use crate::app::helpers::canvas_helpers::*;
use crate::app::helpers::math_helpers::*;
use crate::app::helpers::mouse_helpers::handle_mouse_event;
use crate::app::stroke_rendering::catmull_rom;
use crate::app::stroke_rendering::cubic::draw_smooth_line;

use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::{CanvasRenderingContext2d, MouseEvent};

type ContextRef = Option<CanvasRenderingContext2d>;

#[component]
pub fn Canvas() -> impl IntoView {
    let (is_mouse_down, set_is_mouse_down) = create_signal(MouseButton::None);

    let (context_ref, set_context_ref) = create_signal(ContextRef::None);

    let (points, set_points) = create_signal(Vec::<(f64, f64)>::new());

    let canvas_ref = create_node_ref::<html::Canvas>();

    let (last_move_time, set_last_move_time) = create_signal(js_sys::Date::now());

    let current_mode = use_context::<ReadSignal<Mode>>();

    let (strokes, set_strokes) = create_signal(Vec::<Vec<(f64, f64)>>::new());

    let (offset, set_offset) = create_signal((0.0, 0.0));

    let (redo_stack, set_redo_stack) = create_signal(Vec::<Vec<(f64, f64)>>::new());

    let get_dimensions = move || {
        if let Some(canvas) = canvas_ref.get() {
            (canvas.width() as f64, canvas.height() as f64)
        } else {
            (-1.0, -1.0)
        }
    };

    let window = web_sys::window().unwrap();

    //MOUSE DOWN
    let handle_mouse_down = move |ev: MouseEvent| {
        set_is_mouse_down.update(|state| {
            *state = match ev.button() {
                0 => MouseButton::Left,
                1 => MouseButton::Middle,

                _default => MouseButton::None,
            }
        });

        match is_mouse_down.get() {
            MouseButton::Left => {
                handle_mouse_event(ev, |coordinate| {
                    let coordinate = (coordinate.0 - offset.get().0, coordinate.1 - offset.get().1);
                    set_points.update(|seg| {
                        seg.clear();
                        seg.push(coordinate)
                    });
                });
            }
            MouseButton::Middle => {
                handle_mouse_event(ev, |coordinate| {
                    set_points.update(|seg| {
                        seg.clear();
                        seg.push(coordinate)
                    });
                });
            }
            _default => {}
        }

        if context_ref.get().is_none() {
            let context = get_context(&canvas_ref);
            set_context_ref.set(context);
            context_ref.get().expect("Context is None");
        }
    };

    //MOUSE MOVE
    let handle_mouse_move = move |ev: MouseEvent| {
        let now = js_sys::Date::now();

        if now - last_move_time.get() < 16.0 {
            return;
        }

        set_last_move_time.update(|time| *time = now);

        match is_mouse_down.get() {
            MouseButton::Left => {
                let context = context_ref.get().expect("Context is None");
                let (prev_x, prev_y) = *points.get().last().expect("No Previous Elements");

                handle_mouse_event(ev, |coordinate| {
                    let coordinate = (coordinate.0 - offset.get().0, coordinate.1 - offset.get().1);
                    set_points.update(|seg| seg.push(coordinate));
                });

                let (curr_x, curr_y) = *points.get().last().expect("");

                let distance = (curr_x - prev_x).powi(2) + (curr_y - prev_y).powi(2).sqrt();

                if distance < 10.0 {
                    set_points.update(|seg| {
                        seg.pop().expect("");
                    });
                    return;
                }

                let len = if points.get().len() < 3 {
                    0
                } else {
                    points.get().len() - 3
                };

                if current_mode.expect("Invalid Mode").get() == Mode::Pen {
                    draw_smooth_line(&context, &points.get()[len..].to_vec());
                }
            }
            MouseButton::Middle => {
                let context = context_ref.get().expect("Context is None");

                let (prev_x, prev_y) = *points.get().last().expect("Lol");

                let (curr_x, curr_y) = (ev.x() as f64, ev.y() as f64);

                set_points.update(|points| points.push((curr_x, curr_y)));

                let delta = (curr_x - prev_x, curr_y - prev_y);

                let offset = offset.get();

                log_1(&JsValue::from_str(&format!("({}, {})", offset.0, offset.1)));

                let new_offset = (offset.0 + delta.0, offset.1 + delta.1);

                log_1(&JsValue::from_str(&format!("({}, {})", offset.0, offset.1)));

                set_offset.update(|offset| *offset = new_offset);

                let _ = context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
                let _ = context.translate(new_offset.0, new_offset.1);

                rerender_canvas(&context, &strokes.get(), new_offset);
            }
            _default => {}
        }
    };

    //Mouse UP
    let handle_mouse_up = move |ev: MouseEvent| {
        let context = context_ref.get().expect("Context is None");

        match is_mouse_down.get() {
            MouseButton::Left => {
                handle_mouse_event(ev, |coordinate| {
                    let coordinate = (coordinate.0 + offset.get().0, coordinate.1 + offset.get().1);
                    set_points.update(|seg| seg.push(coordinate));
                });

                match current_mode.expect("Invalid Mode").get() {
                    Mode::Pen => {
                        catmull_rom::draw_smooth_line(&context, &points.get());
                        set_strokes.update(|strokes| strokes.push(points.get()));

                        set_redo_stack.update(|stack| stack.clear());
                    }

                    Mode::Eraser => {
                        let intersects = does_line_intersect(&strokes.get(), &points.get());

                        let mut i = 0;
                        for intesect in intersects {
                            set_strokes.update(|strokes| {
                                strokes.remove(intesect - i);
                            });
                            i += 1;
                        }

                        set_redo_stack.update(|stack| stack.clear());
                    }

                    Mode::PixelEraser => {}
                }

                rerender_canvas(&context, &strokes.get(), offset.get());
            }

            _default => {}
        }

        set_points.update(|list| list.clear());

        set_is_mouse_down.update(|state| *state = MouseButton::None);
    };

    // Call scale_canvas when the component is mounted
    create_effect(move |_| {
        scale_canvas(&canvas_ref);
        let _ = get_context(&canvas_ref)
            .expect("No Canvas")
            .translate(-offset.get().0, -offset.get().1);
    });

    let undo = move || {
        let last_action = strokes.get().pop();
        let context = context_ref.get().expect("Context is None");
        set_strokes.update(|strokes| {
            if strokes.len() > 0 {
                set_redo_stack.update(|redo| redo.push(last_action.expect("Damn")));
                strokes.remove(strokes.len() - 1);
            }
        });
        rerender_canvas(&context, &strokes.get(), offset.get());
    };

    let redo = move || {
        let redo_action = redo_stack.get().pop();
        let context = context_ref.get().expect("Context is None");
        set_redo_stack.update(|redo| {
            if redo.len() > 0 {
                set_strokes.update(|strokes| strokes.push(redo_action.expect("Damn")));
                redo.remove(redo.len() - 1);
            }
        });
        rerender_canvas(&context, &strokes.get(), offset.get());
    };

    window_event_listener(ev::keydown, move |ev| {
        if ev.ctrl_key() && ev.key() == "z" {
            undo();
        }

        if ev.ctrl_key() && ev.key() == "Z" {
            redo();
        }
    });

    window_event_listener(ev::resize, move |_| {
        scale_canvas(&canvas_ref);
    });

    view! {
        <canvas class="my_canvas" node_ref=canvas_ref on:mousedown=handle_mouse_down on:mousemove=handle_mouse_move on:mouseup=handle_mouse_up> </canvas>
    }
}
