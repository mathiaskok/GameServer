use super::{Cell, Board};

pub struct RowsIterator<'board> {
  board: & 'board Board,
  row: usize,
}

impl<'board> RowsIterator<'board> {
  pub fn new(board: & 'board Board) -> Self {
    Self {
      board,
      row: 0
    }
  }
}

impl<'board> Iterator for RowsIterator<'board> {
    //type Item = & 'board [Cell];
    type Item = std::slice::Iter<'board, Cell>;

    fn next(&mut self) -> Option<Self::Item> {
      if self.row < self.board.len() {
        let r = &self.board.rows[self.row];
        self.row += 1;
        Option::Some(r.iter())
      }
      else {
        Option::None
      }
    }
}