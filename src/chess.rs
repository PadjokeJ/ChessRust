use std::collections::HashMap;

pub enum Pieces {
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

pub fn xy_dir(direction: i32) -> i32 {
    direction.abs() / direction
}

pub fn is_different_color(piece1: i8, piece2: i8) -> bool {
    is_white(piece1) != is_white(piece2)
}

pub fn is_sliding_piece(piece_type: i8) -> bool {
    piece_type == Pieces::BISH as i8
    || piece_type == Pieces::QUEE as i8
    || piece_type == Pieces::ROOK as i8
}

pub fn is_white(piece: i8) -> bool {
    piece & 8 == 8
}

pub fn file_of(index: usize) -> i32 {
    let i = index as i32;

    i % 8
}

pub fn rank_of(index: usize) -> i32 {
    let i = index as i32;

    i / 8
}

pub fn index_of(file: i32, rank: i32) -> usize {
    (8 * rank + file) as usize
}

pub fn in_bounds(file: i32, rank: i32) -> bool {
    file < 8 && file >= 0 && rank < 8 && rank >= 0
}

pub fn is_empty(board: &Vec<i8>, file: i32, rank: i32) -> bool {
    in_bounds(file, rank) && board[index_of(file, rank)] == 0
}

pub fn is_enemy(board: &Vec<i8>, white: bool, file: i32, rank: i32) -> bool {
    in_bounds(file, rank)
    && !is_empty(board, file, rank)
    && is_white(board[index_of(file, rank)]) != white
}

pub fn is_empty_or_enemy(board: Vec<i8>, white: bool, file: i32, rank: i32) -> bool {
    is_empty(&board, file, rank) 
    || is_enemy(&board, white, file, rank)
}

pub fn push_if_inbounds(legal_moves: &mut Vec<usize>, file: i32, rank: i32) {
    if in_bounds(file, rank) {
        legal_moves.push(index_of(file.clone(), rank.clone()));
    }
}

pub fn generate_pseudolegal_moves(piece: i8, starting_index: i32, board: &Vec<i8>, is_bit_board_calc: bool) -> Vec<usize> {
    let mut legal_moves: Vec<usize> = Vec::new();
    let piece_type = piece & 7;
    let piece_is_white = is_white(piece);

    if piece_type == Pieces::PAWN as i8 {
        let mut starting_rank = 1;
        let mut dir = Directions::DOWN as i32;
        if piece_is_white {
            starting_rank = 6;
            dir = Directions::UP as i32;
        }
        let x = file_of(starting_index as usize);
        let y = rank_of(starting_index as usize);
        let mut y_move = y + xy_dir(dir);

        if is_empty(board, x, y_move) && !is_bit_board_calc {
            push_if_inbounds(&mut legal_moves, x, y_move);

            if starting_rank == y && is_empty(board, x, y + 2 * xy_dir(dir)){
                y_move = y + (2 * xy_dir(dir));
                push_if_inbounds(&mut legal_moves, x, y_move);
            }
        }
        for x_dir in vec![-1, 1] {
            if is_enemy(board, piece_is_white, x + x_dir, y + xy_dir(dir)) || is_bit_board_calc {
                push_if_inbounds(&mut legal_moves, x + x_dir, y + xy_dir(dir));
            }
        }
    }

    if is_sliding_piece(piece_type) {
        let mut slide_dir: Vec<i32> = Vec::new();

        if piece_type == Pieces::BISH as i8 || piece_type == Pieces::QUEE as i8 {
            slide_dir.push(Directions::UP as i32 + Directions::RIGHT as i32);
            slide_dir.push(Directions::UP as i32 + Directions::LEFT as i32);
            slide_dir.push(Directions::DOWN as i32 + Directions::RIGHT as i32);
            slide_dir.push(Directions::DOWN as i32 + Directions::LEFT as i32);
        }
        if piece_type == Pieces::ROOK as i8 || piece_type == Pieces::QUEE as i8 {
            slide_dir.push(Directions::UP as i32);
            slide_dir.push(Directions::LEFT as i32);
            slide_dir.push(Directions::RIGHT as i32);
            slide_dir.push(Directions::DOWN as i32);
        }

        let starting_rank = rank_of(starting_index as usize);
        let starting_file = file_of(starting_index as usize);

        for dir in slide_dir {
            let dir_rank = if dir.abs() == 1 { 0 } else if dir < -6 { -1 } else { 1 };
            let dir_file = if dir.abs() == 8 { 0 } else if dir.abs() == 7 || dir == -1 { -1 } else { 1 };
            let mut rank = dir_rank + starting_rank;
            let mut file = dir_file + starting_file;

            'slide: loop {
                if !in_bounds(file, rank) {
                    break;
                }
                if is_empty(board, file, rank) {
                    legal_moves.push(index_of(file, rank));
                } else {
                    if is_enemy(board, piece_is_white, file, rank) || is_bit_board_calc {
                        legal_moves.push(index_of(file, rank));
                    }
                    break 'slide;
                }
                rank += dir_rank;
                file += dir_file;
            }
        }
    }

