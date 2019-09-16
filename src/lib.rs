use rand::Rng;
use crate::tile::Tile;
use crate::board::Board;
use crate::tile::Position;


#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    board.set_tile_val(x1, y1, two_or_four()); 
    board.set_tile_val(x2, y2, two_or_four());
    
}

pub fn generate_tile(&mut board: Board, dir: Direction) {
        let mut tile = Tile::new(0,0);
        let mut grid = board.grid;
        tile.set_val(two_or_four());
        let mut ran_co = random_coord();
        let pos;
        match dir {
            Up => pos = Position{x:ran_co, y:3},
            Down => pos = Position{x:ran_co, y:0},
            Left => pos = Position{x:3, y: ran_co},
            Right => pos = Position{x:0,  y: ran_co}
        }
        //TODO: need to check if tile is None, and if so, set it, otherwise, find new tile
        let curr_tile = grid[pos.x][pos.y];
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

fn random_coord() -> u32 {
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