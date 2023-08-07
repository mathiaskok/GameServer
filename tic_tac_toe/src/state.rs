use game_interface::ResultWithInput;

use super::board::Board;
pub use super::board::Color;
pub use super::board::Idx;

#[derive(Clone, Copy, Debug,)]
pub enum Error { InvalidColor, IllegalPly, GameOver }

#[derive(Debug)]
pub struct State {
  board: Board,
  current: Color
}

impl ToString for State {
  fn to_string(&self) -> String {
    self.current().to_string() +
      "\n-----\n" + 
      &self.board.to_string()
  }
}

impl State {
  pub fn new(len: usize) -> State {
    State {
      current: Color::Cross,
      board: Board::new(len)
    }
  }

  pub fn current(&self) -> Color {
    self.current
  }

  pub fn winner(&self) -> Option<Color> {
    self.board.winner()
  }

  pub fn complete(&self) -> bool {
    !self.board.free_cells()
  }

  pub fn ply(mut self, pos: Idx, color: Color) -> ResultWithInput<Self, Error> {
    if self.current != color {
      return ResultWithInput::from(self).with(Error::InvalidColor)
    }
    
    match self.board.winner() {
      Option::Some(_) => ResultWithInput::from(self).with(Error::GameOver),
      Option::None => {
        match self.board.get_mut(pos) {
          Option::None => ResultWithInput::from(self).with(Error::IllegalPly),
          Option::Some(Option::Some(_)) => ResultWithInput::from(self).with(Error::IllegalPly),
          Option::Some(cell) => {
            *cell = Option::Some(color);
            self.current = !color;
            ResultWithInput::from(self)
          }
        }
      }
    }
  }
}