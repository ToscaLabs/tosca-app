use chrono::Datelike;
use chrono::Utc;

use serde::Serialize;

macro_rules! builtin_templates {
    ($(($name:expr, $template:expr)),+) => {
        [
        $(
            (
                $name,
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/templates/", $template)),
            )
        ),+
        ]
    }
}

// Navbar items.
pub(crate) const NAVBAR: &[NavBar] = &[
    NavBar::new("/", "Devices"),
    NavBar::new("policy.html", "Policy"),
];

pub(crate) static TEMPLATES: &[(&str, &str)] = &builtin_templates![
    ("css.custom", "custom.css"),
    ("js.custom", "custom.js"),
    ("layout", "layout.html"),
    ("head", "head.html"),
    ("navbar", "navbar.html"),
    ("scripts", "scripts.html"),
    ("footer", "footer.html"),
    ("index", "index.html"),
    ("devices", "devices.html"),
    ("error", "error.html"),
    ("modal-device", "modal-device.html"),
    ("modal-hazards", "modal-hazards.html")
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
