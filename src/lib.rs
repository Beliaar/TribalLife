extern crate gdnative;
use gdnative::prelude::*;
use gdnative::*;
mod biome_data;
mod biome_dictionary;
mod building;
mod cube_terrain_definition;
mod noise_world;
mod terrain_definition;
mod voxel_generator_df;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_tool_class::<voxel_generator_df::VoxelGeneratorDF>();
    handle.add_class::<biome_data::BiomeData>();
    handle.add_class::<noise_world::NoiseWorld>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
