mod examples;

use std::fmt;
use std::{thread, time};


const STARS : usize = 2;
const GRID_SIZE : usize = 10;
const ADJ : usize = 8;

type Grid = [[bool; GRID_SIZE]; GRID_SIZE];
type SectGrid = [[usize; GRID_SIZE]; GRID_SIZE];

#[derive(Default)]
struct Puzzle {
    sect_shape : SectGrid,
    stars : Grid,
    annotations : Grid
}

impl Puzzle {
    fn placeable(&self, row : usize, col : usize) -> bool {
        self.stars.count_touch(row, col) == 0 && self.stars.count_col(col) < STARS && self.stars.count_row(row) < STARS && self.stars.count_sect(self.sect_shape[row][col], &self.sect_shape) < STARS
    }
}

trait TwoNotTouchRules {
    fn count_col(&self, col : usize) -> usize;
    fn count_row(&self, row : usize) -> usize;
    fn count_sect(&self, sect : usize, sect_shape : & SectGrid) -> usize;
    fn count_touch(&self, row : usize, col : usize) -> usize;
    fn fill_col(&mut self, col : usize, except : &Grid) -> usize;
    fn fill_row(&mut self, row : usize, except : &Grid) -> usize;
    fn fill_sect(&mut self, sect : usize, sect_shape : & SectGrid, except : &Grid) -> usize;
    fn fill_touch(&mut self, row : usize, col : usize) -> usize;
}

impl TwoNotTouchRules for Grid {
    fn count_col(&self, col : usize) -> usize {
        let mut ret : usize = 0;
        for r in 0..GRID_SIZE as usize {
            if self[r][col] {
                ret += 1;
            }
        }
        ret
    }
    fn count_row(&self, row : usize) -> usize {
        let mut ret : usize = 0;
        for c in 0..GRID_SIZE as usize {
            if self[row][c] {
                ret += 1;
            }
        }
        ret
    }
    fn count_sect(&self, sect : usize, sect_shape : & SectGrid) -> usize{
        let mut ret : usize = 0;
            for r in 0..GRID_SIZE as usize {
                for c in 0..GRID_SIZE as usize {
                    if sect_shape[r][c] == sect && self[r][c] {
                        ret += 1;
                    }
                }
            }
        ret
    }
    fn count_touch(&self, row : usize, col : usize) -> usize {
        let mut ret : usize = 0;
            for r in -1..2 as i32 {
                for c in -1..2 as i32 {
                    let rtouch = r + row as i32;
                    let ctouch = c + col as i32;
                    if rtouch >= 0 && rtouch < GRID_SIZE as i32 && ctouch >= 0 && ctouch < GRID_SIZE as i32 && !(c == 0 && r == 0) {
                        if self[rtouch as usize][ctouch as usize] { //this is terrible, pls fix TODO
                            ret += 1;
                        }
                    }
                }
            }
        ret
    }
    fn fill_col(&mut self, col : usize, except : &Grid) -> usize {
        let mut ret : usize = 0;
            for r in 0..GRID_SIZE as usize {
                if !except[r][col]{
                    self[r][col] = true;
                    ret += 1;
                }
            }
        ret
    }
    fn fill_row(&mut self, row : usize, except : &Grid) -> usize {
        let mut ret : usize = 0;
            for c in 0..GRID_SIZE as usize {
                if !except[row][c]{
                    self[row][c] = true;
                    ret += 1;
                }
            }
        ret
    }
    fn fill_sect(&mut self, sect : usize, sect_shape : & SectGrid, except : &Grid) -> usize {
      let mut ret : usize = 0;
        for r in 0..GRID_SIZE as usize {
            for c in 0..GRID_SIZE as usize {
                if !except[r][c] && sect_shape[r][c] == sect {
                    self[r][c] = true;
                    ret += 1;
                }
            }
        }
        ret
    }
    fn fill_touch(&mut self, row : usize, col : usize) -> usize {
        let mut ret : usize = 0;
            for r in -1..2 as i32{
                for c in -1..2 as i32 {
                    let rtouch = r + row as i32;
                    let ctouch = c + col as i32;
                    if rtouch >= 0 && rtouch < GRID_SIZE as i32 && ctouch >= 0 && ctouch < GRID_SIZE as i32 && !(c == 0 && r == 0) {
                        //this is terrible, pls fix TODO
                        self[rtouch as usize][rtouch as usize] = true;
                        ret += 1;
                        
                    }
                }
            }
        ret
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*println!("");
        println!("Annotations:");
        for r in 0..GRID_SIZE {
            print!("[");
            for c in 0..GRID_SIZE {
                print!("{}", if self.annotations[r][c] {"O"} else {"."});
            }
            println!("]");
        }
        println!("");
        println!("Stars:");
        for r in 0..GRID_SIZE {
            print!("[");
            for c in 0..GRID_SIZE {
                print!("{}", if self.stars[r][c] {"X"} else {"."});
            }
            println!("]");
        }
        println!("");
        println!("Sections:");
        for r in 0..GRID_SIZE {
            print!("[");
            for c in 0..GRID_SIZE {
                print!("{}", self.sect_shape[r][c]);
            }
            println!("]");
        }
        println!("");*/
        println!("Assembled:");
        let mut assemble : [[usize; GRID_SIZE * 2 - 1]; GRID_SIZE * 2 - 1] = [[0; GRID_SIZE * 2 - 1]; GRID_SIZE * 2 - 1];
        let atlas = vec!['.',/*'□'*/' ','|','-','+','X','O']; //don't edit, sum match code and setting code assumes this format
        for r in 0..(GRID_SIZE * 2 - 1) {
            for c in 0..(GRID_SIZE * 2 - 1) {
                if r % 2 == 1 {
                    if c % 2 == 1 {
                        assemble[r][c] = 0;
                    } else {
                        assemble[r][c] = if self.sect_shape[r/2][c/2] != self.sect_shape[r/2+1][c/2] {3} else {0};
                    }
                } else {
                    if c % 2 == 1 {
                        assemble[r][c] = if self.sect_shape[r/2][c/2] != self.sect_shape[r/2][c/2+1] {2} else {0};
                    } else {
                        assemble[r][c] = if self.stars[r/2][c/2] {5} else if self.annotations[r/2][c/2] {6} else {1};
                    }
                }
            }
        }
        for r in (1..(GRID_SIZE * 2 - 1)).step_by(2)  {
            for c in (1..(GRID_SIZE * 2 - 1)).step_by(2)  {
                match assemble[r][c+1] + assemble[r][c-1] + assemble[r-1][c] + assemble[r+1][c] {
                    4 => {assemble[r][c] = 2;},
                    6 => {assemble[r][c] = 3;},
                    2 | 3 | 5 | 7 | 8 | 10 => {assemble[r][c] = 4;},
                    0 => {},
                    other => {dbg!(other); panic!("unexpected characters in debug image assembly")}
                };
            }
        }
        for r in 0..(GRID_SIZE * 2 - 1) {
            print!("[");
            for c in 0..(GRID_SIZE * 2 - 1) {
                print!("{}", atlas[assemble[r][c]]);
            }
            println!("]");
        }
        Ok(())
    }
}

