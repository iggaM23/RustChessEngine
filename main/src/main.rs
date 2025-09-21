mod pieces_enum;

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut board = [' '; 128];

    fen_to_board(START_POS, &mut board);
    print_board(&board);
    println!();
    fen_to_board("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1", &mut board);
    print_board(&board);
    println!();
    fen_to_board("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2", &mut board);
    print_board(&board);
}

// fn on_board(cell: usize) -> bool {
//     return cell & 0x88 == 0;
// }

fn fen_to_board(fen: &str, board: &mut [char; 128])
{
    let mut parts = fen.split(" ");
    let mut row:usize = 0;
    let mut col:usize = 0;
    for c in parts.nth(0).unwrap().chars() {
        if c.is_numeric(){
            println!("{}", c as u32 - '0' as u32);
            for _ in 0..c as u32 - '0' as u32 {
                let idx = (row << 4) | col;
                board[idx] = ' ';
                col += 1;
            }
        }
        else {
            if c != '/' {
                let idx = (row << 4) | col;
                board[idx] = c;
                col += 1;
            }
            else {
                row += 1;
                col = 0;
            }
        }

    }
}

fn print_board(board: &[char; 128]) {
    for row in 0..8 {
        for col in 0..8 {
            let idx = (row << 4) | col;
            print!("{}  ", if board[idx] != ' ' { board[idx] } else if (row + col) % 2 == 0 { '□' } else { '■' });
            //print!("{}  ", board[idx]);
        }
        println!();
    }
}