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

    /**
     * 翻转，不优雅 有空改改
     * 
     */
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);
                next[idx] = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }
        self.cells = next;
    }

    /**
     * 去对应的行
     */
    fn live_neighbour_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
    
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
    
        count
    }

    pub fn row_as_string(&self, row: usize) -> Option<String> {
        if row < self.height {
            let mut row_string = String::new();
            let start = self.get_index(row, 0);
            let end = self.get_index(row, self.width);
            let line = &self.cells[start..end];
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                row_string.push(symbol);
            }
            Some(row_string)
        } else {
            None
        }
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


