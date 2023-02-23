#![feature(default_free_fn)]

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

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

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

    let mut cells_vec: Vec<cell::Cell> = vec![];
    
    let mut rng: randomize::PCG32 = generate_seed().into();

    let mut i: i128 = 0;

    let mut a: i128 = 0;

    for _ in 0..(WIDTH * HEIGHT) {
        if randomize::f32_half_open_right(rng.next_u32()) > 0.9 {
            cells_vec.push(cell::Cell { alive: true });
            i = i + 1;
        } else {
            cells_vec.push(cell::Cell { alive: false });
            a = a + 1;
        }
    }

    // print!("{i} {a}");

    /*for _ in 0..=cells_vec.len() {
        print!("{}", cells_vec[i].alive);
    }   */

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

        if let Event::RedrawRequested(_) = event {
            the_grid.draw(pixels.get_frame_mut(), &cells_vec);
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        if input.update(&event) {
            if input.key_pressed(winit::event::VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            /*if input.key_pressed(winit::event::VirtualKeyCode::Space) {
                Grid.draw(pixels.get_frame_mut());
            }*/
        }
    });
}

fn generate_seed() -> (u64, u64) {
    use byteorder::{ByteOrder, NativeEndian};
    use getrandom::getrandom;

    let mut seed = [0_u8; 16];

    getrandom(&mut seed).expect("failed to getrandom");

    (
        NativeEndian::read_u64(&seed[0..8]),
        NativeEndian::read_u64(&seed[8..16]),
    )
}
