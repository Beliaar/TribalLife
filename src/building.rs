use gdnative::api::Spatial;
use gdnative::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Building {}

#[methods]
impl Building {
    fn new(_owner: &Spatial) -> Self {
        Building {}
    }
}
