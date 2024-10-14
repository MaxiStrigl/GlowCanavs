use ev::MouseEvent;
use leptos::*;

#[derive(PartialEq, Clone)]
enum Mode {
    Pen,
    Eraser,
}

#[component]
pub fn BottomBar() -> impl IntoView {

    let (current_mode, set_current_mode) = create_signal(Mode::Pen);

    let mut current_Mode: Mode = Mode::Pen;

    let on_eraser_click = move |_| {
        set_current_mode.update(|mode| *mode = Mode::Eraser)
    };

    let on_pen_click = move |_| {
        set_current_mode.update(|mode| *mode = Mode::Pen)
    };

    view! {
        <div class="bottom_bar">
            <div class="button" class:active=move || current_mode.get() == Mode::Pen on:click=on_pen_click>
                    <img src="public/pen.svg" class="icon pen" alt="Tauri logo"/>
            </div>
            <div class="button" class:active=move || current_mode.get() == Mode::Eraser on:click=on_eraser_click>
                <img src="public/eraser.svg" class="icon pen" alt="Tauri logo"/>

            </div>
        </div>
    }
}
