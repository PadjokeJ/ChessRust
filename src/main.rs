use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::{keyboard::Keycode};
use sdl2::image::LoadTexture;
use sdl2::image::InitFlag;

use std::cmp::min;

use vectors::v2::V2;

use crate::chess::{checkmate, file_of, index_of, is_white, rank_of, Pieces};

extern crate sdl2;

mod vectors;
mod fen;
mod chess;

mod bot;

fn main() {
    let debug_bitboard = false;

    let bot_playing = false;

    let mut board: Vec<i8> = fen::translate_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

    let mut en_passant_index: usize = 999;
    let mut will_end = false;

    let res: (u32, u32) = (640, 640);

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window: Window = video_subsystem
        .window("Chess", res.0, res.1)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    
    let b_pieces = vec![
        texture_creator.load_texture("src/images/b_pawn.png").ok().unwrap(),
        texture_creator.load_texture("src/images/b_bishop.png").ok().unwrap(),
        texture_creator.load_texture("src/images/b_knight.png").ok().unwrap(),
        texture_creator.load_texture("src/images/b_rook.png").ok().unwrap(),
        texture_creator.load_texture("src/images/b_queen.png").ok().unwrap(),
        texture_creator.load_texture("src/images/b_king.png").ok().unwrap()
    ];
    let w_pieces = vec![
        texture_creator.load_texture("src/images/w_pawn.png").ok().unwrap(),
        texture_creator.load_texture("src/images/w_bishop.png").ok().unwrap(),
        texture_creator.load_texture("src/images/w_knight.png").ok().unwrap(),
        texture_creator.load_texture("src/images/w_rook.png").ok().unwrap(),
        texture_creator.load_texture("src/images/w_queen.png").ok().unwrap(),
        texture_creator.load_texture("src/images/w_king.png").ok().unwrap()
    ];

    let promotion_texture = texture_creator.load_texture("src/images/promotion.png").ok().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut mouse_coords = V2::zero();
    let mut pick_up: bool = false;
    let mut release: bool = false;
    let mut hand: i8 = 0;
    let mut original_index: usize = 0;

    let mut is_white_turn = true;
    let mut pawn_promoting = false;
    let mut promotion_index = 0;
    let promotion_choices = [Pieces::BISH as i8, Pieces::KNIG as i8, Pieces::ROOK as i8, Pieces::QUEE as i8];
    let mut promoted_into;
    let mut promotion_box_x = 0;
    let mut promotion_box_y = 0;


    let mut legal_piece_moves: Vec<usize> = Vec::new();
    let mut bitboard: u64 = 0;

    let mut delta_time: f32;
    let max_fps = 60.0;

    'main: loop {
        let dt = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::MouseMotion { x, y, .. } => {
                    mouse_coords.x = x as f32;
                    mouse_coords.y = y as f32;
                }
                Event::MouseButtonDown { .. } => {
                    pick_up = !pawn_promoting; // makes sure you can't pickup a piece when you're promoting
                }
                Event::MouseButtonUp { .. } => {
                    if pawn_promoting {
                        if mouse_coords.x as i32 >= promotion_box_x
                        && mouse_coords.x as i32 <= promotion_box_x + 320
                        && mouse_coords.y as i32 >= promotion_box_y + 35
                        && mouse_coords.y as i32 <= promotion_box_y + 120 {
                            let choice_index = (mouse_coords.x as i32 - promotion_box_x) / 80;
                            promoted_into = promotion_choices[choice_index as usize];

                            board[promotion_index] = promoted_into + if !is_white_turn { 8 } else { 0 };
                            pawn_promoting = false;
                        }
                    } else {
                        release = true;
                    }
                }
                _ => (),
            }
        }
        if bot_playing && !is_white_turn && !pawn_promoting {
            let (index, move_index) = bot::turn(is_white_turn, &board, bitboard, en_passant_index);

            board[move_index] = board[index];
            board[index] = 0;
            is_white_turn = true;

            if checkmate(is_white_turn, &board, bitboard, en_passant_index) {
                println!("Checkmate !");
                will_end = true;
            }
        }

        if pick_up {
            let x = mouse_coords.x as i32 / 80;
            let y = mouse_coords.y as i32 / 80;

            let index = (y * 8 + x) as usize;
            original_index = index;
            hand = board[index];
            board[index] = 0;

            if hand != 0 {
                bitboard = chess::generate_bit_board(&board, is_white_turn);
                legal_piece_moves = chess::generate_legal_moves(hand, original_index as i32, &board, bitboard, en_passant_index);
            }

            pick_up = false;
        }
        if release && hand != 0 {                
            let x = mouse_coords.x as i32 / 80;
            let y = mouse_coords.y as i32 / 80;

            if chess::in_bounds(x, y) {
                let index = chess::index_of(x, y) as usize;

                let legal = is_white_turn == is_white(hand) && legal_piece_moves.contains(&index);

                if legal 
                && index != original_index{
                    if hand & 7 == Pieces::PAWN as i8 {
                        if index == en_passant_index {
                            board[index_of(file_of(en_passant_index), rank_of(original_index))] = 0;
                        }
                        en_passant_index = 999;
                        if is_white(hand) && rank_of(original_index) - 2 == rank_of(index) {
                            en_passant_index = original_index - 8;
                        } else if !is_white(hand) && rank_of(original_index) + 2 == rank_of(index) {
                            en_passant_index = original_index + 8;
                        }

                        if rank_of(index) == 0 || rank_of(index) == 7 {
                            promotion_index = index;
                            pawn_promoting = true;
                        }
                    }
                    board[index] = hand;
                    hand = 0;
                    is_white_turn = !is_white_turn;

                    if checkmate(is_white_turn, &board, bitboard, en_passant_index) {
                        println!("Checkmate !");

                        will_end = true;
                    }
                } else {
                    board[original_index] = hand;
                    hand = 0;
                }

                legal_piece_moves.clear();

                release = false;
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for i in 0..64 {
            let r = Rect::new(
                i % 8 * 80,
                i / 8 * 80,
                80,
                80
            );

            let mut color = if i % 2 + (i / 8) % 2 == 1 { Color::RGB(78, 73, 95) } 
                else { Color::RGB(246, 214, 189) };
            
            if 2u64.pow(i as u32) & bitboard != 0 && debug_bitboard {
                color = Color::RGB(255, color.g / 4, color.b / 4);
            }

            canvas.set_draw_color(color);

            _ = canvas.fill_rect(r);
        }

        for i in 0..64 {
            if board[i] != 0 {
                let texture;

                if chess::is_white(board[i]){
                    let index = board[i] as usize - 9;
                    texture = &w_pieces[index];
                } else {
                    let index = board[i] as usize - 1;
                    texture = &b_pieces[index];
                }

                let attributes = texture.query();
                let src_rect = Rect::new(0, 0, attributes.width, attributes.height);

                let dest_rect = Rect::new(i as i32 % 8 * 80, i as i32 / 8 * 80, 80, 80);

                _ = canvas.copy(texture, src_rect, dest_rect);
            }
        }

        for i in 0..64 {
            if legal_piece_moves.contains(&(i as usize)) {
                let legal_indicator = Rect::new(file_of(i as usize) * 80 + 30, rank_of(i as usize) * 80 + 30, 20, 20);
                let outline = Rect::new(file_of(i as usize) * 80 + 25, rank_of(i as usize) * 80 + 25, 30, 30);

                canvas.set_draw_color(Color::RGB(8, 20, 30));
                _ = canvas.fill_rect(outline);
                canvas.set_draw_color(Color::RGB(153, 117, 119));
                _ = canvas.fill_rect(legal_indicator);
            }
        }

        if hand != 0 {
            let texture;

            if chess::is_white(hand){
                let index = hand as usize - 9;
                texture = &w_pieces[index];
            } else {
                let index = hand as usize - 1;
                texture = &b_pieces[index];
            }

            let attributes = texture.query();
            let src_rect = Rect::new(0, 0, attributes.width, attributes.height);

            let dest_rect = Rect::new(mouse_coords.x as i32 - 48, mouse_coords.y as i32 - 48, 96, 96);

            _ = canvas.copy(texture, src_rect, dest_rect);
        }
        if pawn_promoting {
            let attributes = promotion_texture.query();
            let src_rect = Rect::new(0, 0, attributes.width, attributes.height);
            promotion_box_x = min(file_of(promotion_index) * 80, 310);
            promotion_box_y = min(rank_of(promotion_index) * 80, 520);
            let dest_rect = Rect::new(promotion_box_x - 5, promotion_box_y, 330, 120);

            _ = canvas.copy(&promotion_texture, src_rect, dest_rect);

            for i in 0..4 {
                let texture = if !is_white_turn { &w_pieces[promotion_choices[i] as usize - 1] } else { &b_pieces[promotion_choices[i] as usize - 1] };

                let attributes = texture.query();
                let src = Rect::new(0, 0, attributes.width, attributes.height);
                let dst = Rect::new(dest_rect.x + 5 + i as i32 * 80, dest_rect.y + 35, attributes.width * 5, attributes.height * 5);

                _ = canvas.copy(texture, src, dst);
            }
        }

        canvas.present();

        if will_end {
            break 'main;
        }

        let frame_delay = 1.0 / max_fps;
        delta_time = dt.elapsed().as_secs_f32();

        if frame_delay - delta_time > 0.0 {
            thread::sleep(Duration::from_secs_f32(frame_delay - delta_time));
        }
        
    }
    let checkmate_texture = texture_creator.load_texture("src/images/checkmate.png").ok().unwrap();
    if !will_end {
        return;
    }
    'gameover: loop {
        let dt = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'gameover,
                _ => (),
            }
        }

        let checkmate_attributes = checkmate_texture.query();
        let src = Rect::new(0, 0, checkmate_attributes.width, checkmate_attributes.height);
        let dest = Rect::new(145, 245, 350, 130);

        _ = canvas.copy(&checkmate_texture, src, dest);

        let texture = if !is_white_turn { &w_pieces[Pieces::KING as usize - 1] } else { &b_pieces[Pieces::KING as usize - 1] };

        let attributes = texture.query();
        let src = Rect::new(0, 0, attributes.width, attributes.height);
        let dst = Rect::new(280, 285, 80, 80);

        _ = canvas.copy(texture, src, dst);
        

        canvas.present();

        let frame_delay = 1.0 / max_fps;
        delta_time = dt.elapsed().as_secs_f32();

        if frame_delay - delta_time > 0.0 {
            thread::sleep(Duration::from_secs_f32(frame_delay - delta_time));
        }
    }
}

