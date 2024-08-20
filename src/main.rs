pub mod math;
pub mod painting;
//use painting::canvas::dda;
//use painting::canvas::bresenham;
//use crate::painting::canvas::draw_polygon;
use crate::painting::shapes::Polygon;
use crate::painting::shapes::Polygon;
use painting::palette::Palette;
use crate::painting::canvas::{draw_polygon, set_pixel};

//use painting::canvas::draw_circle;
use anyhow::Result;
use painting::canvas::scanline;
use painting::palette::Palette;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;


pub fn main() -> Result<()> {
    let width = 800;
    let height = 800;
    let sdl_context = sdl2::init().expect("Expected to initialize sdl2");

    let video_subsystem = sdl_context
        .video()
        .expect("Expected to initialize video_subsystem");
    let window = video_subsystem
        .window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .expect("Expected to start window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Expected to build the renderer");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Expected to initialize event pump");

    let mut palette = Palette::init();

    'running: loop {
        canvas.set_draw_color(prim_color);
        canvas.clear();
        let exs = palette.check_points();
        for (k, v) in exs.iter() {
            canvas.set_draw_color(*k);
            for p in v {
                set_pixel(&mut canvas, width, height, *p);
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
