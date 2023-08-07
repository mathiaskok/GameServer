#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlyError {
  IllegalPly,
  InvalidUser, 
  UnknownUser,
  GameOver
}