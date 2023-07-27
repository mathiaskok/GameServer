pub mod color;
pub mod board;
pub mod game;

pub mod tic_tac_toe {
}



#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub enum Color { Nought, Cross }

#[derive(Debug)]
pub struct Board {
  cells: Vec<Vec<Option<Color>>>
}

impl std::ops::Index<(usize,usize)> for Board {
  type Output = Option<Color>;

  fn index(&self, index:(usize,usize)) -> &Self::Output{
    let (row,col) = index;
    &self.cells[row][col]
  }
}

impl std::ops::IndexMut<(usize,usize)> for Board {
  fn index_mut(&mut self, index:(usize,usize)) -> &mut Self::Output{
    let (row,col) = index;
    &mut self.cells[row][col]
  }
}

impl Board {
  pub fn empty(length:usize) -> Self {
    Board {
      cells: vec![vec![Option::None; length]; length]
    }
  }

  pub fn len(&self) -> usize {
    self.cells.len()
  }

  fn rows(&self) {
    
  }
}