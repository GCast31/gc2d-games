/*

 ██████╗  ██████╗██████╗ ██████╗               ████████╗██╗██╗     ███████╗███╗   ███╗ █████╗ ██████╗ 
██╔════╝ ██╔════╝╚════██╗██╔══██╗              ╚══██╔══╝██║██║     ██╔════╝████╗ ████║██╔══██╗██╔══██╗
██║  ███╗██║      █████╔╝██║  ██║    █████╗       ██║   ██║██║     █████╗  ██╔████╔██║███████║██████╔╝
██║   ██║██║     ██╔═══╝ ██║  ██║    ╚════╝       ██║   ██║██║     ██╔══╝  ██║╚██╔╝██║██╔══██║██╔═══╝ 
╚██████╔╝╚██████╗███████╗██████╔╝                 ██║   ██║███████╗███████╗██║ ╚═╝ ██║██║  ██║██║     
 ╚═════╝  ╚═════╝╚══════╝╚═════╝                  ╚═╝   ╚═╝╚══════╝╚══════╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝     
                                                                                                      
    @GCast31 
 */

use std::{collections::HashMap, hash::Hash};

use gc2d::{color::Color, fonts::FontsManager};

/*
    TYPE OF TILEMAP
 */
pub enum TypeTileMap {
    FromSimpleFile(String),
    FromTileSet(String, u32, u32),
    Rectangle(Color),
}

/*
 * TILEMAP
 */

pub struct TileMap<T: Eq + Hash> {
    tile_height: usize,
    tile_width: usize,
    map: Option<Vec<Vec<T>>>,
    map_tiles: HashMap<T, TypeTileMap>,
}

impl<T: Eq + Hash> TileMap<T> {
    /*
     * new() 
     * 
     * @brief : Create a new tilemap
     */
    pub fn new(map_tiles: HashMap<T, TypeTileMap>, tile_width: usize, tile_height: usize) -> Self {
        Self { 
            tile_height,
            tile_width,
            map: None, 
            map_tiles,
        }
    }

    /*
     * set_map()
     * 
     * @brief : Initialize current map
     */

    pub fn set_map(&mut self, map: Option<Vec<Vec<T>>>) {
        self.map = map;
    }

    /*
     * Drawing the map
     */
    pub fn draw(&self, gc2d: &mut gc2d::gc2d::Gc2d, fonts: &mut FontsManager) {
        if let Some(map) = &self.map {
            for (line, value_line) in map.iter().enumerate() {
                for (column, value_column) in value_line.iter().enumerate() {
                    if let Some(tile_definition) = self.map_tiles.get(&value_column) {
                        match tile_definition {
                            TypeTileMap::FromSimpleFile(filename) => {
                                gc2d.graphics.draw(
                                    filename.as_str(), 
                                    (self.tile_width * column) as f32, 
                                    (self.tile_height * line) as f32 , 
                                    0.
                                );
                            },
                            TypeTileMap::FromTileSet(filename, column, line) => {
                                todo!("GC2D Quad todo");
                            },
                            TypeTileMap::Rectangle(color) => {
                                gc2d.graphics.rectangle(
                                    gc2d::graphics::DrawMode::Fill,
                                    (self.tile_width * column) as f32, 
                                    (self.tile_height * line) as f32 ,
                                    self.tile_width as f32,
                                    self.tile_height as f32, 
                                    Some(color.clone()),
                                );
                            },
                        }
                    }
                }
            }
        }
    }
}