// start 30 may 2020 at 11:06 and finished at 15:24 same day

extern crate rand;

use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
	let mut array: [char; 10] = ['n', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
	let mut current_player = get_starting_player(); // 0 = O and 1 = X

	print_board(&array);
	loop {
		let input = get_user_input(current_player);

		// fill the tile
		let success = fill_tile_with(&mut array, input, current_player);
		if !success { continue;	}
		print_board(&array);

		if check_win(&array, 'X') == true {
			print!("\t  Player X won\n");
			break
		} else if check_win(&array, 'O') == true {
			print!("\t  Player O won\n");
			break;
		} else { // draw
			if array[1] != ' ' && array[2] != ' ' && array[3] != ' ' && array[4] != ' ' &&
				array[5] != ' ' && array[6] != ' ' && array[7] != ' ' && array[8] != ' ' && array[9] != ' ' {
				print!("\t\t  Draw\n");
				break;
			}
		}
		// swap players
		current_player = if current_player == 0 { 1 } else { 0 };
	}
}

fn fill_tile_with(tile: &mut [char; 10], input: u8, current_player: u8) -> bool {
	//if tile is empty then if player is X fill tile with X else if player is O then fill tile with O
	if input == 1 && tile[1] == ' ' { if current_player == 1 { tile[1] = 'X'; } else if current_player == 0 { tile[1] = 'O'; }}
	else if input == 2 && tile[2] == ' ' { if current_player == 1 { tile[2] = 'X'; } else if current_player == 0 { tile[2] = 'O'; }}
	else if input == 3 && tile[3] == ' ' { if current_player == 1 { tile[3] = 'X'; } else if current_player == 0 { tile[3] = 'O'; }}
	else if input == 4 && tile[4] == ' ' { if current_player == 1 { tile[4] = 'X'; } else if current_player == 0 { tile[4] = 'O'; }}
	else if input == 5 && tile[5] == ' ' { if current_player == 1 { tile[5] = 'X'; } else if current_player == 0 { tile[5] = 'O'; }}
	else if input == 6 && tile[6] == ' ' { if current_player == 1 { tile[6] = 'X'; } else if current_player == 0 { tile[6] = 'O'; }}
	else if input == 7 && tile[7] == ' ' { if current_player == 1 { tile[7] = 'X'; } else if current_player == 0 { tile[7] = 'O'; }}
	else if input == 8 && tile[8] == ' ' { if current_player == 1 { tile[8] = 'X'; } else if current_player == 0 { tile[8] = 'O'; }}
	else if input == 9 && tile[9] == ' ' { if current_player == 1 { tile[9] = 'X'; } else if current_player == 0 { tile[9] = 'O'; }}
	else { return false }
	return true;
}

fn check_win(array: &[char; 10], player: char) -> bool {
	// vertical lines
	if array[7] == player && array[4] == player && array[1] == player { return true; }
	else if array[8] == player && array[5] == player && array[2] == player { return true; }
	else if array[9] == player && array[6] == player && array[3] == player { return true; }
	// horizontal lines
	else if array[7] == player && array[8] == player && array[9] == player { return true; }
	else if array[4] == player && array[5] == player && array[6] == player { return true; }
	else if array[1] == player && array[2] == player && array[3] == player { return true; }
	//diagonal lines
	else if array[7] == player && array[5] == player && array[3] == player { return true; }
	else if array[9] == player && array[5] == player && array[1] == player { return true; }
	else { return false; }
}

fn get_starting_player() -> u8 {
	let mut rng = rand::thread_rng();
	return rng.gen_range(0, 2);
}

fn get_user_input(current_player: u8) -> u8 {
	if current_player == 0 { print!("\tPlayer O turn: "); } else { print!("\tPlayer X turn: ");	}
	io::stdout().flush();
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("error: unable to read user input");
	return input.trim().parse().expect("please give me correct string number!");
}

fn print_board(array: &[char; 10]) {
	clear_screen();
	print!("\n\n\t    TicTacToe\n\n");
	print!("\t     |     |     \n");
	print!("\t  {}  |  {}  |  {}  \n", array[7], array[8], array[9]);
	print!("\t_____|_____|_____\n");
	print!("\t     |     |     \n");
	print!("\t  {}  |  {}  |  {}  \n", array[4], array[5], array[6]);
	print!("\t_____|_____|_____\n");
	print!("\t     |     |     \n");
	print!("\t  {}  |  {}  |  {}  \n", array[1], array[2], array[3]);
	print!("\t     |     |     \n\n");
}

fn clear_screen() {
	print!("\x1B[2J"); // clears the screen, I think?
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // position cursor at 1,1
}