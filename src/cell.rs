#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell {
    pub alive: bool,
    pub coord_x: u64,
    pub coord_y: u64,
}

pub fn generate_seed() -> (u64, u64) {
    use byteorder::{ByteOrder, NativeEndian};
    use getrandom::getrandom;

    let mut seed = [0_u8; 16];

    getrandom(&mut seed).expect("failed to getrandom");

    (
        NativeEndian::read_u64(&seed[0..8]),
        NativeEndian::read_u64(&seed[8..16]),
    )
}

pub fn vec_gen(width: u32, height: u32) -> Vec<Cell> {
    let mut vec_generated: Vec<Cell> = vec![];

    let mut rng: randomize::PCG32 = generate_seed().into();

    let mut i = 0;
    let mut a = 0;

    while a < height {
        for _ in 1..=width {
            if randomize::f32_half_open_right(rng.next_u32()) > 0.9 {
                vec_generated.push(Cell {
                    alive: true,
                    coord_x: i,
                    coord_y: a as u64,
                });
            } else {
                vec_generated.push(Cell {
                    alive: false,
                    coord_x: i,
                    coord_y: a as u64,
                });
            }
            i += 1;
        }

        i = 0;

        a += 1;
    }

    return vec_generated;
}
