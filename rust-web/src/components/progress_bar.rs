use leptos::prelude::*;

/// progress towards goal
#[component]
pub fn ProgressBar(
    /// displayed progress value
    #[prop(into)]
    progress: Signal<i32>,
    /// max value of progress bar
    #[prop(default = 100)]
    max: u16,
) -> impl IntoView {
    view! { <progress max=max value=progress /> }
}
