use std::fs;
use yaml_rust::{Yaml, YamlLoader};

struct BundleWine {
    build: String,
    version: String,
    arch: String,
}

pub struct Config {
    app_name: String,
    icon: String,
    run: Vec<Vec<String>>,
    entrypoint: Vec<String>,
    volumes: Vec<String>,
    winetricks_verbs: Vec<String>,
    bundle_winetricks: bool,
    bundle_wine: BundleWine,
    base_prefix: String,
    wine_arch: String,
    sandbox: bool,
}

impl Config {
    pub fn get_portable_wine_url(&self) -> String {
        return format!(
            "https://dl.winehq.org/wine-builds/macosx/pool/portable-winehq-{}-{}-osx{}.tar.gz",
            &self.bundle_wine.build, &self.bundle_wine.version, &self.bundle_wine.arch
        );
    }
}

pub fn load(path: &str) -> Config {
    let contents = fs::read_to_string(path).expect("Unable to read config file");
    let config_yaml = &YamlLoader::load_from_str(&contents).unwrap()[0];

    let app_name = read_app_name(config_yaml);
    let icon = read_icon(config_yaml);
    let run = read_run(config_yaml);
    let entrypoint = read_entrypoint(config_yaml);
    let volumes = read_volumes(config_yaml);
    let winetricks_verbs = read_winetricks_verbs(config_yaml);
    let bundle_winetricks = read_bundle_winetricks(config_yaml);
    let bundle_wine = read_bundle_wine(config_yaml);
    let base_prefix = read_base_prefix(config_yaml);
    let wine_arch = read_wine_arch(config_yaml);
    let sandbox = read_sandbox(config_yaml);

    return Config {
        app_name,
        icon,
        run,
        entrypoint,
        volumes,
        winetricks_verbs,
        bundle_winetricks,
        bundle_wine,
        base_prefix,
        wine_arch,
        sandbox,
    };
}

fn read_app_name(yaml: &Yaml) -> String {
    return read_string(yaml, "app_name", "My App");
}

fn read_icon(yaml: &Yaml) -> String {
    return read_string(yaml, "icon", "");
}

fn read_bundle_winetricks(yaml: &Yaml) -> bool {
    return read_bool(yaml, "bundle_winetricks", false);
}

fn read_base_prefix(yaml: &Yaml) -> String {
    return read_string(yaml, "base_prefix", "");
}

fn read_wine_arch(yaml: &Yaml) -> String {
    return read_string(yaml, "wine_arch", "win64");
}

fn read_sandbox(yaml: &Yaml) -> bool {
    return read_bool(yaml, "sandbox", true);
}

fn read_run(yaml: &Yaml) -> Vec<Vec<String>> {
    // run is a vector of vectors of strings
    let mut run: Vec<Vec<String>> = vec![];
    if yaml["run"].as_str().is_some() {
        run.push(vec![yaml["run"].as_str().unwrap().to_string()])
    } else if yaml["run"].as_vec().is_some() {
        let run_list = yaml["run"].as_vec().unwrap();
        for prog in run_list {
            let prog_entry = read_vector_string(prog, false);
            run.push(prog_entry)
        }
    }

    return run;
}

fn read_entrypoint(yaml: &Yaml) -> Vec<String> {
    let entrypoint_yaml = &yaml["entrypoint"];
    return read_vector_string(entrypoint_yaml, false);
}

fn read_volumes(yaml: &Yaml) -> Vec<String> {
    let volumes_yaml = &yaml["volumes"];
    return read_vector_string(volumes_yaml, true);
}

fn read_winetricks_verbs(yaml: &Yaml) -> Vec<String> {
    let winetricks_verbs_yaml = &yaml["winetricks_verbs"];
    return read_vector_string(winetricks_verbs_yaml, true);
}

fn read_bundle_wine(yaml: &Yaml) -> BundleWine {
    let mut version = "5.0".to_string();
    let mut build = "stable".to_string();
    let mut arch = "64".to_string();

    let bundle_wine_opt = yaml["bundle_wine"].as_hash();
    if bundle_wine_opt.is_some() {
        if bundle_wine_opt.unwrap()[&Yaml::from_str("build")]
            .as_str()
            .is_some()
        {
            build = bundle_wine_opt.unwrap()[&Yaml::from_str("build")]
                .as_str()
                .unwrap()
                .to_string();
        }

        if bundle_wine_opt.unwrap()[&Yaml::from_str("version")]
            .as_str()
            .is_some()
        {
            version = bundle_wine_opt.unwrap()[&Yaml::from_str("version")]
                .as_str()
                .unwrap()
                .to_string();
        }

        if bundle_wine_opt.unwrap()[&Yaml::from_str("arch")]
            .as_str()
            .is_some()
        {
            arch = bundle_wine_opt.unwrap()[&Yaml::from_str("arch")]
                .as_str()
                .unwrap()
                .to_string();
        }
    }
    return BundleWine {
        build,
        version,
        arch,
    };
}

fn read_string(yaml: &Yaml, key: &str, default: &str) -> String {
    return yaml[key].as_str().unwrap_or(default).to_string();
}

fn read_bool(yaml: &Yaml, key: &str, default: bool) -> bool {
    return yaml[key].as_bool().unwrap_or(default);
}

fn read_vector_string(yaml: &Yaml, default: bool) -> Vec<String> {
    if yaml.is_badvalue() && default {
        return vec![];
    }

    let mut v = vec![];
    if yaml.as_str().is_some() {
        v.push(yaml.as_str().unwrap().to_string())
    } else if yaml.as_vec().is_some() {
        let list = yaml.as_vec().unwrap();
        for entry in list {
            if entry.as_str().is_some() {
                v.push(entry.as_str().unwrap().to_string())
            } else {
                panic!("unexpected, was looking for str")
            }
        }
    } else {
        panic!("unexpected, was looking for vec or str")
    }

    return v;
}

// for prog_exec in &run {
//   let prog = &prog_exec[0];
//   let args = &prog_exec[1..];
//   let mut child  = Command::new(prog).args(args).spawn().expect("no work");
//   child.wait().expect("hmm");
//   break;
// }
