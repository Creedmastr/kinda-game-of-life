use crate::cell::Cell;

#[derive(Clone)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn draw(&self, screen: &mut [u8], vec: &Vec<Cell>) {
        let mut b = 0;

        debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (_c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            let color;
            if  vec[b].alive == true {
                color = [1, 0xF0, 0xF8, 0xFF];
            } else {
                color = [0x0, 0x0, 0x0, 0x0];
            }
            pix.copy_from_slice(&color);
            b = b + 1;
        }
    }

    pub fn update(&self, screen: &mut [u8], vec: &Vec<Cell>) {
        
        let mut b = 0;

        for (_c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            let mut color;
            if b < 12000 && b > 0 {
                if vec[b + 1].alive == false && vec[b - 1].alive == false {
                    color = [0x0, 0x0, 0x0, 0x0];
                } else {
                    color = [1, 0xF0, 0xF8, 0xFF];
                }

            } else {
                color = [0x0, 0x0, 0x0, 0x0];
            }

            pix.copy_from_slice(&color);
            b = b + 1;
        }

    }
}
