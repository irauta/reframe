
#![macro_use]

extern crate regl;

pub mod error;
#[macro_use]
pub mod attribute;
pub mod uniform;
#[macro_use]
pub mod std140;
pub mod mesh;
pub mod vertex_array;

pub use regl::load_with;

pub type ReframeResult<T> = Result<T, ReframeError>;

pub use error::ReframeError;

#[cfg(test)]
mod tests {

    use ::uniform::{Vec2,Vec3,Vec4};
    std140!(pub struct Foo140 {
        pub a: f32,
        pub b: [Vec4; 4],
        pub c: Vec3,
        pub d: Vec2
    });

}
