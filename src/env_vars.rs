struct EnvVars {
    data_dir: PathBuf,
}

impl EnvVars {
    fn get() -> Result<EnvVars> {
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
}
