//use std::env;


fn main() {
    //let mut args = env::args();
    let s = String::from("\n
                          | 2 |   |   |   |   |\n
                          |   |   |   |   |   |\n
                          |   |   |   |   |   |\n
                          |   |   |   |   |   |\n
                          |   |   |   |   |   |\n");

    println!("{}", s);
}

struct Position {
    x: u32,
    y: u32,
}
struct Tile {
    pos: Position,
    val: Option<u32>
}

struct Board {
    grid: Vec<Vec<Tile>>,
    score: Option<u32>,
}

impl Tile {
    fn new<'a>(x: u32, y: u32) -> Self {
        let pos = Position{x: x, y:y};
        let tile = Tile{pos: pos, val: None};
        tile
    }
}