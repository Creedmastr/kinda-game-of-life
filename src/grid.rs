use std::future::Future;

use crate::{cell::Cell, HEIGHT, WIDTH};
use rayon::prelude::*;

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

    pub fn update(&self, screen: &mut [u8], vec: &Vec<Cell>, px: u32, height: u32, width: u32, rule_value: usize) {
        
        let mut b: usize = 0;
        let mut color ;

        debug_assert_eq!(screen.len(), 4 * self.cells.len());

        for (_c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {

            if b < (px - 10) as usize  && b > 10 {
                let reference: Cell = Cell {
                    alive: true,
                    coord_x: b - rule_value,
                    coord_y: vec[b].coord_y
                };
    
                if vec.contains(&reference) == true {
                    color = [1, 0xF0, 0xF8, 0xFF];
                } else {
                    color = [0x0, 0x0, 0x0, 0x0];
                    
                }
            } else {
                color = [0x0, 0x0, 0x0, 0x0];
            }

            if b > width as usize {
                b = 0;
            } else {
                b += 1;
            }

            pix.copy_from_slice(&color);
        }
    }
}
