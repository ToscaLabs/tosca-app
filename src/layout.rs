use chrono::Datelike;
use chrono::Utc;

use serde::Serialize;

use crate::language::lang;

// Index route.
pub(crate) const INDEX_ROUTE: &str = "/";

// Web app title.
const WEB_APP_TITLE: &str = "Tosca";

// Navbar items.
pub(crate) const NAVBAR: &[NavBar] = &[
    NavBar::new(INDEX_ROUTE, lang::INDEX_ITEM),
    NavBar::new("/privacy", lang::PRIVACY_ITEM),
];

pub(crate) fn footer() -> String {
    format!("Copyright © {} tosca", Utc::now().year())
}

#[derive(Serialize)]
pub(crate) struct NavBar {
    href: &'static str,
    name: &'static str,
}

impl NavBar {
    const fn new(href: &'static str, name: &'static str) -> Self {
        Self { href, name }
    }
}

#[derive(Serialize)]
pub(crate) struct RenderLayout {
    title: &'static str,
    navbar: &'static [NavBar],
    footer: String,
}

impl RenderLayout {
    pub(crate) fn new() -> Self {
        Self {
            title: WEB_APP_TITLE,
            navbar: NAVBAR,
            footer: footer(),
        }
    }
}
