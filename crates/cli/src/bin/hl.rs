use homelab_cli::config::Config;
use std::{fs::File, io::Read};

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let mut f = File::open("client.toml")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let cfg: Config = toml::from_slice(buffer.as_ref())?;
    println!("{:#?}", cfg);
    Ok(())
}
