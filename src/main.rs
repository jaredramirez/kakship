mod error;

use crate::error::Error;
use kak::{escape::Mode, face};
use std::{env, path::Path, process::Command};

fn main() -> Result<(), Error> {
    let opt_config_dir = env::var("kak_config");
    let opt_runtime_dir = env::var("kak_runtime");

    let config_dir = match (opt_config_dir, opt_runtime_dir) {
        (Ok(dir), _) => Ok(dir),
        (_, Ok(dir)) => Ok(dir),
        _ => Err(Error::ConfigVarError(env::VarError::NotPresent)),
    }?;

    let config = Path::new(&config_dir).join("starship.toml");

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
