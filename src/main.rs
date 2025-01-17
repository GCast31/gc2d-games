/*
 * Todo : TileSet
 */

use std::collections::HashMap;

use gc2d::{event::EventLoop, color::Color, image::Quad};
use gc2d_games::tilemap::{TileMap, TypeTileMap, TileMapDetail, TileDescription};

const FONT_MAIN: &str = "assets/fonts/PixelMaster.ttf";
const FONT_MAIN_SIZE: u16 = 20;

const MAP_TILE_HEIGHT: usize = 16;
const MAP_TILE_WIDTH: usize = 16;

const WINDOW_HEIGHT: usize = MAP_TILE_HEIGHT * 10;
const WINDOW_WIDTH: usize = MAP_TILE_WIDTH * 10;

struct Game {
    tilemap: Option<TileMap<u32, MyTileDescription>>,
}

struct MyTileDescription {
    name: String,
}

impl Default for MyTileDescription {
    fn default() -> Self {
        Self { 
            name: String::new(),
        }
    }
}

impl TileDescription for MyTileDescription {}

impl Default for Game {
    fn default() -> Self {
        Self {  
            tilemap: None,
        }
    }
}

impl Game {
    /*
     * map_level1
     * 
     * @brief: Create the first level
     */
    fn map_level1(&mut self) {
        if let Some(map) = &mut self.tilemap {
            map.set_map(Some(vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 2, 2, 2, 1, 2, 2, 2, 2, 1],
                vec![1, 3, 1, 3, 1, 3, 3, 3, 3, 1],
                vec![1, 4, 4, 1, 1, 1, 4, 1, 4, 1],
                vec![1, 5, 1, 5, 1, 5, 1, 5, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ]));
        }
    }
}

impl EventLoop for Game {

    fn load(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, _audio_manager: &mut gc2d::audio::AudioManager) -> Result<(), gc2d::event::EventError> {
        // Informations of the WINDOW
        gc2d.window.set_title("Demo");
        gc2d.window.set_size(WINDOW_WIDTH as f32 * 3., WINDOW_HEIGHT as f32 * 3.);

        gc2d.graphics.set_scale(3., 3.);

        // Add assets (fonts / images)
        gc2d.graphics.new_font(FONT_MAIN, FONT_MAIN_SIZE);

        gc2d.graphics.new_image("assets/images/grassCenter.png").unwrap();
        gc2d.graphics.new_image("assets/images/liquidLava.png").unwrap();
        gc2d.graphics.new_image("assets/images/liquidWater.png").unwrap();
        gc2d.graphics.new_image("assets/images/tileset.png").unwrap();

        // Create the definition of each tile
        let tiles_definition = HashMap::from([
            (1, TileMapDetail { 
                type_tilemap: TypeTileMap::FromFile("assets/images/grassCenter.png".to_string(), None),
                description: Some(MyTileDescription { name: String::from("Grass")}),
            }),
            (2, TileMapDetail {
                type_tilemap: TypeTileMap::FromFile("assets/images/liquidLava.png".to_string(), None),
                description: Some(MyTileDescription { name: String::from("LiquidLava")}),
            }),
            (3, TileMapDetail {
                type_tilemap: TypeTileMap::FromFile("assets/images/liquidWater.png".to_string(), None),
                description: Some(MyTileDescription { name: String::from("LiquidWater")}),
            }),
            (4, TileMapDetail {
                type_tilemap: TypeTileMap::FromFile("assets/images/tileset.png".to_string(), Some(Quad {
                    x: 0.,
                    y: 0.,
                    width: 16.,
                    height: 16.,
                })),
                description: Some(MyTileDescription { name: String::from("Tileset 1-1")}),
            }),
            (5, TileMapDetail {
                type_tilemap: TypeTileMap::FromFile("assets/images/tileset.png".to_string(), Some(Quad {
                    x: 16.,
                    y: 16.,
                    width: 16.,
                    height: 16.,
                })),
                description: Some(MyTileDescription { name: String::from("Tileset 2-2")}),
            }),
        ]);

        // Create tilemap
        self.tilemap = Some(TileMap::new(tiles_definition, MAP_TILE_WIDTH, MAP_TILE_HEIGHT));

        // Load level1 of map
        self.map_level1();

        Ok(())
    }

    fn update(&mut self, _gc2d: &mut gc2d::gc2d::Gc2d, _dt: f32, _audio_manager: &mut gc2d::audio::AudioManager) -> Result<(), gc2d::event::EventError> {
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, fonts: &mut gc2d::fonts::FontsManager, _dt: f32) -> Result<(), gc2d::event::EventError> {
        
        // Draw the map
        if let Some(map) = &self.tilemap {
            map.draw(gc2d);

            // Display informations of tile at position x, y of the mouse
            if let Some(map_description) = map.get_tile_at_position(gc2d.mouse.x, gc2d.mouse.y, &gc2d) {
                gc2d.graphics.print_full(
                    format!("x: {}, y: {} : {}", gc2d.mouse.x, gc2d.mouse.y, map_description.name.clone()), 
                    10., 10., 0., 0.30f32, 0.30f32, 0f32, 0f32,
                    Some(Color::BLACK), 

                    fonts
                );
            }
        }

        Ok(())
    }
}

/*
 ██╗   ███╗ █████╗ ██╗███╗   ██╗
████╗ ████║██╔══██╗██║████╗  ██║
██╔████╔██║███████║██║██╔██╗ ██║
██║╚██╔╝██║██╔══██║██║██║╚██╗██║
██║ ╚═╝ ██║██║  ██║██║██║ ╚████║
╚═╝     ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝
                            
 */
fn main() {
    gc2d::gc2d::Gc2d::new()
        .run(Game::default()).unwrap();
}
