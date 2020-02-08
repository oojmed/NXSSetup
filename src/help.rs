use crate::ui::utils;

extern crate num_cpus;

use std::env;

pub fn help() {
    println!("{}\n", crate::title());

    println!("{}
    Cores / Threads: {} (logical, {} physical)
    DevKitPro: {}\n",

    utils::title("System"),
    num_cpus::get(), num_cpus::get_physical(),
    env::var("DEVKITPRO").unwrap());

    println!("{}
    nxssetup [OPTIONS]
    
{}
    {}          {}
    {}       {}
    {}          {}
    {}        {}
    
{}",
    utils::title("Usage"), utils::title("Options"),
    utils::bold("-h, --help"), utils::italic("Shows help information (this)"),
    utils::bold("-v, --version"), utils::italic("Shows the version and then exits"),
    utils::bold("--no-color"), utils::italic("Run NXSSetup with no color"),
    utils::bold("--no-styling"), utils::italic("Run NXSSetup with no styling (including color)"),
    utils::bold("Either --no-color or --no-styling, both is not allowed"));
}