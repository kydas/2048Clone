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
    fn new(x: u32, y: u32) -> Self {
        let pos = Position{x: x, y:y};
        let tile = Tile{pos: pos, val: None};
        tile
    }
    fn merge(&mut self, mTile: &mut Tile){
        if self.val == mTile.val {
            mTile.val = None;
            match self.val {
                None => self.val = mTile.val,
                Some(i) => self.val = Some(i*2),
            };
        };
    }
    fn set_val(&mut self, value: u32){
        match self.val {
            None => self.val = Some(value),
            Some(i) => self.val = Some(value),
        };
    }
}

#[cfg(test)]
mod tile_tests {
    use super::*;
    #[test]
    fn test_new() {
        let test_tile = Tile::new(0, 1);
        assert_eq!(None, test_tile.val);
        assert_eq!(0, test_tile.pos.x);
        assert_eq!(1, test_tile.pos.y);
    }
    #[test]
    fn test_merge_same_val(){
        let mut test_tile1 = Tile::new(0, 1);
        test_tile1.set_val(4);
        let mut test_tile2 = Tile::new(1,1);
        test_tile2.set_val(4);
        test_tile1.merge(&mut test_tile2);
        assert_eq!(8, test_tile1.val.unwrap());
        assert_eq!(None, test_tile2.val);
    }

    #[test]
    fn test_merge_none_some() {
        let mut test_tile = Tile::new(0,1);
        let mut test_tile2 = Tile::new(1,1);
        test_tile2.set_val(8);
        test_tile.merge(&mut test_tile2);
        assert_eq!(8, test_tile.val.unwrap());
        assert_eq!(None, test_tile2.val);
    }
}