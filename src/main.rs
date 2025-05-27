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
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    let dir = PathBuf::from(&env::var("LOCALAPPDATA").expect("env var LOCALAPPDATA not found"))
        .join("ShowFlow");

    let window = tao::window::WindowBuilder::new()
        .with_resizable(true)
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

            // use_init_i18n(|| {
            //     I18nConfig::new(langid!("en-US"))
            //         .with_locale((
            //             langid!("en-US"),
            //             include_str!("../assets/i18n/en-US.ftl"),
            //         ))
            //         .with_locale((
            //             langid!("uk-UA"),
            //             include_str!("../assets/i18n/uk-UA.ftl"),
            //         ))
            // });

            rsx! {
                document::Link { rel: "icon", href: FAVICON }
                document::Link { rel: "stylesheet", href: TAILWIND_CSS }
                Router::<Route> {}
            }
        });
}
