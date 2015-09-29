
use ::regl::VertexAttributeType;

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C,packed)]
#[derive(Debug,Clone,Copy)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct BaseAttribute {
    pub attribute_type: VertexAttributeType,
    pub size: u8,
    pub normalize: bool,
}
