use crate::biome_data::BiomeData;
use crate::biome_dictionary::BiomeDataArray;
use gdnative::api::{Object, Resource, Texture, VoxelBuffer, VoxelGenerator};
use gdnative::core_types::{StringArray, VariantArray, Vector2, Vector3};
use gdnative::prelude::user_data::UserData;
use gdnative::prelude::*;
use gdnative::*;
use std::{collections::HashMap, str::FromStr};
use thread_access::Shared;

#[derive(NativeClass)]
#[inherit(VoxelGenerator)]
#[user_data(user_data::MutexData<VoxelGeneratorDF>)]
#[register_with(Self::register_class)]
pub struct VoxelGeneratorDF {
    #[property]
    elevation_map: Option<Ref<Texture>>,
    #[property]
    biome_map: Option<Ref<Texture>>,
    #[property]
    max_height: i64,
    map_height_data: Vec<i64>,
    map_type_data: Vec<i64>,
    size: Vector2,
    data_calculated: bool,
    biome_dictionary: VariantArray<Unique>,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl VoxelGeneratorDF {
    /// The "constructor" of the class.
    fn new(_owner: &VoxelGenerator) -> Self {
        VoxelGeneratorDF {
            elevation_map: None,
            biome_map: None,
            max_height: 100,
            map_height_data: Vec::new(),
            map_type_data: Vec::new(),
            size: Vector2::new(0.0, 0.0),
            data_calculated: false,
            biome_dictionary: VariantArray::new(),
        }
    }

    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&mut self, owner: &VoxelGenerator) {
        if !self.data_calculated {
            self.update_map_data(owner);
        }
    }

    fn register_class(builder: &ClassBuilder<Self>) {
        builder
            .add_property("BiomeCount")
            .with_usage(PropertyUsage::EDITOR)
            .with_default(0)
            .with_getter(|instance, _| instance.biome_dictionary.len() as i32)
            .with_setter(Self::set_biome_count)
            .done();
    }

    fn set_biome_count(&mut self, owner: &VoxelGenerator, value: i32) {
        if value > self.biome_dictionary.len() {
            for _ in self.biome_dictionary.len()..value {
                self.biome_dictionary
                    .push(Self::create_biome_data().into_base());
            }
        } else {
            for _ in 0..self.biome_dictionary.len() - value {
                self.biome_dictionary.pop();
            }
        }
        owner.property_list_changed_notify();
    }

    fn create_biome_data() -> Instance<BiomeData, Shared> {
        Instance::<BiomeData, Unique>::new().into_shared()
    }

    #[export]
    pub fn _get(&self, _owner: &VoxelGenerator, property: GodotString) -> Variant {
        if property.begins_with(&GodotString::from_str("biome_data/")) {
            let index: String = property.to_string().splitn(2, '/').skip(1).collect();
            let index = match i32::from_str(index.as_str()) {
                Ok(index) => index,
                Err(_) => panic!(),
            };
            if self.biome_dictionary.len() > index {
                self.biome_dictionary.get(index)
            } else {
                Variant::new()
            }
        } else {
            Variant::new()
        }
    }

    #[export]
    pub fn _set(&mut self, _owner: &VoxelGenerator, property: GodotString, value: Variant) -> bool {
        if property.begins_with(&GodotString::from_str("biome_data/")) {
            let index: String = property.to_string().splitn(2, '/').skip(1).collect();
            let index = match i32::from_str(index.as_str()) {
                Ok(index) => index,
                Err(_) => panic!(),
            };
            if self.biome_dictionary.len() <= index {
                self.biome_dictionary.resize(index + 1);
            }

            self.biome_dictionary.insert(index, value);

            true
        } else {
            false
        }
    }

    #[export]
    pub fn _get_property_list(&self, _owner: &VoxelGenerator) -> VariantArray<Shared> {
        let list = unsafe { VariantArray::new_shared().assume_unique() };
        for index in 0..self.biome_dictionary.len() {
            let info = unsafe {
                let info = Dictionary::new_shared().assume_unique();
                info.insert(
                    &GodotString::from_str("name").to_variant(),
                    &GodotString::from_str(format!("biome_data/{}", index)).to_variant(),
                );
                info.insert(
                    &GodotString::from_str("type").to_variant(),
                    &Variant::from_u64(VariantType::Object as u64),
                );
                info.into_shared()
            };
            list.push(&Variant::from_dictionary(&info));
        }
        list.into_shared()
    }

