use rusqlite::{Connection, Result};

mod sql {
  pub const CREATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXSITS Players
(
  game_id INTEGER NOT NULL,
  user_id Integer NOT NULL,
  game_player TEXT NOT NULL,
  PRIMARY KEY(game_id, user_id),
  FOREIGN KEY(game_id) REFERENCES Games(id),
  FOREIGN KEY(user_id) REFERENCES Users(id),
);  
  "#;

  pub const CREATE_PLAYER: &str = r#"
INSERT INTO Players (game_id, user_id, game_player)
VALUES (?1, ?2, ?3);
  "#;

  pub const GET_PLAYERS_FOR_GAME: &str = r#"
SELECT user_id, game_player
FROM Players
WHERE game_id = ?1;
  "#;
}

pub struct DBO {
  pub game_id: u64,
  pub user_id: u64,
  pub game_player: String
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

  pub fn create_player(&self, game_id: u64, user_id: u64, game_player: &str) -> Result<()> {
    self.execute(sql::CREATE_PLAYER, (game_id, user_id, game_player))?;
    Ok(())
  }

  pub fn get_players_for_game(&self, game_id: u64) -> Result<Vec<DBO>> {
    let mut stm = self.prepare(sql::GET_PLAYERS_FOR_GAME)?;
    let players = stm.query_map([game_id], |row|
      Ok (
        DBO {
          game_id, 
          user_id: row.get(0)?,
          game_player: row.get(1)?
        }))?;
    
    players.collect()
  }
}