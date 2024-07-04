use std::io;
use std::io::Write;

// Define a constant for the board size
const BOARD_SIZE: usize = 3;

// Enum to represent players
#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

// Struct to represent the game
struct TicTacToe {
    board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
}

impl TicTacToe {
    // Initialize a new game
    fn new() -> Self {
        Self {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::X,
        }
    }

    // Display the game board
    fn display_board(&self) {
        println!("-------------");
        for row in &self.board {
            print!("|");
            for &cell in row {
                match cell {
                    Some(Player::X) => print!(" X |"),
                    Some(Player::O) => print!(" O |"),
                    None => print!("   |"),
                }
            }
            println!("\n-------------");
        }
    }

    // Check if a player has won
    fn check_win(&self, player: Player) -> bool {
        let b = &self.board;
        // Check rows and columns
        for i in 0..BOARD_SIZE {
            if (0..BOARD_SIZE).all(|j| b[i][j] == Some(player)) || (0..BOARD_SIZE).all(|j| b[j][i] == Some(player)) {
                return true;
            }
        }
        // Check diagonals
        if (0..BOARD_SIZE).all(|i| b[i][i] == Some(player)) || (0..BOARD_SIZE).all(|i| b[i][BOARD_SIZE - 1 - i] == Some(player)) {
            return true;
        }
        false
    }

    // Place a marker on the board
    fn make_move(&mut self, row: usize, col: usize) -> Result<(), String> {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Err("Invalid move: Out of bounds.".to_string());
        }
        if self.board[row][col].is_some() {
            return Err("Invalid move: Cell already taken.".to_string());
        }
        self.board[row][col] = Some(self.current_player);
        Ok(())
    }

    // Switch turns between players
    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    // Run the game loop
    fn run(&mut self) {
        loop {
            self.display_board();
            println!("Player {:?}'s turn.", self.current_player);
            println!("Enter your move (row and column, 0-indexed):");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input.");
            let mut parts = input.trim().split_whitespace();
            
            let row: usize = match parts.next() {
                Some(value) => value.parse().unwrap(),
                None => {
                    println!("Invalid input. Please enter row and column numbers.");
                    continue;
                }
            };

            let col: usize = match parts.next() {
                Some(value) => value.parse().unwrap(),
                None => {
                    println!("Invalid input. Please enter row and column numbers.");
                    continue;
                }
            };

            match self.make_move(row, col) {
                Ok(_) => {
                    if self.check_win(self.current_player) {
                        self.display_board();
                        println!("Player {:?} wins!", self.current_player);
                        break;
                    }
                    self.switch_player();
                }
                Err(err) => println!("{}", err),
            }
        }
    }
}

fn main() {
    println!("Welcome to Tic Tac Toe!");

    let mut game = TicTacToe::new();
    game.run();
}
