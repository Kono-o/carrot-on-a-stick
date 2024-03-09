use std::env;

pub(crate) const OS: &str = env::consts::OS;
pub(crate) const CRATE_NAME: &str = "carrot-on-a-stick";

pub(crate) const PACK_DIR_LIN: &str = r"/.minecraft/resourcepacks/";
pub(crate) const PACK_DIR_WIN: &str = r"/AppData/Roaming/.minecraft/resourcepacks/";
pub(crate) const ASSETS_TEX_DIR: &str = r"/assets/minecraft/textures/block/";
pub(crate) const CRATE_PACKS_PATH: &str = r"./packs/";

pub(crate) const ATLAS_SIZE: u32 = 768;

