use rand::Rng;
use std::fmt::{self, Display};

#[derive(Clone)]
struct Cell {
    walls: u8,
}

impl Cell {
    fn new() -> Cell {
        return Cell { walls: 15 };
    }
    fn north(&self) -> bool {
        return (self.walls & 8) != 0;
    }
    fn south(&self) -> bool {
        return (self.walls & 4) != 0;
    }
    fn east(&self) -> bool {
        return (self.walls & 2) != 0;
    }
    fn west(&self) -> bool {
        return (self.walls & 1) != 0;
    }
}

struct Maze {
    grid: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new(w: usize, h: usize) -> Maze {
        let mut grid: Vec<Cell> = Vec::new();
        let cell = Cell::new();
        for _ in 0..(h*w) {
            grid.push(cell.clone());
        }
        return Maze { 
            grid: grid.clone(),
            width: w,
            height: h,
        };
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..(self.height) {
            for i in 0..2 {
                for x in 0..(self.width) {
                    if i == 0 {
                        if self.grid[y*self.width+x].north() {
                            write!(f, "+--")?;
                        } else {
                            write!(f, "+  ")?;
                        }
                        if x == self.width-1 {
                            write!(f, "+\n")?;
                        }
                    } else if i == 1 {
                        if self.grid[y*self.width+x].west() {
                            write!(f, "|  ")?;
                        }
                        else {
                            write!(f, "   ")?;
                        }
                        if x == self.width-1 {
                            if self.grid[y*self.width+x].east() {
                                write!(f, "|\n")?;
                            }
                            else {
                                write!(f, " \n")?;
                            }
                        }
                    }
                }
            }
            if y == self.height-1 {
                for x in 0..(self.width) {
                    if self.grid[y*self.width+x].south() {
                        write!(f, "+--")?;
                    } else {
                        write!(f, "+  ")?;
                    }
                }
            }
        }
        write!(f, "+\n")
    }
}

fn main() {
    println!("{}", binary_tree_maze(10, 10));
}

fn binary_tree_maze(w: usize, h: usize) -> Maze {
    let mut maze = Maze::new(w, h);
    for x in 0..w {
        for y in 0..h {
            if x == w-1 && y == 0 {
                // nothing
            } else if x == w-1 {
                maze.grid[y*w+x].walls &= 7;
                maze.grid[(y-1)*w+x].walls &= 11;
            } else if y == 0 {
                maze.grid[y*w+x].walls &= 13;
                maze.grid[y*w+x+1].walls &= 14;
            } else {
                if rand::thread_rng().gen_range(0, 2) == 0 {
                    maze.grid[y*w+x].walls &= 7;
                    maze.grid[(y-1)*w+x].walls &= 11;
                } else {
                    maze.grid[y*w+x].walls &= 13;
                    maze.grid[y*w+x+1].walls &= 14;
                }
            }
        }
    }
    return maze;
}
