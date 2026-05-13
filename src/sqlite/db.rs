use anyhow::Result;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Mutex;
/// 实体必须能提供表名和它的唯一 ID
pub trait Entity: Serialize + for<'de> Deserialize<'de> + Clone + Send + 'static {
    fn table_name() -> &'static str;
    fn id(&self) -> &str;
}
pub struct Repository<T: Entity> {
    conn: Mutex<Connection>,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub id: String,
    pub value: String,
    pub name: String,
    pub command_type: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
pub trait FromRow: Sized {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self>;
}

impl FromRow for Config {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Config {
            id: row.get("id")?,
            value: row.get("value")?,
            name: row.get("name")?,
            command_type: row.get("command_type")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}

impl<T: FromRow + Entity> Repository<T> {
    /// 打开或创建数据库，并确保表存在
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open("db/".to_string() + db_name)?;
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                        id TEXT PRIMARY KEY,
                        value TEXT NOT NULL,
                        name TEXT NOT NULL,
                        command_type TEXT NOT NULL,
                        created_at TEXT NOT NULL,
                        updated_at TEXT NOT NULL
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
    pub fn save_config(&self, entity: &Config) -> Result<()> {
        let id = entity.id.as_str();
        let value = entity.value.as_str();
        let name = entity.name.as_str();
        let command_type = entity.command_type.as_str();

        // 当前时间戳（秒）
        let now = chrono::Utc::now().to_string();

        // created_at：如果实体已有则保留，否则用当前时间（新记录）
        let created_at = chrono::Utc::now().to_string();
        // updated_at：总是更新为当前时间
        let updated_at = now;
        let sql = format!(
            "INSERT INTO {} (id, value, name, command_type, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(id) DO UPDATE SET
                     value = excluded.value,
                     name = excluded.name,
                     command_type = excluded.command_type,
                     updated_at = excluded.updated_at",
            T::table_name()
        );

        let conn = self.conn.lock().unwrap();
        conn.execute(
            &sql,
            params![id, value, name, command_type, created_at, updated_at],
        )?;
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
        let sql = format!("SELECT * FROM {}", T::table_name());
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| T::from_row(row))?;
        let mut entities = Vec::new();
        for entity in rows {
            entities.push(entity?);
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