    #[export]
    fn update_map_data(&mut self, _owner: &VoxelGenerator) {
        let elevation_image = match &self.elevation_map {
            Some(elevation_image) => unsafe { elevation_image.assume_safe() }.get_data(),
            None => panic!(""),
        };

        let elevation_image = match elevation_image {
            Some(elevation_image) => elevation_image,
            None => panic!(),
        };

        let biome_image = match &self.biome_map {
            Some(biome_image) => unsafe { biome_image.assume_safe() }.get_data(),
            None => panic!(""),
        };

        let mut biome_lookup = Vec::new();
        for data in &self.biome_dictionary {
            let resource = data.try_to_object::<Resource>();

            if let Some(resource) = resource {
                let resource = unsafe { resource.assume_safe() };
                let data = BiomeData::new(&resource);

                biome_lookup.push(data);
            }
        }

        unsafe { elevation_image.assume_safe() }.lock();
        self.size = unsafe { elevation_image.assume_safe() }.get_size();
        let vec_length = (self.size.x * self.size.y) as usize;
        self.map_height_data = Vec::with_capacity(vec_length);
        self.map_height_data.resize(vec_length, 0);
        self.map_type_data = Vec::with_capacity(vec_length);
        self.map_type_data.resize(vec_length, 0);
        for x in 0..self.size.x as usize {
            for z in 0..self.size.y as usize {
                let map_pos = Vector2::new(x as f32, z as f32);
                let mut voxel_type = 1;
                if let Some(biome_image) = &biome_image {
                    unsafe { biome_image.assume_safe() }.lock();
                    let biome_color: Color =
                        unsafe { biome_image.assume_safe() }.get_pixelv(map_pos);
                    let data = biome_lookup
                        .iter()
                        .find(|data| data.color.v() == biome_color.v());
                    if let Some(data) = data {
                        voxel_type = data.voxel_type;
                    }
                    unsafe { biome_image.assume_safe() }.unlock();
                }
                let elevation_color = unsafe { elevation_image.assume_safe() }.get_pixelv(map_pos);
                let value = elevation_color.v();
                let height = ((self.max_height as f32) * value) as i64;
                let index = z * (self.size.y as usize) + x;
                self.map_height_data[index] = height;
                self.map_type_data[index] = voxel_type;
            }
        }
        unsafe { elevation_image.assume_safe() }.unlock();
        self.data_calculated = true;
    }

    #[export]
    fn get_used_channels_mask(&self, _owner: &VoxelGenerator) -> i64 {
        1 << VoxelBuffer::CHANNEL_TYPE
    }

    #[export]
    fn generate_block(
        &self,
        _owner: &VoxelGenerator,
        buffer: Ref<VoxelBuffer>,
        origin: Vector3,
        _lod: i64,
    ) {
        let x_size;
        let y_size;
        unsafe {
            x_size = buffer.assume_safe().get_size_x();
            y_size = buffer.assume_safe().get_size_z();
        }
        for x in 0..x_size {
            for z in 0..y_size {
                let map_pos = Vector2::new(
                    self.size.x / 2.0 + origin.x + x as f32,
                    self.size.y / 2.0 + origin.z + z as f32,
                );
                if !(map_pos.x < self.size.x)
                    || !(map_pos.y < self.size.y)
                    || !(map_pos.x > 0.0)
                    || !(map_pos.y > 0.0)
                {
                    continue;
                }
                let index = (map_pos.y * self.size.y + map_pos.x) as usize;
                let height = self.map_height_data[index] as i64;
                let voxel_type = self.map_type_data[index];
                let top_y = origin.y as i64 + (y_size);
                if top_y < height {
                    for y in 0..y_size {
                        unsafe {
                            buffer.assume_safe().set_voxel(
                                voxel_type,
                                x,
                                y,
                                z,
                                VoxelBuffer::CHANNEL_TYPE,
                            );
                        }
                    }
                } else if (origin.y as i64) < self.max_height && height > origin.y as i64 {
                    for y in 0..(top_y - height) {
                        unsafe {
                            buffer.assume_safe().set_voxel(
                                voxel_type,
                                x,
                                y,
                                z,
                                VoxelBuffer::CHANNEL_TYPE,
                            );
                        }
                    }
                }
            }
        }
    }
}
