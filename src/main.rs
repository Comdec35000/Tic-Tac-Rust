
#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
  x: i8,
  y: i8
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellState {
  X,
  O,
  None
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameState {
  Run,
  XWon,
  OWon,
  Tie
}

fn main() {

  // Setup
  let mut grid = [[CellState::None; 3]; 3];
  let mut state = GameState::Run;
  let mut turn = CellState::X;

  // Mainloop
  while state == GameState::Run {

    // Render
    print!("\n\n");
    draw(grid);

    match turn {
      CellState::X => println!("\n\n{}", "It's X's turn !"),
      CellState::O => println!("\n\n{}", "It's O's turn !"),
      _ => println!("\n\n{}\n", "An error oucurred..."),
    }

    // Handle input
    let mut coo: Coordinate;
    
    loop {
      let input = get_input_coord();
        
      if input.is_none() {
        print!("{}", "Invalid input, try again...");
        continue;
      }
      coo = input.unwrap();
      if grid[coo.x as usize][coo.y as usize] != CellState::None {
        print!("{}", "Cell alredy used !");
        continue;
      }
      break;
    }

    // Place the cell
    grid[coo.x as usize][coo.y as usize] = turn;
    // Switch turn
    if turn == CellState::X {
      turn = CellState::O;
    } else {
      turn = CellState::X;
    }

    // Check if someone winned
    state = check_win(grid);
  }

  match state {
    GameState::XWon => {
      println!("{}", "\n\nX Won ! Congratulations !");
    },
    GameState::OWon => {
      println!("{}", "\n\nO Won ! Congratulations !");
    }
    GameState::Tie => {
      println!("{}", "\n\nTie ! Try an other game !");
    }
    _ => {
      println!("{}", "An Error occured");
    }
  }
  
}


pub fn draw(grid: [[CellState; 3]; 3]) {
  print!("{esc}c", esc = 27 as char);
  println!("   1  2  3");
  let mut rowid = 0;
  for row in &grid {
    print!("{} ", char::from_u32(rowid + 65).unwrap());
    
    for cell in row.iter() {
      let symbol: &str;

      match cell {
        CellState::X => symbol = "X",
        CellState::O => symbol = "O",
        _ => symbol = ".",
      }
      
      print!("|{}|", symbol);
    }

    rowid += 1;
    print!("\n");
  }
}


pub fn handle_input() -> String {

    use std::io::{stdin,stdout,Write};

    let mut s=String::new();
    print!("\nWhere to play ? : ");

    let _=stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return s;
}


pub fn get_input_coord() -> Option<Coordinate> {
  let input = handle_input();
  
  let y = (input.chars().nth(0).unwrap() as i8) - ('A' as i8);

  let x = (input.chars().nth(1).unwrap() as i8) - ('1' as i8);

  if y > 2 || y < 0 || x > 2 || x < 0 { return None; }
  
  return Some(Coordinate {x: y, y: x});
}


pub fn check_win(grid: [[CellState; 3]; 3]) -> GameState {

    for x in 0..3 {
      for y in 0..3 {
        for direction in [
          Coordinate { x: 1, y: 0 },
          Coordinate { x: 0, y: 1 },
          Coordinate { x: -1, y: 1 },
          Coordinate { x: 1, y: 1 },
        ] {

          let state = check_into(
            Coordinate{x: x, y: y}, 
            direction,
            grid,
            0i8,
            grid[x as usize][y as usize]
          );

          if state != GameState::Run  { return state }
        }
      } 
    }

    return GameState::Run;
}


pub fn check_into(current_coo: Coordinate, vec: Coordinate, grid: [[CellState; 3]; 3], count: i8, state: CellState) -> GameState {
  if state == CellState::None { return GameState::Run };

  let target_coo = Coordinate { x: current_coo.x + vec.x, y: current_coo.y + vec.y };

  if target_coo.x < 0 || target_coo.x > 2 || target_coo.y < 0 || target_coo.y > 2 { return GameState::Run };
  let target_state = grid[target_coo.x as usize][target_coo.y as usize];

  if target_state != state { return GameState::Run };

  if count == 1 {
    match state {
      CellState::X => { return GameState::XWon },
      CellState::O => { return  GameState::OWon },
      _ => { return GameState::Run }
    }

  };

  return check_into(target_coo, vec, grid, count + 1, state);
}
