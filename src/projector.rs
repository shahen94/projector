
use std::{collections::HashMap, path::PathBuf, vec};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
  pub projector: HashMap<PathBuf, HashMap<String, String>>
}

pub struct Projector {
  config: PathBuf,
  pwd: PathBuf,
  data: Data,
}

fn default_data() -> Data {
  return Data {
    projector: HashMap::new(),
  };
}

impl Projector {
  pub fn new(config: PathBuf, pwd: PathBuf) -> Self {
    if std::fs::metadata(&config).is_ok() {
      let contents = std::fs::read_to_string(&config);
      let contents = contents.unwrap_or(String::from("{\"projector\": {}}"));
      let data = serde_json::from_str(&contents);
      let data = data.unwrap_or(default_data());

      return Self {
        config,
        pwd,
        data,
      };
    }

    return Self {
      config,
      pwd,
      data: default_data(),
    };
  }

  pub fn get_value_all(&self) -> HashMap<&String, &String> {
    let mut paths = vec![];
    let mut current = Some(self.pwd.as_path());

    while let Some(p) = current {
      paths.push(p);
      current = p.parent();
    }

    let mut out = HashMap::new();

    for path in paths.into_iter().rev() {
      if let Some(map) = self.data.projector.get(path) {
        out.extend(map);
      }
    }

    return out;
  }

  pub fn remove_value(&mut self, key: &str) {
    self.data.projector
      .entry(self.pwd.clone())
      .or_default()
      .remove(key);
  }

  pub fn set_value(&mut self, key: String, value: String) {
    self.data.projector
      .entry(self.pwd.clone())
      .or_default()
      .insert(key, value);
  }

  pub fn get_value(&self, key: String) -> Option<String> {
    let mut current = Some(self.pwd.as_path());
    let mut out = None;
    
    while let Some(p) = current {
      if let Some(dir) = self.data.projector.get(p) {
        if let Some(value) = dir.get(&key) {
          out = Some(value);
        }
      }

      current = p.parent();
    }

    return out.cloned();
  }

  pub fn save(&self) -> Result<()> {
    if let Some(dir) = self.config.parent() {
      if std::fs::metadata(dir).is_err() {
        std::fs::create_dir_all(dir)?;
      }
    }

    let contents = serde_json::to_string(&self.data)?;
    std::fs::write(&self.config, contents)?;

    return Ok(());
  }
}
