use std::{path::Path,fs::ReadDir,fs};
use std::path::PathBuf;
use concat_string::concat_string;
use dircpy::*;
use crate::global;

pub fn get_from_mc_folder() -> i32 {
   let mut resourcepacks_dir: PathBuf;

   match dirs::home_dir() {
      Some(path) => resourcepacks_dir = path,
      None => return 1
   }
   match global::OS_NAME {
      "linux" => resourcepacks_dir.push(global::RESOURCE_DIRS[0]),
      "windows" => resourcepacks_dir.push(global::RESOURCE_DIRS[1]),
      _ => return 2
   }

   if !resourcepacks_dir.exists() { return 3 }
   let resource_packs: ReadDir = fs::read_dir(&resourcepacks_dir).unwrap();
   unzip_and_copy_packs(resource_packs);
   0
}

fn unzip_and_copy_packs(dirs: ReadDir) {
   for path in dirs {
      let path: PathBuf = path.unwrap().path();
      let path_name: &str = path.file_stem().unwrap().to_str().unwrap();
      let path_extension = path.extension();
      let output_path = concat_string!(global::CRATE_PACKS_PATH,path_name,"/");

      if path_extension == None {
         let mut block_path = path.clone();
         block_path.push(global::TEXTURES_DIRS[0]);
         if Path::new(&block_path).exists() {
            CopyBuilder::new(&path, Path::new(&output_path))
               .overwrite_if_newer(true)
               .overwrite_if_size_differs(true)
               .run().unwrap();
            println!("copied {}", &path_name);
         }
      }
      else if path_extension == Some("zip".as_ref()) || path_extension == Some("mcpack".as_ref()) {
         let _ = zip::ZipArchive::new(fs::File::open(&path).unwrap()).unwrap().extract(&output_path);
         println!("unzipped and copied {}", &path_name);
      }
   }
}
