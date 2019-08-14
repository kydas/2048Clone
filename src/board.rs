

pub mod Board{

    use crate::tile::Tile;

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