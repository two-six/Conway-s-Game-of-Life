pub mod term {
    use std::io::{stdout, Write};
    use crossterm::{
        ExecutableCommand,
        terminal,
        cursor,
        style::Print,
        Result,
        queue,
        terminal::size,
    };
    use super::logic;
    use std::collections::HashSet;

    pub struct Board(pub u16, pub u16);

    pub fn render(b: &HashSet<logic::Pos>) -> Result<()>{
        stdout().execute(terminal::Clear(terminal::ClearType::All))?;
        for el in b {
            write_element(&el)?;
        }
        Ok(())
    }

    pub fn write_border() -> Result<()>{
        let (cols, rows) = size()?;
        for x in 0..cols {
            for y in 0..rows {
                 if (y == 0 || y == rows - 1) || (x == 0 || x == cols - 1) {
                    queue!(stdout(),
                           cursor::MoveTo(x, y),
                           Print("#".to_string()))?;
                }
            }
        }
        stdout().flush()?;
        Ok(())
    }

    pub fn write_element(el: &logic::Pos) -> Result<()> {
        queue!(stdout(),
               cursor::MoveTo(el.0 as u16, el.1 as u16),
               Print("#".to_string()))?;
        stdout().flush()?;
        Ok(())
    }
}

pub mod logic {
    use std::collections::HashSet;
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Pos(pub i32, pub i32);

    pub fn neighbours(p: &Pos, b: &HashSet<Pos>) -> usize {
        b.iter()
            .filter(|l| (l.0-p.0).abs() <= 1 && (l.1-p.1).abs() <= 1 && !(*l == p))
            .count()
    }

    pub fn find_min_x(b: &HashSet<Pos>) -> i32 {
        let mut min = -1;
        for el in b {
            if min == -1 {
                min = el.0;
            } else if el.0 < min {
                min = el.0;
            }
        }
        min
    }

    pub fn find_max_x(b: &HashSet<Pos>) -> i32 {
        let mut max = -1;
        for el in b {
            if el.0 > max {
                max = el.0;
            }
        }
        max
    }

    pub fn find_min_y(b: &HashSet<Pos>) -> i32 {
        let mut min = -1;
        for el in b {
            if min == -1 {
                min = el.1;
            } else if el.1 < min {
                min = el.1;
            }
        }
        min
    }

    pub fn find_max_y(b: &HashSet<Pos>) -> i32 {
        let mut max = -1;
        for el in b {
            if el.1 > max {
                max = el.0;
            }
        }
        max
    }
}

pub mod game {
    use std::collections::HashSet;
    use std::fs;
    use super::logic;
    use super::term;
    use crossterm::{terminal, Result};
    use std::{thread, time};

    pub fn parse_input_board(f: &str) -> HashSet<logic::Pos> {
        let contents = fs::read_to_string(f)
            .expect("Something went wrong with file");
        let lines: Vec<&str> = contents.split("\n").collect();

        let mut x = HashSet::new();
        for l in 0..lines.len() {
           for (i, c) in lines[l].chars().enumerate() {
               if c == '#' {
                   x.insert(logic::Pos(i as i32, l as i32));
               }
           }
        }
        x
    }

    pub fn next_turn(b: &HashSet<logic::Pos>) -> HashSet<logic::Pos> {
        let (cols, rows) = terminal::size()
            .expect("Error trying to get size of terminal");
        let mut n_cells = HashSet::new();
        for col in 0..cols {
            for row in 0..rows {
                let n = logic::neighbours(&logic::Pos(col as i32, row as i32), b);
                if n == 2 && b.contains(&logic::Pos(col as i32, row as i32)) {
                    n_cells.insert(logic::Pos(col as i32, row as i32));
                } else if n == 3 {
                    n_cells.insert(logic::Pos(col as i32, row as i32));
                }
            }
        }
        n_cells
    }

    pub fn center_board(elems: &HashSet<logic::Pos>) -> HashSet<logic::Pos> {
        let (cols, rows) = terminal::size()
            .expect("Error trying to get size of terminal");
        let (min_x, min_y) = (logic::find_min_x(elems), logic::find_min_y(elems));
        let (max_x, max_y) = (logic::find_max_x(elems), logic::find_max_y(elems));
        let mut elems_c = HashSet::new();
        for el in elems {
            elems_c.insert(logic::Pos((cols as i32)/2-(max_x-min_x)/2 + el.0, (rows as i32)/2-(max_y-min_y)/2 + el.1));
        }
        elems_c
    }

    pub fn play(t: &u64, f: &str) -> Result<()> {
        let mut board = parse_input_board(f);
        board = center_board(&board);
        loop {
            if let Err(_) = term::render(&board) {
                break;
            }
            thread::sleep(time::Duration::from_millis(*t));
            board = next_turn(&board);
        }
        Ok(())
    }
}
