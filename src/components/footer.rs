use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! { footer {
        class:"bg-slate-500 h-10"
    } }
}