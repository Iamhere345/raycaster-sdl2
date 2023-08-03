use std::time::Instant;
use std::collections::HashSet;

use sdl2::pixels::Color; 
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub mod graphics;
pub mod raycaster;
pub mod vec2;

use crate::graphics::*;
use crate::raycaster::*;

fn main() {
    
    let mut screen: PixelBuf = Vec::with_capacity(SCREEN_HEIGHT * SCREEN_WIDTH);
    unsafe { screen.set_len(SCREEN_HEIGHT * SCREEN_WIDTH) };

    let mut scene = Scene::init();

    let mut time = Instant::now();
    let mut old_time = Instant::now();

    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();

    let window = video.window("Raycasting", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .expect("Unable to build window");

    let mouse = sdl_ctx.mouse();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 25));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    for (i, colour) in screen.iter().enumerate() {

        let x = i % SCREEN_WIDTH;
        let y = i / SCREEN_WIDTH;

        canvas.set_draw_color(Color::RGB(colour[0], colour[1], colour[2]));
        canvas.draw_point((x as i32, y as i32)).unwrap();

    }

    mouse.set_relative_mouse_mode(true);

    let mut paused = false;
    let mut mouse_captured = true;
    'main: loop {
        /*
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - 1));
        canvas.clear();
        */

        old_time = time;
        time = Instant::now();

        let delta_time = (time.elapsed().as_secs_f64() - old_time.elapsed().as_secs_f64()).abs();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                
                Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
                    paused = !paused;
                    mouse_captured = !mouse_captured;
                    mouse.set_relative_mouse_mode(mouse_captured);
                }
                
                _ => {}
            }
        }

        if paused {
            std::thread::sleep(Duration::from_millis(1));
            continue;
        }

        let keys: HashSet<_> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        input(keys, event_pump.relative_mouse_state().x() as f64, delta_time, &mut scene);

        update(&mut canvas, &mut scene);

        canvas.present();

        //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

}