use leptos::*;

#[component]
pub fn ColorDisplay(color: String) -> impl IntoView {
    view! {
       <div class="color-display" >
            <div class="color-circle" style:background-color=color.clone()/>
            <div>{color}</div>
       </div>
    }
}
