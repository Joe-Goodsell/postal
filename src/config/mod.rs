pub struct Config {}

impl Config {
    pub async fn new() -> anyhow::Result<Config> {
        Ok(Config {})
    }

    pub async fn write_config() -> anyhow::Result<()> {
        Ok(())
    }
}
