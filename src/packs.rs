use concat_string::concat_string;
use std::{fs,path::Path};
use dircpy::CopyBuilder;
use zip::ZipArchive;

use crate::{global, log};

#[derive(Debug)]
pub enum PackErrorType{
  MissingDir,
}

pub fn get_packs(packs_dir: &Path) -> Result<(),PackErrorType> {
  if !packs_dir.exists() { return Err(PackErrorType::MissingDir) }
  for pack in fs::read_dir(&packs_dir).unwrap() {
    let pack_dir = pack.unwrap().path();
    let pack_name = pack_dir.file_stem().unwrap().to_str().unwrap();
    log::msg(&concat_string!("reading [" ,pack_name, "]..."));
    let out_dir = concat_string!(global::PACKS_OUT_PATH,pack_name,"/");
    match pack_dir.extension() {
      Some(ex) => try_unzip(&pack_dir, &pack_name, &ex.to_str().unwrap(), &out_dir),
      None => try_copy(&pack_dir, &pack_name, &out_dir),
    }
  }
  Ok(())
}

fn try_copy(dir: &Path, out: &str) {
  let je_tex_dir = Path::new(dir).join(global::TEXTURES_PATHS[0]);
  let be_tex_dir = Path::new(dir).join(global::TEXTURES_PATHS[1]);
  if !(je_tex_dir.exists() || be_tex_dir.exists()) {
    log::msg(&"has no textures");
    return;
  }
  CopyBuilder::new(&dir, out)
    .overwrite_if_newer(true).overwrite_if_size_differs(true)
    .run().unwrap();
  log::msg(&concat_string!("was copied to ", out));
}

fn try_unzip(dir: &Path, ex: &str, out: &str) {
  if !is_valid_extension(ex) {
    log::msg(&concat_string!("not a resource pack"));
    return;
  }
  ZipArchive::new(fs::File::open(&dir).unwrap()).unwrap().extract(&out).unwrap();
  log::msg(&concat_string!("unzipped to ", out));
}

fn has_textures(dir: &Path) {
  let je_tex_dir = Path::new(dir).join(global::TEXTURES_PATHS[0]);
  let be_tex_dir = Path::new(dir).join(global::TEXTURES_PATHS[1]);
  if !(je_tex_dir.exists() || be_tex_dir.exists()) {
    log::msg(&"has no textures");
    return;
  }
}

fn is_valid_extension(ex: &str) -> bool {
  for extension in global::PACK_EXTENSIONS.iter() {
    if ex.eq(*extension) { return true }
  }
  false
}