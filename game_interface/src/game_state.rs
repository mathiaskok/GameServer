#[derive(Debug, Clone, Copy)]
pub enum GameState<Player> {
  Ongoing(Player),
  Winner(Player),
  Draw
}

impl<Player> GameState<Player> {
  pub fn map<B, F>(self, map: F) -> GameState<B>
  where F: FnOnce(Player) -> B {
    match self {
      GameState::Ongoing(p) => GameState::Ongoing(map(p)),
      GameState::Winner(p) => GameState::Winner(map(p)),
      GameState::Draw => GameState::Draw
    }
  }
}