fn deduce(puzzle : &mut Puzzle) -> usize /*returns a count of how many changes made*/ {
    let mut ret : usize = 0;
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if puzzle.annotations[r][c] {
                continue;
            }
            if puzzle.annotations.count_col(c) >= GRID_SIZE - STARS || puzzle.annotations.count_row(r) >= GRID_SIZE - STARS || puzzle.annotations.count_sect(puzzle.sect_shape[r][c], &puzzle.sect_shape) >= GRID_SIZE - STARS {
                puzzle.stars[r][c] = true;
                ret += 1;
            }
        }
    }
    ret
}

fn annotate(puzzle : &mut Puzzle) -> usize /*returns a count of how many changes made*/ {
    let mut ret : usize = 0;
    //step 1; legal annotations
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if puzzle.stars[r][c] {
                ret += puzzle.annotations.fill_touch(r, c);
                if puzzle.stars.count_row(r) == STARS {
                    ret += puzzle.annotations.fill_row(r, &puzzle.stars);
                }
                if puzzle.stars.count_col(c) == STARS{
                    ret += puzzle.annotations.fill_col(c, &puzzle.stars);
                }
                if puzzle.stars.count_sect(puzzle.sect_shape[r][c], &puzzle.sect_shape) == STARS {
                    ret += puzzle.annotations.fill_sect(puzzle.sect_shape[r][c], &puzzle.sect_shape, &puzzle.stars)
                }
            }
        }
    }
    //step 2a; logical annotations AA (depth 1; if a section would force a star to be in a certain col/row, then blank the rest)
    //step 2b; logical annotations AB (if section would force a star to touch a certain cell, the blank it)
    //step 3; logical annotations B (if a star would block candidates enough to make it impossible, blank it)
    ret
}

fn backtrack(puzzle : &mut Puzzle, start : usize) -> Result<(), ()> {
    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    let row = start / GRID_SIZE;
    let col = start % GRID_SIZE;
    if col == 0 && row != 0 {
        if puzzle.stars.count_row(row - 1) != STARS { return Err(()); }
        if row == GRID_SIZE { return Ok(()); }
    }
    // dbg!(puzzle.stars.count_touch(row, col));
    // dbg!(puzzle.stars.count_col(col));
    // dbg!(puzzle.stars.count_row(row));
    // dbg!(puzzle.stars.count_sect(puzzle.sect_shape[row][col], &puzzle.sect_shape));
    if puzzle.placeable(row, col) {
        puzzle.stars[row][col] = true;
        // dbg!(&puzzle);
        // thread::sleep(ten_millis);
    } else {
        // dbg!(&puzzle);
        // dbg!(row);
        // dbg!(col);
        // let mut input = String::new();
        // match std::io::stdin().read_line(&mut input) {
        //     Ok(_goes_into_input_above) => {},
        //     Err(_no_updates_is_fine) => {},
        // }
        match backtrack(puzzle, start + 1) {
            Ok(()) => return Ok(()),
            Err(()) => return Err(())
        };
    }
    // dbg!(&puzzle);
    // dbg!(row);
    // dbg!(col);
    // let mut input = String::new();
    // match std::io::stdin().read_line(&mut input) {
    //     Ok(_goes_into_input_above) => {},
    //     Err(_no_updates_is_fine) => {},
    // }
    match backtrack(puzzle, start + 1) {
        Ok(()) => return Ok(()),
        Err(()) => {}
    };
    puzzle.stars[row][col] = false;
    //dbg!(&puzzle);
    backtrack(puzzle, start + 1)
}
 
fn main() {
    let mut puz = Puzzle {sect_shape:crate::examples::example(), stars : Default::default(), annotations : Default::default()};
    dbg!(&puz);
    // for r in 2..GRID_SIZE {
    //     puz.annotations[r][0] = true;
    // }
    // dbg!(&puz);
    // deduce(&mut puz);
    // dbg!(&puz);
    match backtrack(&mut puz, 0) {
        Ok(()) => (),
        Err(()) => ()
    };
    dbg!(&puz);
}

mod test {

    #[test]
    fn simple_board() {
        
    }
}