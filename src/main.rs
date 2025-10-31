use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn main() -> Result<()> {
    let args = std::env::args();
    let mut args_v: Vec<String> = args.collect();
    let _cmd = args_v.remove(0);
    let config = load_config()?;
    let b = {
        //println!("{:#?}", config);
        //for arg in args { println!("{arg}"); }
        let args_s = args_v.join(" ");
        //println!("{args_s}"); std::process::exit(0);
        is_exclusive(&config, &args_s)
    };
    let exit_status = if b {
        //
        let cmd2 = args_v.remove(0);
        let status = std::process::Command::new(&cmd2)
            .args(&args_v)
            .status()
            .expect("failed to execute process: {sccache}");
        if !status.success() {
            let dbg_s = args_v.join(" ");
            eprintln!("AAA: {cmd2} {dbg_s}");
        }
        status
    } else {
        //
        let sccache = {
            let s = config.build.rustc_wrapper;
            if s.is_empty() {
                let home = std::env::var("HOME")?;
                format!("{home}/.cargo/sccache")
            } else {
                s.clone()
            }
        };
        let status = std::process::Command::new(&sccache)
            .args(&args_v)
            .status()
            .expect("failed to execute process: {sccache}");
        if !status.success() {
            let dbg_s = args_v.join(" ");
            eprintln!("BBB: {sccache} {dbg_s}");
        }
        status
    };
    if !exit_status.success() {
        let code = exit_status.code().unwrap();
        std::process::exit(code);
    }
    Ok(())
}

fn load_config() -> Result<Config> {
    use std::io::Read;
    let config_path = config_path()?;
    let config_p = std::path::Path::new(&config_path);
    if !config_p.exists() {
        save_default_config(&config_path)?;
    }
    let mut fd = std::fs::File::open(config_p)?;
    let mut s = String::new();
    fd.read_to_string(&mut s)?;
    let config: Config = toml::from_str(&s)?;
    Ok(config)
}

fn config_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    let mut pb = PathBuf::new();
    pb.push(home);
    pb.push(".config");
    pb.push("sccache-exclusive.toml");
    Ok(pb)
}

fn save_default_config(path: &PathBuf) -> Result<()> {
    use std::io::Write;
    let mut fd = std::fs::File::create(path)?;
    let home = std::env::var("HOME")?;
    let content = format!(
        r#"[build]
rustc-wrapper = "{home}/.cargo/bin/sccache"

[[exclusive]]
string = "--crate-name XXX"

[[exclusive]]
string = "--crate-name wayland_client"

[[exclusive]]
string = "--crate-name wayland_protocols"
"#
    );
    fd.write_all(content.as_bytes())?;
    Ok(())
}

fn is_exclusive(config: &Config, args_s: &str) -> bool {
    for exclusive in config.exclusive.iter() {
        if let Some(_n) = args_s.find(&exclusive.string) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    build: Build,
    exclusive: Vec<Exclusive>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Build {
    #[serde(rename = "rustc-wrapper")]
    rustc_wrapper: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Exclusive {
    string: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let config = Config {
            build: Build {
                rustc_wrapper: "AAAA".to_string(),
            },
            exclusive: vec![
                Exclusive {
                    string: "BBB".to_string(),
                },
                Exclusive {
                    string: "CCC".to_string(),
                },
            ],
        };
        let toml = toml::to_string(&config).unwrap();
        let expect = r#"[build]
rustc-wrapper = "AAAA"

[[exclusive]]
string = "BBB"

[[exclusive]]
string = "CCC"
"#;
        assert_eq!(toml, expect);
    }
    #[test]
    fn test2() {
        let config: Config = toml::from_str(
            r#"
[build]
rustc-wrapper = "/home/aki-akaguma/.cargo/bin/sccache"

[[exclusive]]
string = "--crate-name wayland_protocols"

[[exclusive]]
string = "--crate-name wayland_dev"
"#,
        )
        .unwrap();
        //println!("{:#?}", config);
        let target = format!("{:#?}", config);
        let expect = r#"Config {
    build: Build {
        rustc_wrapper: "/home/aki-akaguma/.cargo/bin/sccache",
    },
    exclusive: [
        Exclusive {
            string: "--crate-name wayland_protocols",
        },
        Exclusive {
            string: "--crate-name wayland_dev",
        },
    ],
}"#;
        assert_eq!(target, expect);
    }
}
