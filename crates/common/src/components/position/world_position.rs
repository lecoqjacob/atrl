use crate::prelude::*;
use bevy::math::Vec3Swizzles;

#[derive(
    Reflect, Component, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Debug,
)]
#[reflect(Component)]
pub struct WorldPosition {
    pub position: IVec3,
}

impl WorldPosition {
    pub fn xy(&self) -> IVec2 { self.position.xy() }
}