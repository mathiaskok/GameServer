#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color { Nought, Cross }

impl ToString for Color {
  fn to_string(&self) -> String {
    match self {
      Color::Nought => String::from("X"),
      Color::Cross => String::from("O")
    }
  }
}