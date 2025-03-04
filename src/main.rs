#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tinytemplate;

use serde_json::value::Value;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tinytemplate::error::Result;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
    name: String,
    package: String,
    java_version: String,
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("3 arguments (name, package, java version) are required");
        return Ok(());
    }

    let context = Context {
        name: args.get(1).unwrap().to_string(),
        package: args.get(2).unwrap().to_string(),
        java_version: args.get(3).unwrap().to_string(),
    };

    let pomxml = include_str!("../pom.xml.template");
    let source = include_str!("../Main.java.template");
    let mut tt = TinyTemplate::new();
    tt.add_formatter("lower", |val: &Value, result: &mut String| -> Result<()> {
        match val.as_str() {
            Some(s) => result.push_str(&s.to_lowercase()),
            None => (),
        }

        return Ok(());
    });
    tt.add_template("pomxml", pomxml)?;
    tt.add_template("source", source)?;

    let rendered_pom = tt.render("pomxml", &context)?;
    let rendered_source = tt.render("source", &context)?;
    let base = Path::new(&context.name);

    let src_path = base
        .join("src/main/java")
        .join(context.package.replace(".", "/"))
        .join(context.name.to_lowercase());

    fs::create_dir(base)?;

    fs::create_dir_all(src_path.clone())?;

    let mut f_pom = File::create(base.join("pom.xml"))?;
    f_pom.write_all(rendered_pom.as_bytes())?;
    f_pom.sync_data()?;

    let mut f_source = File::create(src_path.join("Main.java"))?;
    f_source.write_all(rendered_source.as_bytes())?;
    f_source.sync_data()?;

    return Ok(());
}
