use clap::Clap;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Initialize an example config file
#[derive(Clap)]
pub struct Init {
    /// Path to where you want the example config file
    #[clap(short, long, default_value = "app.boxwine.toml")]
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
# boxwine to create the app.
# "client" refers to the computer that is going to be
# running the Mac app
# "wineprefix" refers to the virtual windows installation where all
# of the windows files are stored. Think of it as C:/

[app]
# name of your app, default "My App"
name = "My App"

# path to the icon to be used, default empty    
icon = "path/to/icon.png"

[app.entrypoint]
# the path of the program to start when you run the app, in the wine prefix, required.
program = "C:/to/run/in/wine/program.exe"

# any additional arguments you want to pass to 
args = ["--some-arg true", "--another-one"]


[wine]
 
# if you want to choose a specific build and version of wine
[wine.build]
# default "stable"
branch = "stable"

# default "5.0"
version = "5.0"

# default "64"
arch = "64"

[wine.prefix]
# you can also specify the windows architecture, default win64.
# If you use base_prefix, this will be ignored.
#
prefix_arch = "win64"

# if you want to use an existing wineprefix as the base, you can specify its
# path, default empty
#
base_prefix = "path/to/existing/wineprefix/on/host"

# Sandbox the wineprefix, default true.
# Can also be enabled by specifying "sandbox" as a verb to winetricks
#
sandbox = true

# if you want to copy any files or folders over to the wineprefix,
# you can specify the file/folder on the host to copy
# into the wineprefix. Both Windows paths with backslashes and
# forward slashes are supported. Default empty.
#
[[wine.volume]]
from = "on/host/file.txt"
to = 'C:\in\wine\file.txt'

[[wine.volume]]
from = "on/host/folder"
to = "C:/in/wine/folder"

# programs that you want to run/install in the wineprefix, default empty.
# You can specify programs on the host or in the wineprefix
#
[[wine.run]]
program = "on/host/Setup.exe"

# Use $WINEPREFIX variable to access the C:/ in wineprefix
[[wine.run]]
program = "$WINEPREFIX/in/wine/some-file.exe"  

# use a list to specify arguments that you want to pass in
[[wine.run]]
program = "on/host/other-file.exe"
args = ["--some-arg true", "--another-one"]


[winetricks]
# if you want to install any verbs from winetricks, you can
# specify the verbs to install here, default empty
#
verbs = [
"directshow",  # for some sound fixes
"directplay"   # for local multiplayer
]

# if you want to bundle winetricks into the app, default false
#
bundle = false
"###;
