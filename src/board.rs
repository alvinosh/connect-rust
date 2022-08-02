use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
enum Player {
    Red,
    Blue,
}
#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Player(Player),
}

pub struct Board {
    pub cells: Vec<Cell>,
    size: usize,
    turn: Player,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let mut cells: Vec<Cell> = Vec::with_capacity(size * size);
        for _ in 0..size * size {
            cells.push(Cell::Empty);
        }
        Board {
            cells,
            size,
            turn: Player::Red,
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;

    fn index(&self, (col, row): (usize, usize)) -> &Cell {
        &self.cells[col + row * self.size]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Cell {
        &mut self.cells[col + row * self.size]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.size {
            for _ in 0..self.size {
                write!(f, "|---|")?;
            }
            write!(f, "\n")?;
            for col in 0..self.size {
                match &self[(col, row)] {
                    Cell::Empty => write!(f, "| . |")?,
                    Cell::Player(player) => match player {
                        Player::Red => write!(f, "| R |")?,
                        Player::Blue => write!(f, "| B |")?,
                    },
                }
            }
            write!(f, "\n")?;
            for _ in 0..self.size {
                write!(f, "|---|")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
