pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Stylize},
    terminal::{self, ClearType},
    Command,
};
use crossterm::{
    style::{Color, Print, SetForegroundColor},
    Result,
};
use std::fmt;
use std::io;

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Red,
    Blue,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Blue => write!(f, "Blue"),
            Player::Red => write!(f, "Red"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Player(Player),
}

pub struct Board {
    pub cells: Vec<Cell>,
    cols: usize,
    rows: usize,
    turn: Player,
    empty: Cell,
}

impl Board {
    pub fn new(cols: usize, rows: usize) -> Board {
        let mut cells: Vec<Cell> = Vec::with_capacity(cols * rows);
        for _ in 0..cols * rows {
            cells.push(Cell::Empty);
        }
        Board {
            cells,
            cols,
            rows,
            turn: Player::Red,
            empty: Cell::Empty,
        }
    }

    pub fn draw(&self) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("It is {}'s turn", self.turn))
        )?;
        execute!(io::stdout(), SetForegroundColor(Color::Reset))?;
        print!("\n");

        for row in 0..self.rows {
            for _ in 0..self.cols {
                print!("+----");
            }
            print!("+\n");
            for col in 0..self.cols {
                print!("|");
                match &self[(col as isize, row as isize)] {
                    Cell::Empty => print!("    "),
                    Cell::Player(player) => match player {
                        Player::Red => {
                            execute!(io::stdout(), SetForegroundColor(Color::Reset))?;
                            execute!(
                                io::stdout(),
                                SetForegroundColor(Color::Red),
                                Print(format!(" O  "))
                            )?;
                            execute!(io::stdout(), SetForegroundColor(Color::Reset))?;
                        }
                        Player::Blue => {
                            execute!(io::stdout(), SetForegroundColor(Color::Reset))?;
                            execute!(
                                io::stdout(),
                                SetForegroundColor(Color::Blue),
                                Print(format!(" O  "))
                            )?;
                            execute!(io::stdout(), SetForegroundColor(Color::Reset))?;
                        }
                    },
                }
            }
            print!("|");

            print!("\n");
        }

        for _ in 0..self.cols {
            print!("+----");
        }
        println!("+");

        for i in 0..self.cols {
            print!("  {}  ", i + 1);
        }
        println!("");
        Ok(())
    }

    pub fn drop(&mut self, column: u32) -> std::result::Result<String, String> {
        for i in (0..self.rows).rev() {
            match &self[(column as isize, i as isize)] {
                Cell::Empty => {
                    self[(column as isize, i as isize)] = Cell::Player(self.turn);
                    if self.check_win(column as isize, i as isize) {
                        match self.turn {
                            Player::Blue => return Ok("Blue Won".to_string()),
                            Player::Red => return Ok("Red Won".to_string()),
                        };
                    }
                    self.turn = match self.turn {
                        Player::Red => Player::Blue,
                        Player::Blue => Player::Red,
                    };
                    return Ok("Valid Column".to_string());
                }
                _ => {}
            }
        }

        Err("Column Is Full".to_string())
    }

    pub fn check_win(&self, col: isize, row: isize) -> bool {
        for i in 0..4 {
            let offset = (0 - i)..=(3 - i);

            // vertical check
            if offset
                .clone()
                .into_iter()
                .filter(|x| match self[(col, row + x)] {
                    Cell::Empty => false,
                    Cell::Player(player) => {
                        if player == self.turn {
                            true
                        } else {
                            false
                        }
                    }
                })
                .count()
                >= 4
            {
                return true;
            }

            // horizontal check
            if offset
                .clone()
                .into_iter()
                .filter(|x| match self[(col + x, row)] {
                    Cell::Empty => false,
                    Cell::Player(player) => {
                        if player == self.turn {
                            true
                        } else {
                            false
                        }
                    }
                })
                .count()
                >= 4
            {
                return true;
            }

            // diag 1 check
            if offset
                .clone()
                .into_iter()
                .filter(|x| match self[(col + x, row + x)] {
                    Cell::Empty => false,
                    Cell::Player(player) => {
                        if player == self.turn {
                            true
                        } else {
                            false
                        }
                    }
                })
                .count()
                >= 4
            {
                return true;
            }

            // diag 2 check
            if offset
                .clone()
                .into_iter()
                .filter(|x| match self[(col + x, row - x)] {
                    Cell::Empty => false,
                    Cell::Player(player) => {
                        if player == self.turn {
                            true
                        } else {
                            false
                        }
                    }
                })
                .count()
                >= 4
            {
                return true;
            }
        }

        false
    }
}

impl Index<(isize, isize)> for Board {
    type Output = Cell;

    fn index(&self, (col, row): (isize, isize)) -> &Cell {
        if col < 0 || col > (self.cols - 1) as isize {
            return &Cell::Empty;
        }
        if row < 0 || row > (self.rows - 1) as isize {
            return &Cell::Empty;
        }
        &self.cells[col as usize + row as usize * self.cols]
    }
}

impl IndexMut<(isize, isize)> for Board {
    fn index_mut(&mut self, (col, row): (isize, isize)) -> &mut Cell {
        if col < 0 || col > self.cols as isize {
            return &mut self.empty;
        }
        if row < 0 || row > self.rows as isize {
            return &mut self.empty;
        }
        &mut self.cells[col as usize + row as usize * self.cols]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for _ in 0..self.cols {
                write!(f, "|---|")?;
            }
            write!(f, "\n")?;
            for col in 0..self.cols {
                match &self[(col as isize, row as isize)] {
                    Cell::Empty => write!(f, "|   |")?,
                    Cell::Player(player) => match player {
                        Player::Red => {
                            write!(f, "| O |")?;
                        }
                        Player::Blue => {
                            write!(f, "| O |")?;
                        }
                    },
                }
            }
            write!(f, "\n")?;
            for _ in 0..self.cols {
                write!(f, "|---|")?;
            }
            write!(f, "\n")?;
        }

        for i in 0..self.cols {
            write!(f, "  {}  ", i + 1)?;
        }
        writeln!(f, "")?;
        Ok(())
    }
}
