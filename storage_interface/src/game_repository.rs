use std::io::Result;

pub trait GameRepository {
  type Game;
  type Config;
  type Player;
  type Ply;
  type State;

  fn get_players(&self, game: &Self::Game) -> Result<Vec<Self::Player>>;
  fn get_config(&self, game: &Self::Game) -> Result<Self::Config>;
  fn get_last_state(&self, game: &Self::Game) -> Result<Self::State>;
  fn get_plies(&self, game: &Self::Game) -> Result<Vec<Self::Ply>>;

  fn create_game(&self, game: &Self::Game, config: &Self::Config, players: &[Self::Player]) -> Result<()>;
  fn set_last_state(&self, game: &Self::Game, state: Self::State) -> Result<()>;
  fn add_ply(&self, game: &Self::Game, ply: &Self::Ply) -> Result<()>;
}