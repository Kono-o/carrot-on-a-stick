use concat_string::concat_string;
use std::{fs,fs::ReadDir,path::{Path, PathBuf}};
use dircpy::*;
use zip::ZipArchive;
use crate::{global, log};

pub fn iter_packs(packs_dir: &PathBuf) -> Result<(),&str> {
  if !packs_dir.exists() {
    return Err("packs::copy_packs -> directory does not exist!")
  }
  let packs: ReadDir = fs::read_dir(&packs_dir).unwrap();
  for pack in packs {
    let pack_dir = pack.unwrap().path();
    let pack_name = pack_dir.file_stem().unwrap().to_str().unwrap();
    let out_dir = concat_string!(global::PACKS_OUT_PATH,pack_name,"/");
    match pack_dir.extension() {
      Some(exten) => try_unzip(&pack_dir, &pack_name, &exten.to_str().unwrap(), &out_dir),
      None => try_copy(&pack_dir, &pack_name, &out_dir),
    }
  }
  Ok(())
}

fn try_copy(dir: &PathBuf, name: &str, out: &str) {
  let je_tex_dir = Path::new(dir).join(global::TEXTURES_PATHS[0]);
  let be_tex_dir = Path::new(dir).join(global::TEXTURES_PATHS[1]);
  if !(je_tex_dir.exists() || be_tex_dir.exists()) {
    log::msg(&concat_string!(name, " has no textures"));
    return;
  }
  CopyBuilder::new(&dir, Path::new(out))
    .overwrite_if_newer(true).overwrite_if_size_differs(true)
    .run().unwrap();
  log::msg(&concat_string!(name, " was copied to ", out));
}

fn try_unzip(dir: &PathBuf, name: &str, ex: &str, out: &str) {
  if !is_valid_extension(ex) {
    log::msg(&concat_string!(name, " is not a pack"));
    return;
  }
  let _ = ZipArchive::new(fs::File::open(&dir).unwrap()).unwrap().extract(&out);
  log::msg(&concat_string!(name, " was unzipped to ", out));
}

fn is_valid_extension(exten: &str) -> bool {
  for extension in global::PACK_EXTENSIONS.iter() {
    if exten.eq(*extension) { return true }
  }
  false
}