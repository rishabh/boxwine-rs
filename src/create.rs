use crate::config;
use clap::Clap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use ureq;

/// Create a Mac app from a config file
#[derive(Clap)]
pub struct Create {
    /// Path to config file
    #[clap(short, long, default_value = "app.boxwine.yaml")]
    file: String,
}

pub fn create(opts: Create) {
    let config = config::load("./example_config.yaml");
    // make .app directory

    // download portable wine into it
    let url = config.get_portable_wine_url();
    download_portable_wine(url);
}

fn download_portable_wine(url: String) {
    // download portable wine
    let tarball_name = Path::new(&url).file_name().unwrap().to_str().unwrap();
    print!("Downloading {} ... ", tarball_name);
    let resp = ureq::get(&url).call();
    print!("Done!\n");

    print!("Writing archive to disk ... ");
    let len = resp
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap();

    let mut reader = resp.into_reader();
    let mut bytes = vec![];
    reader.read_to_end(&mut bytes).unwrap();

    let path = Path::new(tarball_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(bytes.as_slice()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => print!("Done!\n"),
    }

    print!("Extracting archive ... ");
    // subprocess tar -xf
    let status = Command::new("tar")
        .arg("-xf")
        .arg(tarball_name)
        .status()
        .expect("Unable to extract");
    print!("Done!\n");
}
