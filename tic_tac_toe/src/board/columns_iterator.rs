use super::{Cell, Board};

pub struct ColumnIterator<'board> {
  board: & 'board Board,
  col: usize,
  row: usize
}

impl<'board> ColumnIterator<'board> {
  fn new(board: & 'board Board, col: usize) -> Self {
    Self {
      board,
      col,
      row: 0
    }
  }
}

impl<'board> Iterator for ColumnIterator<'board> {
    type Item = & 'board Cell;

    fn next(&mut self) -> Option<Self::Item> {
      if self.row < self.board.len() {
        let r = self.row;
        self.row += 1;
        Option::Some(&self.board.rows[r][self.col])
      }
      else {
        Option::None
      }
    }
}

pub struct ColumnsIterator<'board> {
  board: & 'board Board,
  col: usize
}

impl<'board> ColumnsIterator<'board> {
    pub fn new(board: & 'board Board) -> Self { 
      Self { board, col: 0 } 
    }
}

impl<'board> Iterator for ColumnsIterator<'board> {
    type Item = ColumnIterator<'board>;

    fn next(&mut self) -> Option<Self::Item> {
      if self.col < self.board.len() {
        let col_iter = ColumnIterator::new(self.board, self.col);
        self.col += 1;
        Option::Some(col_iter)
      }
      else {
        Option::None
      }
    }
}