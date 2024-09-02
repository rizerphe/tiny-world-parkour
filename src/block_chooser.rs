use rand::{rngs::SmallRng, Rng, SeedableRng};
use valence::{
    block::{PropName, PropValue},
    prelude::BiomeId,
    registry::RegistryIdx,
    BlockState,
};

// Create a random choice helper as we'll use that a lot
fn random_choice<T>(choices: &[T]) -> T
where
    T: Copy,
{
    let mut rng = SmallRng::from_entropy();
    choices[rng.gen_range(0..choices.len())]
}

pub fn choose_block(biome: Option<BiomeId>) -> BlockState {
    biome.map_or(BlockState::POLISHED_DIORITE, |biome| {
        match biome.to_index() {
            1 => {
                // plains
                random_choice(&[
                    BlockState::GRASS_BLOCK,
                    BlockState::COARSE_DIRT,
                    BlockState::DIRT,
                    BlockState::PODZOL,
                    BlockState::HAY_BLOCK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            2 => {
                // sunflower_plains
                random_choice(&[
                    BlockState::GRASS_BLOCK,
                    BlockState::COARSE_DIRT,
                    BlockState::YELLOW_CONCRETE,
                    BlockState::DIRT,
                    BlockState::PODZOL,
                    BlockState::HAY_BLOCK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            3 => {
                // snowy_plains
                random_choice(&[
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            4 => {
                // ice_spikes
                random_choice(&[
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::LIGHT_BLUE_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::LIGHT_BLUE_WOOL,
                    BlockState::CYAN_WOOL,
                    BlockState::BONE_BLOCK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            5 => {
                // desert
                random_choice(&[
                    BlockState::SAND,
                    BlockState::SANDSTONE,
                    BlockState::RED_SAND,
                    BlockState::RED_SANDSTONE,
                    BlockState::ORANGE_CONCRETE,
                    BlockState::ORANGE_TERRACOTTA,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            6 => {
                // swamp
                random_choice(&[
                    BlockState::GRASS_BLOCK,
                    BlockState::COARSE_DIRT,
                    BlockState::DIRT,
                    BlockState::PODZOL,
                    BlockState::HAY_BLOCK,
                    BlockState::OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            7 => {
                // mangrove_swamp
                random_choice(&[
                    BlockState::GRASS_BLOCK,
                    BlockState::COARSE_DIRT,
                    BlockState::DIRT,
                    BlockState::PODZOL,
                    BlockState::HAY_BLOCK,
                    BlockState::OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            8 => {
                // forest
                random_choice(&[
                    BlockState::OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::OAK_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            9 => {
                // flower_forest
                random_choice(&[
                    BlockState::OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::OAK_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            10 => {
                // birch_forest
                random_choice(&[
                    BlockState::BIRCH_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::BIRCH_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            11 => {
                // dark_forest
                random_choice(&[
                    BlockState::DARK_OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::RED_MUSHROOM_BLOCK,
                    BlockState::DARK_OAK_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::BLACKSTONE,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::SCULK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            12 => {
                // old_growth_birch_forest
                random_choice(&[
                    BlockState::BIRCH_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::BIRCH_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            13 => {
                // old_growth_pine_taiga
                random_choice(&[
                    BlockState::SPRUCE_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::SPRUCE_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::PODZOL,
                    BlockState::POLISHED_ANDESITE,
                    BlockState::BROWN_CONCRETE_POWDER,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            14 => {
                // old_growth_spruce_taiga
                random_choice(&[
                    BlockState::SPRUCE_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::SPRUCE_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::PODZOL,
                    BlockState::POLISHED_ANDESITE,
                    BlockState::BROWN_CONCRETE_POWDER,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            15 => {
                // taiga
                random_choice(&[
                    BlockState::SPRUCE_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::SPRUCE_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::PODZOL,
                    BlockState::POLISHED_ANDESITE,
                    BlockState::BROWN_CONCRETE_POWDER,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            16 => {
                // snowy_taiga
                random_choice(&[
                    BlockState::SPRUCE_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::SPRUCE_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::PODZOL,
                    BlockState::POLISHED_ANDESITE,
                    BlockState::BROWN_CONCRETE_POWDER,
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            17 => {
                // savanna
                random_choice(&[
                    BlockState::HAY_BLOCK,
                    BlockState::RED_SAND,
                    BlockState::RED_SANDSTONE,
                    BlockState::ORANGE_CONCRETE,
                    BlockState::ORANGE_TERRACOTTA,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            18 => {
                // savanna_plateau
                random_choice(&[
                    BlockState::HAY_BLOCK,
                    BlockState::RED_SAND,
                    BlockState::RED_SANDSTONE,
                    BlockState::ORANGE_CONCRETE,
                    BlockState::ORANGE_TERRACOTTA,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            19 => {
                // windswept_hills
                random_choice(&[
                    BlockState::POLISHED_ANDESITE,
                    BlockState::POLISHED_DIORITE,
                    BlockState::POLISHED_GRANITE,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_QUARTZ,
                    BlockState::SMOOTH_BASALT,
                    BlockState::SMOOTH_STONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            20 => {
                // windswept_gravelly_hills
                random_choice(&[
                    BlockState::POLISHED_ANDESITE,
                    BlockState::POLISHED_DIORITE,
                    BlockState::POLISHED_GRANITE,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_QUARTZ,
                    BlockState::SMOOTH_BASALT,
                    BlockState::SMOOTH_STONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            21 => {
                // windswept_forest
                random_choice(&[
                    BlockState::OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            22 => {
                // windswept_savanna
                random_choice(&[
                    BlockState::HAY_BLOCK,
                    BlockState::RED_SAND,
                    BlockState::RED_SANDSTONE,
                    BlockState::ORANGE_CONCRETE,
                    BlockState::ORANGE_TERRACOTTA,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            23 => {
                // jungle
                random_choice(&[
                    BlockState::JUNGLE_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::JUNGLE_WOOD,
                    BlockState::STRIPPED_JUNGLE_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::LIME_CONCRETE_POWDER,
                    BlockState::GREEN_CONCRETE,
                    BlockState::GREEN_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            24 => {
                // sparse_jungle
                random_choice(&[
                    BlockState::JUNGLE_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::JUNGLE_WOOD,
                    BlockState::STRIPPED_JUNGLE_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::LIME_CONCRETE_POWDER,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            25 => {
                // bamboo_jungle
                random_choice(&[
                    BlockState::JUNGLE_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::JUNGLE_WOOD,
                    BlockState::STRIPPED_JUNGLE_WOOD,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::LIME_CONCRETE_POWDER,
                    BlockState::GREEN_CONCRETE,
                    BlockState::GREEN_WOOL,
                    BlockState::BAMBOO_BLOCK,
                    BlockState::STRIPPED_BAMBOO_BLOCK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            26 => {
                // badlands
                random_choice(&[
                    BlockState::ORANGE_CONCRETE,
                    BlockState::ORANGE_TERRACOTTA,
                    BlockState::RED_SAND,
                    BlockState::RED_SANDSTONE,
                    BlockState::SMOOTH_RED_SANDSTONE,
                    BlockState::YELLOW_CONCRETE,
                    BlockState::YELLOW_TERRACOTTA,
                    BlockState::GOLD_BLOCK,
                    BlockState::DEEPSLATE_GOLD_ORE,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            27 => {
                // eroded_badlands
                random_choice(&[
                    BlockState::ORANGE_CONCRETE,
                    BlockState::ORANGE_TERRACOTTA,
                    BlockState::RED_SAND,
                    BlockState::RED_SANDSTONE,
                    BlockState::SMOOTH_RED_SANDSTONE,
                    BlockState::YELLOW_CONCRETE,
                    BlockState::YELLOW_TERRACOTTA,
                    BlockState::GOLD_BLOCK,
                    BlockState::DEEPSLATE_GOLD_ORE,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            28 => {
                // wooded_badlands
                random_choice(&[
                    BlockState::ORANGE_CONCRETE,
                    BlockState::ORANGE_TERRACOTTA,
                    BlockState::RED_SAND,
                    BlockState::RED_SANDSTONE,
                    BlockState::SMOOTH_RED_SANDSTONE,
                    BlockState::YELLOW_CONCRETE,
                    BlockState::YELLOW_TERRACOTTA,
                    BlockState::GOLD_BLOCK,
                    BlockState::DEEPSLATE_GOLD_ORE,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            29 => {
                // meadow
                random_choice(&[
                    BlockState::GRASS_BLOCK,
                    BlockState::COARSE_DIRT,
                    BlockState::DIRT,
                    BlockState::PODZOL,
                    BlockState::HAY_BLOCK,
                    BlockState::OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::MAGENTA_CONCRETE_POWDER,
                    BlockState::MAGENTA_CONCRETE,
                    BlockState::MAGENTA_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            30 => {
                // cherry_grove
                random_choice(&[
                    BlockState::OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::STRIPPED_CHERRY_WOOD,
                    BlockState::CHERRY_WOOD,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            31 => {
                // grove
                random_choice(&[
                    BlockState::OAK_LEAVES,
                    BlockState::AZALEA_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::STRIPPED_CHERRY_WOOD,
                    BlockState::CHERRY_WOOD,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            32 => {
                // snowy_slopes
                random_choice(&[
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            33 => {
                // frozen_peaks
                random_choice(&[
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::SMOOTH_BASALT,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::POLISHED_DEEPSLATE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            34 => {
                // jagged_peaks
                random_choice(&[
                    BlockState::POLISHED_ANDESITE,
                    BlockState::POLISHED_DIORITE,
                    BlockState::POLISHED_GRANITE,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_QUARTZ,
                    BlockState::SMOOTH_BASALT,
                    BlockState::SMOOTH_STONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            35 => {
                // stony_peaks
                random_choice(&[
                    BlockState::POLISHED_ANDESITE,
                    BlockState::POLISHED_DIORITE,
                    BlockState::POLISHED_GRANITE,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SMOOTH_QUARTZ,
                    BlockState::SMOOTH_BASALT,
                    BlockState::SMOOTH_STONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            36 => {
                // river
                random_choice(&[
                    BlockState::DRIED_KELP_BLOCK,
                    BlockState::SMOOTH_STONE,
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::CRACKED_STONE_BRICKS,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            37 => {
                // frozen_river
                random_choice(&[
                    BlockState::DRIED_KELP_BLOCK,
                    BlockState::SMOOTH_STONE,
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::CRACKED_STONE_BRICKS,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            38 => {
                // beach
                random_choice(&[
                    BlockState::SAND,
                    BlockState::RED_SAND,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            39 => {
                // snowy_beach
                random_choice(&[
                    BlockState::SAND,
                    BlockState::RED_SAND,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            40 => {
                // stony_shore
                random_choice(&[
                    BlockState::POLISHED_ANDESITE,
                    BlockState::POLISHED_DIORITE,
                    BlockState::POLISHED_GRANITE,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::POLISHED_DEEPSLATE,
                    BlockState::SMOOTH_QUARTZ,
                    BlockState::SMOOTH_BASALT,
                    BlockState::SMOOTH_STONE,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            41 => {
                // warm_ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SEA_LANTERN,
                    BlockState::TUBE_CORAL_BLOCK,
                    BlockState::BRAIN_CORAL_BLOCK,
                    BlockState::BUBBLE_CORAL_BLOCK,
                    BlockState::FIRE_CORAL_BLOCK,
                    BlockState::HORN_CORAL_BLOCK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            42 => {
                // lukewarm_ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SEA_LANTERN,
                    BlockState::TUBE_CORAL_BLOCK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            43 => {
                // deep_lukewarm_ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SCULK,
                    BlockState::SMOOTH_BASALT,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SEA_LANTERN,
                    BlockState::TUBE_CORAL_BLOCK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            44 => {
                // ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SEA_LANTERN,
                    BlockState::AZALEA_LEAVES,
                    BlockState::OAK_WOOD,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            45 => {
                // deep_ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SCULK,
                    BlockState::SMOOTH_BASALT,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SEA_LANTERN,
                    BlockState::AZALEA_LEAVES,
                    BlockState::OAK_WOOD,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            46 => {
                // cold_ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SEA_LANTERN,
                    BlockState::AZALEA_LEAVES,
                    BlockState::OAK_WOOD,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            47 => {
                // deep_cold_ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SCULK,
                    BlockState::SMOOTH_BASALT,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SEA_LANTERN,
                    BlockState::AZALEA_LEAVES,
                    BlockState::OAK_WOOD,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            48 => {
                // frozen_ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SEA_LANTERN,
                    BlockState::AZALEA_LEAVES,
                    BlockState::OAK_WOOD,
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            49 => {
                // deep_frozen_ocean
                random_choice(&[
                    BlockState::PRISMARINE,
                    BlockState::PRISMARINE_BRICKS,
                    BlockState::DARK_PRISMARINE,
                    BlockState::SCULK,
                    BlockState::SMOOTH_BASALT,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::SEA_LANTERN,
                    BlockState::AZALEA_LEAVES,
                    BlockState::OAK_WOOD,
                    BlockState::SNOW_BLOCK,
                    BlockState::WHITE_CONCRETE,
                    BlockState::WHITE_TERRACOTTA,
                    BlockState::CYAN_CONCRETE,
                    BlockState::WHITE_WOOL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            50 => {
                // mushroom_fields
                random_choice(&[
                    BlockState::MUSHROOM_STEM,
                    BlockState::RED_MUSHROOM_BLOCK,
                    BlockState::BROWN_MUSHROOM_BLOCK,
                    BlockState::POLISHED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            51 => {
                // dripstone_caves
                random_choice(&[
                    BlockState::DRIPSTONE_BLOCK,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::SCULK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            52 => {
                // lush_caves
                random_choice(&[
                    BlockState::AZALEA_LEAVES,
                    BlockState::OAK_LEAVES,
                    BlockState::CHERRY_LEAVES,
                    BlockState::AZALEA,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            53 => {
                // deep_dark
                random_choice(&[
                    BlockState::DEEPSLATE,
                    BlockState::DEEPSLATE_BRICKS,
                    BlockState::DEEPSLATE_TILES,
                    BlockState::POLISHED_DEEPSLATE,
                    BlockState::SMOOTH_BASALT,
                    BlockState::SMOOTH_STONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            54 => {
                // nether_wastes
                random_choice(&[
                    BlockState::GLOWSTONE,
                    BlockState::NETHER_BRICKS,
                    BlockState::NETHER_GOLD_ORE,
                    BlockState::NETHERITE_BLOCK,
                    BlockState::NETHERRACK,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            55 => {
                // warped_forest
                random_choice(&[
                    BlockState::WARPED_NYLIUM,
                    BlockState::WARPED_WART_BLOCK,
                    BlockState::NETHERRACK,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            56 => {
                // crimson_forest
                random_choice(&[
                    BlockState::CRIMSON_NYLIUM,
                    BlockState::CRIMSON_FUNGUS,
                    BlockState::CRIMSON_ROOTS,
                    BlockState::NETHERRACK,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            57 => {
                // soul_sand_valley
                random_choice(&[
                    BlockState::SOUL_SOIL,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            58 => {
                // basalt_deltas
                random_choice(&[
                    BlockState::BASALT,
                    BlockState::POLISHED_BASALT,
                    BlockState::BLACKSTONE,
                    BlockState::GILDED_BLACKSTONE,
                    BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
                ])
            }
            59 => {
                // the_end
                BlockState::PURPUR_BLOCK
            }
            60 => {
                // end_highlands
                BlockState::MAGENTA_CONCRETE
            }
            61 => {
                // end_midlands
                BlockState::PURPUR_BLOCK
            }
            62 => {
                // small_end_islands
                BlockState::PURPUR_BLOCK
            }
            63 => {
                // end_barrens
                BlockState::PURPUR_BLOCK
            }
            _ => random_choice(&[
                BlockState::POLISHED_DIORITE,
                BlockState::POLISHED_ANDESITE,
                BlockState::POLISHED_GRANITE,
                BlockState::POLISHED_BLACKSTONE,
                BlockState::POLISHED_BLACKSTONE,
                BlockState::SMOOTH_QUARTZ,
                BlockState::SMOOTH_BASALT,
                BlockState::SMOOTH_STONE,
                BlockState::REDSTONE_LAMP.set(PropName::Lit, PropValue::True),
            ]),
        }
    })
}
