use std::{fs::ReadDir,fs,path::PathBuf};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use concat_string::concat_string;
use crate::global;

pub fn bake()
{
   let mut atlas_image: RgbaImage = RgbaImage::new(global::ATLAS_SIZE, global::ATLAS_SIZE);
   let mut memory: Vec<bool> = vec![false; global::TEX_LIST.len()];
   let packs: ReadDir = fs::read_dir(global::CRATE_PACKS_PATH).unwrap();
   
   for pack in packs {
      let pack: PathBuf = pack.unwrap().path();
      let block_path: PathBuf = concat_block_path(&pack);
      if !block_path.exists() { continue }
      println!("baking {}", pack.file_stem().unwrap().to_str().unwrap());
      for (i,tex_name) in global::TEX_LIST.iter().enumerate() {
         let mut texture_path: PathBuf = block_path.clone();
         texture_path.push(concat_string!(tex_name, ".png"));

         let texture_image: DynamicImage;
         if !texture_path.exists() { texture_image = image::open("./packs/missing.png").unwrap() }
         else { texture_image = image::open(texture_path.clone()).unwrap(); }

         if memory[i] { continue }
         if !memory[i] && texture_path.exists() { memory[i] = true; }

         let texture_image = add_borders(texture_image);
         
         for(x,y,_pixel) in texture_image.enumerate_pixels() {
            if x >= global::TEX_SIZE || y >= global::TEX_SIZE { continue }
            let mut x_off: u32 = ((i as u32) * global::TILE_SIZE) + x;
            let y_off: u32 = (x_off/ global::ATLAS_SIZE) * global::TILE_SIZE + y;
            x_off = x_off  % global::ATLAS_SIZE;

            if (x_off,y_off) >= (global::ATLAS_SIZE, global::ATLAS_SIZE) { continue }

            let texture_pixel: Rgba<u8> = *texture_image.get_pixel(x, y);
            atlas_image.put_pixel(x_off, y_off, texture_pixel);
         }
      }
   }
   atlas_image.save(format!("{}{}", global::CRATE_PACKS_PATH, "blocks.png")).unwrap();
   println!("blocks.png saved!");
}

fn concat_block_path(path: &PathBuf) -> PathBuf {
   let mut java_block_path = path.clone();
   java_block_path.push(global::TEXTURES_DIRS[0]);
   if java_block_path.exists() { return java_block_path; }

   let mut bedrock_block_path = path.clone();
   bedrock_block_path.push(global::TEXTURES_DIRS[1]);
   return bedrock_block_path;
}

fn add_borders(image: DynamicImage) -> RgbaImage {
   image.to_rgba8();
   let mut new_image: RgbaImage = RgbaImage::new(global::TILE_SIZE,global::TILE_SIZE);
   for (x,y,pixel) in new_image.enumerate_pixels_mut() {
      if x == 0 || y == 0 || x == global::TILE_SIZE-1 || y == global::TILE_SIZE-1 { continue }
      *pixel = image.get_pixel(x-1,y-1)
   }
   return new_image;
}