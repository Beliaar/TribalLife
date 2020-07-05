use crate::{biome_data, cube_terrain_definition, voxel_generator_df::VoxelGeneratorDF};
use gdnative::api::{Node, ResourceLoader, Spatial, Texture, VoxelTerrain};
use gdnative::core_types::StringArray;
use gdnative::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct NoiseWorld {
    #[property]
    elevation_map: Option<Ref<Texture>>,
    #[property]
    biome_map: Option<Ref<Texture>>,
    #[property]
    water_types: StringArray,
    #[property]
    area_radius: u32,
    #[property]
    terrain: Option<Ref<VoxelTerrain>>,
}

#[methods]
impl NoiseWorld {
    fn new(_owner: &Spatial) -> Self {
        NoiseWorld {
            elevation_map: None,
            biome_map: None,
            water_types: StringArray::new(),
            area_radius: 0,
            terrain: None,
        }
    }

    fn _ready(mut self, owner: &Spatial) {}
}
