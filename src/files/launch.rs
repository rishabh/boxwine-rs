use crate::config::Config;
use anyhow::{Context, Result};
use std::fs::File;
use std::fs::Permissions;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

// We can create a macro so we don't have to put this string literal in the middle of our code
macro_rules! format_launch_script {
    ( $($e:expr),* ) => {
    format!(r###"#!/bin/sh

# Get directory of this executable, shamelessly taken from https://stackoverflow.com/a/246128
DIR="$( cd "$( dirname "${{BASH_SOURCE[0]}}" )" >/dev/null 2>&1 && pwd )"


WINEPREFIX="${{DIR}}/{}"

if test -f "${{WINEPREFIX}}.tar.gz"; then
    echo "Uncompressing wineprefix ..."
    tar -xzf "${{WINEPREFIX}}.tar.gz" -C "${{WINEPREFIX}}"
    rm "${{WINEPREFIX}}.tar.gz"
    echo "Done!"
fi

WINE="${{DIR}}/wine/bin/wine"

# Launch wine
WINEPREFIX="${{WINEPREFIX}}" "${{WINE}}" start 'C:/windows/system32/cmd.exe'
"###, $($e,)+);
    }
}

pub fn create_launch(config: &Config, wineprefix_name: &str, app_path: &Path) -> Result<()> {
    let launch_script_path = app_path.join("Contents/MacOS/launch");

    let launch_script = format_launch_script!(wineprefix_name);

    let mut file = File::create(&launch_script_path).with_context(|| "Creating Info.plist")?;
    file.write_all(launch_script.as_bytes())
        .with_context(|| "Writing Info.plist")?;

    // Make it executable
    file.set_permissions(Permissions::from_mode(0o755))
        .with_context(|| "Setting exec permission on launch file")?;

    Ok(())
}
