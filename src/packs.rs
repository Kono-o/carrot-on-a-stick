use std::{path::Path,fs::ReadDir,fs};
use dircpy::*;
extern crate dirs;
use crate::consts;
use crate::log;

pub(crate) fn get_packs() -> Option<String> {
   let home_path;
   match dirs::home_dir() {
      Some(dir) => home_path = dir.display().to_string(),
      None => return Some("could not find home dir!".to_string())
   }
   let res_packs_path;
   match consts::OS {
      "linux" => res_packs_path = format!("{}{}", home_path, consts::PACK_DIR_LIN),
      "windows" => res_packs_path = format!("{}{}", home_path, consts::PACK_DIR_WIN),
      _ => return Some(format!("{} is currently not supported!", consts::OS).to_string())
   }
   if !Path::new(&res_packs_path).exists() { return Some("resource packs folder does not exist!".to_string()) }
   
   let resource_packs_paths: ReadDir = fs::read_dir(res_packs_path).unwrap();
   unzip_or_copy(resource_packs_paths);
   None
}

fn unzip_or_copy(dirs: ReadDir) {
   for path in dirs {
      let path = path.unwrap().path();
      let path_name = path.file_stem().unwrap().to_str().unwrap();
      let path_extension = path.extension();
      let output = format!("{}{}/",consts::OUT_PATH,path_name);
      
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