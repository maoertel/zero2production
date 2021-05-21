#[derive(serde::Deserialize)]
pub struct Settings {
  pub server: Server,
  pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct Server {
  pub host: String,
  pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
  pub username: String,
  pub password: String,
  pub port: u16,
  pub host: String,
  pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  let mut settings = config::Config::default();
  // Add configuration values from a file named `config`.
  // It will look for any top-level file with an extension
  // that `config` knows how to parse: yaml, json, etc.
  settings.merge(config::File::with_name("config"))?;
  settings.try_into()
}

impl DatabaseSettings {
  pub fn connection_string(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username, self.password, self.host, self.port, self.database_name
    )
  }

  pub fn connection_string_without_db(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}",
      self.username, self.password, self.host, self.port
    )
  }
}
