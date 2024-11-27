mod dice_roller;
struct Board {
  width :u8,
  height :u8,
}

fn initialize_custom_board(width: u8, height: u8) -> Board {
    Board {
        width,
        height
    }
}
fn initialize_default_board() -> Board {
    Board {
        width: 8,
        height: 8
    }
}

fn main() {
    let a_board = initialize_custom_board(10,4);
    let b_board = initialize_default_board();
    println!("Initialized a default board of {} squares", area(&a_board));
    println!("Initialized a custom board of {} squares", area(&b_board));
}

fn area(board: &Board) -> u8 {
    board.width * board.height
}
