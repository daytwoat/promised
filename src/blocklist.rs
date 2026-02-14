use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, bail};



pub struct BlockedItem{
    pub term: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blocklist {
    pub(crate) domains: Vec<String>, //testing with just strings
}

impl Blocklist {
    pub fn new() -> Self {
        Blocklist { domains: Vec::new() }
    }

    pub fn add(&mut self, domain: &str) -> Result<()> {
        if self.domains.contains(&domain.to_string()) {
            bail!("Domain already in blocklist");
        }
        self.domains.push(domain.to_string());
        Ok(())
    }

    pub fn remove(&mut self, domain: &str) -> Result<()> {
        if let Some(pos) = self.domains.iter().position(|d| d == domain) {
            self.domains.remove(pos);
            Ok(())
        } else {
            bail!("Domain not found in blocklist");
            Ok(())
        }
    }

    pub fn is_blocked(&self, domain: &str) -> bool {
        self.domains.iter().any(|d| d == domain)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        if !Path::new(path).exists() {
            return Ok(Blocklist::new());
        }
        let data = fs::read_to_string(path)?;
        let bl: Blocklist = serde_json::from_str(&data)?;
        Ok(bl)
    }
}
