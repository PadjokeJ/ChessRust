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

use vectors::v2::V2;

use crate::chess::{file_of, is_white, rank_of};

extern crate sdl2;

mod vectors;
mod fen;
mod chess;

fn main() {    
    let mut board: Vec<i8> = fen::translate_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

    println!("{:?}", board);


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

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut mouse_coords = V2::zero();
    let mut pick_up: bool = false;
    let mut release: bool = false;
    let mut hand: i8 = 0;
    let mut original_index: usize = 0;

    let mut is_white_turn = true;

    let mut legal_piece_moves: Vec<usize> = Vec::new();

    let mut delta_time: f32 = 0f32;
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
                    pick_up = true;
                }
                Event::MouseButtonUp { .. } => {
                    release = true;
                }
                _ => (),
            }
            if pick_up {
                let x = mouse_coords.x as i32 / 80;
                let y = mouse_coords.y as i32 / 80;

                let index = (y * 8 + x) as usize;
                original_index = index;
                hand = board[index];
                board[index] = 0;

                if hand != 0 {
                    legal_piece_moves = chess::generate_pseudolegal_moves(hand, original_index as i32, &board, false);
                }

                pick_up = false;
            }
            if release && hand != 0 {                
                let x = mouse_coords.x as i32 / 80;
                let y = mouse_coords.y as i32 / 80;

                if chess::in_bounds(x, y) {
                    let index = chess::index_of(x, y) as usize;
                    let two_pow_index = 2u64.pow(index as u32);
                    println!("index: {:?}, twopow : {:?}", index, two_pow_index);

                    let mut legal = is_white_turn == is_white(hand) && legal_piece_moves.contains(&index);
                    
                    if hand & 7 == chess::Pieces::KING as i8 
                    && (two_pow_index & chess::generate_bit_board(&board, is_white_turn) == two_pow_index){
                        legal = false;
                        print!("did bitboard run, ");
                    }

                    if legal 
                    && index != original_index{
                        board[index] = hand;
                        hand = 0;
                        is_white_turn = !is_white_turn;
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
                if i % 2 + (i / 8) % 2 == 1 {
                    canvas.set_draw_color(Color::RGB(78, 73, 95));
                } else {
                    canvas.set_draw_color(Color::RGB(246, 214, 189));
                }
                
                _ = canvas.fill_rect(r);
            }

            for i in 0..64 {
                if board[i] != 0 {
                    let mut texture;

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
                let mut texture;

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

            canvas.present();

            let frame_delay = 1.0 / max_fps;
            delta_time = dt.elapsed().as_secs_f32();

            if frame_delay - delta_time > 0.0 {
                thread::sleep(Duration::from_secs_f32(frame_delay - delta_time));
            }
        }
    }
}