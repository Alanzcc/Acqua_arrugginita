pub mod math;
pub mod painting;
use painting::canvas::dda;
//use painting::canvas::bresenham;
use painting::palette::Palette;

use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

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
    // bresenham(&mut ex, 0, 0, 799, 799, Color::RGB(40, 30, 180));

    dda(&mut ex, 0, 0, 799, 799, Color::RGB(0, 150, 100));
    dda(&mut ex, 799, 0, 0, 799, Color::RGB(100, 150, 0));

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
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let exs = ex.check_points();
        for (k, v) in exs.iter() {
            canvas.set_draw_color(*k);
            for p in v {
                canvas.draw_point(*p);
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
