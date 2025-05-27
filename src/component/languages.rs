use crate::{app::*, component::*, service::*, t};
use ::dioxus::prelude::*;

pub fn Languages() -> Element {
    let languages = use_i18n().langs();
    let cfg = use_state().cfg();
    let mut active_lang =
        use_signal(move || cfg().get("language").unwrap_or(use_i18n().active()()));
        
    if languages.len() < 2 {
        return rsx!();
    }

    let change_lang = move |event: Event<FormData>| {
        if let Some(FormValue(value)) = event.values().get("lang-dropdown") {
            (!value.is_empty()).then(move || {
                let value = value[0].clone();
                cfg().set("language", value.clone());
                use_i18n().change(&value);
                active_lang.set(value);
            });
        }
    };
    
    use_hook(|| use_i18n().change(&active_lang()));
    
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
                    onchange: change_lang,
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
