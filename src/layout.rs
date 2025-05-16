use chrono::Datelike;
use chrono::Utc;

use serde::Serialize;

// Navbar items.
pub(crate) const NAVBAR: &[NavBar] = &[
    NavBar::new("/", "Devices"),
    NavBar::new("policy.html", "Policy"),
];

pub(crate) fn footer() -> String {
    format!("Copyright © {} ascot", Utc::now().year())
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
