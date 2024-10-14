use leptos::*;

#[component]
pub fn BottomBar() -> impl IntoView {


    view! {
        <div class="bottom_bar"> 
            <div class="button"> 
                    <img src="public/pen.svg" class="icon pen" alt="Tauri logo"/>
            </div>
            <div class="button"/>
        </div>
    }
}
