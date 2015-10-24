
#![macro_use]

extern crate regl;

#[macro_use]
pub mod attribute;
pub mod uniform;
#[macro_use]
pub mod std140;
pub mod mesh;

pub use regl::load_with;

use ::uniform::{Vec2,Vec3,Vec4};
std140!(pub struct Foo140 {
    pub a: f32,
    pub b: [Vec4; 4],
    pub c: Vec3,
    pub d: Vec2
});

#[test]
fn foo() {
    Foo140::dump_meta();
}
