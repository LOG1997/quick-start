use anyhow::Result;
use rusqlite::Result as SqliteResult;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Mutex;
/// 实体必须能提供表名和它的唯一 ID
pub trait Entity: Serialize + for<'de> Deserialize<'de> + Clone + Send + 'static {
    fn table_name() -> &'static str;
    fn id(&self) -> i64;
}
pub struct Repository<T: Entity> {
    conn: Mutex<Connection>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Entity> Repository<T> {
    /// 打开或创建数据库，并确保表存在
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            )",
            T::table_name()
        );
        conn.execute(&sql, [])?;
        Ok(Repository {
            conn: Mutex::new(conn),
            _marker: std::marker::PhantomData,
        })
    }

    /// 插入或替换（按 ID）
    pub fn save(&self, entity: &T) -> Result<()> {
        let data = serde_json::to_string(entity)?;
        let sql = format!(
            "INSERT OR REPLACE INTO {} (id, data) VALUES (?1, ?2)",
            T::table_name()
        );
        let conn = self.conn.lock().unwrap();
        conn.execute(&sql, params![entity.id(), data])?;
        Ok(())
    }

    /// 根据 ID 查询
    pub fn find_by_id(&self, id: i64) -> Result<Option<T>> {
        let sql = format!("SELECT data FROM {} WHERE id = ?1", T::table_name());
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let data: String = row.get(0)?;
            let entity = serde_json::from_str(&data)?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    /// 查询所有实体
    pub fn find_all(&self) -> Result<Vec<T>> {
        let sql = format!("SELECT data FROM {}", T::table_name());
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;
        let mut entities = Vec::new();
        while let Some(row) = rows.next()? {
            let data: String = row.get(0)?;
            let entity = serde_json::from_str(&data)?; // anyhow::Error 可以承载 serde_json::Error
            entities.push(entity);
        }
        Ok(entities)
    }

    /// 删除指定 ID
    pub fn delete(&self, id: i64) -> Result<bool> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", T::table_name());
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute(&sql, params![id])?;
        Ok(affected > 0)
    }

    /// 清空表
    pub fn delete_all(&self) -> Result<()> {
        let sql = format!("DELETE FROM {}", T::table_name());
        let conn = self.conn.lock().unwrap();
        conn.execute(&sql, [])?;
        Ok(())
    }
}
