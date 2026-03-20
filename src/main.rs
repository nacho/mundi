mod application;
mod map_quiz_view;
mod map_widget;
mod quiz;
mod region_names;
mod registry;
mod window;

mod config {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

use anyhow::Result;
use gettextrs::{bind_textdomain_codeset, bindtextdomain, setlocale, textdomain, LocaleCategory};
use gio::prelude::*;
use std::path::PathBuf;

use application::LearnMapsApplication;

fn run_application() -> Result<()> {
    setlocale(LocaleCategory::LcAll, "");

    let localedir = PathBuf::from(config::DATADIR).join("locale");
    bindtextdomain(config::PACKAGE, localedir)?;
    bind_textdomain_codeset(config::PACKAGE, "UTF-8")?;
    textdomain(config::PACKAGE)?;

    gio::resources_register_include!("resources.gresource")?;

    let app = LearnMapsApplication::new();
    app.run();

    Ok(())
}

fn main() {
    if let Err(e) = run_application() {
        eprintln!("Application initialization failed: {e}");
    }
}
