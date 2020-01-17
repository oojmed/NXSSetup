mod ui;
mod options;
mod builder;
mod exporter;

use ui::prelude::*;
use builder::prelude::*;

extern crate termion;
extern crate crossterm;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub static mut SELF_BUILD: bool = true;
pub static mut DEBUG: bool = true;

fn build_info() -> String {
    let arch = if cfg!(target_arch = "x86_64") { "x86_64" }
        else if cfg!(target_arch = "x86") { "x86" }
        else if cfg!(target_arch = "powerpc") { "powerpc" }
        else if cfg!(target_arch = "powerpc64") { "powerpc64"}
        else if cfg!(target_arch = "arm") { "arm" }
        else if cfg!(target_arch = "aarch64") { "aarch64" }
        else { "unknown" };

    let os = if cfg!(target_os = "windows") { "windows" }
        else if cfg!(target_os = "macos") { "macos" }
        else if cfg!(target_os = "ios") { "ios" }
        else if cfg!(target_os = "linux") { "linux" }
        else if cfg!(target_os = "android") { "android" }
        else if cfg!(target_os = "freebsd") { "freebsd" }
        else if cfg!(target_os = "dragonfly") { "dragonfly" }
        else if cfg!(target_os = "openbsd") { "openbsd" }
        else if cfg!(target_os = "netbsd") { "netbsd" }
        else { "unknown" };

    return format!("{} {}", os, arch);
}

fn title() -> String {
    let name_start = utils::for_rgb(colors::BLUE.0, colors::BLUE.1, colors::BLUE.2, "NX");
    let name_end = utils::for_rgb(colors::RED.0, colors::RED.1, colors::RED.2, "SSetup");
    let name = utils::bold(&(name_start + name_end.as_str()));

    let version_text = utils::italic(&("v".to_string() + VERSION + " " + build_info().as_str()));

    return format!("{} {}", name, version_text);
}

fn top_message() {
    utils::clear();

    println!("{}", utils::center(title().as_str(), format!("NXSSetup v{} {}", VERSION, build_info()).chars().count() as u16));
    println!("");
    println!("{}", utils::center((utils::bold(utils::for_rgb(colors::RED.0, colors::RED.1, colors::RED.2, "backspace or q").as_str()) + utils::italic(utils::for_rgb(colors::RED.0, colors::RED.1, colors::RED.2, " to go back").as_str()).as_str()).as_str(), 25));
}

fn help() {
    println!("{}\n", title());

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

fn before_show() {
    top_message();
}

fn show_start() {
    before_show();

    let mut method = window::new("Method", "regular", vec![
        checkbox::new("Self-build (download and compile yourself)".to_string(), false, false, true),
        checkbox::new("Pre-built (download official releases)".to_string(), false, false, true) ]);

    input(method);

    unsafe {
        if check_last_chosen("Self-build (download and compile yourself)") {
            show_self_warning();
        } else if check_last_chosen("Pre-built (download official releases)") {
            SELF_BUILD = false;

            show_cfw();
        }
    }
}

fn show_self_warning() {
    before_show();

    let mut self_warn = window::new("Self-build Warning", "warning", vec![
        checkbox::new("Self-building requires some programs and tools.".to_string(), false, false, false),
        checkbox::new("This is for advanced and technical users only.".to_string(), false, false, false),
        checkbox::new("".to_string(), false, false, false),
        checkbox::new("Continue".to_string(), false, false, true),
        checkbox::new("Return".to_string(), false, false, true) ]);

    input(self_warn);

    unsafe {
        if check_last_chosen("Continue") {
            checkbox::CHOSEN[1] = "".to_string();
            show_debug();
        } else {
            checkbox::CHOSEN = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
            show_start();
        }
    }
}

fn show_debug() {
    before_show();

    let mut debug = window::new("Debug", "regular", vec![
        checkbox::new("Debug Builds".to_string(), false, false, true),
        checkbox::new("Release (No Debug) Builds".to_string(), false, false, true) ]);

    input(debug);

    unsafe {
        if check_last_chosen("Release (No Debug) Builds") {
            DEBUG = false;
        }

        show_cfw();
    }
}

fn show_cfw() {
    before_show();

    let mut cfw = window::new("Choose CFW", "regular", vec![
        checkbox::new("Atmosphère".to_string(), false, false, true),
        checkbox::new("None".to_string(), false, false, true) ]);

    input(cfw);

    unsafe {
        if check_last_chosen("Atmosphère") {
            builder::check_outdir_exists();
            builder::check_builddir_exists();

            if SELF_BUILD {
                builder::build(builder::BuildItem { name: "Atmosphere", git: "https://github.com/Atmosphere-NX/Atmosphere.git" });
            } else {
                release::get("atmosphere-nx/atmosphere", "out/sd", 0);
            }

            show_end();
        } else if check_last_chosen("None") {
            // just continue

            show_end();
        } else { // pressed back
            checkbox::CHOSEN = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
            show_start();
        }
    }
}

fn show_end() {
    before_show();

    unsafe {
        let mut end = window::new("Finished", "regular", vec![
            checkbox::new("NXSSetup has finished. See below for the actions NXSSetup has done:".to_string(), false, false, false),
            checkbox::new("".to_string(), false, false, false),
            
            checkbox::new(format!("CFW: {} {} {}", if SELF_BUILD { "Self built" } else { "Downloaded" }, if DEBUG { "(debug)" } else { "(no debug)" }, checkbox::CHOSEN[2]), false, false, false),
            
            checkbox::new("".to_string(), false, false, false),

            //if !cfg!(windows) { checkbox::new(utils::bold(utils::for_rgb(20, 200, 20, "Export SD files to a device (experimental, overwrites)").as_str()), false, false, true) } else { checkbox::new(utils::strikethrough(utils::for_rgb(20, 200, 20, "Export SD files to a device (No Windows Support)").as_str()), false, false, false) },

            checkbox::new(utils::bold(utils::for_rgb(colors::BLUE.0, colors::BLUE.1, colors::BLUE.2, "Export SD files to a directory (overwrites)").as_str()), false, false, true),
            
            checkbox::new("".to_string(), false, false, false),

            checkbox::new(utils::bold(utils::for_rgb(colors::RED.0, colors::RED.1, colors::RED.2, "Exit").as_str()), false, false, true)
        ]);

        input(end);

        if check_last_chosen("Export SD files to a device (experimental, overwrites)") {
            
        } else if check_last_chosen("Export SD files to a directory (overwrites)") {
            copy_dir();
        } // Else exit (by continuing)
    }
}

fn copy_dir() {
    utils::clear();
    print!("Please enter the wanted directory / path: ");

    std::io::stdout().flush();

    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input).expect("Unable to get user input.");

    input = input.trim().to_string();

    println!("{:?}", input);

    exporter::dir::export(input);
}

