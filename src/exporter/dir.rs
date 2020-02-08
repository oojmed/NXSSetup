use std::fs;
use std::env;
use std::path::*;
use std::io;

extern crate walkdir;
use walkdir::WalkDir;

use std::process::{Command, Stdio};

use crate::ui::utils;
use std::io::{Write, stdout, stdin};

fn copy_dir<P, Q>(from: P, from_str: String, to: Q) -> io::Result<u64> where P: AsRef<Path>, Q: AsRef<Path> {
    let to_buf = to.as_ref().to_path_buf();
    let iter = WalkDir::new(from);
    let mut counter = 0u64;

    for res in iter.min_depth(1) {
        let r = res.unwrap();

        let path = r.path();

        let rp_s = path.to_str().unwrap().replace(from_str.as_str(), "");
        let rp = &Path::new(rp_s.as_str());

        let op = &to_buf.as_path().join(rp);

        println!("From Path: {} - Relative: {} - Out: {}", path.display(), rp.display(), op.display());
        println!("To Path: {}", to_buf.as_path().join(path.file_name().unwrap().to_str().unwrap()).display());

        if path.is_file() {
            println!("^ is file");

            fs::copy(path, op);

            counter += 1;
        } else if path.is_dir() {
            println!("^ is dir");

            if rp.exists() {
                fs::remove_dir_all(op)?;
            }
        
            fs::create_dir(op)?;

            //let count = copy_dir(path, to_buf.as_path().join(path.file_name().unwrap().to_str().unwrap())).unwrap();
            //counter += count;
        } else {
            // Skip other content
        }
    }
    Ok(counter)
}

pub fn export() { //pub fn export(out: String) {
    /*let working_dir_raw = env::current_dir().unwrap();
    let working_dir = working_dir_raw.to_str().unwrap();

    println!("{}", working_dir);

    println!("{}", copy_dir("out/sd/", "out/sd/".to_string(), out.clone()).unwrap());

    crate::ui::utils::clear();

    println!("Exported to dir {} successfully", out.clone());*/

    let working_dir_raw = std::env::current_dir().unwrap();
    let working_dir = working_dir_raw.to_str().unwrap();

    if cfg!(windows) {
        let mut cmds = "cd ".to_string() + working_dir + " & set /P dir=Please enter the wanted directory / path: & echo %dir% & xcopy out\\sd\\* %dir% /S /Y /I & pause";

        println!("{}", cmds);

        let mut foo = Command::new("cmd").arg("/c").arg("start").arg("cmd").arg("/c").arg(cmds)
            .stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn().unwrap();
        
        foo.wait();
    } else {
        utils::clear();
        print!("Please enter the wanted directory / path: ");

        std::io::stdout().flush();

        let mut input = String::new();
    
        std::io::stdin().read_line(&mut input).expect("Unable to get user input.");

        input = input.trim().to_string();
    
        println!("{:?}", input);

        copy_dir("out/sd/", "out/sd/".to_string(), input.clone()).unwrap();  
        
        println!("Exported to dir {} successfully", input.clone())
    }
}