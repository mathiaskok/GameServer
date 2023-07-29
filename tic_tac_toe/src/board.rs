mod rows_iterator;
mod columns_iterator;
mod diagonals_iterator;

pub use super::color::*;
pub use rows_iterator::*;
pub use columns_iterator::*;
pub use diagonals_iterator::*;

pub type Cell = Option<Color>;
pub type Idx = (usize,usize);

#[derive(Debug)]
pub struct Board {
  rows: Vec<Vec<Cell>>
}

impl std::ops::Index<Idx> for Board {
  type Output = Cell;

  fn index(&self, index: Idx) -> &Self::Output {
    self.get(index).unwrap()
  }
}

impl std::ops::IndexMut<Idx> for Board {
  fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
    self.get_mut(index).unwrap()
  }
}

impl Board {
  pub fn new(len: usize) -> Self {
    Board {
      rows: vec![vec![Option::None; len]; len]
    }
  }

  pub fn get(&self, index: Idx) -> Option<&Cell> {
    self.rows.get(index.0).and_then(|r| r.get(index.1))
  }

  pub fn get_mut(&mut self, index: Idx) -> Option<&mut Cell> {
    self.rows.get_mut(index.0).and_then(|r| r.get_mut(index.1))
  }

  pub fn len(&self) -> usize {
    self.rows.len()
  }

  fn rows(&self) -> RowsIterator {
    RowsIterator::new(self)
  }

  fn cols(&self) -> ColumnsIterator {
    ColumnsIterator::new(self)
  }

  fn left_diagonal(&self) -> LeftDiagonalIterator {
    LeftDiagonalIterator::new(self)
  }

  fn right_diagonal(&self) -> RightDiagonalIterator {
    RightDiagonalIterator::new(self)
  }

  fn line_winner<'a, Iter>(line: & 'a mut Iter) -> Cell 
  where Iter: Iterator<Item=& 'a Cell> {
    let fst = line.next().unwrap();

    for next in line{
      if *next != *fst {
        return Option::None
      }
    }
    *fst
  }

  pub fn winner(&self) -> Cell {
    for mut row in self.rows() {
      if let win@Option::Some(_) = Board::line_winner(&mut row) {
        return win
      }
    }

    for mut col in self.cols() {
      if let win@Option::Some(_) = Board::line_winner(&mut col) {
        return win
      }
    }

    if let win@Option::Some(_) = Board::line_winner(&mut self.left_diagonal()) {
      return win
    }

    if let win@Option::Some(_) = Board::line_winner(&mut self.right_diagonal()) {
      return win
    }

    Option::None
  }
}