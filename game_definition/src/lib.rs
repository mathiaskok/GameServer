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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlyError {
  IllegalPly,
  InvalidUser, 
  UnknownUser,
  GameOver
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResultWithInput<Result, Error>(Result, Option<Error>);

impl<Result, Error> ResultWithInput<Result, Error> {
  pub fn from(result: Result) -> Self {
    ResultWithInput(result, Option::None)
  }

  pub fn with(self, error: Error) -> Self {
    ResultWithInput(self.0, Option::Some(error))
  }

  pub fn result(&self) -> &Result {
    &self.0
  }

  pub fn into_result(self) -> Result {
    self.0
  }

  pub fn error(&self) -> &Option<Error> {
    &self.1
  }

  pub fn into_error(self) -> Option<Error> {
    self.1
  }

  pub fn map<B, F>(self, map: F) -> ResultWithInput<B, Error>
  where F: FnOnce(Result) -> B {
    ResultWithInput(map(self.0), self.1)
  }

  pub fn map_err<B, F>(self, map: F) -> ResultWithInput<Result, B>
  where F: FnOnce(Error) -> B {
    ResultWithInput(self.0, self.1.map(map))
  }
}

pub trait Game<'state, 'ply, 'player, 'config> {
  type State;
  type Ply;
  type Player;
  type Config;

  fn initialize(&self, config: &Self::Config) -> Self::State;

  fn game_state(&self, state: &Self::State) -> GameState<Self::Player>;

  fn ply(&self, state: Self::State, ply: Self::Ply, player: Self::Player) -> ResultWithInput<Self::State, PlyError>;
}