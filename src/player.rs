use clgame::{Location, Entity};

use crate::{Direction, block::{Block, BlockType}};


#[derive(Debug)]
pub struct Player {
    pub x: u8,
    pub y: u8,
    pub icon: String
}

impl Player {
    pub fn update_pos(&self, pos: Location) -> (u8, u8) {
        let new_x: u8;
        let new_y: u8;

        if pos.x > 7 {
            new_x = 7;
        } else {
            new_x = pos.x;
        }
        
        if pos.y > 7 {
            new_y = 7;
        } else {
            new_y = pos.y;
        }
        
        (new_x, new_y)
    }
    
    pub fn new() -> Player {
        Player { x: 1, y: 1, icon: "💜".to_string() }
    }
    
    pub fn change_loc(&mut self, dir: Direction, block_list: &mut Vec<Block>) {
        match dir {
            Direction::Up => {
                if self.y == 0 {
                    let (new_x, new_y) = self.update_pos(Location::new(self.x, 0_));
                    self.x = new_x;
                    self.y = new_y;
                } else if Player::check_collision(block_list, Location::new(self.x, self.y-1), dir) {
                
                } else {
                    let (new_x, new_y) = self.update_pos(Location::new(self.x, self.y-1));
                    self.x = new_x;
                    self.y = new_y;
                    
                }
            },
            Direction::Down => {
                if Player::check_collision(block_list, Location::new(self.x, self.y+1), dir) {
                
                } else {
                    let (new_x, new_y) = self.update_pos(Location::new(self.x, self.y+1));
                    self.x = new_x;
                    self.y = new_y;
                }
            },
            Direction::Left => {
                if self.x == 0 {
                    let (new_x, new_y) = self.update_pos(Location::new(0, self.y));
                    self.x = new_x;
                    self.y = new_y;
                } else if Player::check_collision(block_list, Location::new(self.x-1, self.y), dir) {
                    return;
                } else {
                    let (new_x, new_y) = self.update_pos(Location::new(self.x-1, self.y));
                    self.x = new_x;
                    self.y = new_y;
                    
                }
            },
            Direction::Right => {
                if Player::check_collision(block_list, Location::new(self.x+1, self.y), dir) {
                    return;
                } else {
                    let (new_x, new_y) = self.update_pos(Location::new(self.x+1, self.y));
                    self.x = new_x;
                    self.y = new_y;
                }
            },
        }
    }

    pub fn check_collision(blocks: &mut Vec<Block>, new_pos: Location, dir: Direction) -> bool {
        let block_clone = blocks.clone();
        for block in  blocks {
            if let BlockType::Wall = block.type_of {
                if block.x == new_pos.x && block.y == new_pos.y {
                    return true;
                }
            }
            
            if let BlockType::Moveable = block.type_of {
                if block.x == new_pos.x && block.y == new_pos.y {
                    return block.change_loc(&dir, &block_clone);
                }
            }
        }
        
        false
    }
}

impl Entity for Player {
    fn return_location(&self) -> Location {
        Location {x: self.x, y: self.y}
    }

    fn return_icon(&self) -> &String {
        &self.icon
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y, icon: self.icon.clone() }
    }
}