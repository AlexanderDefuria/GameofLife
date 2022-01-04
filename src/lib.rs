extern crate core;

use std::collections::HashMap;
use std::rc::Rc;
use std::ops;
use std::ops::Add;
use std::hash::Hash;
use std::cell::RefCell;


pub struct Space {
    pub grid: HashMap<Point, Rc<RefCell<Cell>>>
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


pub fn get_alive_neighbours(space: &mut Space, cell: &Rc<RefCell<Cell>>) -> Vec<Rc<RefCell<Cell>>> {
    let mut output: Vec<Rc<RefCell<Cell>>> = Vec::new();

    for n in DIFF {
        if space.grid.contains_key(&(cell.borrow().pos+n)) {
            output.push(space.grid.get(&(cell.borrow().pos + n)).unwrap().clone());
        }
    }
    output
}

pub fn get_all_neighbours(space: &mut Space, cell: &Rc<RefCell<Cell>>) -> Vec<Rc<RefCell<Cell>>> {
    let mut output: Vec<Rc<RefCell<Cell>>> = get_alive_neighbours(space, cell);
    let mut alive : Vec<Point> = Vec::new();

    for n in output.iter() {
        alive.push(n.borrow().pos);
    }

    for n in DIFF {
        if !alive.contains(&(cell.borrow().pos + n)) {
            output.push(Rc::new(RefCell::new(Cell {
                alive: false,
                update: false,
                pos: cell.borrow().pos + n
            })))
        }
    }
    output
}

impl Space {

    pub fn setup(&mut self, pos: (i32, i32)) {
        let pos=Point{x:0,y:0}.add(pos);
        if !self.grid.contains_key(&pos) {
            self.grid.insert(pos, Rc::new(RefCell::from(Cell {
                alive: true,
                update: true,
                pos,
            })));
        }
    }

    pub fn get_column(self, x:i32, y1: i32, y2: i32) {
        // Send the column | that is desired and the range of said column.
        // Returns all indies that are alive.
        let mut out: Vec<i32> = Vec::new();
        for y in y1..y2 {
            let cell = self.grid.get(&Point{ x, y });
            if !cell.is_none() {
                out.push(cell.unwrap().borrow().pos.y);
            }
        }
    }

    pub fn evaluate(&mut self) {
        // Get number of cells in the grid.
        let size = self.grid.len();

        // Create a list of references to all cell in grid.
        let mut keys:Vec<&Rc<RefCell<Cell>>> = Vec::new();
        for x in self.grid.keys() {
            keys.push(self.grid.get(x).unwrap());
        }

        // Put all values from grid into a Vec of Rc<RefCell<Cell>>
        let mut update : Vec<(Point,Rc<RefCell<Cell>>)> = Vec::new();
        let mut cells: Vec<Rc<RefCell<Cell>>> = Vec::new();
        for _x in 0..size {
            cells.push(Rc::clone(keys.pop().unwrap()));
        }

        // For each entry in the grid.
        for _x in 0..size {
            // Get the current cell from the cells Vec<&Rc<RefCell<Cell>>>
            let cell: Rc<RefCell<Cell>> = cells.pop().unwrap();


            // This function is designed to fill all_neighbours with the
            let mut count_neighbours: i32 = 0;

            // Create a list of all relevant neighbours in all neighbours.
            let all_neighbours: Vec<Rc<RefCell<Cell>>> = get_all_neighbours(self, &cell);

            for n in all_neighbours.iter() {
                if !n.borrow().alive {
                    let count_neighbours = get_alive_neighbours(self, n).len();
                    if count_neighbours == 3 {
                        update.push((n.borrow().pos, Rc::new(RefCell::new(Cell {
                            alive: false,
                            update: true,
                            pos: n.borrow().pos
                        }))));
                    }
                } else {
                    count_neighbours += n.borrow().alive as i32;
                }
            }

            let mut ref_cell = cell.borrow_mut();
            match count_neighbours {
                2 | 3 => { ref_cell.update = true; }
                _ => { ref_cell.update = false; }
            }

            // Debug - Display the current cell.
            //print!("{} Neighbours; ", count_neighbours);
            //println!("{:?},\tNeighbours: {:?}", ref_cell, all_neighbours);

        }
        let mut size = update.len();
        for _x in 0..size {
            let cell = update.pop().unwrap();
            if cell.1.borrow().update {
                self.grid.insert(cell.0, cell.1);
            }
        }
        let mut kill: Vec<Point> = Vec::new();
        for x in self.grid.values() {
            let mut actual = x.borrow_mut();
            actual.alive = actual.update;
            if actual.alive == false {
                kill.push(actual.pos.clone())
            }
        }
        size = kill.len();
        for _x in 0..size {
            self.grid.remove(&kill.pop().unwrap());
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

