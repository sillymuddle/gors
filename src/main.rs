//============================================================================
// Go playing bot
//============================================================================

use std::fmt;
use std::ops::Index;
use std::string::ToString;


//----------------------------------------------------------------------------
// Communication with Game Engine
//----------------------------------------------------------------------------
//
// This should run in a separate thread (or even two), listening for commands
// from the server, and replying with moves. In this way the engine can
// do its thing, and not have to poll all the time.
//
//----------------------------------------------------------------------------



//----------------------------------------------------------------------------
// Players
//----------------------------------------------------------------------------

trait Player 
{
	fn get_move(&self, board: &Board) -> Move;
}


struct ConsolePlayer {
	name: String
}

impl Player for ConsolePlayer {

	fn get_move(&self, board: &Board) -> Move
	{
		println!("Your turn {}.\n", self.name);
		println!("{}", board);
		
		Move::Pass
	}
}

impl ConsolePlayer {

	fn new(name: String) -> ConsolePlayer
	{
		ConsolePlayer{name: name}
	}
}


//----------------------------------------------------------------------------
// Board
//----------------------------------------------------------------------------

const SIDE: usize = 9;

#[derive(Copy, PartialEq, Clone)]
enum Piece {
	NONE = 0,
	BLACK = 1,
	WHITE = 2
}


impl ToString for Piece {
	fn to_string(&self) -> String {
		match *self {
			Piece::NONE => ".".to_string(),
			Piece::BLACK => "X".to_string(),
			Piece::WHITE => "O".to_string()
		}
	}
}


struct Board
{
	piece: [[Piece; SIDE]; SIDE],
	next_move: Piece
}


impl Default for Board {
	fn default() -> Board {
	   Board { piece: [[Piece::NONE; SIDE]; SIDE],
		 	 			 next_move: Piece::BLACK }
		 
	}
}


impl Index<(u32, u32)> for Board {
  type Output = Piece;

  fn index(&self, _index: (u32, u32)) -> &Piece {
		&self.piece[_index.0 as usize][_index.1 as usize]
	}
}


// Display
impl fmt::Display for Board {	
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		
		// Header
		write!(f, "    ");
		for col_number in 0..SIDE {
			write!(f, "{:2}", col_number);
		}
		write!(f, "\n");
		
		// Body
		for (row_number, row) in self.piece.iter().enumerate() {
			write!(f, "{:3}: ", row_number);
			for piece in row.iter() {
      	write!(f, "{} ", (*piece).to_string());
			}
			write!(f, "\n");
		}
		write!(f, "\n     Next move: {}\n\n", ((self.next_move).to_string()) )
	}
}
	
	
enum Move {
	Pass,
  Place(u8, u8)
}




impl Board
{
	fn is_valid_move(&self, mv: &Move) -> bool {
		match *mv {
			Move::Pass => true,
			Move::Place(row, col) => self.piece[row as usize][col as usize] == Piece::NONE
		}
	}
	
	fn make_move(&mut self, mv: &Move)  {
		if self.is_valid_move(mv) {
		
			match *mv {
				Move::Pass => {},
				Move::Place(row, col) => {
					self.piece[row as usize][col as usize] = self.next_move;
				}
			}
			
			self.next_move = match self.next_move {
				Piece::WHITE => Piece::BLACK,
				Piece::BLACK => Piece::WHITE,
				Piece::NONE  => panic!("Either black or white should play next.")
			}
			
		}
	}

}


//----------------------------------------------------------------------------
// Main
//----------------------------------------------------------------------------

fn main() {
  println!("\nGo Rust (v0.1)\n==============\n");
	let mut board = Board::default();
	println!("{}", board);
	
	board.piece[0][0] = Piece::BLACK;
	board.piece[1][1] = Piece::WHITE;
	
	println!("{}", board);
	
	board.make_move(&Move::Place(3,3));
	
	println!("{}", board);
	
	let player = ConsolePlayer::new("Alice".to_string());
	let mv = player.get_move(&board);
	
}
