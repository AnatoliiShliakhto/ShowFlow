mod menu;
mod theme;
mod play;
mod refresh;
mod folder;
mod cancel;
mod exit;
mod close;
mod settings;
mod question;
mod repeat;
mod view;
mod stack;
mod language;

use ::dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Icons {
    Menu,
    Theme,
    Play,
    Refresh,
    Folder,
    Cancel,
    Close,
    Exit,
    Settings,
    Question,
    Repeat,
    View,
    Stack,
    Language,
}

#[component]
pub fn Icon(#[props] icon: Icons, #[props(into)] class: &'static str) -> Element {
    match icon {
        Icons::Menu => menu::Icon(class),
        Icons::Theme => theme::Icon(class),
        Icons::Play => play::Icon(class),
        Icons::Refresh => refresh::Icon(class),
        Icons::Folder => folder::Icon(class),
        Icons::Cancel => cancel::Icon(class),
        Icons::Close => close::Icon(class),
        Icons::Exit => exit::Icon(class),
        Icons::Settings => settings::Icon(class),
        Icons::Question => question::Icon(class),
        Icons::Repeat => repeat::Icon(class),
        Icons::View => view::Icon(class),
        Icons::Stack => stack::Icon(class),
        Icons::Language => language::Icon(class),
    }
}
