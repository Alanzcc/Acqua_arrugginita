use std::time::Duration;
use anyhow::Result;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureAccess;
use sdl2::Sdl;
use sdl2::surface::Surface;
use painting::canvas::scanline;
use painting::palette::Palette;
use crate::painting::shapes::Polygon;

pub mod math;
pub mod painting;
pub fn main() -> Result<()> {
    let width = 1366;
    let height = 768;
    let o_w: u32 = 100;
    let o_h: u32 = 100;

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

    let mut ex = Palette::init();
    /*
        // bresenham(&mut ex, 0, 0, 799, 799, Color::RGB(40, 30, 180));
        //dda(&mut ex, 0, 0, 799, 799, Color::RGB(0, 150, 100));
        //dda(&mut ex, 799, 0, 0, 799, Color::RGB(100, 150, 0));
        let pol = Polygon::new(
            vec![
                 Point::from((200, 200)),
                 Point::from((300, 100)),
                 Point::from((400, 200)),
                 Point::from((400, 400)),
                 Point::from((200, 400)),
            ]);
    */

    canvas.render_target_supported();

    let texture_creator = canvas.texture_creator();
    //let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
    //surface.
    //let mut texture_fs = surface.as_texture(&texture_creator)?;
    let mut tex = texture_creator.load_texture("src/img/kirby_dormindo.jpg").unwrap();

    //let mut texture = texture_creator
    //.create_texture_target(texture_creator.default_pixel_format(), 150, 150)
   // .unwrap();
    let _result = canvas.with_texture_canvas(&mut tex, |texture_canvas| {
        //texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        texture_canvas.clear();
        //texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        texture_canvas.fill_rect(Rect::new(o_w as i32, o_h as i32, 100, 100)).unwrap();
    });


    //let tex = canvas.with_texture_canvas(&mut texel, PixelFormatEnum::RGB24)?;


    let x: i32 = (o_w / 2) as i32;
    let y: i32 = (o_h / 2) as i32;
    let src = Rect::new(0, 0, width, height);
    let dest = Rect::new(683 as i32 / 2, 384 as i32 / 2, o_w, o_h);
    //let dest = Rect::new((x - ((o_w / 2) as i32)) as i32, y - ((o_h / 2) as i32), o_w, o_h);
    let center = Point::new((o_w / 2) as i32, (o_h / 2) as i32);

    

    //draw_polygon(&mut ex, &pol, Color::RGB(40, 30, 180))
    let pol = Polygon::new(vec![
        Point::from((200, 200)),
        Point::from((300, 100)),
        Point::from((400, 200)),
        Point::from((400, 400)),
        Point::from((200, 400)),
    ]);


    //draw_circle(&mut ex, Color { r: 255, g: 100, b: 0, a: 255 }, Point::new(400, 400), 200.0);
    scanline(&mut ex, &pol, Color::RGB(40, 30, 180));

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

        //let exs = ex.check_points();
        // for (k, v) in exs.iter() {
        //     canvas.set_draw_color(*k);
        //   for p in v {
        //        canvas.draw_point(*p).expect("Expected to draw pixel");
        //    }
        // }
        canvas.copy_ex(&tex, src, dest, 0.0, center, false, false).expect("TODO: panic message");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
