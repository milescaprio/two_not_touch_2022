mod examples;

use std::fmt;

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
    fn is_legal(&self, row : usize, col : usize) -> bool {
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
                    if c != 0 && r != 0 && self[r as usize + row][c as usize + col]{
                        ret += 1;
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
                    if c != 0 && r != 0 {
                        self[r as usize + row][c as usize + col] = true;
                        ret += 1;
                    }
                }
            }
        ret
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("");
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

fn annotations(puzzle : &mut Puzzle) -> usize /*returns a count of how many changes made*/ {
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

fn backtrack(puzzle : &mut Puzzle) {
    todo!();
}
 
fn main() {
    let mut puz = Puzzle {sect_shape:crate::examples::example(), stars : Default::default(), annotations : Default::default()};
    dbg!(&puz);
    for r in 2..GRID_SIZE {
        puz.annotations[r][0] = true;
    }
    dbg!(&puz);
    deduce(&mut puz);
    dbg!(&puz);
}