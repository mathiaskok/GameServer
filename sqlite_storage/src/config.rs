use rusqlite::{Connection, Result};

mod sql {
  pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXSITS Configurations
(
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  config TEXT NOT NULL
);  
  "#;

  pub const CREATE_CONFIG: &str = r#"
INSERT INTO Configurations (config)
VALUES (?1);
  "#;

  pub const GET_CONFIG: &str = r#"
SELECT config
FROM Configurations
WHERE id = ?1;
  "#;
}

pub struct DBO {
  pub id: u64,
  pub config: String
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

impl <'a> Handler<'a> {
  pub fn create_table(&self) -> Result<()> {
    self.execute(sql::CREATE_TABLE,())?;
    Ok(())
  }

  pub fn create_game(&self, config: &str) -> Result<()> {
    self.execute(sql::CREATE_CONFIG, [config])?;
    Ok(())
  }

  pub fn get_game(&self, id: u64) -> Result<DBO> {
    let mut stm = self.prepare(sql::GET_CONFIG)?;
    stm.query_row([id], |row|
      Ok (
        DBO {
          id, 
          config: row.get(0)?
        }))
  }
}