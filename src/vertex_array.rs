
use ::regl::VertexArray;
use ::ReframeResult;
use ::attribute::NamedBaseAttributes;
use ::attribute::MapNameToAttributeIndex;

pub fn make_simple_vertex_array<V: NamedBaseAttributes, M: >(
        vertices: &[V],
        indices: Option<&[u16]>,
        attribute_indices: &M
    ) -> ReframeResult<VertexArray> {
    let attributes = <V as NamedBaseAttributes>::attributes();
    unimplemented!()
}
