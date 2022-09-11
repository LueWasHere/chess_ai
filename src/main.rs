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

fn get_move() -> Vec<String> {
    let mut move_1 = String::new();
    let mut move_2 = String::new();
    println!("Peice to move: ");
    io::stdin()
            .read_line(&mut move_1)
            .expect("Failed to read input");
    println!("Move to: ");
    io::stdin()
            .read_line(&mut move_2)
            .expect("Failed to read input");
    return [move_1, move_2].to_vec();
}

fn redo_board(move_l: Vec<String>, mut board: Vec<char>) -> Vec<char> {
    let mut y_offset: usize = 0;
    let mut x_offset: usize = 0;
    let peice: char;
    for character in move_l[0].chars() {
        match character {
            'a' => y_offset = 56,
            'b' => y_offset = 48,
            'c' => y_offset = 40,
            'd' => y_offset = 32,
            'e' => y_offset = 24,
            'f' => y_offset = 16,
            'g' => y_offset = 8,
            'h' => (),
            '1' => x_offset = 7,
            '2' => x_offset = 6,
            '3' => x_offset = 5,
            '4' => x_offset = 4,
            '5' => x_offset = 3,
            '6' => x_offset = 2,
            '7' => x_offset = 1,
            '8' => (),
            _ => ()
        }
    }
    peice = board[x_offset.clone()+y_offset.clone()];
    board[x_offset.clone()+y_offset.clone()] = ' ';
    for character in move_l[1].chars() {
        match character {
            'a' => y_offset = 56,
            'b' => y_offset = 48,
            'c' => y_offset = 40,
            'd' => y_offset = 32,
            'e' => y_offset = 24,
            'f' => y_offset = 16,
            'g' => y_offset = 8,
            'h' => (),
            '1' => x_offset = 7,
            '2' => x_offset = 6,
            '3' => x_offset = 5,
            '4' => x_offset = 4,
            '5' => x_offset = 3,
            '6' => x_offset = 2,
            '7' => x_offset = 1,
            '8' => (),
            _ => ()
        }
    }
    board[x_offset.clone()+y_offset.clone()] = peice;
    return board;
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
    // * Game loop
    let term = Term::stdout();
    match term.clear_screen() {
        Ok(_) => (),
        Err(_) => (),
    };
    while 1==1 {
        draw_board(board.clone());
        let move_l = get_move();
        board = redo_board(move_l, board);
        match term.clear_screen() {
            Ok(_) => (),
            Err(_) => (),
        };
    }
}
