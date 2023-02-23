
#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub alive: bool,
}


impl Cell {
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_state(&mut self, state: bool) {
        self.alive = state;
    }

    pub fn new(alive: bool) -> Self {
        Self {
            alive: alive
        }
    }
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

pub fn vec_gen(px: u32) -> Vec<Cell> {
    let mut vec_generated: Vec<Cell> = vec![];

    let mut rng: randomize::PCG32 = generate_seed().into();

    for _ in 0..px {
        if randomize::f32_half_open_right(rng.next_u32()) > 0.9 {
            vec_generated.push(Cell { alive: true });
        } else {
            vec_generated.push(Cell { alive: false });
        }
    }

    return vec_generated;
}