use std::env;

pub const OS_NAME: &str = env::consts::OS;

pub const CRATE_NAME: &str = "carrot-on-a-stick";
pub const CRATE_PACKS_PATH: &str = "./packs/";

pub const RESOURCE_DIRS: [&str;2] = [
//linux
".minecraft/resourcepacks/",
//windows
"AppData/Roaming/.minecraft/resourcepacks/"
];

pub const TEXTURES_DIRS: [&str;2] = [
//java
"assets/minecraft/textures/block/",
//bedrock
"textures/blocks/"
];

pub const TEX_SIZE: u32 = 16;
pub const TILE_SIZE: u32 = TEX_SIZE + 2;
pub const ATLAS_SIZE: u32 = TILE_SIZE * 64;

pub const TEX_LIST: [&str; 181] = [
"grass_block_top",
"stone",
"dirt",
"grass_block_side",
"oak_planks",
"smooth_stone_slab_side",
"smooth_stone",
"bricks",
"tnt_side",
"tnt_top",
"tnt_bottom",
"cobweb",
"poppy",
"dandelion",
"&portal",
"oak_sapling",
"cobblestone",
"bedrock",
"sand",
"gravel",
"oak_log",
"oak_log_top",
"iron_block",
"gold_block",
"diamond_block",
"&chest1",
"&chest2",
"&chest3",
"red_mushroom",
"brown_mushroom",
"jungle_sapling",
"&fire",
"gold_ore",
"iron_ore",
"coal_ore",
"bookshelf",
"mossy_cobblestone",
"obsidian",
"grass_block_side_overlay",
"short_grass",
"&doublechest",
"&doublechest",
"crafting_table_top",
"furnace_front",
"furnace_side",
"dispenser_front",
"dispenser_front_vertical",
"sponge",
"glass",
"diamond_ore",
"redstone_ore",
"oak_leaves",
"coarse_dirt",
"stone_bricks",
"dead_bush",
"fern",
"&doublechestback",
"&doublechestback",
"crafting_table_side",
"crafting_table_front",
"furnace_front_on",
"furnace_top",
"spruce_sapling",
"white_wool",
"spawner",
"snow",
"ice",
"grass_block_snow",
"cactus_top",
"cactus_side",
"cactus_bottom",
"clay",
"sugar_cane",
"note_block",
"jukebox_top",
"lily_pad",
"mycelium_side",
"mycelium_top",
"birch_sapling",
"torch",
"oak_door_top",
"iron_door_top",
"ladder",
"oak_trapdoor",
"iron_bars",
"farmland",
"farmland_moist",
"wheat_stage0",
"wheat_stage1",
"wheat_stage2",
"wheat_stage3",
"wheat_stage4",
"wheat_stage5",
"wheat_stage6",
"wheat_stage7",
"lever",
"oak_door_bottom",
"iron_door_bottom",
"redstone_torch",
"mossy_stone_bricks",
"cracked_stone_bricks",
"pumpkin_top",
"netherrack",
"soul_sand",
"glowstone",
"piston_top_sticky",
"piston_top",
"piston_side",
"piston_bottom",
"piston_inner",
"pumpkin_stem",
"rail_corner",
"black_wool",
"gray_wool",
"redstone_torch_off",
"spruce_log",
"birch_log",
"pumpkin_side",
"carved_pumpkin",
"jack_o_lantern",
"cake_top",
"cake_side",
"cake_inner",
"cake_bottom",
"red_mushroom_block",
"brown_mushroom_block",
"attached_pumpkin_stem",
"rail",
"red_wool",
"pink_wool",
"repeater",
"spruce_leaves",
"red_sandstone_bottom",
"&bedtop",
"&bedtop",
"melon_side",
"melon_top",
"cauldron_top",
"cauldron_inner",
"jukebox_side",
"mushroom_stem",
"mushroom_block_inside",
"vine",
"lapis_block",
"green_wool",
"lime_wool",
"repeater_on",
"glass_pane_top",
"&bed",
"&bed",
"&bed",
"&bed",
"jungle_log",
"cauldron_side",
"cauldron_bottom",
"brewing_stand_base",
"brewing_stand",
"end_portal_frame_top",
"end_portal_frame_side",
"lapis_ore",
"brown_wool",
"yellow_wool",
"powered_rail",
"redstone_dust_line0",
"redstone_dust_line1",
"enchanting_table_top",
"dragon_egg",
"cocoa_stage2",
"cocoa_stage1",
"cocoa_stage0",
"emerald_ore",
"tripwire_hook",
"tripwire",
"end_portal_frame_eye",
"end_stone",
"sandstone_top",
"blue_wool",
"light_blue_wool",
"powered_rail_on",
"redstone_dust_dot",
"acacia_log",
];