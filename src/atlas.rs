use std::{fs::ReadDir,fs,path::{Path, PathBuf}};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use crate::consts;

pub(crate) fn bake()
{
   let atlas_size = consts::ATLAS_SIZE;
   let mut atlas_image: RgbaImage = RgbaImage::new(atlas_size,atlas_size);
   
   let packs_paths: ReadDir = fs::read_dir(consts::CRATE_PACKS_PATH).unwrap();
   
   for texture_packs in packs_paths {
      let texture_packs = texture_packs.unwrap().path();
      let blocks_path = format!("{}{}", texture_packs.to_str().unwrap(), consts::ASSETS_TEX_DIR);
      if !Path::new(&blocks_path).exists() {continue};
      
      let textures: ReadDir = fs::read_dir(blocks_path).unwrap();
      
      for (i,texture) in textures.enumerate() {
         let texture:PathBuf = texture.unwrap().path();
         
         if texture.extension() == Some("png".as_ref()) {
            let texture:DynamicImage = image::open(texture).unwrap();
            
            for(x,y,_pixel) in texture.to_rgba8().enumerate_pixels() {
               if x >= 16 || y >= 16 {continue};
               let mut x_off: u32 = ((i as u32) * 16) + x;
               let y_off: u32 = (x_off/atlas_size)*16 + y;
               x_off = x_off  % atlas_size;
               
               if (x_off,y_off) >= (atlas_size,atlas_size) {continue};
               
               let texture_pixel: Rgba<u8> = texture.get_pixel(x,y);
               atlas_image.put_pixel(x_off, y_off, texture_pixel);
            }
         }
      }
   }
   atlas_image.save(format!("{}{}",consts::CRATE_PACKS_PATH, "atlas.png")).unwrap();
}