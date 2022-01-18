mod csv;

use crate::csv::util::read_file;

use std::env;
use std::fs;
use std::path::{ Path, PathBuf };

use anyhow::{ bail, Result };
use dotenv::dotenv;
use regex::Regex;

pub fn run() -> Result<()> {
    let file_regex = Regex::new(r"\d{2}-\d{2}-\d{4}").unwrap();
    let vars = get_env_vars()?;

    let data_dir = fs::canonicalize(vars.data_dir).unwrap();
    let read_dir = fs::read_dir(data_dir).unwrap();

    let mut max_whole: usize = 0;
    let mut max_decimal: usize = 0;

    for entry in read_dir.into_iter() {
        let entry = entry.unwrap();

        if !file_regex.is_match(entry.file_name().to_str().unwrap()) {
            continue;
        }

        println!("File name: {:#?}", entry.file_name());

        let day_record = read_file(entry).unwrap();

        for record in day_record {
            let to_check = record.recovered;

            if to_check.is_some() {
                println!("lat: {:?}", to_check);

                let str = format!("{:?}", to_check.unwrap());
println!("str: {}", str);
                let split:Vec<&str> = str.split('.').collect();
println!("lat_split: {:#?}", split);
                let whole   = split[0];
                let mut decimal = split[1];

                if decimal == "0" {
                    decimal = "";
                }

                println!("whole: {:?}, decimal: {:?}", whole, decimal);

                let whole_len = whole.len();
                let decimal_len = decimal.len();

                if whole_len > max_whole {
                    max_whole = whole_len;
                }

                if decimal_len > max_decimal {
                    max_decimal = decimal_len;
                }
            }
        }
    }

    println!("Max whole length: {}", max_whole);
    println!("Max decimal length: {}", max_decimal);

    Ok(())
}

struct EnvVars {
    data_dir: PathBuf,
}

fn get_env_vars() -> Result<EnvVars> {
    dotenv().expect("Failed to read .env file.");

    let env_var = env::var("DATA_DIR");

    if env_var.is_err() {
        bail!("The DATA_DIR env var must be set");
    }

    let env_var = env_var.unwrap();
    let data_dir = Path::new(&env_var);

    if !data_dir.is_dir() {
        bail!("DATA_DIR must be a valid directory");
    }

    let buff_path = fs::canonicalize(data_dir).unwrap();

    Ok(EnvVars {
        data_dir: buff_path,
    })
}
