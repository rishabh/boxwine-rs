macro_rules! format_info_plist {
    ($e:expr) => (format!(r###"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>ExecutableFileName</string>
  <key>CFBundleIconFile</key>
  <string>AppIcon</string>
</dict>
</plist>"###, e));
}
