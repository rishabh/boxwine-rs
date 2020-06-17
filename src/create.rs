use crate::config;
use clap::Clap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use ureq;

/// Create a Mac app from a config file
#[derive(Clap)]
pub struct Create {
    /// Path to config file
    #[clap(short, long, default_value = "app.boxwine.toml")]
    file: String,

    /// Path where you want your app
    #[clap(short, long, default_value = "My App.app")]
    output: String,
}

pub fn create(opts: Create) {
    let output_path = Path::new(&opts.output);
    let temp_app_path = &fs::canonicalize(output_path.join(".boxwine"))
        .expect("unable to specify temporary app directory");

    // validate options
    if !opts.output.as_str().ends_with(".app") {
        panic!(r#"output "{}" must end with ".app"#, opts.output);
    }

    // load config file
    let config = &config::load(opts.file);

    // make .app.boxwine directory and set up inner directories
    create_app_bundle(temp_app_path);

    // download portable wine into it
    let url = config.get_portable_wine_url();
    download_portable_wine(url, temp_app_path);

    // create wineprefix
    let wineprefix_path = create_wineprefix(config, temp_app_path);

    // use winetricks to download/install verbs
    install_winetricks_verbs(config, wineprefix_path);
}

/// Create app bundle
fn create_app_bundle<P: AsRef<Path>>(app_path: P) {
    fs::create_dir(app_path).expect("unable to create app directory");

    let mac_os = app_path.join("Contents/MacOS");
    fs::create_dir_all(mac_os).expect("unable to create Contents/MacOS directory");

    let resources = app_path.join("Contents/Resources");
    fs::create_dir_all(resources).expect("unable to create Contents/Resources directory");
}

/// Download portable wine from {url} and extract it
fn download_portable_wine<P: AsRef<Path>>(url: String, app_path: P) {
    let tarball_name = Path::new(&url).file_name().unwrap().to_str().unwrap();

    println!("Downloading {} ... ", tarball_name);
    let resp = ureq::get(&url).call();

    println!("Done!");

    // println!("Writing archive to disk ... ");
    //
    // let len = resp
    //     .header("Content-Length")
    //     .and_then(|s| s.parse::<usize>().ok())
    //     .unwrap();
    //
    // let mut reader = resp.into_reader();
    // let mut bytes = Vec::with_capacity(len); // pre-alloc to save some time
    // reader.read_to_end(&mut bytes).unwrap();
    //
    // let mut file = match File::create(&tarball_name) {
    //     Err(why) => panic!("couldn't create {}: {}", tarball_name, why),
    //     Ok(file) => file,
    // };
    //
    // match file.write_all(bytes.as_slice()) {
    //     Err(why) => panic!("couldn't write to {}: {}", tarball_name, why),
    //     Ok(_) => print!("Done!\n"),
    // }
    //
    // println!("Extracting archive ... ");
    //
    // // It's actually faster to call tar -xf directly to unpack archives than
    // // using any Rust crate to do so as of today (06-15-2020)
    // // This now means we have a system dependency on tar. And that's fine.
    // let status = Command::new("tar")
    //     .arg("-xf")
    //     .arg(tarball_name)
    //     .status()
    //     .expect("Unable to extract");
    //
    // println!("Done!");
}

/// Create wineprefix
fn create_wineprefix<P: AsRef<Path>>(config: &config::Config, temp_app_path: P) {
    let base_prefix = config.get_base_prefix();

    // if the user defined a base prefix, copy it over
    // otherwise, create one using the wine binary inside the app
}

/// Install winetricks verbs
fn install_winetricks_verbs<P: AsRef<Path>>(config: &config::Config, wineprefix_path: P) {
    let verbs = config.get_verbs();
    for verb in verbs:
        // install verb

    let sandbox = config.get_sandbox();
    // install sandbox
}