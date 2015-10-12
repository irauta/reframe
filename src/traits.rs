
use ::regl::VertexAttributeType;
use ::attribute::BaseAttribute;

pub trait VertexComponent {
    fn attribute_type() -> VertexAttributeType;
    fn component_count() -> u8;
}

pub trait NamedBaseAttributes {
    fn attributes() -> Vec<(String, BaseAttribute)>;
}
