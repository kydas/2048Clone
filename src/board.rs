use crate::tile::Tile;


#[derive(Clone)]
pub struct Board{
    pub grid: Vec<Vec<Tile>>,
    pub score: Option<u32>,
}

impl Board {

    pub fn init_grid() -> Self{
        let mut row: Vec<Vec<Tile>> = Vec::new();
        for x in 0..4 {
            let mut col: Vec<Tile> = Vec::new();
            for y in 0..4 {
                col.push(Tile::new(x, y));
            }
            row.push(col);
        }
        let board = Board{grid: row, score: None};
        board
    }

    fn mov_left(&mut self) {
        let grid = &mut self.grid;
        // 0 .. 3 because we don't want to go off end of array
        for y in 0..4 {
            let mut x = 0;
            while x < 3 {
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x+1][y].clone();
                if curr.get_val() == None && next.get_val() != None {
                    curr.mov(next);
                    grid[x][y] = curr.clone();
                    grid[x+1][y] = next.clone();
                    if x != 0 {
                        x -= 1; 
                    } else {
                        x += 1;
                    }
                } else {
                    x += 1;
                }
            }
        }
    }

    fn mov_right(&mut self) {
        let grid = &mut self.grid;
        // 5 .. 1  because we don't want to go off end of array ... TODO this comment is stale
        for y in 0..4 {
            let mut x = 3;
            while x > 0 {
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x-1][y].clone();
                if curr.get_val() == None && next.get_val() != None {
                    curr.mov(next);
                    grid[x][y] = curr.clone();
                    grid[x-1][y] = next.clone();
                    if x != 3 {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                } else {
                    x -= 1;
                }
            }
        }
    }

    fn mov_up(&mut self) {
        let grid = &mut self.grid;
        // 5 .. 1  because we don't want to go off end of array
        for x in 0..4 {
            let mut y = 0;
            while y < 3 {
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x][y+1].clone();
                if curr.get_val() == None && next.get_val() != None {
                    curr.mov(next);
                    grid[x][y] = curr.clone();
                    grid[x][y+1] = next.clone();
                    if y != 0 {
                        y -= 1;
                    } else {
                        y += 1;
                    }
                } else {
                    y += 1;
                }
            }
        }
    }

    fn mov_down(&mut self) {
        let grid = &mut self.grid;
        // 5 .. 1  because we don't want to go off end of array
        for x in 0..4 {
            let mut y = 3;
            while y > 0 {
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x][y-1].clone();
                if curr.get_val() == None && next.get_val() != None {
                    curr.mov(next);
                    grid[x][y] = curr.clone();
                    grid[x][y-1] = next.clone();
                    if y != 3 {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                } else {
                    y -= 1;
                }   
            }
        }    
    }

    pub fn merge_left(&mut self) {
        self.mov_left();
        let grid = &mut self.grid;
        for y in 0 .. 4 {
            for x in 0 .. 3 {
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x+1][y].clone();
                if curr.get_val() == next.get_val() && curr.get_val() != None {
                    curr.merge(next);
                    grid[x][y] = curr.clone();
                    grid[x+1][y] = next.clone();
                }
                
            }
        }
        self.mov_left();
    }

    pub fn merge_right(&mut self){
        self.mov_right();
        let grid = &mut self.grid;
        for y in 0 .. 4 {
            for x in (1 .. 4).rev() {
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x-1][y].clone();
                if curr.get_val() == next.get_val() && curr.get_val() != None {
                    curr.merge(next);
                    grid[x][y] = curr.clone();
                    grid[x-1][y] = next.clone();
                }
                
            }
        }
        self.mov_right();
    }

    pub fn merge_up(&mut self){
        self.mov_up();
        let grid = &mut self.grid;
        for x in 0 .. 4 {
            for y in 0 .. 3 {
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x][y+1].clone();
                if curr.get_val() == next.get_val() && curr.get_val() != None {
                    curr.merge(next);
                    grid[x][y] = curr.clone();
                    grid[x][y+1] = next.clone();
                }
                
            }
        }
        self.mov_up();
    }

    pub fn merge_down(&mut self){
        self.mov_down();
        let grid = &mut self.grid;
        for x in 0 .. 4 {
            for y in (1 .. 4).rev() {
                let curr = &mut grid[x][y].clone();
                let next = &mut grid[x][y-1].clone();
                if curr.get_val() == next.get_val() && curr.get_val() != None {
                    curr.merge(next);
                    grid[x][y] = curr.clone();
                    grid[x][y-1] = next.clone();
                }
                
            }
        }
        self.mov_down();
    }

    pub fn set_tile_val(&mut self, x: usize, y: usize, val: u32) {
        let grid = &mut self.grid;
        let mut tile = Tile::new(x,y);
        tile.set_val(val);
        let x_co = x;
        let y_co = y;
        grid[x_co][y_co] =  tile;
    }

}
#[cfg(test)]
mod board_tests {
    use super::*;
    #[test]
    fn test_mov_left_one(){
        let mut test_board = Board::init_grid();
        test_board.grid[1][0].set_val(2);
        test_board.mov_left();
        assert_eq!(2, test_board.grid[0][0].get_val().unwrap());
    }

