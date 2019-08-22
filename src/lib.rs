ust std::collections::HashSet;
use crate::tile::Tile;
use crate::board::Board;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn generate_tile(&mut board, dir: Direction, &mut empty_tiles: HashSet<&Position>) {
        let mut tile = Tile::new(0,0);
        let mut pos = rand::thread_rng().gen_range(0, 4);
    }