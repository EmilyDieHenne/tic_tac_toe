use std::fmt;
use std::io;

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Clone, Copy, Debug)]
enum TileState {
    X,
    O,
    Empty,
}
impl fmt::Display for TileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TileState::X => write!("x"),
        }
    }
}
#[derive(PartialEq)]
enum GameState {
    TurnX,
    TurnO,
    GameOver,
}
fn main() {
    start_game();
}

fn start_game() {
    let current_difficulty: Difficulty = get_difficulty();
    match current_difficulty {
        Difficulty::Easy => print!("isy"),
        Difficulty::Medium => print!("mödium"),
        Difficulty::Hard => print!("haerd"),
    }
    let mut board = vec![TileState::Empty; 9];
    let mut current_game_state = GameState::TurnX;

    while (current_game_state != GameState::GameOver) {
        print_board(&board);
    }
}
fn print_board(board: &Vec<TileState>) {
    println!("{}{}{}\n{}{}{}\n{}{}{}\n", board[0])
}

fn get_difficulty() -> Difficulty {
    let mut input: String = String::new();
    println!("Please select your difficulty! (Easy, Medium, Hard) ");
    io::stdin().read_line(&mut input).expect("Mist");

    return match input.trim().to_lowercase().as_str() {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        "hard" => Difficulty::Hard,
        &_ => get_difficulty(),
    };
}
