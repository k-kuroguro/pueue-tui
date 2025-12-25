use std::{path::PathBuf, sync::Arc};
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

#[cfg(test)]
mod tests {
   use super::*;

   use std::fs;

   use tempfile::tempdir;
   use testcontainers::{
      ContainerAsync, GenericBuildableImage, GenericImage, ImageExt,
      core::{BuildImageOptions, IntoContainerPort, WaitFor, wait::LogWaitStrategy},
      runners::{AsyncBuilder, AsyncRunner},
   };

   fn create_config(
      daemon_cert_path: &Option<PathBuf>,
      shared_secret_path: &Option<PathBuf>,
      port: u16,
   ) -> String {
      format!(
         r#"
shared:
   use_unix_socket: false
   host: 0.0.0.0
   port: {}
   daemon_cert: {}
   shared_secret_path: {}
"#,
         port,
         daemon_cert_path
            .as_ref()
            .map_or("null".to_string(), |p| p.to_string_lossy().to_string()),
         shared_secret_path
            .as_ref()
            .map_or("null".to_string(), |p| p.to_string_lossy().to_string())
      )
   }

   async fn create_container(
      config_dir: PathBuf,
   ) -> color_eyre::Result<(ContainerAsync<GenericImage>, PathBuf)> {
      let image = GenericBuildableImage::new("pueue-tui-test", "latest")
         .with_dockerfile_string(
            r#"
            FROM rust:latest
            RUN cargo install --locked pueue
            CMD ["pueued", "--config", "/root/.config/pueue/pueue.yaml", "-vvv"]
         "#,
         )
         .build_image_with(BuildImageOptions::new().with_skip_if_exists(true))
         .await?;

      let container = image
         .with_exposed_port(6924.tcp())
         .with_wait_for(WaitFor::Log(LogWaitStrategy::stderr("Binding to address")))
         .with_copy_to(
            "/root/.config/pueue/pueue.yaml",
            create_config(&None, &None, 6924).as_bytes().to_vec(),
         )
         .start()
         .await?;

      let port = container.get_host_port_ipv4(6924).await?;
      let cert_path = config_dir.join("daemon.cert");
      let secret_path = config_dir.join("shared_secret");
      let config_path = config_dir.join("pueue.yaml");
      container
         .copy_file_from(
            "/root/.local/share/pueue/certs/daemon.cert",
            cert_path.clone(),
         )
         .await?;
      container
         .copy_file_from(
            "/root/.local/share/pueue/shared_secret",
            secret_path.clone(),
         )
         .await?;

      fs::write(
         &config_path,
         create_config(&Some(cert_path), &Some(secret_path), port),
      )?;

      Ok((container, config_path))
   }

   #[tokio::test]
   async fn test_client_initialization() {
      let dir = tempdir().unwrap();
      let (_container, config_path) = create_container(dir.path().to_path_buf())
         .await
         .expect("Failed to create container.");

      let client = Client::new(&Some(config_path), &None).await;

      assert!(client.is_ok());
   }
}
