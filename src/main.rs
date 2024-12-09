use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::path::Path;

use minijinja::{context, Environment};

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

fn main() {
    let mut env = Environment::new();

    // env.add_filter("comment_license", comment_license);

    let path = Path::new("render");

    create_dir_all(&path).unwrap();

    let mut index = HashMap::new();
    index.insert("name", "hello");

    let contexts = vec![index];

    for ((name, src), context) in TEMPLATES.iter().zip(contexts) {
        env.add_template(name, src)
            .expect("Internal error, built-in template");

        let template = env.get_template(name).unwrap();
        let filled_template = template.render(&context).unwrap();
        write(path.join(src), filled_template).unwrap();
    }
}
