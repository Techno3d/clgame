pub mod loader;
pub mod lib;
pub mod block;
pub mod grid;
pub mod player;

use std::{io, process};
use std::env;
use block::{Block, BlockType};
use clgame::{Entity, Location, Direction};
use loader::read_level;
use player::Player;

use crate::grid::Grid;

extern crate term_size; 
extern  crate serde;

fn main() {
    let mut blocks = vec![
        create_wall(Location::new(0,0)), 
        create_wall(Location::new(1,0)), 
        create_wall(Location::new(2,0)), 
        create_wall(Location::new(3,0)),
        create_moveable(Location::new(5,5))
    ];
    let mut player = Player::new();

    let args: Vec<String> = env::args().collect();
    if args.contains(&"-d".to_string()) {
        if let Some(s) = args.last() {
            if s != "-d" {
                blocks = read_level(s.to_string());
            } else {
                println!("No file was given, using the default level");
            }
        }
    } else if args.contains(&"-h".to_string()) {
        println!("-h to show help message\n-d to load in level file\n\nHave fun");
        process::exit(0);
    }
    
    loop {
        let mut grid = Grid{grid: [["☐ "; 8]; 8]};
        let mut inp = String::new();
        let dir: Direction;

        grid.edit(&player);
        for block in blocks.iter(){
            grid.edit(block);
        }

        grid.print();
        
        //Direction
        println!("Which Direction to move? w a s d (q to quit)");
        io::stdin().read_line(&mut inp).expect("Bruh Fail"); 
        
        match inp.trim() {
            "w" => dir = Direction::Up,
            "s" => dir = Direction::Down,
            "a" => dir = Direction::Left,
            "d" => dir = Direction::Right,
            "q" => process::exit(0),
            _ => continue
        }

        //Move Char
        player.change_loc(dir, &mut blocks);
    }
}

fn create_wall(pos: Location) -> Block {
    Block::new(pos, "⬜".to_string(), BlockType::Wall)
}

fn create_moveable(pos: Location) -> Block {
    Block::new(pos, "🟘 ".to_string(), BlockType::Moveable)
}

fn win() {
    println!("You won, the (kind of terrible) game!");
    process::exit(0);
}