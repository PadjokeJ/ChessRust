use crate::chess::{self, generate_all_legal_moves};

pub fn turn(is_white_turn: bool, board: &Vec<i8>, bitboard: u64, en_passant_index: usize) -> (usize, usize) {
    let legal_moves = generate_all_legal_moves(is_white_turn, board, bitboard, en_passant_index);


    for piece in legal_moves.clone().into_keys() {
        let move_vec = legal_moves[&piece].clone();
        if move_vec.len() != 0 {
            return (piece, move_vec[0]);
        }
    }
    (0, 0)
}