//use std::env;
extern crate termion;

pub mod tile;
pub mod board;
pub mod run;

use std::io;


use std::char;
use crate::run::init_board;
use crate::board::Board;
use crate::tile::Tile;
use crate::run::game_over;
use crate::run::gen_tile;
use crate::run::mov;

use crate::run::Direction::Up;
use crate::run::Direction::Down;
use crate::run::Direction::Left;
use crate::run::Direction::Right;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use indoc::indoc;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut is_game_over = false;
    let mut board = init_board();

    // TEST BOARD
    // for x in 0.. 4 {
    //     for y in 0 .. 4 {
    //         board.grid[x][y].set_val(2);
    //     }
    // }

    write!(stdout,
           "{}{}q to exit. Vim Navigation. Here is your starting board:\n
           {}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           board,
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::All)
                .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('h') => {
                let pre_move = board.clone();
                //mov(&mut board, Left);
                board.merge_left();
                if pre_move != board {
                    gen_tile(&mut board);
                }
                print!("{}", board)
            },
            Key::Char('j') => {
                let pre_move = board.clone();
                //mov(&mut board, Down);
                board.merge_down();
                if pre_move != board {
                    gen_tile(&mut board);
                }
                print!("{}", board)
            },
            Key::Char('k') => {
                let pre_move = board.clone();
                //mov(&mut board, Up);
                board.merge_up();
                if pre_move != board {
                    gen_tile(&mut board);
                }
                print!("{}", board)
            },
            Key::Char('l') => {
                let pre_move = board.clone();
                //mov(&mut board, Right);
                board.merge_right();
                if pre_move != board {
                    gen_tile(&mut board);
                }
                print!("{}", board)
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // let mut is_game_over = false;
    // let mut board = init_board();
    // println!("{}", board);
    // while is_game_over == false {
    //     println!("{}", board);
    //     let input = getch();
    //     let in_ch = char::from_u32(input as u32).expect("Invalid char");
    //     match in_ch {
    //         h => board.merge_left(),
    //         j => board.merge_down(),
    //         k => board.merge_up(),
    //         l => board.merge_right(),
    //         q => break,
    //         _ => println!("{} is not a valid move! Try using Vim controls", in_ch)
    //     }
    //     is_game_over = game_over(&board);
    // }
}

// fn print_board (board: &Board) -> String {
//     let grid = board.grid;
//           },
//     let fmt_str = format!("| {} | {}  | {}  | {}  |\n
//                            | {} | {}  | {}  | {}  |\n
//                            | {} | {}  | {}  | {}  |\n
//                            | {} | {}  | {}  | {}  |", 
//  grid[0][0], grid[1][0], grid[2][0], grid[3][0],
//  grid[0][1], grid[1][1], grid[2][1], grid[3][1],
//  grid[0][2], grid[1][2], grid[2][2], grid[3][2],
//  grid[0][3], grid[1][3], grid[2][3], grid[3][3]);
//     indoc!(fmt_str)
// }
