extern crate core;

use std::collections::{ HashMap};
use std::rc::Rc;
use std::ops;
use std::ops::{Add, Deref};
use std::hash::Hash;
use std::borrow::{Borrow, BorrowMut};


pub struct Space {
    pub grid: HashMap<Point, Rc<Cell>>
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub alive: bool,
    pub update: bool,
    pub pos: Point
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

static DIFF: [(i32, i32); 8] = [(1, 1),(1, 0),(1, -1),(0, -1),(-1, -1),(-1, 0),(-1, 1),(0, 1)];


pub fn get_alive_neighbours(space: &mut Space, cell: &Rc<Cell>) -> Vec<Rc<Cell>> {
    let mut output: Vec<Rc<Cell>> = vec![];

    for n in DIFF {
        if space.grid.contains_key(&(cell.pos+n)) {
            output.push(space.grid.get(&(cell.pos + n)).unwrap().clone());
        }
    }
    output
}
pub fn get_all_neighbours(space: &mut Space, cell: &Rc<Cell>) -> Vec<Rc<Cell>> {
    let mut output: Vec<Rc<Cell>> = vec![];
    let alive_rc = get_alive_neighbours(space, cell);
    let mut alive : Vec<Point> = Vec::new();
    for n in alive_rc {
        alive.push(n.pos);
        output.push(n.clone());
    }

    for n in DIFF {
        if !alive.contains(&(cell.pos + n)) {
            output.push(Rc::new(Cell {
                alive: false,
                update: false,
                pos: cell.pos+n
            }))
        }
    }
    output
}

impl Space {

    pub fn setup(&mut self, pos: (i32, i32)) ->  Rc<Cell> {
        let pos=Point{x:0,y:0}.add(pos);
        if !self.grid.contains_key(&pos) {
            let cell = Rc::new(Cell {
                alive: true,
                update: true,
                pos,
            });
            self.grid.insert(pos, Rc::clone(&cell));
        }
        Rc::clone(self.grid.get(&pos).unwrap())
    }

    pub fn get_column(self, x:i32, y1: i32, y2: i32) {
        // Send the column | that is desired and the range of said column.
        // Returns all indies that are alive.
        let mut out: Vec<i32> = Vec::new();
        for y in y1..y2 {
            let cell = self.grid.get(&Point{ x, y });
            if !cell.is_none() {
                out.push(cell.unwrap().pos.y);
            }
        }
    }

    pub fn evaluate(&mut self) {
        // Get number of cells in the grid.
        let size = self.grid.len();

        // Create a list of references to all cell in grid.
        let mut keys:Vec<&Rc<Cell>> = Vec::new();
        for x in self.grid.keys() {
            keys.push(self.grid.get(x).unwrap());
        }

        // Put all values from grid into a Vec of Rc<Cell>
        let mut update : Vec<(Point,Rc<Cell>)> = Vec::new();
        let mut cells: Vec<Rc<Cell>> = Vec::new();
        for _x in 0..size {
            cells.push(Rc::clone(keys.pop().unwrap()));
        }

        // For each entry in the grid.
        for _x in 0..size {
            // Get the current cell from the cells Vec<&Rc<Cell>>
            let cell: Rc<Cell> = cells.pop().unwrap();
            let mut ref_cell: Cell = *cell.borrow();


            // Create a list of all relevant neighbours in all neighbours.
            let mut alive_neighbours: Vec<Rc<Cell>> =  get_alive_neighbours( self,&cell);
            let mut all_neighbours: Vec<Rc<Cell>> = Vec::new();

            // Alive neighbours is used to help create dummy nodes.

            // This function is designed to fill all_neighbours with the
            let mut count_neighbours: i32 = 0;
            /*
            for diff in DIFF {
                let neighbour = alive_neighbours.last();
                if neighbour == None {
                    all_neighbours.push(Rc::new(Cell {
                        alive: false,
                        update: false,
                        pos: ref_cell.pos + diff
                    }));
                } else{ // if cell.pos + diff == neighbour.unwrap().pos
                    all_neighbours.push(alive_neighbours.pop().unwrap().clone());

                    count_neighbours+=1;
                }
                println!("{:?}", all_neighbours.last().unwrap().pos);
            } */

            all_neighbours = get_all_neighbours(self, &cell);


            for n in all_neighbours.iter() {
                if !n.alive {
                    let mut count_neighbours = 0;
                    /*
                    for diff in DIFF {
                        let neighbour = self.grid.get(&(x.pos + diff));
                        if neighbour != None {
                            count_neighbours += neighbour.unwrap().alive as i32;
                        }
                    }

                     */
                    count_neighbours = get_alive_neighbours(self, n).len();
                    if count_neighbours == 3 {
                        ref_cell.update = true;
                        let temp = Rc::new(Cell {
                            alive: false,
                            update: true,
                            pos: n.pos
                        });
                        update.push((n.pos, temp.clone()));
                    } else {

                    }
                }
            }

            for n in 0..8 {
                count_neighbours += all_neighbours.get(n).unwrap().alive as i32;
            }

            match count_neighbours {
                2 | 3 => { ref_cell.update = true; }
                _ => { ref_cell.update = false; }
            }

            // Debug - Display the current cell.
            //print!("{} Neighbours; ", count_neighbours);
            //println!("{:?},\tNeighbours: {:?}", ref_cell, all_neighbours);

            self.grid.insert(ref_cell.pos, Rc::new(ref_cell));

        }
        let mut size = update.len();
        for _x in 0..size {
            let cell = update.pop().unwrap();

            if cell.1.update {
                self.grid.insert(cell.0, cell.1);
            }

        }
        let mut temp: Vec<Rc<Cell>> = Vec::new();
        for x in self.grid.values() {
            let cell = Rc::clone(x);
            let mut actual = *cell.deref();
            actual.alive = actual.update;
            temp.push(Rc::new(actual));
        }
        size = temp.len();
        for _x in 0..size {
            let cell = temp.pop().unwrap();
            if !cell.alive {
                self.grid.remove(&cell.pos);
            } else {
                self.grid.insert(cell.pos, Rc::clone(&cell));
            }
        }


        println!("Grid:\t{:?}", self.grid.keys());
    }




}


impl Cell {
    pub fn update (&mut self, next: bool) {
        self.update = next;
    }
}


impl ops::Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {

        Point{ x: self.x + rhs.0, y: self.y + rhs.1}
    }
}

impl PartialEq for Point{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

