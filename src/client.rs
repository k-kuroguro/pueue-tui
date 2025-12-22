use std::{
   path::PathBuf,
   sync::Arc,
};
use tokio::sync::Mutex;

use color_eyre::eyre::{WrapErr, bail};
use pueue_lib::{
   Request, Response, Settings, State,
   network::{self, socket::ConnectionSettings},
   secret::read_shared_secret,
};

#[derive(Clone)]
pub struct Client {
   connection: Arc<Mutex<network::Client>>,
}

impl Client {
   pub async fn new(
      config: &Option<PathBuf>,
      profile: &Option<String>,
   ) -> color_eyre::Result<Self> {
      let (mut settings, config_found) =
         Settings::read(config).wrap_err("Failed to read configuration.")?;
      if let Some(profile) = profile {
         settings.load_profile(profile)?;
      }

      // Error if no configuration file can be found, as this is an indicator, that the daemon hasn't been started yet.
      if !config_found {
         bail!("Couldn't find a configuration file. Did you start the daemon yet?");
      }

      let connection_settings = ConnectionSettings::try_from(settings.shared.clone())?;
      let secret = read_shared_secret(&settings.shared.shared_secret_path())?;
      let connection = network::Client::new(connection_settings, &secret, true)
         .await
         .context("Failed to initialize client.")?;

      Ok(Self {
         connection: Arc::new(Mutex::new(connection)),
      })
   }

   pub async fn status(&self) -> color_eyre::Result<State> {
      let mut connection = self.connection.lock().await;
      connection.send_request(Request::Status).await?;
      let response = connection.receive_response().await?;

      match response {
         Response::Status(state) => Ok(*state),
         _ => unreachable!(),
      }
   }
}
