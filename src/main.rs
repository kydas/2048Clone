//use std::env;

pub mod tile;
pub mod board;
pub mod run;

use std::io;
use crate::run::init_board;
use crate::board::Board;
use crate::tile::Tile;
use crate::run::game_over;



fn main() {
    let mut is_game_over = false;
    let mut board = init_board();
    while is_game_over == false {
        println!("{}", board);
        
        is_game_over = game_over(&board);
    }
}

// fn print_board (board: &Board) {
//     let mut s = String::from("");
//     let grid = board.grid;
//     let mut count = 0;
//     for x in 0 .. 4 {
//         for y in 0 .. 4 {
//             if count % 4 == 0 {
//                 s.push_str("\n");
//             }
//             let tile = grid[x][y];
//             let val = tile.get_val();
//             s.push_str("|" + val + "|");
//         }
//     }
//     println!("{}", s);
// }
