use flate2::read::GzDecoder;
use reqwest;
use serde_json::Value;
use std::{env, fs::File, io::Write, process::Command, result::Result};
use tar::Archive;
use toml::Table;

type CommandResult<T> = Result<T, Box<dyn std::error::Error>>;

fn install_wheel(wheel_url: Option<&str>) {
    let body = reqwest::blocking::get(wheel_url.unwrap())
        .unwrap()
        .bytes()
        .unwrap();
    let mut out = File::create("urllib3.whl").unwrap();

    let mut pos = 0;
    while pos < body.len() {
        let bytes_written = out.write(&body[pos..]).unwrap();
        pos += bytes_written;
    }
}

//  TO-DO
//
// fn search_package(package_name: &str) {
//     let url = format!("https://pypi.org/simple/{}", package_name);
//     let resp = reqwest::blocking::get(url).unwrap();
//     println!("{:?}", resp.text());
// }

fn install_tar(package_name: &str, tar_url: Option<&str>) {
    /* Installs a .tar.gz package from its PyPI url.
     */
    let body = reqwest::blocking::get(tar_url.unwrap())
        .unwrap()
        .bytes()
        .unwrap();
    let tar_path = format!("{}.tar.gz", package_name);
    let mut out = File::create(&tar_path).unwrap();

    let mut pos = 0;
    while pos < body.len() {
        let bytes_written = out.write(&body[pos..]).unwrap();
        pos += bytes_written;
    }
    let tar_gz = File::open(&tar_path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let _ = archive.unpack(".");

    let orig_dir = env::current_dir().unwrap();

    let _ = env::set_current_dir(package_name).unwrap();

    let l = Command::new("../env/bin/python")
        .arg("setup.py")
        .arg("install")
        .status()
        .unwrap();

    let _ = env::set_current_dir(orig_dir);
    dbg!(l);
}

pub fn exec(package_name: &str, use_wheel: bool) -> CommandResult<()> {
    let version = "1.26.18";
    let url = format!("https://pypi.org/pypi/{package_name}/{version}/json");

    let resp = reqwest::blocking::get(url)?.text()?;
    let json: Value = serde_json::from_str(&resp)?;

    let wheel_url: Option<&str> = Some(json["urls"][0]["url"].as_str().unwrap());
    let tar_url: Option<&str> = Some(json["urls"][1]["url"].as_str().unwrap());

    if use_wheel {
        install_wheel(wheel_url);
    } else {
        install_tar(&format!("{}-{}", package_name, version), tar_url);
    }

    Ok(())
}
