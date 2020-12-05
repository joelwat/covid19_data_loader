use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let data_dir: &Path;
    // let data_dir_contents: String;

    let env_var = env::var("DATA_DIR").expect("The DATA_DIR env var must be set");

    data_dir = Path::new(&env_var);

    if !data_dir.is_dir() {
        return;
    }

    let read_dir = fs::read_dir(fs::canonicalize(data_dir).unwrap()).unwrap();

    for entry in read_dir.into_iter() {
        let entry = entry.unwrap();

        println!("File name: {:#?}", entry.file_name());
    }
}
