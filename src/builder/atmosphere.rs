use crate::builder::release;
use crate::builder::generic;
use crate::builder::BuildItem;
use crate::ui::utils;

use std::env;
use std::fs;
use std::path;

pub fn prepare() {
    release::get("atmosphere-nx/atmosphere", "build/atmosphere-release", 0);

    if cfg!(windows) {
        // Download zip binary because Atmo requires it in the makefile, otherwise it crashes
        // VirusTotal for the URL / binary: https://www.virustotal.com/gui/url/e0367d7eca1a360907afe4d97fed5ae1ee63642149630f5bfbd137f23aeba6b8/detection

        println!("Downloading zip(.exe) binary as Atmosphere's Makefile depends on it");

        release::download("https://github.com/bmatzelle/gow/raw/master/bin/zip.exe", "build/zip.exe");

        let working_dir_raw = env::current_dir().unwrap();
        let working_dir = working_dir_raw.to_str().unwrap();

        println!("SET PATH=%PATH%;{}\\build", working_dir);
        generic::run("SET", format!("PATH=%PATH%;{}\\build", working_dir).as_str());
    }
}

pub fn build() {
    let builditem = BuildItem { name: "Atmosphere", git: "https://github.com/Atmosphere-NX/Atmosphere.git" };

    prepare();

    let working_dir_raw = env::current_dir().unwrap();
    let working_dir = working_dir_raw.to_str().unwrap();

    let make_args: String = if cfg!(windows) {
        format!("SEPT_00_ENC_PATH={}\\build\\atmosphere-release\\sept\\sept-secondary_00.enc SEPT_01_ENC_PATH={}\\build\\atmosphere-release\\sept\\sept-secondary_01.enc", working_dir, working_dir)
    } else {
        format!("SEPT_00_ENC_PATH={}/build/atmosphere-release/sept/sept-secondary_00.enc SEPT_01_ENC_PATH={}/build/atmosphere-release/sept/sept-secondary_01.enc", working_dir, working_dir)
    };

    let dist_make_arg = unsafe { if crate::DEBUG { "dist" } else { "dist-no-debug" }};

    println!("{}", dist_make_arg);

    generic::build(builditem, (dist_make_arg.to_string() + " " + make_args.as_str()).as_str());

    /*utils::clear();
    println!("Building dist...");

    unsafe {
        if crate::DEBUG {
            generic::make("build/atmosphere", format!("dist {}", make_args).as_str());
        } else {
            generic::make("build/atmosphere", format!("dist-no-debug {}", make_args).as_str());
        }
    }*/

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
            println!("File is a zip archive, extracting to out/sd");
            release::unzip(path.to_str().unwrap(), "out/sd");
        }

        if extension == "bin" {
            println!("File is a payload, copying to out/payloads/");
            fs::copy(path_p, format!("out/payloads/{}", path_p.file_name().unwrap().to_str().unwrap()))?;
        }
    }

    Ok(())

    //release::unzip("build/atmosphere/out/*.zip", "out/atmosphere");
}