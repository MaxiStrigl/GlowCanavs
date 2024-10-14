use crate::app::enums::drawing_mode::Mode;
use leptos::*;

#[component]
pub fn BottomBar() -> impl IntoView {
    let setter = use_context::<WriteSignal<Mode>>().expect("to have found the setter provided");
    let getter = use_context::<ReadSignal<Mode>>().expect("to have found the setter provided");

    let on_eraser_click = move |_| setter.update(|mode| *mode = Mode::Eraser);

    let on_pen_click = move |_| setter.update(|mode| *mode = Mode::Pen);

    view! {
        <div class="bottom_bar">
            <div class="button" class:active=move || getter.get() == Mode::Pen on:click=on_pen_click>
                <img src="public/pen.svg" class="icon pen" alt="Tauri logo"/>
            </div>
            <div class="button" class:active=move || getter.get() == Mode::Eraser on:click=on_eraser_click>
                <img src="public/eraser.svg" class="icon pen" alt="Tauri logo"/>
            </div>
        </div>
    }
}
