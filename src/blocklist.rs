use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlockedItem {
    pub term: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blocklist {
    pub(crate) domains: Vec<BlockedItem>,
    // pub days: [u8; 7],
    // pub duration: u16,
}

impl Blocklist {
    pub fn new() -> Self {
        Blocklist {
            domains: Vec::new(),
        }
    }

    pub fn add(&mut self, domain: &str) -> Result<()> {
        if self.domains.iter().any(|d| d.term == domain) {
            bail!("Domain already in blocklist");
        }
        self.domains.push(BlockedItem {
            term: domain.to_string(),
        });
        Ok(())
    }

    pub fn remove(&mut self, domain: &str) -> Result<()> {
        if let Some(pos) = self.domains.iter().position(|d| d.term == domain) {
            self.domains.remove(pos);
            Ok(())
        } else {
            bail!("Domain not found in blocklist");
        }
    }

    pub fn is_blocked(&self, domain: &str) -> bool {
        self.domains.iter().any(|d| d.term == domain)
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
        if data.trim().is_empty() {
            return Ok(Blocklist::new());
        }

        let bl: Blocklist = serde_json::from_str(&data)?;
        Ok(bl)
    }
}
