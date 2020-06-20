use crate::config;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// We can create a macro so we don't have to put this string literal in the middle of our code
macro_rules! format_info_plist {
    ( $($e:expr),* ) => {
    format!(r###"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>{}</string>
  <key>CFBundleIconFile</key>
  <string>{}</string>
</dict>
</plist>"###, $($e,)+);
    }
}

pub fn create_info_plist(config: &config::Config, app_path: &Path) -> Result<()> {
    let info_plist_path = app_path.join("Info.plist");

    let mut app_icon_path = &"".to_string();
    if config.get_app_icon().is_some() {
        app_icon_path = config.get_app_icon().as_ref().unwrap();
    }

    let info_plist = format_info_plist!("launch", app_icon_path);

    let mut file = File::create(&info_plist_path).with_context(|| "Creating Info.plist")?;
    file.write_all(info_plist.as_bytes())
        .with_context(|| "Writing Info.plist")?;
    Ok(())
}
