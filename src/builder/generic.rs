use crate::builder::BuildItem;
use crate::ui::utils;

use std::process::{Command, Stdio};

extern crate num_cpus;

pub fn clone(repo: &str, dir: &str) -> std::io::Result<()> {
    let mut output = Command::new("git").arg("clone").arg(repo).arg(dir).stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn()?; //.output()?;

    /*if !output.status.success() {
        println!("Error\n{}", String::from_utf8(output.stderr).unwrap());
    } else {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }*/

    output.wait();

    Ok(())
}

pub fn make(dir: &str, make_args: &str) -> std::io::Result<()> {
    let threads = num_cpus::get();
    let mut output = Command::new("make").args(make_args.split(' ')).arg(format!("-j{}", threads)).current_dir(dir).stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn()?; //.output()?;

    output.wait();

    /*if !output.status.success() {
        println!("Error\n{}", String::from_utf8(output.stderr).unwrap());
    } else {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }*/

    Ok(())
}

pub fn build(builditem: BuildItem, make_args: &str) {
    let dir = format!("build/{}", builditem.name.to_lowercase());
    
    utils::clear();

    println!("Cloning git repository {} (git clone {} {})...", builditem.git, builditem.git, dir);

    clone(builditem.git, dir.as_str());

    utils::clear();

    println!("Building...");

    make(dir.as_str(), make_args);

    println!("Built");
}