use crate::config;
use crate::files::info_plist;
use crate::files::launch;

use anyhow::{Context, Result};
use clap::Clap;
use fs_extra::dir::CopyOptions;
use fs_extra::error::ErrorKind::OsString;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use ureq;

const WINEPREFIX_DIR_NAME: &str = "wineprefix";

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

pub fn create(opts: Create) -> Result<()> {
    // validate options
    if !opts.output.as_str().ends_with(".app") {
        panic!(r#"output "{}" must end with ".app"#, opts.output);
    }

    // load config file
    let config = &config::load(opts.file);

    // make .app.boxwine directory and set up inner directories
    let temp_app_path = create_app_bundle(config, &opts.output)?;

    // download portable wine into it
    let url = config.get_portable_wine_url();
    let wine_archive_path = download_portable_wine(url, &temp_app_path)?;

    // extract wine and get the location of the wine directory
    let wine_dir = extract_wine(wine_archive_path, &temp_app_path)?;

    // initialize the wineprefix
    let wineprefix_path = initialize_wineprefix(config, &wine_dir, &temp_app_path)?;

    // use winetricks to download/install verbs
    install_winetricks_verbs(config, &wineprefix_path)?;

    // Copy files/directories, pre-install
    copy_volumes(config, &wineprefix_path, false)?;

    // Install the programs
    install_programs(config, &wine_dir, &wineprefix_path)?;

    // Copy files/directories, post-install
    copy_volumes(config, &wineprefix_path, true)?;

    // Post-install
    // compress the wineprefix if configured
    compress_wineprefix(config, &wineprefix_path)?;

    Ok(())
}

/// Create app bundle
fn create_app_bundle(config: &config::Config, output_path: &String) -> Result<PathBuf> {
    let temp_app_path = Path::new(output_path);

    fs::create_dir(&temp_app_path)
        .with_context(|| format!("Creating {}", temp_app_path.display()))?;

    let contents_resources = temp_app_path.join("Contents/Resources");
    fs::create_dir_all(contents_resources)
        .with_context(|| "Creating Contents/Resources directory")?;

    let contents_macos = temp_app_path.join("Contents/MacOS");
    fs::create_dir_all(contents_macos).with_context(|| "Creating Contents/MacOS directory")?;

    info_plist::create_info_plist(config, temp_app_path)?;
    launch::create_launch(config, WINEPREFIX_DIR_NAME, temp_app_path)?;

    Ok(temp_app_path.to_path_buf())
}

/// Download portable wine from {url} and extract it
fn download_portable_wine(url: String, app_path: &PathBuf) -> Result<PathBuf> {
    let tarball_name = Path::new(&url).file_name().unwrap().to_str().unwrap();
    let tarball_path = app_path.join("Contents/MacOS").join(tarball_name);

    println!("Downloading {} ... ", tarball_name);
    let resp = ureq::get(&url).call();
    println!("Done!");

    println!("Writing archive to disk ... ");

    // TODO: Don't depend on Content-Length for initializing byte buffer if we somehow can't parse it
    let len = resp.header("Content-Length").unwrap().parse::<usize>()?;

    let mut reader = resp.into_reader();
    let mut bytes = Vec::with_capacity(len); // pre-alloc for performance reasons
    reader.read_to_end(&mut bytes)?;

    let mut file = fs::File::create(&tarball_path)?;

    file.write_all(bytes.as_slice())?;
    println!("Done!");

    Ok(tarball_path)
}

fn extract_wine(wine_archive_path: PathBuf, app_path: &PathBuf) -> Result<PathBuf> {
    println!("Extracting archive ... ");

    // Directory containing wine
    let contents_macos = &app_path.join("Contents/MacOS");

    // It's actually faster to call tar -xf directly to unpack archives than
    // using any Rust crate to do so as of today (06-15-2020)
    // This now means we have a system dependency on tar. And that's fine.
    Command::new("tar")
        .arg("-xf")
        .arg(wine_archive_path.to_str().unwrap())
        .arg("-C")
        .arg(contents_macos)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| "Extracting Wine archive")?;

    fs::remove_file(wine_archive_path).with_context(|| "Removing Wine archive")?;

    // Wine is extracted to a folder called "usr", let's rename it to "wine", instead.
    let orig_wine_dir = contents_macos.join("usr");
    let new_wine_dir = contents_macos.join("wine");

    fs::rename(orig_wine_dir, &new_wine_dir)
        .with_context(|| r#"Renaming Wine directory from "usr" to "wine""#)?;
    println!("Done!");

    Ok(new_wine_dir)
}

/// Create wineprefix
fn initialize_wineprefix(
    config: &config::Config,
    wine_dir: &PathBuf,
    app_path: &PathBuf,
) -> Result<PathBuf> {
    // if the user defined a base prefix, copy it over
    let base_prefix = config.get_base_prefix();
    let macos_path = app_path
        .join("Contents/MacOS/")
        .canonicalize()
        .with_context(|| "Getting absolute path to the MacOS directory in the app")?;

    let wineprefix_path = macos_path.join(WINEPREFIX_DIR_NAME);

    match base_prefix {
        Some(b) => copy_wineprefix(b, &wineprefix_path),
        None => create_wineprefix(config, wine_dir, &wineprefix_path),
    }?;

    Ok(wineprefix_path)
}

fn copy_wineprefix(base_prefix: &String, wineprefix_path: &PathBuf) -> Result<()> {
    let copy_options = fs_extra::dir::CopyOptions::new();

    fs_extra::dir::copy(base_prefix, wineprefix_path, &copy_options).with_context(|| {
        format!(
            "Copying base wineprefix from {} to {}",
            base_prefix,
            wineprefix_path.display()
        )
    })?;

    Ok(())
}

fn create_wineprefix(
    config: &config::Config,
    wine_dir: &PathBuf,
    wineprefix_path: &PathBuf,
) -> Result<()> {
    let wineboot_path = wine_dir.join("bin/wineboot");

    Command::new(wineboot_path)
        .arg("-u")
        .env("WINEPREFIX", wineprefix_path)
        .env("WINEDLLOVERRIDES", config.get_wine_dll_overrides())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| format!("Creating wineprefix at {}", wineprefix_path.display()))?;

    Ok(())
}

