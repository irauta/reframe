
use ::std::rc::Rc;
use ::regl::{VertexArray,PrimitiveMode,IndexType};

pub enum DrawType {
    /// Non-indexed drawing.
    NonIndexed,
    /// Indexed drawing.
    Indexed {
        /// u8, u16 or u32 indices?
        index_type: IndexType,
        /// How far in the index buffer is the first index?
        starting_index: u32,
    },
}

pub struct IndexedDrawParameters {
    /// Triangles, lines or something else.
    pub primitive_mode: PrimitiveMode,
    /// Non-indexed or indexed? See the enum for details.
    pub draw_type: DrawType,
    /// First vertex in the vertex buffer to draw, or the baseindex when doing indexed drawing.
    pub first_vertex: u32,
    /// How many vertices/elements to draw. When drawing triangles, value of this field is
    /// 3 * number of triangles.
    pub count: u32,
    /// Not really supported yet...but should tell the number of instances. Zero for effectively
    /// no instancing.
    pub instance_count: u32,
    /// Vertex array that has the vertices and indices to draw from.
    /// Not sure if this field belongs to this structure.
    pub vertex_array: Rc<VertexArray>
}
