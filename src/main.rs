mod winsdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::{thread, time};
use winsdl::Winsdl;

fn main() {
    let mut winsdl: Winsdl = Winsdl::new(800, 800).unwrap();

    winsdl.canvas.set_draw_color(Color::RGB(0, 255, 255));
    winsdl.canvas.clear();
    winsdl.canvas.present();

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        winsdl.render_color(Color::RGB(255, 255, 0));
        thread::sleep(time::Duration::from_secs(1));
    }
}
