use crate::prelude::*;
use atrl_macros::impl_as_primative;

#[derive(
    Reflect,
    FromReflect,
    Debug,
    Default,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[repr(u8)] // this must match with vision component
pub enum VisionType {
    #[default]
    None = 0,
    Blind = 1 << 0,
    BlackAndWhite = 1 << 1,
    Colored = 1 << 2,
    Infared = 1 << 3,
    XRay = 1 << 4,
    Any = !0,
}

impl From<VisionType> for u8 {
    fn from(value: VisionType) -> Self {
        value.to_u8().unwrap()
    }
}

impl_as_primative!(VisionType);