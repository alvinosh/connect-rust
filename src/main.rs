use std::io;

use clap::Parser;
use crossterm::style::{Color, Print, SetForegroundColor};
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Stylize},
    terminal::{self, ClearType},
    Command, Result,
};
mod args;
mod board;

pub fn read_char() -> Result<char> {
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) = event::read()?
        {
            println!("{}", c);
            return Ok(c);
        }
    }
}

fn main() -> Result<()> {
    let args = args::Args::parse();
    let cols = args.cols;
    let rows = args.rows;

    let mut board = board::Board::new(cols, rows);
    println!("Starting Game, enter q to quit...");

    let mut error = String::from("");

    loop {
        queue!(
            io::stdout(),
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(format!("Press Q to quit Anytime"))
        )?;
        println!("");
        println!("");
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("{}", error))
        )?;
        println!("");

        board.draw()?;

        execute!(
            io::stdout(),
            SetForegroundColor(Color::Blue),
            Print(format!("Enter A Number from 1 to {}: ", cols))
        )?;

        println!();
        if let Ok(line) = read_char() {
            if line.eq(&'q') {
                break;
            };

            let column: u32 = match line.to_digit(10) {
                Some(number) => {
                    if number >= 1 && number <= cols as u32 {
                        number
                    } else {
                        error = format!("Number Must be Between 1 and {}", cols);
                        0
                    }
                }
                None => {
                    error = format!("Not A number");
                    0
                }
            };

            if column == 0 {
                continue;
            }

            match board.drop(column - 1) {
                Ok(val) => match val.as_str() {
                    "Blue Won" => {
                        board.draw()?;

                        execute!(
                            io::stdout(),
                            SetForegroundColor(Color::DarkBlue),
                            Print(format!("Blue Won"))
                        )?;
                        break;
                    }
                    "Red Won" => {
                        board.draw()?;

                        execute!(
                            io::stdout(),
                            SetForegroundColor(Color::DarkRed),
                            Print(format!("Red Won"))
                        )?;
                        break;
                    }
                    _ => {}
                },
                Err(e) => error = e,
            }
        }
    }

    execute!(io::stdout(), SetForegroundColor(Color::Reset),)?;

    Ok(())
}
