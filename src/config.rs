use std::path::PathBuf;

use anyhow::{anyhow, Context, Ok, Result};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub config: PathBuf,
    pub pwd: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self, Self::Error> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            config,
            pwd,
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }
        let term = value.get(0).expect("expect to exists");

        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!("add command requires 2 arguments"));
            }

            let mut drain = value.drain(1..=2);

            return Ok(Operation::Add(
                drain.next().expect("expect to exists"),
                drain.next().expect("expect to exists"),
            ));
        }

        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!("add command requires 1 arguments"));
            }

            let arg = value.pop().expect("expect to exists");

            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow!("too many arguments"));
        }

        let arg = value.pop().expect("expect to exists");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let loc = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        format!("{}/.config", home)
    });

    let mut loc = PathBuf::from(loc);

    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    return Ok(std::env::current_dir().context("Error getting current directory")?);
}


#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{config::Operation, opts::Opts};

    use super::Config;

  #[test]
  fn test_print_all() -> Result<()> {
    let opts: Config = Opts {
      config: None,
      pwd: None,
      args: vec![],
    }.try_into()?;

    assert_eq!(opts.operation, Operation::Print(None));

    return Ok(());
  }

  #[test]
  fn test_print_key() -> Result<()> {
    let opts: Config = Opts {
      config: None,
      pwd: None,
      args: vec!["foo".into()],
    }.try_into()?;

    assert_eq!(opts.operation, Operation::Print(Some("foo".into())));

    return Ok(());
  }
}