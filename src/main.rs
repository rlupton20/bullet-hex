extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;

use std::time::Duration;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem.window("renderer", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    'render: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return,
                _ => {}
            }

            canvas.clear();
            canvas.present();

            ::std::thread::sleep(Duration::new(0, 1000000000));
        }
    }
}
