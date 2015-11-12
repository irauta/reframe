
extern crate regl;

pub mod error;
pub mod program;
pub mod attribute;
pub mod uniform;
pub mod std140;
pub mod mesh;
pub mod vertex_array;

pub type ReframeResult<T> = Result<T, ReframeError>;

pub use error::ReframeError;
