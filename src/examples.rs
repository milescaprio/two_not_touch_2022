const STARS : usize = 2;
const GRID_SIZE : usize = 10;

pub fn example() -> [[usize; GRID_SIZE]; GRID_SIZE] { 
[
    [0,0,0,0,0,0,0,0,1,1],
    [0,0,0,0,0,0,2,0,1,1],
    [3,4,4,4,0,2,2,2,1,1],
    [3,3,3,3,5,6,6,2,1,1],
    [3,3,3,5,5,5,6,6,1,1],
    [3,3,7,5,5,5,6,6,1,1],
    [3,3,7,7,7,6,6,6,8,1],
    [3,3,3,3,3,3,6,6,8,1],
    [3,9,9,9,9,9,9,9,8,8],
    [9,9,9,9,9,9,9,9,9,8]
]
}