/// Install winetricks verbs
fn install_winetricks_verbs(config: &config::Config, wineprefix_path: &PathBuf) -> Result<()> {
    let mut verbs: Vec<String> = config.get_verbs().clone();

    // also sandbox the prefix if we want to
    if *config.get_sandbox() {
        verbs.push("sandbox".to_string());
    }

    // install verbs
    Command::new("winetricks")
        .args(verbs)
        .env("WINEPREFIX", wineprefix_path)
        .env("WINEDLLOVERRIDES", config.get_wine_dll_overrides())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| "Installing verbs")?;

    Ok(())
}

fn copy_volumes(
    config: &config::Config,
    wineprefix_path: &PathBuf,
    post_install: bool,
) -> Result<()> {
    let volumes = config.get_volumes();
    let dosdrives = wineprefix_path.join("dosdrives");
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;

    for volume in volumes {
        let volume_from = &volume.from;
        let volume_to = &volume.to;
        let volume_post_install = &volume.post_install.unwrap_or_default();

        if post_install == *volume_post_install {
            // *super* lazy check to see if the output path is in the wineprefix
            assert!(volume_to.starts_with("c:"));

            let to_wineprefix = dosdrives.join(volume_to);
            println!("to_wineprefix: {}", to_wineprefix.to_str().unwrap());

            // create directories so we can copy items to directories that don't yet exist
            fs_extra::dir::create_all(&to_wineprefix.join(".."), false).with_context(|| {
                format!(
                    "Creating directories to copy volume {} to {}",
                    volume_from, volume_to
                )
            })?;

            let mut from_paths = vec![];
            if volume_from.starts_with("c:") {
                from_paths.push(dosdrives.join(volume_from))
            } else {
                from_paths.push(Path::new(volume_from).to_path_buf())
            }

            fs_extra::copy_items(&from_paths, &to_wineprefix, &options).with_context(|| {
                format!(
                    "Copying volume from {} to {}",
                    volume_from,
                    to_wineprefix.to_str().unwrap()
                )
            })?;
        }
    }

    Ok(())
}

fn install_programs(
    config: &config::Config,
    wine_dir: &PathBuf,
    wineprefix_path: &PathBuf,
) -> Result<()> {
    let runs = config.get_runs();
    let wine_path = wine_dir.join("bin/wine");

    for run in runs {
        let mut prog = Command::new(&wine_path);
        let mut prog_with_args = prog.arg(wineprefix_path).arg("start").arg(&run.program);

        if run.args.is_some() {
            let run_args = run.args.as_ref().unwrap();
            let osstr_args = run_args.iter().map(OsStr::new).collect::<Vec<&OsStr>>();

            prog_with_args = prog_with_args.args(osstr_args);
        }

        prog_with_args
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .with_context(|| format!("Running program {}", run.program))?;
    }
    Ok(())
}

fn compress_wineprefix(config: &config::Config, wineprefix_path: &PathBuf) -> Result<()> {
    if *config.get_compress_wineprefix() {
        Command::new("tar")
            .arg("-czf")
            .arg(wineprefix_path.join(format!("../{}.tar.gz", WINEPREFIX_DIR_NAME)))
            .arg("-C")
            .arg(wineprefix_path)
            .arg(".")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .with_context(|| "Compressing wineprefix")?;

        fs::remove_dir_all(wineprefix_path).with_context(|| "Removing wineprefix dir")?;
    }

    Ok(())
}
