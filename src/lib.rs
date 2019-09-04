use rand::Rng;
use crate::tile::Tile;
use crate::board::Board;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn generate_tile(&mut board: Board, dir: Direction) {
        let mut tile = Tile::new(0,0);
        tile.set_val(2);
        let mut pos = rand::thread_rng().gen_range(0, 4);
    }