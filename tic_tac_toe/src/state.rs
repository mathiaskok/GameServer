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

  pub fn ply(mut self, pos: Idx, color: Color) -> Result<Self, Error> {
    if self.current != color {
      return Result::Err(Error::InvalidColor)
    }
    
    match self.board.winner() {
      Option::Some(_) => Result::Err(Error::GameOver),
      Option::None => {
        match self.board.get_mut(pos) {
          Option::None => Result::Err(Error::IllegalPly),
          Option::Some(Option::Some(_)) => Result::Err(Error::IllegalPly),
          Option::Some(cell) => {
            *cell = Option::Some(color);
            self.current = !color;
            Result::Ok(self)
          }
        }
      }
    }
  }
}