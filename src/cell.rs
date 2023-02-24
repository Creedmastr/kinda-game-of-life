
#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub alive: bool,
    pub coord_x: u64,
    pub coord_y: u64,
}


impl Cell {
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_state(&mut self, state: bool) {
        self.alive = state;
    }

    pub fn new(alive: bool, coord_x: u64, coord_y: u64) -> Self {
        Self {
            alive,
            coord_x,
            coord_y,
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

pub fn vec_gen(px: u32, WIDTH: u32, HEIGHT: u32) -> Vec<Cell> {
    let mut vec_generated: Vec<Cell> = vec![];

    let mut rng: randomize::PCG32 = generate_seed().into();

    let mut i = 0;
    let mut a = 0;

    while a < 12001 {
        for _ in 0..WIDTH {
            if randomize::f32_half_open_right(rng.next_u32()) > 0.9 {
                vec_generated.push(Cell {
                     alive: true,
                     coord_x: i,
                     coord_y: a
                });
            } else {
                vec_generated.push(Cell {
                     alive: false,
                     coord_x: i,
                     coord_y: a 
                });
            }
            i += 1;
        }

        a += 1;
    }
    
    return vec_generated;
}


















/*
for _ in 0..px {
    if randomize::f32_half_open_right(rng.next_u32()) > 0.9 {
        vec_generated.push(Cell {
             alive: true 
        });
    } else {
        vec_generated.push(Cell {
             alive: false 
        });
    }
} */