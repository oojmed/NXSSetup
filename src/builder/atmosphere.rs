use crate::builder::release;
use crate::builder::generic;
use crate::builder::BuildItem;
use crate::ui::utils;

use std::env;
use std::fs;
use std::path;

pub fn prepare() {
    release::get("atmosphere-nx/atmosphere", "build/atmosphere-release", 0);
}

pub fn build() {
    let builditem = BuildItem { name: "Atmosphere", git: "https://github.com/Atmosphere-NX/Atmosphere.git" };

    prepare();

    let working_dir_raw = env::current_dir().unwrap();
    let working_dir = working_dir_raw.to_str().unwrap();

    let make_args = format!("SEPT_00_ENC_PATH={}/build/atmosphere-release/sept/sept-secondary_00.enc SEPT_01_ENC_PATH={}/build/atmosphere-release/sept/sept-secondary_01.enc", working_dir, working_dir);
    generic::build(builditem, make_args.as_str());

    utils::clear();
    println!("Building dist...");

    generic::make("build/atmosphere", format!("dist {}", make_args).as_str());

    utils::clear();
    process();
}

pub fn process() -> std::io::Result<()> {
    for entry in fs::read_dir("build/atmosphere/out")? {
        let entry = entry?;
        let path: path::PathBuf = entry.path();

        println!("Found file in build/atmosphere/out: {}", path.display());

        let path_p = path.as_path();
    
        let extension = path_p.extension().unwrap();
        if extension == "zip" {
            println!("File is a zip archive, extracting to out/atmosphere");
            release::unzip(path.to_str().unwrap(), "out/atmosphere");
        }

        if extension == "bin" {
            println!("File is a payload, copying to out/");
            fs::copy(path_p, format!("out/{}", path_p.file_name().unwrap().to_str().unwrap()))?;
        }
    }

    Ok(())

    //release::unzip("build/atmosphere/out/*.zip", "out/atmosphere");
}