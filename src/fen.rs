pub fn letter_to_piece(symbol: char) -> i8 {
    let is_white = symbol.is_ascii_uppercase();

    let pieces = "pbnrqk";

    let piece = pieces.chars().position(|i| i == symbol.to_ascii_lowercase()).unwrap_or(0) as i8 + 1;

    if is_white {
        return piece + 8;
    }
    piece
}

pub fn translate_fen(fen: String) -> Vec<i8> {
    let mut board: Vec<i8> = Vec::with_capacity(64);
    for _ in 0..64 {
        board.push(0);
    }
    let mut i: usize = 0;

    for c in fen.chars() {
        if c != '/' {
            if c.is_numeric() {
                i += (c.to_digit(10).unwrap_or(1) - 1) as usize;
            } else {
                board[i] = letter_to_piece(c);
            }
            i += 1;
        }
        
    }

    board
}