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

#[derive(Clone, Copy)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy)]
struct Tile {
    pos: Position,
    val: Option<u32>
}

impl Tile {
    fn new(x: u32, y: u32) -> Self {
        let pos = Position{x: x, y:y};
        let tile = Tile{pos: pos, val: None};
        tile
    }

    fn merge(&mut self, m_tile: &mut Tile) {
        if self.val == m_tile.val {
            match self.val {
                None => self.val = m_tile.val,
                Some(i) => self.val = Some(i*2),
            };
            m_tile.val = None;
        };
    }
    fn set_val(&mut self, value: u32){
        match self.val {
            None => self.val = Some(value),
            Some(_i) => self.val = Some(value),
        };
    }

    fn mov(&mut self, other: &mut Tile) {
        match self.val {
            None => {
                self.val = other.val;
                other.val = None;
            },
            Some(_i) => ()
        };
    }
}


#[derive(Clone)]
struct Board{
    grid: Vec<Vec<Tile>>,
    score: Option<u32>,
}

impl Board {

    fn init_grid() -> Self{
        let mut row: Vec<Vec<Tile>> = Vec::new();
        for x in 0..5 {
            let mut col: Vec<Tile> = Vec::new();
            for y in 0..5 {
                col.push(Tile::new(x, y));
            }
            row.push(col);
        }
        let board = Board{grid: row, score: None};
        board
    }

    fn mov_left(&mut self) {
        let grid = &mut self.grid;
        // 0 .. 4 because we don't want to go off end of array
        for y in 0..5 {
            for x in 0..4{
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x+1][y].clone();
                if curr.val == None {
                    curr.mov(next);
                    grid[x][y] = curr.clone();
                    grid[x+1][y] = next.clone();
                }
            }
        }
    }

    fn mov_right(&mut self) {
        let grid = &mut self.grid;
        // 5 .. 1  because we don't want to go off end of array
        for y in 0..5 {
            for x in (1..5).rev(){
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x-1][y].clone();
                if curr.val == None {
                    curr.mov(next);
                    grid[x][y] = curr.clone();
                    grid[x-1][y] = next.clone();
                }
            }
        }
    }

    fn mov_up(&mut self) {
        let grid = &mut self.grid;
        // 5 .. 1  because we don't want to go off end of array
        for x in 0..5 {
            for y in 0..4{
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x][y+1].clone();
                if curr.val == None {
                    curr.mov(next);
                    grid[x][y] = curr.clone();
                    grid[x][y+1] = next.clone();
                }
            }
        }
    }

    fn mov_down(&mut self) {
        let grid = &mut self.grid;
        // 5 .. 1  because we don't want to go off end of array
        for x in 0..5 {
            for y in (1..5).rev(){
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x][y-1].clone();
                if curr.val == None {
                    curr.mov(next);
                    grid[x][y] = curr.clone();
                    grid[x][y-1] = next.clone();
                }
            }
        }    
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
        assert_eq!(None, test_tile.val);
        assert_eq!(8, test_tile2.val.unwrap());
    }

    #[test]
    fn test_mov() {
        let mut test_tile = Tile::new(0,1);
        let mut test_tile2 = Tile::new(1,1);
        test_tile2.set_val(8);
        test_tile.mov(&mut test_tile2);
        assert_eq!(None, test_tile2.val);
        assert_eq!(8, test_tile.val.unwrap()); 
    }

    mod board_tests {
        use super::*;
        #[test]
        fn test_mov_left_one(){
            let mut test_board = Board::init_grid();
            test_board.grid[1][0].set_val(2);
            test_board.mov_left();
            assert_eq!(2, test_board.grid[0][0].val.unwrap());
        }

        #[test]
        fn test_mov_right_one(){
            let mut test_board = Board::init_grid();
            test_board.grid[1][0].set_val(2);
            test_board.mov_right();
            assert_eq!(2, test_board.grid[2][0].val.unwrap());
        }

        #[test]
        fn test_mov_up_one() {
            let mut test_board = Board::init_grid();
            test_board.grid[1][2].set_val(2);
            test_board.mov_up();
            assert_eq!(2, test_board.grid[1][1].val.unwrap());
        }

        #[test]
        fn test_mov_down_one() {
            let mut test_board = Board::init_grid();
            test_board.grid[1][2].set_val(2);
            test_board.mov_down();
            assert_eq!(2, test_board.grid[1][3].val.unwrap());
        }
    }
}