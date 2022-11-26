#![deny(clippy::all)]
#![forbid(unsafe_code)]

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

mod draw;
mod particle;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 450;
const ROW_LEN: usize = 900;
const COL_LEN: usize = 450;
const TILE_SIZE: usize = 4;
const FPS: f64 = 20.0;
const GLOBE_RAD: u32 = 40;

const RED: [u8; 4] = [0xdd, 0x40, 0x3a, 0xff];
const GREEN: [u8; 4] = [0x69, 0x7a, 0x21, 0xff];
const BLUE: [u8; 4] = [0x05, 0x29, 0x9e, 0xff];
const GREY: [u8; 4] = [0x3e, 0x42, 0x4b, 0xff];
const WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const GLASS: [u8; 4] = [0xc7, 0xe3, 0xe1, 0xff];

const EMIT_X: i32 = (ROW_LEN / TILE_SIZE / 2) as i32;
const EMIT_Y: i32 = (COL_LEN / TILE_SIZE / 2) as i32;

fn draw_particles(frame: &mut [u8], queue: &Vec<particle::Particle>) {
    for p in queue.iter() {
        draw::draw_tile(
            frame,
            ROW_LEN,
            COL_LEN,
            p.px as usize,
            p.py as usize,
            TILE_SIZE,
            WHITE,
        );
    }
}

fn in_globle(x: i32, y: i32) -> bool {
    (EMIT_X - x) * (EMIT_X - x) + (EMIT_Y - y) * (EMIT_Y - y) <= (GLOBE_RAD * GLOBE_RAD) as i32 + 1
}

fn update_particles(queue: &mut Vec<particle::Particle>) {
    for p in queue.iter_mut() {
        if in_globle(p.px + p.vx, p.py + p.vy) {
            (*p).particle_move();
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

    let mut particle_queue = Vec::new();

    event_loop.run(move |event, _, control_flow| {
        //Stamp time
        let at_last_frame = time::Instant::now();
        //Push a snow
        particle_queue.push(particle::Particle {
            px: EMIT_X,
            py: EMIT_Y,
            vx: rand::thread_rng().gen_range(-2..3),
            vy: -1,
        });
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw::clear_frame(pixels.get_frame_mut(), GREY);
            draw::draw_circle(
                pixels.get_frame_mut(),
                ROW_LEN,
                COL_LEN,
                TILE_SIZE,
                ROW_LEN / TILE_SIZE / 2,
                COL_LEN / TILE_SIZE / 2,
                GLOBE_RAD,
                GLASS,
            );
            update_particles(&mut particle_queue);
            draw_particles(pixels.get_frame_mut(), &particle_queue);
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        let time_to_draw = at_last_frame.elapsed().as_secs_f64();
        if time_to_draw < (1.0 / FPS) {
            sleep(time::Duration::from_secs_f64((1.0 / FPS) - time_to_draw));
        }
        println!("FPS: {}", 1.0 / at_last_frame.elapsed().as_secs_f64());
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
        window.request_redraw()
    });
}
