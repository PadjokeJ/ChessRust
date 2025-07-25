use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::{keyboard::Keycode, render::Canvas};

extern crate sdl2;

mod vectors;

fn main() {
    let res: (u32, u32) = (640, 480);

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem
        .window("Template Game", res.0, res.1)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

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


            canvas.present();

            let frame_delay = 1.0 / max_fps;
            delta_time = dt.elapsed().as_secs_f32();

            if frame_delay - delta_time > 0.0 {
                thread::sleep(Duration::from_secs_f32(frame_delay - delta_time));
            }
        }
    }
}