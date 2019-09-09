use rand::Rng;
use crate::tile::Tile;
use crate::board::Board;
use crate::tile::Position;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn random_coord() -> u32 {
    rand::thread_rng().gen_range(0,4)
}

pub fn init_board(){
    let mut board = Board::init_grid();
    let x1 = random_coord();
    let y1 = random_coord();
    let x2;
    let y2;
    while x1 == x2 && y1 == y2 {
        x2 = random_coord();
        y2 = random_coord();
    }
    board.set_tile_val(x1, y1, 2); 
    board.set_tile_val(x2, y2, 2);
    //TODO: set this so that whether it is a 2 or 4 is random, as below
}

pub fn generate_tile(&mut board: Board, dir: Direction) {
        let mut tile = Tile::new(0,0);
        let val_weight = rand::thread_rng().gen_range(0, 10);
        if val_weight > 6 {
            tile.set_val(4);
        } else {
            tile.set_val(2);
        }
        let mut ran_co = random_coord();
        let pos;
        match dir {
            Up => pos = Position{x:ran_co, y:3},
            Down => pos = Position{x:ran_co, y:0},
            Left => pos = Position{x:3, y: ran_co},
            Right => pos = Position{x:0,  y: ran_co}
        }
        //TODO: need to check if tile is None, and if so, set it, otherwise, find new tile
    }