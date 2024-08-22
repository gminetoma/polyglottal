#![allow(non_snake_case)]

mod app;
mod routes;
mod components;

use dioxus_logger::tracing::{info, Level};

fn main(){
    dioxus_logger::init(Level::INFO).expect("Failed to init logger");
    info!("Starting App.");
    app::app_launch();
}