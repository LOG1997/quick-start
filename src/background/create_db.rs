use crate::sqlite::db::{Config, Entity, Repository};
use anyhow::Result;
use lazy_static::lazy_static;

impl Entity for Config {
    fn table_name() -> &'static str {
        "config"
    }
    fn id(&self) -> &str {
        &self.id.as_str()
    }
}
pub struct AppDatabase {
    // 可以保存多个 Repository
    pub config_repo: Repository<Config>,
    // 其他表的 repository...
}
impl AppDatabase {
    pub fn new(db_name: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            config_repo: Repository::<Config>::new(db_name)?,
        })
    }
}

lazy_static! {
   pub static ref DB: AppDatabase = {
        // 打开/创建数据库文件（app.db 会自动生成在项目db文件夹下）
        let repo=AppDatabase::new("app.db");
        repo.unwrap()
    };
}
