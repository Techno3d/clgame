use clgame::Direction;
use crate::{Entity, Location, win};

#[derive(Clone)]
pub enum BlockType {
    Wall,
    Moveable,
    Goal
}

#[derive(Clone)]
pub struct Block {
    pub x: u8,
    pub y: u8,
    pub icon: String,
    pub type_of: BlockType
}

impl Block {
    pub fn new(location: Location, icon: String, r#type: BlockType) -> Block {
        Block { x: location.x, y: location.y, icon, type_of: r#type }
    }

    pub fn change_loc(&mut self, dir: &Direction, block_list: &Vec<Block>) -> bool {
        if let BlockType::Moveable = self.type_of {
            match dir {
                Direction::Up => {
                    if self.y == 0 {
                        let (new_x, new_y, _) = self.update_pos(Location::new(self.x, 0));
                        self.x = new_x;
                        self.y = new_y;
                        return true;
                    } else if Block::check_collision(Location::new(self.x, self.y-1), block_list) {
                        return true;
                    } else {
                        let (new_x, new_y, hit_edge) = self.update_pos(Location::new(self.x, self.y-1));
                        self.x = new_x;
                        self.y = new_y;
                        
                        return hit_edge;
                    }
                },
                Direction::Down => {
                    if Block::check_collision(Location::new(self.x, self.y+1), block_list) {
                        return true;
                    } else {
                        let (new_x, new_y, hit_edge) = self.update_pos(Location::new(self.x, self.y+1));
                        self.x = new_x;
                        self.y = new_y;
                        return hit_edge;
                    }
                },
                Direction::Left => {
                    if self.x == 0 {
                        let (new_x, new_y, _) = self.update_pos(Location::new(0, self.y));
                        self.x = new_x;
                        self.y = new_y;
                        return true;
                    } else if Block::check_collision(Location::new(self.x-1, self.y), block_list) {
                        return true;
                    } else {
                        let (new_x, new_y, hit_edge) = self.update_pos(Location::new(self.x-1, self.y));
                        self.x = new_x;
                        self.y = new_y;
                        return hit_edge;
                    }
                },
                Direction::Right => {
                    if Block::check_collision( Location::new(self.x+1, self.y), block_list) {
                        return true;
                    } else {
                        let (new_x, new_y, hit_edge) = self.update_pos(Location::new(self.x+1, self.y));
                        self.x = new_x;
                        self.y = new_y;
                        return hit_edge;
                    }
                },
            }
        }
        false
    }
    
    fn check_collision(new_pos: Location, blocks: &Vec<Block>) -> bool { 
        for block in blocks {
            if let BlockType::Wall = block.type_of {
                if block.x == new_pos.x && block.y == new_pos.y {
                    return true;
                }
            }

            if let BlockType::Moveable = block.type_of {
                if block.x == new_pos.x && block.y == new_pos.y {
                    println!("yo");
                    return true;
                }
            }
            
            if let BlockType::Goal = block.type_of {
                if block.x == new_pos.x && block.y == new_pos.y {
                    win();
                    return false;
                }
            }
        }
        
        false
    }


    pub fn update_pos(&self, pos: Location) -> (u8, u8, bool) {
        let new_x: u8;
        let new_y: u8;
        let mut hit_edge: bool = false;

        if pos.x > 7 {
            new_x = 7;
            hit_edge = true;
        } else {
            new_x = pos.x;
        }
        
        if pos.y > 7 {
            new_y = 7;
            hit_edge = true;
        } else {
            new_y = pos.y;
        }
        
        (new_x, new_y, hit_edge)
    }
}

impl Entity for Block {
    fn return_location(&self) -> Location {
        Location::new(self.x, self.y)
    }

    fn return_icon(&self) -> &String {
        &self.icon
    }
}
