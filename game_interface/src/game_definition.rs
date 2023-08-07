use crate::{GameState, ResultWithInput, PlyError};

pub trait GameDefinition<'state, 'ply, 'player, 'config> {
  type State;
  type Ply;
  type Player;
  type Config;

  fn initialize(&self, config: &Self::Config) -> Self::State;

  fn game_state(&self, state: &Self::State) -> GameState<Self::Player>;

  fn ply(&self, state: Self::State, ply: Self::Ply, player: Self::Player) -> ResultWithInput<Self::State, PlyError>;
}