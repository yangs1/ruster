#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}


#[derive(Debug)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

/**
 *  大宇宙图表
 */
impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        Universe {
            width: width,
            height: height,
            cells: vec![Cell::Dead; (width * height) as usize], // 将二维数字拉成一维数组
        }
    }

    // 写入生命点
    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    // 获取一维坐标点
    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }
    
}

use std::{fmt, usize};

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for (lindex, &line) in self.cells.iter().enumerate(){
            if (lindex > 0) && (lindex % self.width == 0)  {
                write!(f, "\n")?;
            }
            let symbol = if line == Cell::Dead { '◻' } else { '◼' };
            write!(f, "{}", symbol)?;
        }
        write!(f, "\n")?;

        // for line in self.cells.as_slice().chunks(self.width as usize) {
        //     for &cell in line {
        //         let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        //         write!(f, "{}", symbol)?;
        //     }
        //     write!(f, "\n")?;
        // }

        Ok(())
    }
}


