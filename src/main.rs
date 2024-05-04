use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsystem = sdl_ctx.video()?;

    let window = video_subsystem
        .window("Okno :)", 800, 600)
        .position_centered()
        .build()
        .expect("Could not create a window :(");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not build a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    loop {
        canvas.present();
    }

    Ok(())
}