fn check_last_chosen(wanted: &str) -> bool {
    let i = unsafe { checkbox::CHOSEN.iter().position(|s| s == "").unwrap() };

    if i == 0 {
        return false;
    } else {
        return unsafe { checkbox::CHOSEN[i - 1] == wanted };
    }
}

fn input<'a>(mut window: Box<window::Window<'a>>) {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    window.render();

    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Backspace) | Event::Key(Key::Char('q')) => break,

            Event::Key(Key::Up) => {

            },
            Event::Key(Key::Down) => {

            },

            Event::Key(Key::Char('\n')) => {

            },

            Event::Mouse(me) => {
                let mut exit: bool = false;

                match me {
                    MouseEvent::Press(_, mut x, mut y) => {
                        for i in 0..window.checkboxes.len() {
                            write!(stdout, "{}Click Debug: {}, {} | {}, {} - {}, {} | {} ", termion::cursor::Goto(1, 3 + i as u16), x, y,
                                window.checkboxes[i].collider.start_x, window.checkboxes[i].collider.start_y,
                                window.checkboxes[i].collider.end_x, window.checkboxes[i].collider.end_y,
                                window.checkboxes[i].collider.check(x as i32, y as i32)).unwrap();

                            if window.checkboxes[i].collider.check(x as i32, y as i32) {
                                if window.checkboxes[i].interact() {
                                    exit = true;
                                    break;
                                }
                            }
                        }
                    },
                    _ => (),
                }

                if exit { break; }
            },
            _ => {}
        }

        stdout.flush().unwrap();
    }
}

fn main() {
    let _guard = termion::init();

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    println!("{:?}", args);

    let mut new_options = options::Options { color: true, styling: true };
    
    for a in &args {
        match a.as_str() {
            "-h" | "--help" => return help(),
            "-v" | "--version" => return println!("{}", title()),
            "--no-color" => {
                if new_options.styling {
                    new_options.color = false;
                } else {
                    println!("Only one styling option is allowed (see help for more information)");
                    std::process::exit(1);
                }
            },
            "--no-styling" => {
                if new_options.color {
                    new_options.styling = false;
                } else {
                    println!("Only one styling option is allowed (see help for more information)");
                    std::process::exit(1);
                }
            },
            _ => {
                println!("Unknown argument: {}", a);
                std::process::exit(1);
            }
        }
    }

    options::modifyOptions(new_options);

    unsafe {
        checkbox::CHOSEN = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
    }

    utils::hide_cursor();

    show_start();

    utils::clear();

    utils::show_cursor();
}