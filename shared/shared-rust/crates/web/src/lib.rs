use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod authenticated_user;
pub mod gql;
mod gql_context_extensions;
pub mod middleware;

pub use authenticated_user::*;
pub use gql_context_extensions::*;

pub fn setup_logger() -> anyhow::Result<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::DEBUG)
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  Ok(())
}
