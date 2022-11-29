mod auth_plugin;

use std::{
  fs::File,
  io::Write,
  process::{Command, Stdio}
};

use apollo_router::register_plugin;

use crate::auth_plugin::AuthPlugin;

fn main() -> anyhow::Result<()> {
  let schema = Command::new("rover")
    .arg("supergraph")
    .arg("compose")
    .args(["--config", "supergraph.yml"])
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  register_plugin!("gateway-service", "auth_plugin", AuthPlugin);

  let mut schema_file = File::create("supergraph.graphql")?;
  schema_file.write_all(&schema.stdout)?;

  apollo_router::main()
}
