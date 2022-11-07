use std::{
  fs::File,
  io::Write,
  process::{Command, Stdio}
};

fn main() -> anyhow::Result<()> {
  let schema = Command::new("/root/.rover/bin/rover")
    .arg("supergraph")
    .arg("compose")
    .args(["--config", "supergraph.yml"])
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  let mut schema_file = File::create("supergraph.graphql")?;
  schema_file.write_all(&schema.stdout)?;

  apollo_router::main()
}
