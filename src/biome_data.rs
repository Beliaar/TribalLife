use core_types::{Color, GodotString, StringArray};
use gdnative::api::Resource;

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct BiomeData {
    #[property]
    pub name: GodotString,
    #[property]
    pub color: Color,
    //#[property]
    //pub modifiers: StringArray,
    #[property]
    pub voxel_type: i64,
}

#[methods]
impl BiomeData {
    pub fn new(_owner: &Resource) -> Self {
        BiomeData {
            name: GodotString::new(),
            color: Color::rgb(0.0, 0.0, 0.0),
            //modifiers: StringArray::new(),
            voxel_type: 0,
        }
    }
}
