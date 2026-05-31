use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn main() -> Result<()> {
    let args = std::env::args();
    let mut args_v: Vec<String> = args.collect();
    if args_v.is_empty() {
        return Err(anyhow::anyhow!("No arguments provided"));
    }
    let _cmd = args_v.remove(0);
    let config = load_config()?;
    let b = {
        let args_s = format!(" {} ", args_v.join(" "));
        is_exclusive(&config, &args_s)
    };
    let exit_status = if b {
        let cmd2 = args_v.remove(0);
        run_command(&cmd2, &args_v, "exclusive-cmd")?
    } else {
        let sccache = {
            let s = config.build.rustc_wrapper;
            if s.is_empty() {
                let home = std::env::var("HOME")?;
                format!("{home}/.cargo/sccache")
            } else {
                s.clone()
            }
        };
        run_command(&sccache, &args_v, "sccache-cmd")?
    };
    if !exit_status.success() {
        let code = exit_status.code().unwrap_or(1);
        std::process::exit(code);
    }
    Ok(())
}

fn run_command(
    command: &str,
    args: &[String],
    debug_label: &str,
) -> Result<std::process::ExitStatus> {
    let status = std::process::Command::new(command)
        .args(args)
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to execute '{}': {}", command, e))?;

    if !status.success() {
        let dbg_s = args.join(" ");
        eprintln!("{}: {} {}", debug_label, command, dbg_s);
    }
    Ok(status)
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
    if let Ok(path_str) = std::env::var("SCCACHE_EXCLUSIVE_CONFIG") {
        return Ok(PathBuf::from(path_str));
    }
    let mut pb =
        dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
    pb.push("sccache-exclusive.toml");
    Ok(pb)
}

fn save_default_config(path: &PathBuf) -> Result<()> {
    use std::io::Write;
    let mut fd = std::fs::File::create(path)?;
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .to_string_lossy()
        .to_string();
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
        if let Some(v) = &exclusive.strings {
            if v.iter().all(|s| args_s.contains(&format!(" {} ", s))) {
                return true;
            }
        } else if args_s.contains(&format!(" {} ", exclusive.string)) {
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
    strings: Option<Vec<String>>,
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
                    strings: None,
                },
                Exclusive {
                    string: "CCC".to_string(),
                    strings: None,
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
            strings: None,
        },
        Exclusive {
            string: "--crate-name wayland_dev",
            strings: None,
        },
    ],
}"#;
        assert_eq!(target, expect);
    }
    #[test]
    fn test3() {
        let config: Config = toml::from_str(
            r#"
[build]
rustc-wrapper = "/home/aki-akaguma/.cargo/bin/sccache"

[[exclusive]]
string = "--crate-name wayland_protocols"

[[exclusive]]
string = ""
strings = ["--crate-name wayland_dev", "--target=wasm32-unknown-unknown"]
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
            strings: None,
        },
        Exclusive {
            string: "",
            strings: Some(
                [
                    "--crate-name wayland_dev",
                    "--target=wasm32-unknown-unknown",
                ],
            ),
        },
    ],
}"#;
        assert_eq!(target, expect);
    }
}
