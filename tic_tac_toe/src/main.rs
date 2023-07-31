use std::io::{Write};

use game_definition::*;
use tic_tac_toe::game as tic_tac_toe;

struct Console {
  stdin: std::io::Stdin,
  stdout: std::io::Stderr, 
  buffer: String
}

impl Console {
  fn new() -> Self{
    Console { 
      stdin: std::io::stdin(), 
      stdout: std::io::stderr(), 
      buffer: String::new() }
  }

  fn read_line(&mut self) -> &str {
    self.buffer.clear();
    self.stdin.read_line(&mut self.buffer).unwrap();
    &self.buffer.trim()
  }

  fn write_all(&self, str: &str) {
    let mut lock = self.stdout.lock();
    lock.write_all("----------\n".as_bytes()).unwrap();
    lock.write_all(str.as_bytes()).unwrap();
    lock.write_all("\n".as_bytes()).unwrap();
    lock.write_all("----------\n".as_bytes()).unwrap();
  }
}

fn get_config(console: &mut Console) -> usize {
  let line = console.read_line();
  match line.trim().parse() {
    Result::Ok(len) if len > 1 => len,
    _ => {
      console.write_all("Invalid length. Try again.");
      get_config(console)
    }
  }
}

fn get_ply(console: &mut Console) -> (usize,usize) {
  let line = console.read_line().trim();
  let split: Vec<_> = line.split_ascii_whitespace().map(|s| s.parse()).collect();
  match split.as_slice() {
    [Result::Ok(row),Result::Ok(col)] => (*row,*col),
    _ => {
      console.write_all("Invalid ply. Try again.");
      get_ply(console)
    }
  }
}

fn main() {
  let mut console = Console::new();
  
  console.write_all("Please specify how large the game should be.");
  let len = get_config(&mut console);

  console.write_all("Please specify the name of X");
  let cross = String::from(console.read_line());

  console.write_all("Please specify the name of O");
  let nought = String::from(console.read_line());

  let game = tic_tac_toe::Game::new(&nought, &cross);
  let mut state = game.initialize(&len);
  loop {
    match game.game_state(&state) {
      GameState::Draw => {
        console.write_all("The game was a draw.");
        break;
      },
      GameState::Winner(winner) => {
        console.write_all(&format!("The winner was '{}'", winner));
        break;
      },
      GameState::Ongoing(player) => {
        console.write_all(&state.to_string());
        let ply_result = game.ply(state, get_ply(&mut console), player);
        state = match ply_result.error() {
          Option::None => ply_result.into_result(),
          Option::Some(PlyError::IllegalPly) => {
            console.write_all("Invalid ply. Try again.");
            ply_result.into_result()
          }
          _ => panic!("Invalid result")
        }
      }
    }
  }
}