
pub struct Board {
  width :u8,
  height :u8,
}

fn main() {
    let a_board = Board {
        width: 8,
        height: 8
    };
    println!("Initialized a board of {} squares", area(&a_board))
}

fn area(board: &Board) -> u8 {
    board.width * board.height
}
