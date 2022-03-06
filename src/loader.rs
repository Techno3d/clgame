use std::{path::Path, fs::File, process};
use clgame::{Location};
use serde::Deserialize;

use crate::block::{Block, BlockType};

#[derive(Debug, Deserialize)]
struct Level {
    w: Vec<[u8; 2]>,
    m: Vec<[u8; 2]>,
    g: Vec<[u8; 2]>
}

pub fn read_level(path_to_level: String) -> Vec<Block> {
    let path = Path::new(&path_to_level);
    
    let level_file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("Didn't give correct path to level file, exiting.");
            process::exit(3);
        }
    };
    
    let level: Level = serde_json::from_reader(level_file).expect("error while parsing");

    let mut blocks: Vec<Block> = vec![];
    
    for w in level.w {
        blocks.push(Block::new(Location::new(w[0], w[1]), "⬜".to_string(), BlockType::Wall));
    }

    for m in level.m {
        blocks.push(Block::new(Location::new(m[0], m[1]), "🟘 ".to_string(), BlockType::Moveable));
    }
    
    for g in level.g {
        blocks.push(Block::new(Location::new(g[0], g[1]), "⧇ ".to_string(), BlockType::Goal));
    }

    blocks
}