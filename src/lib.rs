use std::env;
use std::fs;
use std::path::{ Path, PathBuf };

use anyhow::{ bail, Result };

pub fn run() -> Result<()> {
    // let data_dir_contents: String;

    let vars = get_env_vars()?;

    let data_dir = fs::canonicalize(vars.data_dir)?;
    let read_dir = fs::read_dir(data_dir)?;

    for entry in read_dir.into_iter() {
        let entry = entry?;

        println!("File name: {:#?}", entry.file_name());
    }

    return Ok(());
}

struct EnvVars {
    data_dir: PathBuf,
}

fn get_env_vars() -> Result<EnvVars> {
    let env_var = env::var("DATA_DIR");

    if let Err(_) = env_var {
        bail!("The DATA_DIR env var must be set");
    }

    let env_var = env_var.unwrap();
    let data_dir = Path::new(&env_var);

    if !data_dir.is_dir() {
        bail!("DATA_DIR must be a valid directory");
    }

    let buff_path = fs::canonicalize(data_dir)?;

    Ok(EnvVars {
        data_dir: buff_path,
    })
}
