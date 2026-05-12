use crate::sqlite::db::{Entity, Repository};
use anyhow::Result;
use once_cell::sync::OnceCell;
use rusqlite::Result as SqliteResult;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub id: i64,
    pub value: String,
    pub name: String,
}

impl Entity for Config {
    fn table_name() -> &'static str {
        "config"
    }
    fn id(&self) -> i64 {
        self.id
    }
}
pub struct AppDatabase {
    // 可以保存多个 Repository
    pub config_repo: Repository<Config>,
    // 其他表的 repository...
}
impl AppDatabase {
    pub fn new(db_path: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            config_repo: Repository::<Config>::new(db_path)?,
        })
    }
}
