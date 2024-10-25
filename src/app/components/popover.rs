use leptos::*;

#[component]
pub fn Popover<F, IV>(show: ReadSignal<bool>, inner_content: F) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div class="popover-container" class:hidden=move || show.get() == false>
        {inner_content()}
        </div>
    }
}
