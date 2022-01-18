use std::env;

use dotenv::{
    dotenv,
    var,
};

struct EnvVars {
    data_dir: PathBuf,
}

impl EnvVars {
    fn get() -> anyhow::Result<EnvVars> {
        dotenv().with_context(|| String::new("Failed to read .env file."))?;

        let env_var = env::var("DATA_DIR")
            .with_context(|| String::new("The DATA_DIR env var must be set"))?;

        let data_dir = Path::new(&env_var);

        if !data_dir.is_dir() {
            bail!("DATA_DIR must be a valid directory");
        }

        let buff_path = fs::canonicalize(data_dir).unwrap();

        Ok(EnvVars {
            data_dir: buff_path,
        })
    }
}
