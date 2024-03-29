use std::{fs,path::PathBuf};
use std::path::Path;
use concat_string::concat_string;
use image::{DynamicImage, GenericImageView, RgbaImage};
use crate::global;

#[derive(Debug)]
enum Edition {
   Bedrock,
   Java
}

struct Pack {
   pack: PathBuf,
   name: String,
   edition: Edition,
   tex_dir: PathBuf,
}

impl Pack {
   pub fn try_from(path: &PathBuf) -> Option<Self> {
      return match is_valid_pack(&path) {
         None => None,
         Some((edition,tex_dir)) => Some(Self {
            pack: path.clone(),
            name: String::from(path.file_stem().unwrap().to_str().unwrap()),
            edition,
            tex_dir,
         })
      }
   }
}

fn is_valid_pack(path: &PathBuf) -> Option<(Edition, PathBuf)> {
   let tex_dir: PathBuf = Path::join(path, global::TEXTURES_DIRS[0]);
   if Path::join(path, global::PACK_DESC_FILES[0]).exists() && tex_dir.exists() {
      return Some((Edition::Java, tex_dir));
   }
   let tex_dir: PathBuf = Path::join(path, global::TEXTURES_DIRS[1]);
   if Path::join(path, global::PACK_DESC_FILES[1]).exists() && tex_dir.exists() {
      return Some((Edition::Bedrock, tex_dir));
   }
   None
}

struct Atlas {
   image: RgbaImage,
   mem: Vec<bool>
}

impl Atlas {

   pub fn new(size: u32) -> Self {
      return Self {
         image: RgbaImage::new(size, size),
         mem: vec![false; global::TEX_LIST.len()]
      };
   }

   pub fn bake(&mut self) {
      for pack_entry in fs::read_dir(global::CRATE_PACKS_PATH).unwrap() {
         let pack: Pack;
         match Pack::try_from(&pack_entry.unwrap().path()) {
            Some(p) => pack = p,
            None => continue
         }
            println!("{}", pack.name);
         println!("{:?}", pack.edition);
      }
   }

   pub fn save(&mut self, path: &str, name: &str) {
      self.image.save(concat_string!(path,name)).unwrap();
      println!("{} saved!", name);
   }
}


pub fn bake()
{
   let mut atlas_image: RgbaImage = RgbaImage::new(global::ATLAS_SIZE, global::ATLAS_SIZE);
   let mut memory: Vec<bool> = vec![false; global::TEX_LIST.len()];

   let mut x = Atlas::new(global::ATLAS_SIZE);
   x.bake();
   x.save(global::CRATE_PACKS_PATH, "diff.png");

   for pack in fs::read_dir(global::CRATE_PACKS_PATH).unwrap() {
      let block_textures_dir: PathBuf = concat_block_path(&pack.unwrap().path());

      if !block_textures_dir.exists() { continue }
      println!("baking {:?}", block_textures_dir);

      for (i,texture_name) in global::TEX_LIST.iter().enumerate() {

         if memory[i] { continue }
         let mut current_texture: PathBuf = block_textures_dir.clone();
         current_texture.push(concat_string!(texture_name,".png"));
         
         let mut texture_image: DynamicImage;
         if !current_texture.exists() { texture_image = image::open(concat_string!(global::CRATE_PACKS_PATH,"missing.png")).unwrap(); }
         else { texture_image = image::open(current_texture.clone()).unwrap(); memory[i] = true; }

         if texture_image.dimensions() > (global::TEX_SIZE,global::TEX_SIZE) {
            texture_image = texture_image.crop(0,0,global::TEX_SIZE,global::TEX_SIZE)
         }
         let texture_image = add_borders(&mut texture_image);
         
         for(x,y, pixel) in texture_image.enumerate_pixels() {

            let mut x_off: u32 = ((i as u32) * global::TILE_SIZE) + x;
            let y_off: u32 = (x_off/ global::ATLAS_SIZE) * global::TILE_SIZE + y;
            x_off = x_off  % global::ATLAS_SIZE;
            if (x_off,y_off) >= (global::ATLAS_SIZE, global::ATLAS_SIZE) { continue }

            atlas_image.put_pixel(x_off, y_off, *pixel);
         }
      }
   }
   atlas_image.save(concat_string!(global::CRATE_PACKS_PATH, "blocks.png")).unwrap();
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

fn add_borders(image: &mut DynamicImage) -> RgbaImage {
   let mut image = image.to_rgba8();
   let mut new_image: RgbaImage = RgbaImage::new(global::TILE_SIZE,global::TILE_SIZE);
   let mut is_full_block = true;

   for (x,y,pixel) in image.enumerate_pixels_mut(){
      if x == 0 || y == 0 || x == global::TILE_SIZE-1 || y == global::TILE_SIZE-1 {
         if pixel.0[3] == 0 && is_full_block != false { is_full_block = false; }
      }
      new_image.put_pixel(x+1,y+1,*pixel);
   }
      for i in 1..global::TILE_SIZE-1{
         if is_full_block {
            new_image.put_pixel(0, i, *new_image.get_pixel(global::TILE_SIZE-2, i));
            new_image.put_pixel(global::TILE_SIZE-1, i, *new_image.get_pixel(1, i));
            new_image.put_pixel(i, 0, *new_image.get_pixel(i, global::TILE_SIZE-2));
            new_image.put_pixel(i, global::TILE_SIZE-1, *new_image.get_pixel(i, 1));
         }
         else {
            new_image.put_pixel(0, i, *new_image.get_pixel(1, i));
            new_image.put_pixel(global::TILE_SIZE-1, i, *new_image.get_pixel(global::TILE_SIZE-2, i));
            new_image.put_pixel(i, 0, *new_image.get_pixel(i, 1));
            new_image.put_pixel(i, global::TILE_SIZE-1, *new_image.get_pixel(i, global::TILE_SIZE-2));
         }
   }
   if is_full_block {
      new_image.put_pixel(global::TILE_SIZE-1, global::TILE_SIZE-1, *new_image.get_pixel(1, 1));
      new_image.put_pixel(0, 0, *new_image.get_pixel(global::TILE_SIZE-2, global::TILE_SIZE-2));
      new_image.put_pixel(global::TILE_SIZE-1, 0, *new_image.get_pixel(1, global::TILE_SIZE-2));
      new_image.put_pixel(0, global::TILE_SIZE-1, *new_image.get_pixel(global::TILE_SIZE-2, 1));
   }
   else {
      new_image.put_pixel(global::TILE_SIZE-1, global::TILE_SIZE-1, *new_image.get_pixel(global::TILE_SIZE-2, global::TILE_SIZE-2));
      new_image.put_pixel(0, 0, *new_image.get_pixel(1, 1));
      new_image.put_pixel(global::TILE_SIZE-1, 0, *new_image.get_pixel(global::TILE_SIZE-2, 1));
      new_image.put_pixel(0, global::TILE_SIZE-1, *new_image.get_pixel(1, global::TILE_SIZE-2));
   }
   return new_image;
}