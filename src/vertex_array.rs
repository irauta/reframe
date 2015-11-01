
use ::regl::{Context,Buffer,BufferTarget,BufferUsage,VertexArray,VertexAttribute};
use ::ReframeResult;
use ::attribute::BaseAttribute;

pub fn make_simple_vertex_array<V: Copy>(
        context: &mut Context,
        vertices: &[V],
        indices: Option<&[u16]>,
        attributes: &[(u32, BaseAttribute)]
    ) -> ReframeResult<VertexArray> {
    let vertex_buffer = try!(Buffer::new(context, BufferTarget::VertexBuffer, BufferUsage::StaticDraw, vertices));
    let index_buffer = match indices {
        Some(indices) => Some(try!(Buffer::new(context, BufferTarget::IndexBuffer, BufferUsage::StaticDraw, indices))),
        None => None,
    };
    let attributes = attributes.into_iter().map(|attr| VertexAttribute {
        index: attr.0,
        size: attr.1.size,
        attribute_type: attr.1.attribute_type,
        normalized: attr.1.normalized,
        stride: attr.1.stride,
        offset: attr.1.offset,
        vertex_buffer: &vertex_buffer,
    });
    Ok(try!(VertexArray::new(context, attributes, index_buffer.as_ref())))
}
