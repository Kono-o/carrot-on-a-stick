use std::path::PathBuf;
use concat_string::concat_string;
use image::{ImageError, RgbaImage};

use crate::{global, log};

pub enum AtlasErr {
    ImageErr(ImageError),
    MissingDir
}

pub enum Map {
    Color,
    Metal,
    Rough,
    Emit,
    Normal
}
pub struct Atlas {
    image: RgbaImage,
    map: Map,
    rows: u32,
    mem: Vec<bool>,
}

impl Atlas {
    pub fn new(map: Map, size: u32) -> Self {
        Self {
            image: RgbaImage::new(size, size),
            rows: size / global::TILE_SIZE,
            mem: vec![false; global::BLOCK_TEX_LIST.len()],
            map,
        }
    }

    pub fn bake(&mut self) {
    //todo
    }

    pub fn save(&self, dir: &str, name: &str) -> Result<(),AtlasErr> {
        let map = match self.map {
            Map::Color => "color",
            Map::Metal => "metal",
            Map::Rough => "rough",
            Map::Emit => "emit",
            Map::Normal => "normal",
        };
        let name_map_ex = concat_string!(name, "_", map, ".", global::IMAGE_FORMAT);
        let out_dir = PathBuf::from(dir).join(&name_map_ex);
        match self.image.save(out_dir) {
            Ok(..) => (),
            Err(err) => return Err(AtlasErr::ImageErr(err))
        }
        log::msg(&concat_string!("[", name_map_ex, "] saved to ", dir));
        Ok(())
    }
}
