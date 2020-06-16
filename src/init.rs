use clap::Clap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Initialize an example config file
#[derive(Clap)]
pub struct Init {
    /// Path to where you want the example config file
    #[clap(short, long, default_value = "app.boxwine.yaml")]
    file: String,
}

pub fn init(opts: Init) {
    let path = Path::new(&opts.file);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `EXAMPLE_CONFIG` string to `file`, returns `io::Result<()>`
    match file.write_all(EXAMPLE_CONFIG.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote the example config file to {}", display),
    }
}

static EXAMPLE_CONFIG: &str = r###"# This is an example configuration file that can be used
# with boxwine to create Mac apps from wine apps.

# USAGE:
# "host" refers to the computer that is using
#    boxwine to create the app.
# "client" refers to the computer that is going to be
#    running the Mac app
# "wineprefix" refers to the virtual windows installation where all
#    of the windows files are stored. Think of it as C:/

# name of your app, default: My App
#
app_name: My App

# path to the icon to be used, default empty
#
# icon: path/to/icon.png

# programs that you want to run/install in the wineprefix, default empty.
# You can specify programs on the host or in the wineprefix
#
# run:
#   - on/host/Setup.exe
#   - ["on/host/other-file.exe", "--some-arg"]    # use a list to specify arguments that you want to pass in
#   - $WINEPREFIX/in/wine/some-file.exe           # Use $WINEPREFIX to access the C:/ in wineprefix

# the program to start when you run the app, required.
# - can be a path to the program in the form of a string
# - or it can be a list, where the first element is the path
#   to the program with subsequent elements as arguments
#
entrypoint: ["C:/to/run/in/wine/program.exe", "--some-arg"]

# if you want to copy any files or folders over to the wineprefix,
# you can specify the file/folder on the host to copy
# into the wineprefix. Both Windows paths with backslashes and
# forward slashes are supported. Default empty.
#
# volumes:
#   - on/host/file.txt:C:\in\wine\file.txt
#   - on/host/folder:C:/in/wine/folder

# if you want to install any verbs from winetricks, you can
# specify the verbs to install here, default empty
#
# winetricks_verbs:
#   - directshow    # for some sound fixes
#   - directplay    # for local multiplayer

# if you want to bundle winetricks into the app, default false
#
# bundle_winetricks: false

# if you want to bundle wine into the app. If you do not bundle wine,
# the client is responsible for having wine installed
#
# bundle_wine:
#   build: stable   # default stable
#   version: 5.0    # default 5.0
#   arch: 64        # default 64

# if you want to use an existing wineprefix as the base, you can specify its
# path, default empty
#
# base_prefix: path/to/existing/wineprefix/on/host

# you can also specify the windows architecture, default win64.
# If you use a base prefix, this will be ignored.
#
# wine_arch: win64

# Sandbox the wineprefix, default true.
# Can also be enabled by specifying "sandbox" as a verb to winetricks
#
# sandbox: true"###;
