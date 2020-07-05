use crate::biome_data::BiomeData;
use gdnative::api::Object;
use gdnative::core_types::TypedArray;
use gdnative::prelude::*;
use std::collections::HashMap;

pub type BiomeDataArray = TypedArray<Ref<BiomeData>>;
