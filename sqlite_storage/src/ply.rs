use rusqlite::{Connection, Result};

mod sql {
  pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXSITS Plies
(
  game_id INTEGER NOT NULL,
  user_id Integer NOT NULL,
  ply_number INTEGER NOT NULL,
  ply TEXT NOT NULL
);  
  "#;

  pub const CREATE_PLY: &str = r#"
INSERT INTO Plies (game_id, user_id, ply_number, ply)
VALUES (?1, ?2, (SELECT MAX(ply_number) + 1 FROM Plies WHERE game_id = ?1), ?3);
  "#;

  pub const GET_GAME_PLIES: &str = r#"
SELECT user_id, ply_number, ply
FROM Plies
WHERE game_id = ?1
ORDER BY ply_number ASC;
  "#;
}

pub struct DBO {
  pub game_id: u64,
  pub user_id: u64,
  pub ply_number: u64,
  pub ply: String
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

  pub fn create_ply(&self, game_id: &str, ply: &str) -> Result<()> {
    self.execute(sql::CREATE_PLY, [game_id, ply])?;
    Ok(())
  }

  pub fn get_game_plies(&self, game_id: u64) -> Result<DBO> {
    let mut stm = self.prepare(sql::GET_GAME_PLIES)?;
    stm.query_row([game_id], |row|
      Ok (
        DBO {
          game_id,
          user_id: row.get(0)?,
          ply_number: row.get(1)?,
          ply: row.get(2)?
        }))
  }
}