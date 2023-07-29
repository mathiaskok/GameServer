#[derive(Debug, Clone, Copy)]
pub enum GameState<Player> {
  Ongoing(Player),
  Winner(Player),
  Draw
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlyError {
  IllegalPly,
  InvalidUser, 
  UnknownUser,
  GameOver
}

pub trait Game<'state, 'ply, 'player> {
  type State;
  type Ply;
  type Player;

  fn game_state(&self, state: Self::State) -> GameState<Self::Player>;

  fn ply(&self, state: Self::State, ply: Self::Ply, player: Self::Player) -> Result<Self::State, PlyError>;
}