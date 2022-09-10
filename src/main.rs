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

fn get_move() -> &'static str {
    let mut string_form = String::new();
    io::stdin()
            .read_line(&mut string_form)
            .expect("Failed to read input");
    return "x";
}

fn redo_board(move_l: &str, move_p: char) -> Vec<char> {
    let mut y_offset: i32 = 0;
    let mut x_offset: i32 = 0;
    let mut peice: i32 = 0;
    for character in move_l.chars() {
        match character {
            'a' => (),
            'b' => y_offset = 8,
        }
    }
    
    return [].to_vec()
}

fn main() {
    let mut board: Vec<char> = ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜',
                            '♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟',
                            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                            '♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙',
                            '♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'].to_vec();
    let mut turn: char = 'W';
    // * Game loop
    let term = Term::stdout();
    match term.clear_screen() {
        Ok(_) => (),
        Err(_) => (),
    };
    while 1==1 {
        draw_board(board.clone());
        let move_l = get_move();
        board = redo_board(move_l, turn);
        match term.clear_screen() {
            Ok(_) => (),
            Err(_) => (),
        };
        if turn == 'W' {
            turn = 'B';
        } else {
            turn = 'W';
        }
    }
}
