use crate::app::enums::drawing_mode::Mode;
use ev::MouseEvent;
use leptos::*;

#[component]
pub fn BottomBar<F>(on_click:F)  -> impl IntoView where F:Fn(MouseEvent) + 'static {
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

            <div style="display: flex; align-items: center;">
                <div class="button"  on:click=on_click>
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <circle cx="8" cy="8" fill="white" r="8" />
                    </svg>
                </div>
                <img src="public/caret.svg"/>
            </div>

        </div>
    }
}
