use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "bg-slate-500 flex items-center justify-center h-10",
            h1 { class: "w-fit text-xl", "The Guessing Game" }
        }
    }
}