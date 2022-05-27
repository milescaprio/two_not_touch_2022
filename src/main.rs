mod examples;
//use crate::examples;
use std::fmt;
const STARS : usize = 2;
const GRID_SIZE : usize = 10;
const ADJ : usize = 8;

struct Puzzle {
    sect_shape : [[usize; GRID_SIZE]; GRID_SIZE],
    stars : [[bool; GRID_SIZE]; GRID_SIZE],
    annotations : [[bool; GRID_SIZE]; GRID_SIZE]
}

impl Puzzle {
    fn count_sect(&self, sect : usize) -> usize {
        let mut ret : usize = 0;
        for r in 0..GRID_SIZE as usize {
            for c in 0..GRID_SIZE as usize {
                if self.sect_shape[r][c] == sect {
                    ret += self.stars[r][c] as usize;
                }
            }
        }
        ret
    }
    fn count_row(&self, row : usize) -> usize {
        let mut ret : usize = 0;
        for c in 0..GRID_SIZE as usize {
            ret += self.stars[row][c] as usize
        }
        ret
    }
    fn count_col(&self, col : usize) -> usize {
        let mut ret : usize = 0;
        for r in 0..GRID_SIZE as usize {
            ret += self.stars[r][col] as usize
        }
        ret
    }
    fn count_touch(&self, row : usize, col : usize) -> usize {
        let mut ret : usize = 0;
        for r in -1..2 as i32{
            for c in -1..2 as i32 {
                if c != 0 && r != 0 {
                    ret += self.stars[r as usize + row ][c as usize + col] as usize;
                }
            }
        }
        ret
    }

    fn count_sect_anno(&self, sect : usize) -> usize {
        let mut ret : usize = 0;
        for r in 0..GRID_SIZE as usize {
            for c in 0..GRID_SIZE as usize {
                if self.sect_shape[r][c] == sect {
                    ret += self.annotations[r][c] as usize;
                }
            }
        }
        ret
    }
    fn count_row_anno(&self, row : usize) -> usize {
        let mut ret : usize = 0;
        for c in 0..GRID_SIZE as usize {
            ret += self.annotations[row][c] as usize
        }
        ret
    }
    fn count_col_anno(&self, col : usize) -> usize {
        let mut ret : usize = 0;
        for r in 0..GRID_SIZE as usize {
            ret += self.annotations[r][col] as usize
        }
        ret
    }
    fn count_touch_anno(&self, row : usize, col : usize) -> usize {
        let mut ret : usize = 0;
        for r in -1..2 as i32{
            for c in -1..2 as i32 {
                if c != 0 && r != 0 {
                    ret += self.annotations[r as usize + row ][c as usize + col] as usize;
                }
            }
        }
        ret
    }
    /*fn sect(&self, row : usize, col : usize) -> usize {
        self.sect_shape[row][col]
    }*/
    fn is_legal(&self, row : usize, col : usize) -> bool {
        self.count_touch(row, col) == 0 && self.count_col(col) < STARS && self.count_row(row) < STARS && self.count_sect(self.sect_shape[row][col]) < STARS
    }
}

/*trait TNTRules {
    fn count_col() -> usize;
    fn count_row() -> usize;
    fn count_sect() -> usize;
    fn count_touch() -> usize;
}

impl<T : bool, const N: usize> TNTRules for [[T; N]; N] {


}*/

impl Default for Puzzle {
    fn default() -> Puzzle {
        Puzzle {
            sect_shape : [[0; GRID_SIZE]; GRID_SIZE],
            stars : [[false; GRID_SIZE]; GRID_SIZE],
            annotations : [[false; GRID_SIZE]; GRID_SIZE]
        }
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         //.field("sect_shape", &self.sect_shape)
         .field("stars", &self.stars)
         .field("annotations", &self.annotations)
         .finish()
    }
}

fn deduce(puzzle : &mut Puzzle) -> i32 /*returns a count of how many changes made*/ {
    let mut ret : i32 = 0;
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if puzzle.annotations[r][c] == true {
                continue;
            }
            if puzzle.count_col_anno(c) >= GRID_SIZE - STARS || puzzle.count_row_anno(r) >= GRID_SIZE - STARS || puzzle.count_sect_anno(puzzle.sect_shape[r][c]) >= GRID_SIZE - STARS {
                puzzle.stars[r][c] = true;
                ret += 1;
            }
        }
    }
    ret
}

fn annotations(puzzle : &mut Puzzle) -> i32 /*returns a count of how many changes made*/ {
    todo!();
}


 
fn main() {
    let mut puz = Puzzle {sect_shape:crate::examples::example(), stars : [[false; GRID_SIZE]; GRID_SIZE], annotations : [[false; GRID_SIZE]; GRID_SIZE]};
    dbg!(&puz);
    for r in 2..GRID_SIZE { //not 0 or 1 because we need to exclude them to have one empty space
        puz.annotations[r][0] = true;
    }
    dbg!(&puz);
    deduce(&mut puz);
    dbg!(&puz);
}