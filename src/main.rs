pub mod math;
pub mod painting;
use crate::painting::shapes::Polygon;
use painting::palette::Palette;
use crate::painting::canvas::{draw_polygon, set_pixel};

use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Point;


pub fn main() -> Result<()> {
    let sdl_context = sdl2::init().expect("Expected to initialize sdl2");

    let video_subsystem = sdl_context
        .video()
        .expect("Expected to initialize video_subsystem");
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
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

    let mut ex = Palette::init();

    let mut pol = Polygon::new(
        vec![
            Point::from((100, 100)),
            Point::from((200, 100)),
            Point::from((200, 200)),
            Point::from((100, 200)),
        ]);
    //pol.scale(1.9);
    //pol.translate(Point::new(100, 100));
    //pol.rotate(30.0);
    //pol.stretch_x(1.5);
    //pol.stretch_y(1.5);
    //pol.squeeze_x(1.5);
    //pol.squeeze_y(1.5);
    //pol.shear_x(1.5);
    //pol.shear_y(1.5);


    draw_polygon(&mut ex, pol, Color { r: 212, g: 192, b: 100, a: 0 });

    'running: loop {
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
        set_pixel(ex, &mut canvas, 800, 800, Color { r: 255, g: 0, b: 0, a: 255 });
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