    #[test]
    fn test_mov_left_across_entire_board(){
        let mut test_board = Board::init_grid();
        test_board.grid[3][0].set_val(2);
        test_board.mov_left();
        assert_eq!(2, test_board.grid[0][0].get_val().unwrap());
    }

    #[test]
    fn test_mov_left_many() {
        let mut test_board = Board::init_grid();
        test_board.grid[3][0].set_val(2);
        test_board.grid[2][0].set_val(2);
        test_board.mov_left();
        assert_eq!(2, test_board.grid[0][0].get_val().unwrap());
        assert_eq!(2, test_board.grid[1][0].get_val().unwrap());
        assert_eq!(None, test_board.grid[2][0].get_val());
        assert_eq!(None, test_board.grid[3][0].get_val());
    }
    
    #[test]
    fn test_mov_left_many_diff_rows(){
        let mut test_board = Board::init_grid();
        test_board.grid[3][0].set_val(2);
        test_board.grid[3][3].set_val(2);
        test_board.mov_left();
        assert_eq!(2, test_board.grid[0][0].get_val().unwrap());
        assert_eq!(2, test_board.grid[0][3].get_val().unwrap());
        assert_eq!(None, test_board.grid[2][0].get_val());
        assert_eq!(None, test_board.grid[3][3].get_val());
    }

    #[test]
    fn test_mov_right_one(){
        let mut test_board = Board::init_grid();
        test_board.grid[0][0].set_val(2);
        test_board.mov_right();
        assert_eq!(2, test_board.grid[3][0].get_val().unwrap());
    }

    #[test]
    fn test_mov_right_many(){
        let mut test_board = Board::init_grid();
        test_board.grid[0][0].set_val(2);
        test_board.grid[1][0].set_val(2);
        test_board.mov_right();
        assert_eq!(2, test_board.grid[3][0].get_val().unwrap());
        assert_eq!(2, test_board.grid[2][0].get_val().unwrap());
        assert_eq!(None, test_board.grid[0][0].get_val());
        assert_eq!(None, test_board.grid[1][0].get_val());
    }

    #[test]
    fn test_mov_up_one() {
        let mut test_board = Board::init_grid();
        test_board.grid[1][2].set_val(2);
        test_board.mov_up();
        assert_eq!(2, test_board.grid[1][0].get_val().unwrap());
    }

    #[test]
    fn test_mov_up_many(){
        let mut test_board = Board::init_grid();
        test_board.grid[1][3].set_val(2);
        test_board.grid[1][2].set_val(2);
        test_board.grid[0][3].set_val(2);
        test_board.mov_up();
        assert_eq!(2, test_board.grid[1][0].get_val().unwrap());
        assert_eq!(2, test_board.grid[1][1].get_val().unwrap());
        assert_eq!(2, test_board.grid[0][0].get_val().unwrap());
    }

    #[test]
    fn test_mov_down_one() {
        let mut test_board = Board::init_grid();
        test_board.grid[1][0].set_val(2);
        test_board.mov_down();
        assert_eq!(2, test_board.grid[1][3].get_val().unwrap());
    }

    #[test]
    fn test_mov_down_many() {
        let mut test_board = Board::init_grid();
        test_board.grid[1][0].set_val(2);
        test_board.grid[1][1].set_val(2);
        test_board.grid[0][0].set_val(2);
        test_board.mov_down();
        assert_eq!(2, test_board.grid[1][3].get_val().unwrap());
        assert_eq!(2, test_board.grid[1][2].get_val().unwrap());
        assert_eq!(2, test_board.grid[0][3].get_val().unwrap());
    }


    // MERGE TESTS 

