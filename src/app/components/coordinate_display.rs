use leptos::*;

#[component]
pub fn CoordinateDisplay(from: ReadSignal<(f64, f64)>, to: ReadSignal<(f64, f64)>) -> impl IntoView {

    view! {
        <div class="coordinate-display">
           <a> "FROM X: " {move || from.get().0.round()} "; Y: " {move || from.get().1.round()} "| TO X: "{move || to.get().0.round()}" Y: " {move || to.get().1.round()}</a>
        </div>
    }
}
