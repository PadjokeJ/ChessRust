use std::f32::DIGITS;

use sdl2::sys::LineOnOffDash;

enum Pieces {
    PAWN = 1,
    BISH = 2,
    KNIG = 3,
    ROOK = 4,
    QUEE = 5,
    KING = 6,
}

enum Directions {
    UP = -8,
    DOWN = 8,
    LEFT = -1,
    RIGHT = 1,
}

pub fn is_white(piece: i8) -> bool {
    piece & 8 == 8
}

pub fn file_of(index: usize) -> i32{
    let i = index as i32;

    i % 8
}

pub fn rank_of(index: usize) -> i32{
    let i = index as i32;

    i / 8
}

pub fn generate_legal_moves(piece: i8, starting_index: i32, board: &Vec<i8>) -> Vec<usize> {
    let mut legal_moves: Vec<usize> = Vec::new();
    let piece_type = piece & 7;
    println!("piece: {:?}", piece);
    println!("type: {:?}", piece_type);

    if piece_type == Pieces::PAWN as i8 {
        let mut starting_rank = 1;
        let mut dir = Directions::DOWN as i32;
        if is_white(piece) {
            starting_rank = 6;
            dir = Directions::UP as i32;
        }
        let mut index = (starting_index + dir) as usize;
        if board[index] == 0 {
            legal_moves.push(index);

            index = (starting_index + dir * 2) as usize;
            if rank_of(starting_index as usize) == starting_rank && board[index] == 0 {
                legal_moves.push(index);
            }
        }
        index = (starting_index + dir + 1) as usize;
        if rank_of(index) == rank_of((starting_index + dir) as usize) && board[index] != 0 && is_white(piece) != is_white(board[index]) {
            legal_moves.push(index);
        }
        index = (starting_index + dir - 1) as usize;
        if rank_of(index) == rank_of((starting_index + dir) as usize) && board[index] != 0 && is_white(piece) != is_white(board[index]) {
            legal_moves.push(index);
        }
    }
    println!("legal indexes: {:?}", legal_moves);
    legal_moves
}

pub fn is_legal(piece: i8, starting_index: usize, ending_index: usize, board: &Vec<i8>) -> bool {
    generate_legal_moves(piece, starting_index as i32, board).contains(&ending_index)
}