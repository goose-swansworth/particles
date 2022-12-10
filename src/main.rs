#![deny(clippy::all)]
#![forbid(unsafe_code)]

use canvas::Canvas;
use rand::Rng;
use std::time;
use std::time::Instant;
use std::usize;
use std::collections::HashSet;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use image::DynamicImage;

mod particle;
mod canvas;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 450;
const TILE_SIZE: usize = 5;
const FPS: f64 = 40.0;
const GLOBE_RAD: u32 = 40;

const GREY: [u8; 4] = [0x3e, 0x42, 0x4b, 0xff];
const WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const GLASS: [u8; 4] = [0xc7, 0xe3, 0xe1, 0xff];


const CENTRE_X: i32 = (WIDTH as usize / TILE_SIZE / 2) as i32;
const CENTRE_Y: i32 = (HEIGHT as usize / TILE_SIZE / 2) as i32;

const BUMP_CHANCE: f64 = 0.15;

fn wait_for_next_frame(last: Instant, frames: &mut u8) {
    let mut time_to_draw = last.elapsed().as_secs_f64();
    while time_to_draw < (1.0 / FPS)  {
        time_to_draw = last.elapsed().as_secs_f64();
    }
    if *frames == FPS as u8 {
        println!("FPS: {:.0}", 1.0 / last.elapsed().as_secs_f64());
        *frames = 0;
    }
    
}

fn draw_particles(frame: &mut [u8], canvas: &Canvas, queue: &Vec<particle::Particle>) {
    for p in queue.iter() {
        canvas.draw_tile(frame, p.px as usize, p.py as usize, WHITE);
    }
}

fn update_particles(queue: &mut Vec<particle::Particle>, occupied: &mut HashSet<(i32, i32)>) {
    for p in queue.iter_mut() {
        p.update(occupied);
    }
}

fn bump_particles(queue: &mut Vec<particle::Particle>, occupied: &mut HashSet<(i32, i32)>) {
    for p in queue.iter_mut() {
        if !occupied.contains(&(p.px, p.py - 1)) {
            if !occupied.contains(&(p.px - 1, p.py)) {
                if rand::thread_rng().gen_bool(BUMP_CHANCE) {
                     p.shift(p.px - 1, p.py, occupied);
                }
            } else if !occupied.contains(&(p.px + 1, p.py)) {
                if rand::thread_rng().gen_bool(BUMP_CHANCE) {
                    p.shift(p.px + 1, p.py, occupied);
                }
            }
        }
    }
}

fn add_anulus_points(occupied: &mut HashSet<(i32, i32)>, center_x: i32, center_y: i32, in_rad: i32, out_rad: i32) {
    for i in 0..out_rad - in_rad + 1 {
        let points = canvas::circle_coords(center_x, center_y, in_rad + i);
        for p in points.iter() {
            occupied.insert(*p);
        }     
    }    
}

fn add_line_points(occupied: &mut HashSet<(i32, i32)>, (x0, y0): (i32, i32), (x1, y1): (i32, i32), sloped: bool) {
    let points = canvas::line_points((x0, y0), (x1, y1));
    for p in points.iter() {
        occupied.insert(*p);
    }
    if sloped {
        let points = canvas::line_points((x0, y0-1), (x1, y1-1));
        for p in points.iter() {
            occupied.insert(*p);
        }
    }
}

fn draw_house(frame: &mut [u8], canvas: &Canvas, cabin_sprite: &DynamicImage) {
    //Ground line
    canvas.draw_sprite(frame, cabin_sprite, CENTRE_X as usize - 10, CENTRE_Y as usize - 14);
    //Cabin Sprite
    canvas.draw_line(frame, (56, CENTRE_Y - 20), (124, CENTRE_Y - 20), WHITE);
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Snowglobe")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let canvas = Canvas::init(WIDTH as usize, HEIGHT as usize, TILE_SIZE);
    let mut particle_queue = Vec::new();
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    add_anulus_points(&mut occupied, CENTRE_X, CENTRE_Y, GLOBE_RAD as i32, GLOBE_RAD as i32 + 3);
    add_line_points(&mut occupied, (CENTRE_X - 10, CENTRE_Y-14), (CENTRE_X, CENTRE_Y-9), true);
    add_line_points(&mut occupied, (CENTRE_X, CENTRE_Y-9), (CENTRE_X + 10, CENTRE_Y - 14), true);
    
    add_line_points(&mut occupied, (56, CENTRE_Y - 20), (124, CENTRE_Y - 20), false);
    let mut mouse_x: f32 = CENTRE_X as f32;
    let mut mouse_y: f32 = CENTRE_Y as f32;
    
    let cabin_sprite = image::open("src/resourses/cabin.png").unwrap();
    
    let mut frames: u8 = 0;

    event_loop.run(move |event, _, control_flow| {
        //Stamp time
        let at_last_frame = time::Instant::now();
        
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            canvas.clear(pixels.get_frame_mut(), GREY);
            canvas.draw_circle(pixels.get_frame_mut(), CENTRE_X as usize, CENTRE_Y as usize, GLOBE_RAD, GLASS);
            update_particles(&mut particle_queue, &mut occupied);
            draw_particles(pixels.get_frame_mut(), &canvas, &particle_queue);
            bump_particles(&mut particle_queue, &mut occupied);
            draw_house(pixels.get_frame_mut(), &canvas, &cabin_sprite);
            canvas.fill(pixels.get_frame_mut(), CENTRE_X as i32, CENTRE_Y as i32 -25, &mut occupied, WHITE);
            frames += 1;
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            
            if input.mouse_held(0) {
                (mouse_x, mouse_y) = match input.mouse() {
                    None => (mouse_x, mouse_y),
                    Some((x, y)) => (x, y)
                };
                let px = canvas.mouse_to_canvas(true, mouse_x) as i32;
                let py = canvas.mouse_to_canvas(false, mouse_y) as i32;
                //Push a snow
                particle_queue.push(particle::Particle {
                    px,
                    py,
                    vx: rand::thread_rng().gen_range(-1..2),
                    vy: -1,
                });
                occupied.insert((px, py));
                
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            wait_for_next_frame(at_last_frame, &mut frames);

        // Update internal state and request a redraw
        }
        window.request_redraw();
    });
}
