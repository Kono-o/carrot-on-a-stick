use concat_string::concat_string;
use std::{fs,fs::ReadDir,path::{Path, PathBuf}};
use dircpy::*;
use crate::{global, log};

pub fn iter_packs(packs_dir: &PathBuf) -> Result<(),&str> {
  if !packs_dir.exists() {
    return Err("packs::copy_packs -> directory does not exist!")
  }
  let packs: ReadDir = fs::read_dir(&packs_dir).unwrap();
  for pack in packs {
    let pack_dir = pack.unwrap().path();
      copy_pack(&pack_dir);
  }
  Ok(())
}

fn copy_pack(dir: &PathBuf) {
  let pack_name = dir.file_stem().unwrap().to_str().unwrap();
  let pack_ex = dir.extension();
  let dir = Path::new(dir).join(global::TEXTURES_PATHS[0]);
  if dir.exists() {
    let out_dir = concat_string!(global::PACKS_OUT_PATH,pack_name,"/");
    CopyBuilder::new(&dir, Path::new(&out_dir))
      .overwrite_if_newer(true).overwrite_if_size_differs(true)
      .run().unwrap();
    log::print(&concat_string!(pack_name, " was copied to ", out_dir));
  }
  else {
    log::print(&concat_string!(pack_name, " has no block textures"));
  }
  
}