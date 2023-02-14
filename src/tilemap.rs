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

use gc2d::color::Color;

/*
    TYPE OF TILEMAP
 */
pub enum TypeTileMap {
    FromSimpleFile(String),
    FromTileSet(String, u32, u32),
    Rectangle(Color),
}

pub struct TileMapDetail<T: TileDescription> {

    pub type_tilemap: TypeTileMap,
    pub description: Option<T>,
}

/*
 * TILEMAP
 */

pub struct TileMap<T: Eq + Hash, U: TileDescription> {
    tile_height: usize,
    tile_width: usize,
    map: Option<Vec<Vec<T>>>,
    tiles_definition: HashMap<T, TileMapDetail<U>>,
}

pub trait TileDescription {}

impl<T: Eq + Hash, U: TileDescription> TileMap<T, U> {
    /*
     * new() 
     * 
     * @brief : Create a new tilemap
     */
    pub fn new(tiles_definition: HashMap<T, TileMapDetail<U>>, tile_width: usize, tile_height: usize) -> Self {
        Self { 
            tile_height,
            tile_width,
            map: None, 
            tiles_definition,
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
     * get_tile_at_position()
     * 
     * @brief: Get the at position x, y 
     */
    pub fn get_tile_at_position(&self, x: f32, y: f32) -> Option<&U> {

        if let Some(map) = &self.map {

            let closure = |pos: f32, size: f32| {
                if self.tile_height != 0 {
                    (pos / size).floor() as usize
                } else { 0 }
            };

            let line = closure(y, self.tile_height as f32);
            let column = closure(x, self.tile_width as f32);

            if line < map.capacity() {
                let tile_line = &map[line];
                if column < tile_line.capacity() {
                    let tile = &tile_line[column];
                    if let Some(detail) = self.tiles_definition.get(&tile) {
                        if let Some(description) = &detail.description {
                            return Some(&description);
                        }
                    }
                }
            }
        }

        None
    }

    /*
     * Draw()
     *
     * @brief: Drawing the map
     */
    pub fn draw(&self, gc2d: &mut gc2d::gc2d::Gc2d) {
        if let Some(map) = &self.map {
            for (line, value_line) in map.iter().enumerate() {
                for (column, value_column) in value_line.iter().enumerate() {
                    if let Some(tile_definition) = self.tiles_definition.get(&value_column) {
                        match &tile_definition.type_tilemap {
                            TypeTileMap::FromSimpleFile(filename) => {
                                gc2d.graphics.draw(
                                    filename.as_str(), 
                                    (self.tile_width * column) as f32, 
                                    (self.tile_height * line) as f32 , 
                                    0.
                                );
                            },
                            TypeTileMap::FromTileSet(_, _, _) => {
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