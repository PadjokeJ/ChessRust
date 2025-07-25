use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::{keyboard::Keycode, render::Canvas};
use sdl2::image::LoadTexture;
use sdl2::image::InitFlag;

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

    let mut input_vec: (bool, bool, bool, bool) = (false, false, false, false);

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
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => input_vec.0 = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => input_vec.1 = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => input_vec.2 = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => input_vec.3 = true,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => input_vec.0 = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => input_vec.1 = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => input_vec.2 = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => input_vec.3 = false,

                _ => (),
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
            canvas.present();

            let frame_delay = 1.0 / max_fps;
            delta_time = dt.elapsed().as_secs_f32();

            if frame_delay - delta_time > 0.0 {
                thread::sleep(Duration::from_secs_f32(frame_delay - delta_time));
            }
        }
    }
}