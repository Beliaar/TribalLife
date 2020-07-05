use crate::{building::Building, terrain_definition::TerrainDefinition};
use gdnative::api::{
    CubeMesh, Material, Mesh, MeshInstance, Node, Object, Spatial, SpatialMaterial, VoxelTool,
};
use gdnative::core_types::{Color, Vector3};
use gdnative::prelude::AsArg;
use gdnative::thread_access::{Shared, Unique};
use gdnative::*;
use std::convert::TryFrom;
use std::{collections::HashMap, sync::Arc};

#[derive(NativeClass)]
#[inherit(Object)]
pub struct CubeTerrainDefinition {
    #[property]
    cube_prefab: Option<Ref<CubeMesh>>,
    valid_voxels: Vec<i64>,
    invalid_voxels: Vec<i64>,
    spatial: Option<Ref<Spatial, Shared>>,
    center: Vector3,
    location: Vector3,
    building: Option<Building>,
    outline_mesh: Option<Ref<Mesh>>,
    invalid_blocks: Vec<String>,
    is_valid: bool,
    cubes: HashMap<String, Ref<CubeMesh>>,
    size: Vector3,
}

impl CubeTerrainDefinition {
    fn new(_owner: &Object) -> Self {
        CubeTerrainDefinition {
            cube_prefab: None,
            valid_voxels: Vec::new(),
            invalid_voxels: Vec::new(),
            spatial: None,
            center: Vector3::new(0.0, 0.0, 0.0),
            location: Vector3::new(0.0, 0.0, 0.0),
            building: None,
            outline_mesh: None,
            invalid_blocks: Vec::new(),
            is_valid: false,
            cubes: HashMap::new(),
            size: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    fn update_mesh(mut self) {
        let spatial = Spatial::new();
        let min_x = self.location.x as i32;
        let max_x = (self.location.x + self.size.x) as i32;

        let min_y = self.location.y as i32;
        let max_y = (self.location.y + self.size.y) as i32;

        let min_z = self.location.z as i32;
        let max_z = (self.location.z + self.size.z) as i32;
        for x in min_x..max_x {
            for y in min_y..max_y {
                for z in min_z..max_z {
                    let instance = MeshInstance::new();
                    let cube = self.create_cube();
                    let adjusted_x = x as f32 - self.center.x;
                    let adjusted_y = y as f32 - self.center.y;
                    let adjusted_z = z as f32 - self.center.z;
                    let material = SpatialMaterial::new();
                    material.set_albedo(Color::rgb(1.0, 1.0, 1.0));
                    let cube_id = format!("{},{},{}", adjusted_x, adjusted_y, adjusted_z);
                    self.cubes.insert(cube_id.clone(), cube);

                    unsafe {
                        let cube = self.cubes[&cube_id].assume_safe();
                        //let material: Ref<Material> =
                        //    material.to_material().duplicate(false).unwrap().into();

                        cube.set_material(material);

                        instance.set_translation(Vector3::new(
                            0.2 + adjusted_x + (x as f32 * 0.2),
                            0.2 + adjusted_y - (y as f32 * 0.2),
                            -0.2 + adjusted_z + (z as f32 * 0.2),
                        ));
                        instance.set_mesh(cube);
                    }
                    unsafe {
                        spatial.add_child(instance, false);
                    }
                    if self.center == Vector3::new(adjusted_x, adjusted_y, adjusted_z) {
                        let center_cube = self.create_cube();
                        let material = SpatialMaterial::new();
                        material.set_albedo(Color::rgba(1.0, 1.0, 1.0, 0.5));
                        material.set_flag(SpatialMaterial::FEATURE_TRANSPARENT, true);
                        unsafe {
                            center_cube.assume_safe().set_material(material);
                        }
                        let center_instance = MeshInstance::new();
                        unsafe {
                            center_instance.set_mesh(center_cube.assume_safe());
                            spatial.add_child(center_instance, false);
                        }
                    }
                }
            }
        }
        spatial.set_translation(self.location);
        self.spatial = Some(spatial.into_shared());
    }

    fn create_cube(&self) -> Ref<CubeMesh> {
        let cube = CubeMesh::new();
        cube.set_size(Vector3::new(1.5, 1.5, 1.5));
        cube.into_shared()
    }
}

impl TerrainDefinition for CubeTerrainDefinition {
    fn check_terrain_and_update_blocks(&mut self, start: Vector3, voxel_tool: VoxelTool) -> bool {
        let start_position = start;
        let mut is_valid = true;
        let min_x = self.location.x;
        let max_x = self.location.x + self.size.x;

        let min_y = self.location.y;
        let max_y = self.location.y + self.size.y;

        let min_z = self.location.z;
        let max_z = self.location.z + self.size.z;
        self.invalid_blocks.clear();
        for x in min_x as i32..max_x as i32 {
            for y in min_y as i32..max_y as i32 {
                for z in min_z as i32..max_z as i32 {
                    let adjusted_x = x as f32 - self.center.x;
                    let adjusted_y = y as f32 - self.center.y;
                    let adjusted_z = z as f32 - self.center.z;
                    let position =
                        start_position + Vector3::new(adjusted_x, adjusted_y, adjusted_z);
                    let cube_id = format!("{},{},{}", adjusted_x, adjusted_y, adjusted_z);
                    if !self.cubes.contains_key(&cube_id) {
                        godot_warn!(
                            "Cube at position {},{},{} was not created",
                            adjusted_x,
                            adjusted_y,
                            adjusted_z
                        );
                        continue;
                    }
                    let cube = self.cubes.get(&cube_id).unwrap();
                    let v_id = voxel_tool.get_voxel(position);
                    let mut block_valid = true;
                    if !self.valid_voxels.is_empty() {
                        block_valid = self.valid_voxels.contains(&v_id);
                    }
                    if !self.invalid_voxels.is_empty() {
                        block_valid = !self.invalid_voxels.contains(&v_id);
                    }
                    let material: Option<Ref<SpatialMaterial>>;
                    unsafe {
                        material = cube
                            .assume_safe()
                            .material()
                            .and_then(|material| material.cast());
                    }
                    if let Some(material) = material {
                        unsafe {
                            if !block_valid {
                                material.assume_safe().set_albedo(Color::rgb(1.0, 0.0, 0.0));

                                self.invalid_blocks.push(cube_id);
                            } else {
                                material
                                    .assume_safe()
                                    .set_albedo(Color::rgba(0.0, 1.0, 0.0, 0.1));
                                material
                                    .assume_safe()
                                    .set_flag(SpatialMaterial::FEATURE_TRANSPARENT, true);
                            }
                        }
                    }
                    is_valid = is_valid && block_valid;
                }
            }
        }
        return is_valid;
    }
}
