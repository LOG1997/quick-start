use crate::sqlite::db::{Entity, Repository};
use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

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

lazy_static! {
   pub static ref DB: AppDatabase = {
        // 打开/创建数据库文件（app.db 会自动生成在项目根目录）
        let repo=AppDatabase::new("app.db");
        repo.unwrap()
    };
}
