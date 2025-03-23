use anyhow::Result;
use crc32fast::hash;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

#[derive(Serialize, Deserialize)]
pub(crate) struct Cache {
    files: HashMap<String, u32>,
}

impl Cache {
    pub(crate) fn new() -> Result<Cache> {
        if fs::exists(".pdcache")? {
            let source = fs::read_to_string(".pdcache")?;
            let cache: Cache = ron::from_str(&source)?;
            Ok(cache)
        } else {
            Ok(Self {
                files: HashMap::new(),
            })
        }
    }

    pub(crate) fn save(&self) -> Result<()> {
        let source = ron::to_string(self)?;
        fs::write(".pdcache", &source)?;
        Ok(())
    }

    pub(crate) fn check<P: AsRef<Path>>(&self, path: P) -> Result<bool> {
        let p = Self::get_path_str(path);
        if let Some(checksum) = self.files.get(&p) {
            let data = fs::read(&p)?;
            Ok(*checksum == hash(&data))
        } else {
            Ok(false)
        }
    }

    pub(crate) fn update<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let p = Self::get_path_str(path);
        let data = fs::read(&p)?;
        self.files.insert(p, hash(&data));
        Ok(())
    }

    fn get_path_str<P: AsRef<Path>>(path: P) -> String {
        path.as_ref().to_str().unwrap().replace("\\", "/")
    }
}
