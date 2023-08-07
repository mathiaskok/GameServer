use rusqlite::{Connection, Result};

mod sql {
  pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXSITS Games
(
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  type TEXT NOT NULL,
  config_id INTEGER NOT NULL,
  current_state TEXT NOT NULL,
  FOREIGN KEY(config_id) REFERENCES Configurations(id)
);  
  "#;

  pub const CREATE_GAME: &str = r#"
INSERT INTO Games (type, config_id, current_state)
VALUES (?1, ?2, ?3);
  "#;

  pub const GET_GAME: &str = r#"
SELECT type, config_id, current_state
FROM Games
WHERE id = ?1;
  "#;
}

pub struct DBO {
  pub id: u64,
  pub game_type: String,
  pub config_id: u64,
  pub current_state: String
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

  pub fn create_game(&self, game_type: &str, config_id: u64, current_state: &str) -> Result<()> {
    self.execute(sql::CREATE_GAME, (game_type, config_id, current_state))?;
    Ok(())
  }

  pub fn get_game(&self, id: u64) -> Result<DBO> {
    let mut stm = self.prepare(sql::GET_GAME)?;
    stm.query_row([id], |row|
      Ok (
        DBO {
          id, 
          game_type: row.get(0)?,
          config_id: row.get(1)?,
          current_state: row.get(2)?
        }))
  }
}