pub mod generic;
pub mod atmosphere;
pub mod release;

use std::fs;
use std::path;

pub mod prelude {
    pub use crate::builder::atmosphere;
    pub use crate::builder::release;
    pub use crate::builder::generic;
}

pub struct BuildItem<'a> {
    pub name: &'a str,
    pub git: &'a str
}

pub fn check_builddir_exists() -> std::io::Result<()> {
    if path::Path::new("build").exists() {
        fs::remove_dir_all("build")?;
    }

    fs::create_dir("build")?;

    Ok(())
}

pub fn check_outdir_exists() -> std::io::Result<()> {
    if path::Path::new("out").exists() {
        fs::remove_dir_all("out")?;
    }

    fs::create_dir("out")?;

    let subdirs = vec!["payloads", "sd"];

    for d in subdirs {
        let p = "out/".to_string() + d;
        if path::Path::new(&p).exists() {
            fs::remove_dir_all(p.clone())?;
        }
    
        fs::create_dir(&p)?;
    }

    Ok(())
}

pub fn build(builditem: BuildItem<'_>) {
    check_builddir_exists();
    check_outdir_exists();

    if builditem.name == "Atmosphere" {
        atmosphere::build();
    }
}