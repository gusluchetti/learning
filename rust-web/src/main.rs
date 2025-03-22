use components::progress_bar::ProgressBar;
use leptos::prelude::*;
mod components {
    pub mod progress_bar;
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let crops: Vec<&str> = vec!["Soy", "Cotton", "Corn"];

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
            class:odd=move || count.get() % 2 == 1
        >
            "Click me: "
            {count}
        </button>
        <p>"Double count: " {double_count}</p>
        <ProgressBar progress=count max=20 />
        <ProgressBar progress=Signal::derive(double_count) max=20 />
        <ul>
            {crops.into_iter()
            .map(|c| view! {<li> {c} </li>})
            .collect_view()
            }
        </ul>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
