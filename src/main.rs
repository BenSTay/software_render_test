use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};
use std::time::Duration;

mod drawing;
use crate::drawing::Buffer;

const WIN_H: u32 = 480;
const WIN_W: u32 = 640;
const FPS: u32 = 30;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Test", WIN_W, WIN_H)
        .build()
        .expect("Couldn't create window");

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .expect("Couldn't create canvas");

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(PixelFormatEnum::RGB24, WIN_W, WIN_H)
        .expect("Couldn't create texture");

    let mut event_pump = sdl_context.event_pump()?;

    let mut buffer = Buffer::new(WIN_W as usize, WIN_H as usize);

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
        canvas.clear();

        randomize(&mut buffer);
        buffer.render(&mut texture, &mut canvas)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000 / FPS));
    }
    Ok(())
}

fn randomize(buffer: &mut Buffer) {
    for y in 0..WIN_H as usize {
        for x in 0..WIN_W as usize {
            let v = rand::random();
            buffer.set_pixel(x, y, &[v, v, v])
        }
    }
}
