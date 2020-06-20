use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct App {
    name: String,
    icon: Option<String>,
    entrypoint: Run,
}

#[derive(Deserialize)]
struct Wine {
    build: Build,
    prefix: Prefix,

    #[serde(rename(deserialize = "volume"))]
    volumes: Vec<Volume>,

    #[serde(rename(deserialize = "run"))]
    runs: Vec<Run>,
}

#[derive(Deserialize)]
struct Build {
    branch: String,
    version: String,
    arch: String,
}

#[derive(Deserialize)]
struct Prefix {
    prefix_arch: String,
    base_prefix: Option<String>,
    sandbox: bool,
    install_gecko: bool,
    install_mono: bool,
}

#[derive(Deserialize)]
struct Volume {
    from: String,
    to: String,
}

#[derive(Deserialize)]
struct Run {
    program: String,
    args: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Winetricks {
    verbs: Vec<String>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    app: App,
    wine: Wine,
    winetricks: Winetricks,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            app: App {
                name: "My App".to_string(),
                icon: None,
                entrypoint: Run {
                    program: "".to_string(),
                    args: None,
                },
            },
            wine: Wine {
                build: Build {
                    branch: "stable".to_string(),
                    version: "5.0".to_string(),
                    arch: "64".to_string(),
                },
                prefix: Prefix {
                    prefix_arch: "win64".to_string(),
                    base_prefix: None,
                    sandbox: true,
                    install_mono: true,
                    install_gecko: false,
                },
                volumes: vec![],
                runs: vec![],
            },
            winetricks: Winetricks { verbs: vec![] },
        }
    }
}

impl Config {
    pub fn get_portable_wine_url(&self) -> String {
        let branch = &self.wine.build.branch;
        let version = &self.wine.build.version;
        let arch = &self.wine.build.arch;

        return format!(
            "https://dl.winehq.org/wine-builds/macosx/pool/portable-winehq-{}-{}-osx{}.tar.gz",
            branch, version, arch
        );
    }

    // WINEDLLOVERRIDES will let us ignore gecko and mono
    pub fn get_wine_dll_overrides(&self) -> String {
        let mut overrides: Vec<&str> = Vec::new();
        if !self.wine.prefix.install_mono {
            overrides.push("mscoree");
        }
        if !self.wine.prefix.install_gecko {
            overrides.push("mshtml");
        }
        let winedlloverrides = format!("{}=", overrides.join(","));

        return winedlloverrides;
    }

    pub fn get_app_icon(&self) -> &Option<String> {
        return &self.app.icon;
    }

    pub fn get_verbs(&self) -> &Vec<String> {
        return &self.winetricks.verbs;
    }

    pub fn get_sandbox(&self) -> &bool {
        return &self.wine.prefix.sandbox;
    }

    pub fn get_base_prefix(&self) -> &Option<String> {
        return &self.wine.prefix.base_prefix;
    }
}

pub fn load(path: String) -> Config {
    let contents = fs::read_to_string(path).expect("Unable to read config file");
    return toml::from_str(contents.as_str()).expect("Unable to parse config file");
}
