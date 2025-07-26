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

            if starting_rank == y {
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
    if piece_type == Pieces::BISH as i8 || piece_type == Pieces::QUEE as i8 {
        let mut dir_index = 0;
        let dirs = vec![
            Directions::UP as i32 + Directions::LEFT as i32, 
            Directions::UP as i32 + Directions::RIGHT as i32, 
            Directions::DOWN as i32 + Directions::LEFT as i32, 
            Directions::DOWN as i32 + Directions::RIGHT as i32];

        for dir in dirs {
            let mut i = 1;

            while file_of((starting_index + dir * i) as usize) == file_of(starting_index as usize) + i * ((dir_index % 2) * 2 - 1) && (starting_index + dir * i) >= 0 && (starting_index + dir * i) < 64 {
                let index = (starting_index + dir * i) as usize;
                if board[index] != 0 {
                    if is_different_color(piece, board[index]) {
                        legal_moves.push(index);
                    }
                    if !is_bit_board_calc && board[index] & 7 == Pieces::KING as i8{
                        break;
                    }
                }
                legal_moves.push(index);
                i += 1;
            }
            dir_index += 1;
        }
    }

    if piece_type == Pieces::ROOK as i8 || piece_type == Pieces::QUEE as i8 {
        for dir in vec![Directions::UP as i32, Directions::DOWN as i32] {
            let mut index = starting_index;
            while (0..8).contains(&rank_of(index as usize)) {
                index += dir;
                if index > 63 || index < 0 {
                    break;
                }
                if board[index as usize] != 0 {
                    if is_different_color(piece, board[index as usize]) {
                        legal_moves.push(index as usize);
                    }
                    if !is_bit_board_calc && board[index as usize] & 7 == Pieces::KING as i8{
                        break;
                    }
                }
                legal_moves.push(index as usize);
                
            }
        }
        for dir in vec![Directions::LEFT as i32, Directions::RIGHT as i32] {
            let mut index = starting_index + dir;
            while rank_of(index as usize) == rank_of(starting_index as usize) {
                if index < 0 || index >= 64 {
                    break;
                }
                if board[index as usize] != 0 {
                    if is_different_color(piece, board[index as usize]) {
                        legal_moves.push(index as usize);
                    }
                    if !is_bit_board_calc && board[index as usize] & 7 == Pieces::KING as i8{
                        break;
                    }
                }
                legal_moves.push(index as usize);
                index += dir;
            }
        }    
    }

    if piece_type == Pieces::KNIG as i8 {
        if file_of(starting_index as usize) > 0 {
            if rank_of(starting_index as usize) > 1 {
                let index = (Directions::UP as i32 * 2 + Directions::LEFT as i32 + starting_index) as usize;
                if board[index] == 0 || is_different_color(piece, board[index]) {
                    legal_moves.push(index);
                }
            }
            if rank_of(starting_index as usize) < 6 {
                let index = (Directions::DOWN as i32 * 2 + Directions::LEFT as i32 + starting_index) as usize;
                if board[index] == 0 || is_different_color(piece, board[index]) {
                    legal_moves.push(index);
                }
            }
            if file_of(starting_index as usize) > 1 {
                if rank_of(starting_index as usize) > 1 {
                    let index = (Directions::UP as i32 + Directions::LEFT as i32 * 2 + starting_index) as usize;
                    if board[index] == 0 || is_different_color(piece, board[index]) {
                        legal_moves.push(index);
                    }
                }
                if rank_of(starting_index as usize) < 6 {
                    let index = (Directions::DOWN as i32 + Directions::LEFT as i32 * 2 + starting_index) as usize;
                    if board[index] == 0 || is_different_color(piece, board[index]) {
                        legal_moves.push(index);
                    }
                }
            }
        }
        if file_of(starting_index as usize) < 7 {
            if rank_of(starting_index as usize) > 1 {
                let index = (Directions::UP as i32 * 2 + Directions::RIGHT as i32 + starting_index) as usize;
                if board[index] == 0 || is_different_color(piece, board[index]) {
                    legal_moves.push(index);
                }
            }
            if rank_of(starting_index as usize) < 6 {
                let index = (Directions::DOWN as i32 * 2 + Directions::RIGHT as i32 + starting_index) as usize;
                if board[index] == 0 || is_different_color(piece, board[index]) {
                    legal_moves.push(index);
                }
            }
            if file_of(starting_index as usize) < 6 {
                if rank_of(starting_index as usize) > 1 {
                    let index = (Directions::UP as i32 + Directions::RIGHT as i32 * 2 + starting_index) as usize;
                    if board[index] == 0 || is_different_color(piece, board[index]) {
                        legal_moves.push(index);
                    }
                }
                if rank_of(starting_index as usize) < 6 {
                    let index = (Directions::DOWN as i32 + Directions::RIGHT as i32 * 2 + starting_index) as usize;
                    if board[index] == 0 || is_different_color(piece, board[index]) {
                        legal_moves.push(index);
                    }
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
            let index = dir + starting_index;
            
            if (0..8).contains(&rank_of(index as usize)) 
            && (file_of(starting_index as usize) - file_of(index as usize)).abs() <= 1 
            && index >= 0{
                if board[index as usize] != 0 && is_different_color(piece, board[index as usize]) {
                    legal_moves.push(index as usize);
                }
                if board[index as usize] == 0 {
                    legal_moves.push(index as usize);
                }
            }
        }
    }
    if !is_bit_board_calc {
        println!("legal indexes: {:?}", legal_moves);
    }
    legal_moves
}

pub fn is_legal(piece: i8, starting_index: usize, ending_index: usize, board: &Vec<i8>) -> bool {
    generate_pseudolegal_moves(piece, starting_index as i32, board, false).contains(&ending_index)
}

pub fn generate_all_pseudolegal_moves(board: &Vec<i8>) -> HashMap<usize, Vec<usize>> {
    let mut legal_moves: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut index = 0;
    for piece in board {
        legal_moves.insert(index as usize, generate_pseudolegal_moves(*piece, index, board, false));
    }
    legal_moves
}

pub fn generate_all_colored_pseudolegal_moves(board: &Vec<i8>, is_white_moves: bool, is_bit_board_calc: bool) -> HashMap<usize, Vec<usize>>{
    let mut legal_moves: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut index = 0;
    for piece in board {
        if is_white_moves == is_white(*piece)
        {
            legal_moves.insert(index as usize, generate_pseudolegal_moves(*piece, index, board, is_bit_board_calc));
        }
    }
    legal_moves
}

pub fn generate_bit_board(board: &Vec<i8>, is_white_turn: bool) -> u64 {
    let is_white_bit_board = !is_white_turn;
    let all_legal_moves = generate_all_colored_pseudolegal_moves(board, is_white_bit_board, true);
    let iter_moves = all_legal_moves.clone();
    let mut bitboard: u64 = 0;

    for move_index in iter_moves.into_keys() {
        for moves in all_legal_moves[&move_index].clone() {
            bitboard += 2u64.pow(moves as u32);
            print!("{:?} : {:?}, ", moves, 2u64.pow(moves as u32));
        }
    }

    println!("bitboard : {:?}", bitboard);

    bitboard
}