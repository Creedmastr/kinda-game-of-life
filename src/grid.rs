use crate::cell::Cell;

#[derive(Clone)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn draw_random(&self, screen: &mut [u8], vec: &Vec<Cell>) {
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
        let mut color ;

        debug_assert_eq!(screen.len(), 4 * self.cells.len());

        for (_c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            if b < 11950 && b > 10 {
                if !vec[b+1].alive == true {
                    color = [1, 0xF0, 0xF8, 0xFF];
                } else {
                    color = [0x0, 0x0, 0x0, 0x0];
                }
            } else {
                color = [0x0, 0x0, 0x0, 0x0];
            }

            pix.copy_from_slice(&color);
            b = b + 1;
        }

    }
}