    #[test]
    fn test_merge_left_simple() {
        let mut test_board = Board::init_grid();
        test_board.grid[3][0].set_val(2);
        test_board.grid[2][0].set_val(2);
        test_board.merge_left();
        assert_eq!(4, test_board.grid[0][0].get_val().unwrap());
        assert_eq!(None, test_board.grid[1][0].get_val());
        assert_eq!(None, test_board.grid[2][0].get_val());
        assert_eq!(None, test_board.grid[3][0].get_val());
    }

    #[test]
    fn test_merge_left_multiple_same(){
        let mut test_board = Board::init_grid();
        test_board.grid[3][0].set_val(2);
        test_board.grid[2][0].set_val(2);
        test_board.grid[1][0].set_val(2);
        test_board.grid[0][0].set_val(2);
        test_board.merge_left();
        assert_eq!(4, test_board.grid[0][0].get_val().unwrap());
        assert_eq!(4, test_board.grid[1][0].get_val().unwrap());
        assert_eq!(None, test_board.grid[2][0].get_val());
        assert_eq!(None, test_board.grid[3][0].get_val());
    }

    #[test]
    fn test_merge_left_multiple_rows_mixed_values(){
        let mut test_board = Board::init_grid();
        test_board.grid[3][0].set_val(2);
        test_board.grid[2][0].set_val(2);
        test_board.grid[1][0].set_val(8);
        test_board.grid[0][0].set_val(2);
        test_board.grid[2][3].set_val(4);
        test_board.grid[1][3].set_val(4);
        test_board.merge_left();
        assert_eq!(2, test_board.grid[0][0].get_val().unwrap());
        assert_eq!(8, test_board.grid[1][0].get_val().unwrap());
        assert_eq!(4, test_board.grid[2][0].get_val().unwrap());
        assert_eq!(None, test_board.grid[3][0].get_val());
        assert_eq!(8, test_board.grid[0][3].get_val().unwrap());
        assert_eq!(None, test_board.grid[1][3].get_val());
    }

    #[test]
    fn test_merge_right_simple() {
        let mut test_board = Board::init_grid();
        test_board.grid[0][0].set_val(2);
        test_board.grid[1][0].set_val(2);
        test_board.merge_right();
        assert_eq!(4, test_board.grid[3][0].get_val().unwrap());
        assert_eq!(None, test_board.grid[1][0].get_val());
        assert_eq!(None, test_board.grid[2][0].get_val());
        assert_eq!(None, test_board.grid[0][0].get_val());
    }

    #[test]
    fn test_merge_right_multiple_rows_mixed_values() {
        let mut test_board = Board::init_grid();
        test_board.grid[3][0].set_val(2);
        test_board.grid[2][0].set_val(2);
        test_board.grid[1][0].set_val(8);
        test_board.grid[0][0].set_val(2);
        test_board.grid[2][3].set_val(4);
        test_board.grid[1][3].set_val(4);
        test_board.merge_right();
        assert_eq!(None, test_board.grid[0][0].get_val());
        assert_eq!(2, test_board.grid[1][0].get_val().unwrap());
        assert_eq!(8, test_board.grid[2][0].get_val().unwrap());
        assert_eq!(4, test_board.grid[3][0].get_val().unwrap());
        assert_eq!(8, test_board.grid[3][3].get_val().unwrap());
        assert_eq!(None, test_board.grid[1][3].get_val());
    }

    #[test]
    fn test_merge_up_simple() {
        let mut test_board = Board::init_grid();
        test_board.grid[0][0].set_val(2);
        test_board.grid[0][2].set_val(2);
        test_board.merge_up();
        assert_eq!(4, test_board.grid[0][0].get_val().unwrap());
        assert_eq!(None, test_board.grid[0][1].get_val());
        assert_eq!(None, test_board.grid[0][2].get_val());
        assert_eq!(None, test_board.grid[0][3].get_val());
    }

    #[test]
    fn test_merge_down_simple(){
        let mut test_board = Board::init_grid();
        test_board.grid[0][0].set_val(2);
        test_board.grid[0][2].set_val(2);
        test_board.merge_down();
        assert_eq!(4, test_board.grid[0][3].get_val().unwrap());
        assert_eq!(None, test_board.grid[0][1].get_val());
        assert_eq!(None, test_board.grid[0][2].get_val());
        assert_eq!(None, test_board.grid[0][0].get_val());
    }
}
