use crate::{app::*, component::*, t};
use ::dioxus::prelude::*;
use ::std::ops::Deref;

static THEMES: [&str; 31] = [
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
    "dim",
    "nord",
    "sunset",
    "abyss",
    "silk",
];

#[component]
pub fn Themes() -> Element {
    if THEMES.len() < 2 {
        return rsx!();
    }
    let cfg = use_state().cfg();
    let mut active_theme = use_signal(move || cfg().get("theme").unwrap_or(THEMES[0].to_string()));

    let change_theme = move |event: Event<FormData>| {
        if let Some(FormValue(value)) = event.values().get("theme-dropdown") {
            (!value.is_empty()).then(move || {
                let value = value[0].clone();
                cfg().set("theme", value.clone());
                active_theme.set(value)
            });
        }
    };

    rsx! {
        div {
            class: "dropdown dropdown-end block",
            title: t!("theme-change"),
            button {
                class: "btn btn-ghost join-item",
                tabindex: 0,
                Icon { icon: Icons::Theme, class: "size-6 stroke-current" }
            }
            ul {
                class: "dropdown-content bg-base-200 text-base-content rounded-box",
                class: "top-px max-h-[calc(100vh-11rem)] w-46 overflow-y-auto",
                class: "border border-white/5 shadow-2xl outline-1 outline-black/5 mt-11 z-500",
                tabindex: 0,
                form {
                    onchange: change_theme,
                    for theme in THEMES.iter().map(<&str>::deref) {
                        li {
                            div {
                                class: "bg-base-100 grid shrink-0 grid-cols-2 gap-0.5 rounded-md p-1 shadow-sm",
                                class: "fixed mt-2.75 right-3 z-10",
                                "data-theme": theme,
                                div { class: "bg-base-content size-1 rounded-full" }
                                div { class: "bg-primary size-1 rounded-full" }
                                div { class: "bg-secondary size-1 rounded-full" }
                                div { class: "bg-accent size-1 rounded-full" }
                            }
                            input {
                                class: "theme-controller btn btn-block btn-ghost justify-start",
                                r#type: "radio",
                                name: "theme-dropdown",
                                value: theme,
                                initial_checked: active_theme().eq(theme),
                                aria_label: t!(&format!("theme-{theme}")),
                            }
                        }
                    }
                }
            }
        }
    }
}
