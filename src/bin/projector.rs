use anyhow::Result;
use clap::Parser;
use projector::{opts::Opts, config::{Config, Operation}, projector::Projector};

fn main() -> Result<()> {
  let config: Config = Opts::parse().try_into()?;
  let mut projector = Projector::new(config.config, config.pwd);

  match config.operation {
    Operation::Print(None) => {
      let value = projector.get_value_all();
      let value = serde_json::to_string(&value)?;
      println!("{}", value);
    }
    Operation::Print(Some(key)) => {
      if let Some(value) = projector.get_value(key) {
        println!("{}", value);
      } else {
        println!("Key not found");
      }
    }
    Operation::Add(key, value) => {
      projector.set_value(key, value);
      projector.save()?;
      println!("OK");
    }
    Operation::Remove(key) => {
      projector.remove_value(&key);
      projector.save()?;
      println!("OK");
    }
  }

  return Ok(());
}