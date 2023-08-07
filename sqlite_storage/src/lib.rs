mod user;
mod config;
mod game;
mod player;
mod ply;

use rusqlite::{Connection, Result};

pub struct SqliteConnection {
  conn: Connection
}

impl SqliteConnection {
  pub fn open_connection(path: &str) -> Result<SqliteConnection> {
    let conn = Connection::open(path)?;
    Ok(SqliteConnection {
      conn
    })
  }

  fn with_transaction<R>(&mut self, f: fn(&Connection) -> Result<R>) -> Result<R> {
    let trans = self.conn.transaction()?;
    let result = f(&trans)?;
    trans.commit()?;
    Ok(result)
  }

  pub fn create_tables(&mut self) -> Result<()> {
    self.with_transaction(|c| {
      user::Handler::from(c).create_table()?;
      config::Handler::from(c).create_table()?;
      game::Handler::from(c).create_table()?;
      player::Handler::from(c).create_table()?;
      ply::Handler::from(c).create_table()?;
      Ok(())
    })
  }
}
