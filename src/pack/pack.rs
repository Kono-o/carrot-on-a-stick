use concat_string::concat_string;
use dircpy::CopyBuilder;
use std::ffi::OsStr;
use std::{fs, path::Path};
use zip::ZipArchive;

use crate::{consts, log};

#[derive(Debug)]
pub enum PackErr {
    MissingDir,
    EmptyDir,
}

pub fn get(packs_dir: &Path) -> Result<(), PackErr> {
    if !packs_dir.exists() {
        return Err(PackErr::MissingDir);
    }

    let mut is_empty = true;
    for pack in fs::read_dir(&packs_dir).unwrap() {
        is_empty = false;
        let pack_dir = pack.unwrap().path();
        let pack_name = pack_dir.file_name().unwrap().to_str().unwrap();
        let pack_stem = pack_dir.file_stem().unwrap().to_str().unwrap();

        log::msg(&concat_string!("reading [", pack_name, "]..."));
        let out_dir = concat_string!(consts::PACKS_DIR, pack_stem, "/");
        check_and_copy(&pack_dir, &out_dir);
    }
    if is_empty {
        return Err(PackErr::EmptyDir);
    }
    Ok(())
}

fn check_and_copy(dir: &Path, out: &str) {
    match dir.extension() {
        Some(ex) if is_zip(ex) => {
            ZipArchive::new(fs::File::open(&dir).unwrap())
                .unwrap()
                .extract(&out)
                .unwrap();
            log::msg(&concat_string!("unzipped to ", out));
            return;
        }
        _ => {
            let (je_mcmeta, be_manif) = (
                dir.join(consts::PACK_DESC_FILES[0]),
                dir.join(consts::PACK_DESC_FILES[1]),
            );
            if je_mcmeta.exists() || be_manif.exists() {
                CopyBuilder::new(&dir, out)
                    .overwrite_if_size_differs(true)
                    .overwrite_if_newer(true)
                    .run()
                    .unwrap();
                log::msg(&concat_string!("copied to ", out));
                return;
            }
        }
    }
    log::msg("not a resource pack");
}

fn is_zip(ex: &OsStr) -> bool {
    for extension in consts::PACK_FORMATS.iter() {
        if ex.eq(*extension) { return true; }
    }
    false
}
