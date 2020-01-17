use std::fs;
use std::env;
use std::path::*;
use std::io;

extern crate walkdir;
use walkdir::WalkDir;

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

pub fn export(out: String) {
    let working_dir_raw = env::current_dir().unwrap();
    let working_dir = working_dir_raw.to_str().unwrap();

    println!("{}", working_dir);

    println!("{}", copy_dir("out/sd/", "out/sd/".to_string(), out.clone()).unwrap());

    crate::ui::utils::clear();

    println!("Exported to dir {} successfully", out.clone());
}