use rusqlite::{Connection, Result};

mod sql {
  pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXSITS Users
(
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS user_name_idx
ON Users(name);
"#;

//Perhaps add unique index to name
  pub const CREATE_USER: &str = r#"
INSERT INTO Users (name)
VALUES (?1);
  "#;

  pub const GET_USER_FROM_ID: &str = r#"
SELECT name 
FROM user
WHERE id = ?1
  "#;

  pub const GET_USER_FROM_NAME: &str = r#"
  SELECT id
  FROM user
  WHERE name = ?1;
    "#;
}

pub struct DBO {
  pub id: u64,
  pub name: String
}

#[derive(Debug, Clone, Copy)]
pub struct Handler<'a> {
  conn: & 'a Connection
}

impl<'a> From<& 'a Connection> for Handler<'a> {
  fn from(value: & 'a Connection) -> Self {
    Handler { conn: value }
  }
}

impl<'a> std::ops::Deref for Handler<'a> {
  type Target=Connection;

  fn deref(&self) -> &Self::Target {
    self.conn
  }
}

impl<'a> Handler<'a> {
  pub fn create_table(&self) -> Result<()> {
    self.execute(sql::CREATE_TABLE,())?;
    Ok(())
  }
  
  pub fn create_user(&self, name: &str) -> Result<()> {
    self.execute(sql::CREATE_USER, [name])?;
    Ok(())
  }

  pub fn get_user_from_id(&self, id: u64) -> Result<DBO> {
    let mut stm = self.prepare(sql::GET_USER_FROM_ID)?;
    stm.query_row([id], |row|
      Ok (
        DBO {
          id, 
          name: row.get(0)?
        }))
  }

  pub fn get_user_from_name(&self, name: &str) -> Result<DBO> {
    let mut stm = self.prepare(sql::GET_USER_FROM_NAME)?;
    stm.query_row([name], |row|
      Ok (
        DBO {
          id: row.get(0)?, 
          name: name.to_string()
        }))
  }
}