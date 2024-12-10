use std::fs::{create_dir_all, write};
use std::path::Path;

use minijinja::value::Value;
use minijinja::Environment;

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

static TEMPLATES: &[(&str, &str)] = &builtin_templates![("html.index", "index.html")];

#[derive(Serialize)]
struct Device {
    title: &'static str,
}

impl Device {
    fn new() -> Self {
        Self { title: "hello" }
    }
}

fn create_devices() -> Vec<Device> {
    vec![Device::new()]
}

#[derive(Serialize)]
struct Index {
    title: &'static str,
    discover_message: &'static str,
    device: Vec<Device>,
}

impl Index {
    fn new() -> Self {
        Self {
            title: "Ascot Controller",
            discover_message: "Discover device",
            device: create_devices(),
        }
    }
}

fn main() {
    let mut env = Environment::new();

    // env.add_filter("comment_license", comment_license);

    let path = Path::new("render");

    create_dir_all(&path).unwrap();

    let contexts = vec![Value::from_serialize(Index::new())];

    for ((name, src), context) in TEMPLATES.iter().zip(contexts) {
        env.add_template(name, src)
            .expect("Internal error, built-in template");

        let template = env.get_template(name).unwrap();
        let filled_template = template.render(context).unwrap();
        write(path.join(src), filled_template).unwrap();
    }
}
