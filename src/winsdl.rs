use sdl2::pixels::Color;
use sdl2::{render::WindowCanvas, video::Window, EventPump, Sdl};

pub struct Winsdl {
    pub sdl: Sdl,
    // pub window: Window,
    pub event_pump: EventPump,
    pub canvas: WindowCanvas,
}

impl Winsdl {
    pub fn new(width: u32, height: u32) -> Result<Self, &'static str> {
        let sdl: Sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window: Window = video_subsystem
            .window("GeorgiAdventures", width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas: WindowCanvas = window.into_canvas().build().unwrap();

        let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

        Ok(Winsdl {
            sdl,
            //      window,
            event_pump,
            canvas,
        })
    }

    pub fn render_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        self.canvas.present();
    }
}
