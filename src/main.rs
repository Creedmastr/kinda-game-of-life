#![feature(default_free_fn)]

use cell::vec_gen;
use log::error;
use pixels::Error;
use pixels::Pixels;
use pixels::SurfaceTexture;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod cell;
mod grid;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 400;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Game of Life")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut input = WinitInputHelper::new();

    let mut cells_vec = cell::vec_gen(HEIGHT*WIDTH, WIDTH, HEIGHT);

    let woud = WIDTH as usize;
    let bad = HEIGHT as usize;

    let souze = woud.checked_mul(bad).expect("too big");

    let the_grid = grid::Grid {
        width: WIDTH,
        height: HEIGHT,
        cells: vec![cell::Cell::default(); souze],
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        if let Event::RedrawRequested(_) = event {
            the_grid.draw_random(pixels.get_frame_mut(), &cells_vec);
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(winit::event::VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_held(winit::event::VirtualKeyCode::A) {
                cells_vec = vec_gen(HEIGHT*WIDTH, WIDTH, HEIGHT);
                the_grid.draw_random(pixels.get_frame_mut(), &cells_vec);
                pixels.render().expect("");
            }

            if input.key_pressed(winit::event::VirtualKeyCode::B) {
                cells_vec = vec_gen(HEIGHT*WIDTH, WIDTH, HEIGHT);
                the_grid.draw_random(pixels.get_frame_mut(), &cells_vec);
                pixels.render().expect("");
            }

            if input.key_pressed(winit::event::VirtualKeyCode::Space) {
                the_grid.update(pixels.get_frame_mut(), &cells_vec);
                pixels.render().expect("Couldn't render");
            }
        }
    });
}


