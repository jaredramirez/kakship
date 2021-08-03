mod error;

use crate::error::Error;
use clap::{App, Arg};
use kak::{escape::Mode, face};
use std::{env, env::VarError, path::Path, path::PathBuf, process::Command};

const CONFIG_FILE_NAME: &str = "starship.toml";

fn get_config_from_env(env_var: &str) -> Result<PathBuf, VarError> {
    env::var(env_var).and_then(|dir| {
        let path = Path::new(&dir).join(CONFIG_FILE_NAME);
        if path.exists() {
            Ok(path)
        } else {
            Err(env::VarError::NotPresent)
        }
    })
}

fn main() -> Result<(), Error> {
    let matches = App::new("Kakship")
        .about("Status line Starship wrapper for Kakoune")
        .arg(
            Arg::with_name("starship_path")
                .long("starship_path")
                .takes_value(true)
                .default_value("starship")
                .help("Path to Starship bin"),
        )
        .arg(
            Arg::with_name("starship_shell")
                .long("starship_shell")
                .takes_value(true)
                .default_value("sh")
                .help("Shell for Starship to use"),
        )
        .arg(
            Arg::with_name("starship_config")
                .long("config")
                .short("c")
                .takes_value(true)
                .help("Path to Starship TOML config file. If not specified, we will will first look in $kak_runtime/starship.toml then $kak_config/starship.toml"),
        )
        .arg(
            Arg::with_name("starship_arg")
                .required(true)
                .help("Arguement to forward to Starship"),
        )
        .get_matches();

    let arg = matches.value_of("starship_arg").unwrap();
    let bin = matches.value_of("starship_path").unwrap();
    let shell = matches.value_of("starship_shell").unwrap();
    let opt_config = matches.value_of("starship_config");

    let config = match opt_config {
        Some(config_path) => Path::new(&config_path).into(),
        None => {
            let res_runtime_config = get_config_from_env("kak_runtime");
            let res_config_config = get_config_from_env("kak_config");

            match (res_runtime_config, res_config_config) {
                (Ok(path), _) => Ok(path),
                (_, Ok(path)) => Ok(path),
                _ => Err(Error::ConfigVarError(env::VarError::NotPresent)),
            }?
        }
    };

    let starship = Command::new(bin)
        .env("STARSHIP_SHELL", shell)
        .env("STARSHIP_CONFIG", config)
        .args(&[&arg])
        .output()?;

    return if starship.status.code() != Some(0) {
        Err(Error::StarshipError(
            String::from_utf8_lossy(&starship.stderr).into(),
        ))
    } else {
        let stdout = String::from_utf8_lossy(&starship.stdout);
        if arg == "prompt" {
            face::print(&stdout, Mode::Block);
        } else {
            println!("{}", stdout);
            eprintln!("{}", String::from_utf8_lossy(&starship.stderr));
        }
        Ok(())
    };
}
