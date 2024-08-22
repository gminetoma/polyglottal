use dioxus::prelude::*;
use crate::routes::Route;

pub fn app_launch(){
    launch(App)
}

fn App() -> Element{
    rsx! {
        Router::<Route> {}
    }
}