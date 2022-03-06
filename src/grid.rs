use clgame::Entity;


#[derive(Clone)]
pub struct Grid<'a> {
    pub grid: [[&'a str; 8]; 8]
}

impl<'a> Grid<'a> {
    pub fn edit<T: Entity>(&mut self, entity: &'a T) {
            let pos = entity.return_location();
            
            let x_pos: usize = pos.x.into();
            let y_pos: usize = pos.y.into();

            self.grid[x_pos][y_pos] = &entity.return_icon();
    }
    
    // fn clear(&mut self) {
    //     self.grid = [["☐ "; 8]; 8];
    // }
    
    pub fn print(&self) {
        let len = self.grid[0].len();
        
        //Makes the game happen on a new 'frame'
        match term_size::dimensions() {
            Some((_, h)) =>  {
                for _ in 0..h {
                    println!();
                }
            },
            None => println!("Failed to get term size"),
        } 

        //Prints the game screen
        for (i, _column) in self.grid.iter().enumerate() {
            for j in 0..len {
                print!("{} ", self.grid[j][i]);
                    
                if j >= 7 {
                    println!();
                }
            }
        }
    }
}
