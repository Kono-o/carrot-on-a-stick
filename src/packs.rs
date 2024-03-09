use std::{path::Path,fs::ReadDir,fs};
use dircpy::*;
extern crate dirs;
use crate::{consts,log};

pub(crate) fn get() -> Option<i32> {
   let home_path: String;
   let res_packs_path: String;
   
   match dirs::home_dir() {
      Some(dir) => home_path = dir.display().to_string(),
      None => {log::console("could not find home dir!".to_string()); return Some(0)}
   }
   match consts::OS {
      "linux" => res_packs_path = format!("{}{}", home_path, consts::PACK_DIR_LIN),
      "windows" => res_packs_path = format!("{}{}", home_path, consts::PACK_DIR_WIN),
      _ => {log::console(format!("{} is currently not supported!", consts::OS).to_string()); return Some(1)}
   }
   if !Path::new(&res_packs_path).exists() {
      {log::console("resource packs folder does not exist!".to_string()); return Some(2)}
   }
   
   let resource_packs_paths: ReadDir = fs::read_dir(res_packs_path).unwrap();
   unzip_or_copy(resource_packs_paths);
   None
}

fn unzip_or_copy(dirs: ReadDir) {
   for path in dirs {
      let path = path.unwrap().path();
      let path_name = path.file_stem().unwrap().to_str().unwrap();
      let path_extension = path.extension();
      let output = format!("{}{}/",consts::CRATE_PACKS_PATH,path_name);
      
      if path_extension == None {
         let asset_folder = format!("{}{}", path.to_str().unwrap(), "/assets/");
         if Path::new(&asset_folder).exists() {
            log::console(format!("copying {} (folder)", path_name));
            CopyBuilder::new(&path, &Path::new(&output))
               .overwrite_if_newer(true)
               .overwrite_if_size_differs(true)
               .run().unwrap();
         }
         else {log::console(format!("skipping {} (folder)", path_name));}
      }
      else if path_extension == Some("zip".as_ref())
         || path_extension == Some("mcpack".as_ref()) {
         log::console(format!("extracting {} ({})", path_name, path_extension.unwrap().to_str().unwrap()));
         let zip = fs::File::open(path).unwrap();
         let _ = zip::ZipArchive::new(zip).unwrap().extract(&output);
      }
      else {log::console(format!("skipping {} ({})", path_name, path_extension.unwrap().to_str().unwrap()))}
   }
}