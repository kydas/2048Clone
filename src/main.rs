//use std::env;

pub mod tile;
pub mod board;
pub mod run;
extern crate ncurses;

use std::io;
use std::char;
use ncurses::*;
use crate::run::init_board;
use crate::board::Board;
use crate::tile::Tile;
use crate::run::game_over;



fn main() {
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();

    let mut is_game_over = true;
    let mut board = init_board();
    println!("{}", board);
    while is_game_over == false {
        println!("{}", board);
        let input = getch();
        let in_ch = char::from_u32(input as u32).expect("Invalid char");
        match in_ch {
            h => board.merge_left(),
            j => board.merge_down(),
            k => board.merge_up(),
            l => board.merge_right(),
            _ => println!("{} is not a valid move! Try using Vim controls", in_ch)
        }
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
