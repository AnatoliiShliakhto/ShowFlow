use crate::{app::*, component::*, service::*, t};
use ::dioxus::prelude::*;

pub fn Languages() -> Element {
    let cfg = use_state().cfg();
    let i18n = use_i18n();
    let languages = i18n.langs();

    let active = use_signal(|| cfg.get("language").unwrap_or(i18n.active().peek().clone()));
    use_hook(|| i18n.change(&active()));

    if languages.len() < 2 {
        return rsx!();
    }

    let change = move |value: String| {
        to_owned![active];
        if !value.is_empty() {
            cfg.set("language", value.clone());
            i18n.change(&value);
            active.set(value);
        }
    };

    rsx! {
        div {
            class: "dropdown dropdown-end block",
            title: t!("language-change"),
            button {
                class: "btn btn-ghost join-item",
                tabindex: 0,
                Icon { icon: Icons::Language, class: "size-6" }
            }
            ul {
                class: "dropdown-content bg-base-200 text-base-content rounded-box",
                class: "top-px max-h-[calc(100vh-11rem)] w-52 overflow-y-hidden",
                class: "border border-white/5 shadow-2xl outline-1 outline-black/5 mt-11 z-500",
                tabindex: 0,
                form {
                    onchange: move |evt| { change(evt.value()) } ,
                    for language in languages.into_iter() {
                        li {
                            input {
                                class: "w-full btn btn-block btn-ghost justify-start",
                                r#type: "radio",
                                name: "lang-dropdown",
                                value: language.clone(),
                                aria_label: t!(&format!("language-{language}")),
                            }
                        }
                    }
                }
            }
        }
    }
}