    if piece_type == Pieces::KNIG as i8 {
        let hops_rank = vec![-2, -1, 1, 2];
        let hops_file = vec![-1, -2, -2, -1];

        let start_rank = rank_of(starting_index as usize);
        let start_file = file_of(starting_index as usize);

        let mut rank = start_rank;
        let mut file = start_file;

        for i in [-1, 1] {
            for k in 0..4 {
                rank = start_rank + hops_rank[k];
                file = start_file + hops_file[k] * i;

                if is_empty_or_enemy(board.to_vec(), piece_is_white, file, rank) {
                    push_if_inbounds(&mut legal_moves, file, rank);
                }
            }
        }
    }

    if piece_type == Pieces::KING as i8 {
        let dirs = [
            Directions::UP as i32,
            Directions::LEFT as i32,
            Directions::DOWN as i32,
            Directions::RIGHT as i32,
            Directions::UP as i32 + Directions::LEFT as i32,
            Directions::UP as i32 + Directions::RIGHT as i32,
            Directions::DOWN as i32 + Directions::LEFT as i32,
            Directions::DOWN as i32 + Directions::RIGHT as i32
        ];
        
        for dir in dirs {
            let dir_rank = if dir.abs() == 1 { 0 } else if dir < -6 { -1 } else { 1 };
            let dir_file = if dir.abs() == 8 { 0 } else if dir.abs() == 7 || dir == -1 { -1 } else { 1 };
            let rank = dir_rank + rank_of(starting_index as usize);
            let file = dir_file + file_of(starting_index as usize);

            if is_empty_or_enemy(board.to_vec(), piece_is_white, file, rank) {
                push_if_inbounds(&mut legal_moves, file, rank);
            } else if is_bit_board_calc && !is_empty_or_enemy(board.to_vec(), piece_is_white, file, rank) {
                push_if_inbounds(&mut legal_moves, file, rank);
            }
        }
    }
    if !is_bit_board_calc {
        println!("legal indexes: {:?}", legal_moves);
    }
    legal_moves
}

pub fn generate_bit_board(board: &Vec<i8>, is_white_turn: bool) -> u64 {
    let is_white_bit_board = !is_white_turn;
    let mut bitboard: u64 = 0;

    let mut i = 0;
    for piece in board.to_vec() {
        if piece != 0 && is_white(piece) == is_white_bit_board {
            for attacked_square in generate_pseudolegal_moves(piece, i, &board.to_vec(), true) {
                bitboard |= 2u64.pow(attacked_square as u32);
            }
        }
        i += 1;
    }

    println!("bitboard : {:#b}", bitboard);

    bitboard
}

pub fn generate_legal_moves(piece: i8, starting_index: i32, board: &Vec<i8>, bitboard: u64) -> Vec<usize> {
    let mut pseudo_legal_moves = generate_pseudolegal_moves(piece, starting_index, board, false);

    let king_index = board.iter()
        .position( |id| 
            id & 7 == Pieces::KING as i8 
            && !is_different_color(*id, piece))
        .unwrap_or(999); // if no king on board then it is in hand
    
    println!("king index : {:?}", king_index);

    if king_index != 999 && 2u64.pow(king_index as u32) & bitboard != 0 {
        pseudo_legal_moves.clear();
        println!("check in : {:#b}", 2u64.pow(king_index as u32) & bitboard);
        return pseudo_legal_moves;
    }

    if king_index == 999 {
        let mut i = 0;
        'checker: loop {
            let current_move = pseudo_legal_moves[i];

            if 2u64.pow(current_move as u32) & bitboard == 0 {
                pseudo_legal_moves.remove(i);
            } else {
                i += 1;
            }
            if i >= pseudo_legal_moves.len() || pseudo_legal_moves.len() == 0{
                break 'checker;
            }
        }
    }
    
    pseudo_legal_moves
}