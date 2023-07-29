use game_definition::{GameState, PlyError};

use super::state::*;

type Player<'player> = & 'player str;

#[derive(Debug)]
pub struct Game<'player> {
  nought: Player<'player>,
  cross: Player<'player>
}

fn transform_state_error(error: Error) -> PlyError {
  match error {
    Error::GameOver => PlyError::GameOver,
    Error::IllegalPly => PlyError::IllegalPly,
    Error::InvalidColor => PlyError::InvalidUser
  }
}

impl<'player> Game<'player> {
  pub fn new(nought: Player<'player>, cross:Player<'player>) -> Self {
    Game {nought,cross}
  }

  fn color_to_player(&self, color: Color) -> Player<'player> {
    match color {
      Color::Nought => self.nought,
      Color::Cross => self.cross
    }
  }

  fn player_to_color(&self, player: Player<'player>) -> Option<Color> {
    if player == self.nought {
      Option::Some(Color::Nought)
    }
    else if player == self.cross {
      Option::Some(Color::Cross)
    }
    else {
      Option::None
    }
  }
}

impl<'state, 'ply, 'player, 'config> game_definition::Game<'state, 'ply, 'player, 'config> for Game<'player> {
  type State = State;
  type Ply = (usize,usize);
  type Player = Player<'player>;
  type Config = usize;

  fn initialize(&self, config: &Self::Config) -> Self::State {
    State::new(*config)
  }

  fn game_state(&self, state: Self::State) -> game_definition::GameState<Self::Player> {
    match state.winner() {
      Option::None => GameState::Ongoing(self.color_to_player(state.current())),
      Option::Some(Color::Nought) => GameState::Winner(self.nought),
      Option::Some(Color::Cross) => GameState::Winner(self.cross),
    }
  }

  fn ply(&self, state: Self::State, ply: Self::Ply, player: Self::Player) -> Result<Self::State, game_definition::PlyError> {
    match self.player_to_color(player) {
      Option::None => Result::Err(PlyError::UnknownUser),
      Option::Some(color) => state.ply(ply, color).map_err(transform_state_error)
    }
  }
}