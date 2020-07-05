use api::VoxelTool;
use core_types::Vector3;
use gdnative::*;

pub trait TerrainDefinition {
    fn check_terrain_and_update_blocks(&mut self, start: Vector3, voxel_tool: VoxelTool) -> bool;
}
