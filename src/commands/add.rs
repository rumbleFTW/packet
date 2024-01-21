use reqwest;
use serde_json::Value;
use std::{fs, io::Write, result};

type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

fn install_wheel(wheel_url: Option<&str>) {
    let body = reqwest::blocking::get(wheel_url.unwrap())
        .unwrap()
        .bytes()
        .unwrap();
    let mut out = fs::File::create("urllib3.whl").unwrap();

    let mut pos = 0;
    while pos < body.len() {
        let bytes_written = out.write(&body[pos..]).unwrap();
        pos += bytes_written;
    }
}

fn install_tar(tar_url: Option<&str>) {
    let body = reqwest::blocking::get(tar_url.unwrap())
        .unwrap()
        .bytes()
        .unwrap();
    let mut out = fs::File::create("urllib3.tar.gz").unwrap();

    let mut pos = 0;
    while pos < body.len() {
        let bytes_written = out.write(&body[pos..]).unwrap();
        pos += bytes_written;
    }
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
        install_tar(tar_url);
    }

    Ok(())
}
