
use std::clone::Clone;
use std::fmt::Debug;
use std::marker::Copy;
use ::regl::VertexAttributeType;

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct Vec2Type<T: Debug + Clone + Copy> {
    pub x: T,
    pub y: T,
}

pub type Vec2 = Vec2Type<f32>;
pub type Vec2i8 = Vec2Type<i8>;
pub type Vec2u8 = Vec2Type<u8>;
pub type Vec2i16 = Vec2Type<i16>;
pub type Vec2u16 = Vec2Type<u16>;
pub type Vec2i32 = Vec2Type<i32>;
pub type Vec2u32 = Vec2Type<u32>;
pub type Vec2f32 = Vec2Type<f32>;
pub type Vec2f64 = Vec2Type<f64>;

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct Vec3Type<T: Debug + Clone + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3 = Vec3Type<f32>;
pub type Vec3i8 = Vec3Type<i8>;
pub type Vec3u8 = Vec3Type<u8>;
pub type Vec3i16 = Vec3Type<i16>;
pub type Vec3u16 = Vec3Type<u16>;
pub type Vec3i32 = Vec3Type<i32>;
pub type Vec3u32 = Vec3Type<u32>;
pub type Vec3f32 = Vec3Type<f32>;
pub type Vec3f64 = Vec3Type<f64>;

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct Vec4Type<T: Debug + Clone + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub type Vec4 = Vec4Type<f32>;
pub type Vec4i8 = Vec4Type<i8>;
pub type Vec4u8 = Vec4Type<u8>;
pub type Vec4i16 = Vec4Type<i16>;
pub type Vec4u16 = Vec4Type<u16>;
pub type Vec4i32 = Vec4Type<i32>;
pub type Vec4u32 = Vec4Type<u32>;
pub type Vec4f32 = Vec4Type<f32>;
pub type Vec4f64 = Vec4Type<f64>;

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct RgbType<T: Debug + Clone + Copy> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub type Rgb = RgbType<u8>;
pub type Rgbi8 = RgbType<i8>;
pub type Rgbu8 = RgbType<u8>;
pub type Rgbi16 = RgbType<i16>;
pub type Rgbu16 = RgbType<u16>;
pub type Rgbi32 = RgbType<i32>;
pub type Rgbu32 = RgbType<u32>;
pub type Rgbf32 = RgbType<f32>;
pub type Rgbf64 = RgbType<f64>;

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct RgbaType<T: Debug + Clone + Copy> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

pub type Rgba = RgbaType<u8>;
pub type Rgbai8 = RgbaType<i8>;
pub type Rgbau8 = RgbaType<u8>;
pub type Rgbai16 = RgbaType<i16>;
pub type Rgbau16 = RgbaType<u16>;
pub type Rgbai32 = RgbaType<i32>;
pub type Rgbau32 = RgbaType<u32>;
pub type Rgbaf32 = RgbaType<f32>;
pub type Rgbaf64 = RgbaType<f64>;

macro_rules! impl_vertexcomponent {
    ($impl_basetype:ident : $size:expr ) => (
        impl_vertexcomponent!($impl_basetype<i8> : Byte, $size);
        impl_vertexcomponent!($impl_basetype<u8> : UnsignedByte, $size);
        impl_vertexcomponent!($impl_basetype<i16> : Short, $size);
        impl_vertexcomponent!($impl_basetype<u16> : UnsignedShort, $size);
        impl_vertexcomponent!($impl_basetype<i32> : Int, $size);
        impl_vertexcomponent!($impl_basetype<u32> : UnsignedInt, $size);
        impl_vertexcomponent!($impl_basetype<f32> : Float, $size);
        impl_vertexcomponent!($impl_basetype<f64> : Double, $size);
    );
    ($impl_type:ty : $attribute_type:ident, $size:expr ) => (
        impl ::traits::VertexComponent for $impl_type {
            fn attribute_type() -> ::regl::VertexAttributeType {
                ::regl::VertexAttributeType::$attribute_type
            }
            fn component_count() -> u8 {
                $size
            }
        }
    );
}

impl_vertexcomponent!(Vec2Type : 2);
impl_vertexcomponent!(Vec3Type : 3);
impl_vertexcomponent!(Vec4Type : 4);
impl_vertexcomponent!(RgbType : 3);
impl_vertexcomponent!(RgbaType : 4);

impl_vertexcomponent!(i8 : Byte, 1);
impl_vertexcomponent!(u8 : UnsignedByte, 1);
impl_vertexcomponent!(i16 : Short, 1);
impl_vertexcomponent!(u16 : UnsignedShort, 1);
impl_vertexcomponent!(i32 : Int, 1);
impl_vertexcomponent!(u32 : UnsignedInt, 1);
impl_vertexcomponent!(f32 : Float, 1);
impl_vertexcomponent!(f64 : Double, 1);

#[derive(Debug,Clone,Copy)]
pub struct BaseAttribute {
    pub size: u8,
    pub attribute_type: VertexAttributeType,
    pub normalized: bool,
    pub stride: u32,
    pub offset: u32,
}
