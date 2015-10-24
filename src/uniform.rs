
// Hopefully there's no need to wrap native types
// pub struct Bool;
// pub struct Int;
// pub struct UInt;
// pub struct Float;
// pub struct Double;

pub trait MapBytesMut<'a> {
    type UniformType: 'a;
    type LayoutInfoType;
    fn map_bytes_mut(
        buffer: &'a mut [u8],
        layout_info: Self::LayoutInfoType
    ) -> Self::UniformType where Self::UniformType: 'a;
}

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

// Not yet sure how these would be represented
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

pub fn bytes_as_typed_mut_ref<T: Sized>(buffer: &mut [u8]) -> &mut T {
    let size_of = ::std::mem::size_of::<T>();
    // Will panic is there's not enough data, keeping us "safe"
    let buffer = &mut buffer[0..size_of];
    let ptr = buffer.as_mut_ptr() as *mut T;
    unsafe { &mut *ptr }
}

macro_rules! simple_map_bytes_mut_impl {
    ($uniform_type:ty) => (
        impl<'a> MapBytesMut<'a> for $uniform_type {
            type UniformType = &'a mut $uniform_type;
            // The primitive types don't need anything to construct themselves.
            // Matrices on the other hand need stride info, but they're not handled by this macro.
            type LayoutInfoType = ();

            fn map_bytes_mut(buffer: &'a mut [u8], _: ()) -> &'a mut $uniform_type {
                bytes_as_typed_mut_ref(buffer)
            }
        }
    )
}
simple_map_bytes_mut_impl!(i32);
simple_map_bytes_mut_impl!(u32);
simple_map_bytes_mut_impl!(f32);
simple_map_bytes_mut_impl!(bool);

// Missing: BVec{2,3,4}

simple_map_bytes_mut_impl!(IVec2);
simple_map_bytes_mut_impl!(IVec3);
simple_map_bytes_mut_impl!(IVec4);

simple_map_bytes_mut_impl!(UVec2);
simple_map_bytes_mut_impl!(UVec3);
simple_map_bytes_mut_impl!(UVec4);

simple_map_bytes_mut_impl!(Vec2);
simple_map_bytes_mut_impl!(Vec3);
simple_map_bytes_mut_impl!(Vec4);
