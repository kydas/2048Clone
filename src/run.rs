use rand::Rng;
use crate::tile::Tile;
use crate::board::Board;
use crate::tile::Position;


#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn init_board() -> Board {
    let mut board = Board::init_grid();
    let x1 = random_coord();
    let y1 = random_coord();
    let mut x2 = random_coord();
    let mut y2 = random_coord();
    while x1 == x2 && y1 == y2 {
        x2 = random_coord();
        y2 = random_coord();
    }
    board.set_tile_val(x1, y1, two_or_four()); 
    board.set_tile_val(x2, y2, two_or_four());
    board
}

pub fn game_over(board: Board) -> bool {
    let grid = board.grid;
    for x in 0..4 {
        for y in 0..3 {
            let curr = grid[x][y];
            if curr.val == None {
                return false
            }
            let next_y = grid[x][y+1];
            if next_y.val == None {
                return false
            }
            if next_y.val == curr.val {
                return false
            }
            if x != 3 {
                let next_x = grid[x+1][y];
                if next_x.val == None {
                    return false
                }
                if curr.val == next_x.val {
                    return false
                }
            }

        }
    }
    true
}

pub fn generate_tile(board: Board, dir: Direction) {
        let mut tile = Tile::new(0,0);
        let mut grid = board.grid;
        tile.set_val(two_or_four());
        let ran_co = random_coord();
        let mut pos;
        match dir {
            Up => pos = Position{x:ran_co, y:3},
            Down => pos = Position{x:ran_co, y:0},
            Left => pos = Position{x:3, y: ran_co},
            Right => pos = Position{x:0,  y: ran_co}
        }
        //TODO: need to check if tile is None, and if so, set it, otherwise, find new tile
        let mut curr_tile = grid[pos.x][pos.y];
        let mut count = 1;
        while curr_tile.val != None {
            if count > 3 {
                break;   
                //end game
            }
            if dir == Direction::Up || dir == Direction::Down {
                pos.y = count + pos.y % 4;
            } else {
                pos.x = count + pos.x % 4;
            }
            curr_tile = grid[pos.x][pos.y];
            count += 1;
        }
        tile.pos = pos;
        grid[pos.x][pos.y] = tile;
        
}

// ########## HELPER FUNCTIONS #############

fn random_coord() -> usize {
    rand::thread_rng().gen_range(0,4)
}

fn two_or_four() -> u32 {
    let val_weight = rand::thread_rng().gen_range(0, 10);
        if val_weight > 6 {
            4
        } else {
            2
        }
}



// ######### UNIT TESTS ###############

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn simple_test_end_game(){
        let test_board = init_board();
        assert_eq!(false, game_over(test_board));
    }

    #[test]
    fn test_game_over() {
        let mut test_board = init_board();
        let mut val = 0;
        for x in 0..4 {
            for y in 0..4 {
                val = val + 2;
                test_board.set_tile_val(x, y, val);
            }
        }
        assert_eq!(true, game_over(test_board));
    }

}