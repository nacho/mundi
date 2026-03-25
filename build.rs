use glib_build_tools::compile_resources;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

fn generate_config() {
    let template = fs::read_to_string("src/config.rs.in").expect("Failed to read src/config.rs.in");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");

    let re = Regex::new(r"@(\w+)@").unwrap();
    let content = re
        .replace_all(&template, |captures: &regex::Captures| {
            let var_name = &captures[1];
            env::var(var_name).unwrap_or_else(|_| match var_name {
                "VERSION" => "0.3.0".to_string(),
                "APPLICATION_ID" => "io.github.nacho.mundi".to_string(),
                "GETTEXT_PACKAGE" => "mundi".to_string(),
                "DATADIR" => "/usr/share".to_string(),
                _ => String::new(),
            })
        })
        .into_owned();

    fs::write(&dest_path, content).unwrap();
    println!("cargo:rerun-if-changed=src/config.rs.in");
}

fn main() {
    generate_config();
    compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "resources.gresource",
    );
}
