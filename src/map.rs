// Import the default crate
use crate::prelude::*;

// usize == CPU preferred bits size
const NUM_TILES: usize = ( SCREEN_WIDTH * SCREEN_HEIGHT ) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
   Wall,
   Floor,
}
pub struct Map {
    pub tiles: Vec<TileType>,
}
 //Striding: row-first encoding
    //LINK: {https://en.wikipedia.org/wiki/Stride_of_an_array}
    //return
pub fn map_index(x:i32, y:i32) -> usize {
   ((y*SCREEN_WIDTH) + x) as usize
}
impl Map {
    pub fn new() -> Self {
        Self { tiles: vec![TileType::Floor;NUM_TILES] }
    }

   
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = map_index(x,y);
                match self.tiles[index] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
    
}

