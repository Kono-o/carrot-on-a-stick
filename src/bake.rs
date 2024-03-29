use std::{fs,path::PathBuf};
use std::path::Path;
use concat_string::concat_string;
use image::{DynamicImage, GenericImageView, imageops, RgbaImage};
use crate::global;

#[derive(Debug)]
enum Edition {
   Bedrock,
   Java
}
struct Pack {
   name: String,
   edition: Edition,
   tex_dir: PathBuf,
}

impl Pack {

   pub fn try_from(path: &PathBuf) -> Option<Self> {
      return match is_valid_pack(&path) {
         None => None,
         Some((edition,tex_dir)) => Some(Self {
            name: String::from(path.file_stem().unwrap().to_str().unwrap()),
            edition,
            tex_dir,
         })
      }
   }
}

fn is_valid_pack(path: &PathBuf) -> Option<(Edition, PathBuf)> {
   let tex_dir: PathBuf = Path::join(path, global::TEXTURES_PATHS[0]);
   if Path::join(path, global::PACK_DESC_FILES[0]).exists() && tex_dir.exists() {
      return Some((Edition::Java, tex_dir));
   }
   let tex_dir: PathBuf = Path::join(path, global::TEXTURES_PATHS[1]);
   if Path::join(path, global::PACK_DESC_FILES[1]).exists() && tex_dir.exists() {
      return Some((Edition::Bedrock, tex_dir));
   }
   None
}

#[derive(Debug)]
pub enum Maps {
   Color,
   //Metal, currently unimplemented
   //Rough,
   //Emit,
   //Normal
}
pub struct Atlas {
   image: RgbaImage,
   map: Maps,
   rows: u32,
   mem: Vec<bool>
}

impl Atlas {

   pub fn new(size: u32, map: Maps) -> Self {
      return Self {
         image: RgbaImage::new(size, size),
         map,
         rows: size/global::TILE_SIZE,
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
         println!("baking {} ({:?}) -> {:?}", pack.name, pack.edition, self.map);
         for (n,texture) in global::TEX_LIST.iter().enumerate() {
            println!("{}", texture);
            if self.mem[n] { continue }
            let texture_path: PathBuf = Path::join(&pack.tex_dir, concat_string!(texture, ".png"));
            let mut texture: DynamicImage;
            if texture_path.exists() { texture = image::open(texture_path).unwrap(); self.mem[n] = true; }
            else { texture = image::open("./packs/missing.png").unwrap(); }
            if texture.dimensions() > (global::TEX_SIZE,global::TEX_SIZE) {
               texture = texture.crop(0,0,global::TEX_SIZE,global::TEX_SIZE);
            }
            if texture.dimensions() < (global::TEX_SIZE,global::TEX_SIZE) {
               texture = texture.resize(global::TEX_SIZE,global::TEX_SIZE,imageops::Gaussian);
            }
            let texture: RgbaImage = add_borders(&mut texture);
            let (x_off,y_off) = (n as u32 % self.rows * global::TILE_SIZE, n as u32 / self.rows * global::TILE_SIZE);
            for(x,y,pixel) in texture.enumerate_pixels() {
               self.image.put_pixel(x_off + x, y_off + y, *pixel);
            }
         }
      }
   }

   pub fn save(&mut self, path: &str, name: &str) {
      self.image.save(concat_string!(path,name)).unwrap();
      println!("{} saved!", name);
   }
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