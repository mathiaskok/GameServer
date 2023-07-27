use super::{Cell, Board};

pub struct LeftDiagonalIterator<'board> {
  board: & 'board Board,
  i: usize
}

impl<'board> LeftDiagonalIterator<'board> {
    pub fn new(board: & 'board Board) -> Self { 
      Self { board, i: 0 } }
}

impl<'board> Iterator for LeftDiagonalIterator<'board> {
    type Item = & 'board Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.board.len() {
          let val = &self.board.rows[self.i][self.i];
          self.i += 1;
          Option::Some(val)
        }
        else {
          Option::None
        }
    }
}

pub struct RightDiagonalIterator<'board> {
  board: & 'board Board,
  i: usize
}

impl<'board> RightDiagonalIterator<'board> {
    pub fn new(board: & 'board Board) -> Self { 
      Self { board, i: 0 } }
}

impl<'board> Iterator for RightDiagonalIterator<'board> {
    type Item = & 'board Cell;

    fn next(&mut self) -> Option<Self::Item> {
      let len = self.board.len();
      if self.i < len {
        let val = &self.board.rows[self.i][len - 1 - self.i];
        self.i += 1;
        Option::Some(val)
      }
      else {
        Option::None
      }
    }
}