
use ::regl::VertexArray;
use ::ReframeResult;
use ::attribute::BaseAttribute;

pub fn make_simple_vertex_array<V: Copy>(
        vertices: &[V],
        indices: Option<&[u16]>,
        attributes: &[(u32, BaseAttribute)]
    ) -> ReframeResult<VertexArray> {

    unimplemented!()
}
