extern crate reqwest;
extern crate serde_json;
extern crate zip;

use serde_json::Value;

use std::io;
use std::fs;

use crate::ui::utils;

pub fn download(url: &str, path: &str) -> Result<(), std::io::Error> {
    let mut response = reqwest::blocking::get(url).unwrap();

    let mut dest = fs::File::create(path)?;

    io::copy(&mut response, &mut dest)?;
    Ok(())
}

pub fn unzip(zip: &str, dir: &str) {
    utils::clear();
    println!("Unzipping...");

    let fname = std::path::Path::new(&*zip);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut outpath = std::path::PathBuf::new();
        outpath.push(dir);
        outpath.push(file.sanitized_name());
        // let outpath = file.sanitized_name().push(dir);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}

pub fn get(repo: &str, path: &str, asset_index: usize) {
    crate::ui::utils::clear();

    println!("{} {}", repo, path);

    let client = reqwest::blocking::Client::builder()
        .user_agent("NXSSetup")
        .build().unwrap();

    let mut response = client.get(format!("https://api.github.com/repos/{}/releases/latest", repo).as_str()).send().unwrap().text().unwrap();

    println!("{}", response);

    let json: Value = serde_json::from_str(response.as_str()).unwrap();

    let download_url = json.get("assets").unwrap().get(asset_index).unwrap().get("browser_download_url").unwrap().as_str().unwrap();

    println!("{}", download_url);

    download(download_url, format!("{}.zip", path).as_str());

    unzip(format!("{}.zip", path).as_str(), path);

    fs::remove_file(format!("{}.zip", path));
}