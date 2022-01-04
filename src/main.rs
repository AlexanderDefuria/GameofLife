extern crate game_of_life;

use game_of_life::{Space};


fn main() {
    let mut x = Space {
        grid: Default::default()
    };
    x.setup(( 0, 0));
    x.setup(( 0, 1));
    x.setup(( 0, 2));
    x.setup(( 1, 2));
    x.setup((-1, 1));


    for y in 1..10000 {
        print!("Gen: {}  \t", y);
        x.evaluate();
    }







    println!("Done")
}




