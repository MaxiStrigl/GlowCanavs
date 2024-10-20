use crate::app::enums::drawing_mode::Mode;
use leptos::*;

#[component]
pub fn Popover(show: ReadSignal<bool>) -> impl IntoView {

    view! {
        <div class="popover-container" class:hidden=move || show.get() == false> 

        </div>
    }
}
