mod error;

use crate::error::Error;
use kak::{escape::Mode, face};
use std::{env, env::VarError, path::Path, path::PathBuf, process::Command};

const CONFIG_FILE_NAME: &str = "starship.toml";

fn get_config_from_env(env_var: &str) -> Result<PathBuf, VarError> {
    env::var(env_var).and_then(|dir| {
        let path = Path::new(&dir).join(CONFIG_FILE_NAME);
        println!("{}: {:?} {}", env_var, path, path.exists());
        if path.exists() {
            Ok(path)
        } else {
            Err(env::VarError::NotPresent)
        }
    })
}

fn main() -> Result<(), Error> {
    let res_runtime_config = get_config_from_env("kak_runtime");
    let res_config_config = get_config_from_env("kak_config");

    let config = match (res_runtime_config, res_config_config) {
        (Ok(path), _) => Ok(path),
        (_, Ok(path)) => Ok(path),
        _ => Err(Error::ConfigVarError(env::VarError::NotPresent)),
    }?;

    let args: Vec<String> = env::args().skip(1).collect();
    let starship = Command::new("starship")
        .env("STARSHIP_SHELL", "sh")
        .env("STARSHIP_CONFIG", config)
        .args(&args)
        .output()?;

    return if starship.status.code() != Some(0) {
        Err(Error::StarshipError(
            String::from_utf8_lossy(&starship.stderr).into(),
        ))
    } else {
        let stdout = String::from_utf8_lossy(&starship.stdout);
        if args.first().filter(|v| *v == "prompt").is_some() {
            face::print(&stdout, Mode::Block);
        } else {
            println!("{}", stdout);
            eprintln!("{}", String::from_utf8_lossy(&starship.stderr));
        }
        Ok(())
    };
}
