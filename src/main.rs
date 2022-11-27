#![deny(clippy::all)]
#![forbid(unsafe_code)]

use canvas::Canvas;
use rand::Rng;
use std::thread::sleep;
use std::time;
use std::usize;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

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

fn draw_particles(frame: &mut [u8], canvas: &Canvas, queue: &Vec<particle::Particle>) {
    for p in queue.iter() {
        canvas.draw_tile(frame, p.px as usize, p.py as usize, WHITE);
    }
}

fn in_globle(x: i32, y: i32) -> bool {
    (CENTRE_X - x).pow(2) + (CENTRE_Y - y).pow(2) <= GLOBE_RAD.pow(2) as i32
}

fn update_particles(queue: &mut Vec<particle::Particle>) {
    for p in queue.iter_mut() {
        if in_globle(p.px + p.vx, p.py + p.vy) {
            (*p).particle_move();
            (*p).v_update(rand::thread_rng().gen_range(-1..2), 0);
        } else {
            (*p).v_update(rand::thread_rng().gen_range(-1..2), 0);
        }
    }
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
    let mut mouse_x: f32 = CENTRE_X as f32;
    let mut mouse_y: f32 = CENTRE_Y as f32;

    event_loop.run(move |event, _, control_flow| {
        //Stamp time
        let at_last_frame = time::Instant::now();
        
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            canvas.clear(pixels.get_frame_mut(), GREY);
            canvas.draw_circle(pixels.get_frame_mut(), CENTRE_X as usize, CENTRE_Y as usize, GLOBE_RAD, GLASS);
            update_particles(&mut particle_queue);
            draw_particles(pixels.get_frame_mut(), &canvas, &particle_queue);
            
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
                //Push a snow
                particle_queue.push(particle::Particle {
                    px: canvas.mouse_to_canvas(true, mouse_x) as i32,
                    py: canvas.mouse_to_canvas(false, mouse_y) as i32,
                    vx: rand::thread_rng().gen_range(-2..3),
                    vy: -1,
                });
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

        // Update internal state and request a redraw
        let time_to_draw = at_last_frame.elapsed().as_secs_f64();
        if time_to_draw < (1.0 / FPS) {
            sleep(time::Duration::from_secs_f64((1.0 / FPS) - time_to_draw));
        }
        println!("FPS: {}", 1.0 / at_last_frame.elapsed().as_secs_f64());
        }
        window.request_redraw()
    });
}
