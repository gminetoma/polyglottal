mod home;

use home::Home;
use crate::components::header::Header;
use crate::components::footer::Footer;

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Wrapper)]
    #[route("/")]
    Home {},
}

#[component]
fn Wrapper() -> Element {
    rsx! {
        div { class:"flex flex-col min-h-screen",
            Header {}
            main { class: "flex flex-col grow", Outlet::<Route> {} }
            Footer {}
        }
    }
}