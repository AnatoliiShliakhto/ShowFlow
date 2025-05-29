#![allow(non_snake_case)]
#![windows_subsystem = "windows"]
mod app;
mod component;
mod element;
mod model;
mod page;
mod service;

use crate::{app::*, service::*};
use ::dioxus::{desktop::tao, prelude::*};
use ::std::{env, path::PathBuf};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    let dir = PathBuf::from(&env::var("LOCALAPPDATA").expect("env var LOCALAPPDATA not found"))
        .join("ShowFlow");

    let window = tao::window::WindowBuilder::new()
        .with_resizable(false)
        .with_transparent(true)
        .with_maximized(true)
        .with_always_on_top(false)
        .with_decorations(false)
        .with_fullscreen(Some(tao::window::Fullscreen::Borderless(None)))
        .with_content_protection(true)
        .with_title("ShowFlow");

    LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_data_directory(dir)
                .with_disable_context_menu(true)
                .with_window(window)
                .with_menu(None),
        )
        .launch(|| {
            use_init_i18n();
            use_init_state();

            #[cfg(debug_assertions)]
            dioxus::desktop::use_window().devtool();

            rsx! {
                document::Link { rel: "icon", href: FAVICON }
                document::Link { rel: "stylesheet", href: TAILWIND_CSS }
                Router::<Route> {}
            }
        });
}
