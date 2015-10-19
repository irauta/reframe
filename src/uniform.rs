
// Hopefully there's no need to wrap native types
// pub struct Bool;
// pub struct Int;
// pub struct UInt;
// pub struct Float;
// pub struct Double;

// Deal with booleans later...
/* #[repr(C, packed)]
pub struct BVec2 {
    pub x: bool,
    padding1: [u8; 3],
    pub y: bool,
    padding2: [u8; 3],
}

#[repr(C, packed)]
pub struct BVec3 {
    pub x: bool,
    padding1: [u8; 3],
    pub y: bool,
    padding2: [u8; 3],
    pub z: bool,
    padding3: [u8; 3],
}

#[repr(C, packed)]
pub struct BVec4 {
    pub x: bool,
    padding1: [u8; 3],
    pub y: bool,
    padding2: [u8; 3],
    pub z: bool,
    padding3: [u8; 3],
    pub w: bool,
    padding4: [u8; 3],
} */

#[repr(C)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[repr(C)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

#[repr(C)]
pub struct UVec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[repr(C)]
pub struct UVec4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// Not implemented, only available on OpenGL 4.0 or with ARB_gpu_shader_fp64 extension
// pub struct DVec2;
// pub struct DVec3;
// pub struct DVec4;

pub struct Mat2;
pub struct Mat3;
pub struct Mat4;
pub struct Mat2x3;
pub struct Mat2x4;
pub struct Mat3x2;
pub struct Mat3x4;
pub struct Mat4x2;
pub struct Mat4x3;

pub fn align_up_to(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) / alignment * alignment
}
