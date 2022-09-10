/** Learning Rust
 * * Goal: Try to program a chess game
 * ? Maybe add an ai as well?
 */
use std::io;
use console::Term;


fn draw_board(board: Vec<char>) {
    println!("-----------------------------------------");
    let mut count: i32 = 0;
    for piece in board {
        print!("| {}  ", piece);
        count += 1;
        if count == 8 {
            println!("|\n-----------------------------------------");
            count = 0;
        }
    }
}

fn _get_move() -> &'static str {
    let mut string_form = String::new();
    io::stdin()
            .read_line(&mut string_form)
            .expect("Failed to read input");
    return "x";
}

fn main() {
    let board: Vec<char> = ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜',
                            '♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟',
                            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                            '♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙',
                            '♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'].to_vec();
    // * Game loop
    let term = Term::stdout();
    term.clear_screen();
    while 1==1 {
        draw_board(board.clone());
        let move_l = _get_move();
        term.clear_screen();
    }
}
