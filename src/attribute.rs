
use std::clone::Clone;
use std::fmt::Debug;
use std::marker::Copy;
use std::default::Default;
use ReframeResult;
use ReframeError;
use regl::VertexAttributeType;

pub trait VertexComponent {
    fn attribute_type() -> VertexAttributeType;
    fn component_count() -> u8;
}

pub trait NamedBaseAttributes {
    fn attributes() -> Vec<(String, BaseAttribute)>;
}

pub trait MapNameToAttributeIndex {
    fn map_name<T: AsRef<str>>(&self, name: T) -> Option<u32>;
}

impl<'a, N: AsRef<str>> MapNameToAttributeIndex for &'a [(N, u32)] {
    fn map_name<T: AsRef<str>>(&self, name: T) -> Option<u32> {
        self.iter().find(|&x| x.0.as_ref() == name.as_ref()).map(|x| x.1)
    }
}

pub fn named_attributes_to_indexed_attributes<M: MapNameToAttributeIndex>(
        named_attributes: &[(String, BaseAttribute)],
        mapper: &M
    ) -> ReframeResult<Vec<(u32, BaseAttribute)>> {
    named_attributes.iter()
                    .map(|attr| mapper.map_name(&attr.0).map(|index| (index, attr.1)))
                    .map(|attr| attr.ok_or(ReframeError::AttributeMappingError))
                    .collect()
}

pub type Vec2 = Vec2Type<f32>;
pub type Vec2i8 = Vec2Type<i8>;
pub type Vec2u8 = Vec2Type<u8>;
pub type Vec2i16 = Vec2Type<i16>;
pub type Vec2u16 = Vec2Type<u16>;
pub type Vec2i32 = Vec2Type<i32>;
pub type Vec2u32 = Vec2Type<u32>;
pub type Vec2f32 = Vec2Type<f32>;
pub type Vec2f64 = Vec2Type<f64>;

pub type Vec3 = Vec3Type<f32>;
pub type Vec3i8 = Vec3Type<i8>;
pub type Vec3u8 = Vec3Type<u8>;
pub type Vec3i16 = Vec3Type<i16>;
pub type Vec3u16 = Vec3Type<u16>;
pub type Vec3i32 = Vec3Type<i32>;
pub type Vec3u32 = Vec3Type<u32>;
pub type Vec3f32 = Vec3Type<f32>;
pub type Vec3f64 = Vec3Type<f64>;

pub type Vec4 = Vec4Type<f32>;
pub type Vec4i8 = Vec4Type<i8>;
pub type Vec4u8 = Vec4Type<u8>;
pub type Vec4i16 = Vec4Type<i16>;
pub type Vec4u16 = Vec4Type<u16>;
pub type Vec4i32 = Vec4Type<i32>;
pub type Vec4u32 = Vec4Type<u32>;
pub type Vec4f32 = Vec4Type<f32>;
pub type Vec4f64 = Vec4Type<f64>;

pub type Rgb = RgbType<u8>;
pub type Rgbi8 = RgbType<i8>;
pub type Rgbu8 = RgbType<u8>;
pub type Rgbi16 = RgbType<i16>;
pub type Rgbu16 = RgbType<u16>;
pub type Rgbi32 = RgbType<i32>;
pub type Rgbu32 = RgbType<u32>;
pub type Rgbf32 = RgbType<f32>;
pub type Rgbf64 = RgbType<f64>;

pub type Rgba = RgbaType<u8>;
pub type Rgbai8 = RgbaType<i8>;
pub type Rgbau8 = RgbaType<u8>;
pub type Rgbai16 = RgbaType<i16>;
pub type Rgbau16 = RgbaType<u16>;
pub type Rgbai32 = RgbaType<i32>;
pub type Rgbau32 = RgbaType<u32>;
pub type Rgbaf32 = RgbaType<f32>;
pub type Rgbaf64 = RgbaType<f64>;

macro_rules! vertexcomponent_type {
    ($impl_basetype:ident { $($field_name:ident),+ } ) => (
        #[repr(C,packed)]
        #[derive(Debug,Clone,Copy,Default)]
        pub struct $impl_basetype<T: Debug + Clone + Copy + Default> {
            $(pub $field_name: T),+
        }
        impl<T: Debug + Clone + Copy + Default> $impl_basetype<T> {
            pub fn new($($field_name: T),+) -> $impl_basetype<T> {
                $impl_basetype {
                    $($field_name: $field_name),+
                }
            }
        }
        vertexcomponent_type!($impl_basetype<i8> : Byte { $($field_name),+ });
        vertexcomponent_type!($impl_basetype<u8> : UnsignedByte { $($field_name),+ });
        vertexcomponent_type!($impl_basetype<i16> : Short { $($field_name),+ });
        vertexcomponent_type!($impl_basetype<u16> : UnsignedShort { $($field_name),+ });
        vertexcomponent_type!($impl_basetype<i32> : Int { $($field_name),+ });
        vertexcomponent_type!($impl_basetype<u32> : UnsignedInt { $($field_name),+ });
        vertexcomponent_type!($impl_basetype<f32> : Float { $($field_name),+ });
        vertexcomponent_type!($impl_basetype<f64> : Double { $($field_name),+ });
    );
    ($impl_type:ty : $attribute_type:ident { $($field_name:ident),+ } ) => (
        impl ::attribute::VertexComponent for $impl_type {
            fn attribute_type() -> ::regl::VertexAttributeType {
                ::regl::VertexAttributeType::$attribute_type
            }
            fn component_count() -> u8 {
                // The repetition needs to refer to a variable within the repetition, so let's be
                // redundant and refer to a variable within the repetition
                0 $(+ { let $field_name = 1; $field_name })+
            }
        }
    );
}

vertexcomponent_type!(Vec2Type { x, y });
vertexcomponent_type!(Vec3Type { x, y, z });
vertexcomponent_type!(Vec4Type { x, y, z, w });
vertexcomponent_type!(RgbType { r, g, b });
vertexcomponent_type!(RgbaType { r, g, b, a });

vertexcomponent_type!(i8 : Byte { x });
vertexcomponent_type!(u8 : UnsignedByte { x });
vertexcomponent_type!(i16 : Short { x });
vertexcomponent_type!(u16 : UnsignedShort { x });
vertexcomponent_type!(i32 : Int { x });
vertexcomponent_type!(u32 : UnsignedInt { x });
vertexcomponent_type!(f32 : Float { x });
vertexcomponent_type!(f64 : Double { x });

#[derive(Debug,Clone,Copy)]
pub struct BaseAttribute {
    pub size: u8,
    pub attribute_type: VertexAttributeType,
    pub normalized: bool,
    pub stride: u32,
    pub offset: u32,
}


#[macro_export]
macro_rules! vertex_type {
    (
        pub struct $struct_name:ident {
            $(pub $field_name:ident : $field_type:ident $(( $($key:ident = $value:expr),+ ))* ),+
        }
    ) => (
        #[repr(C)]
        #[derive(Debug,Clone,Copy)]
        pub struct $struct_name {
            $(pub $field_name: $crate::attribute::$field_type ),+
        }

        impl $crate::attribute::NamedBaseAttributes for $struct_name {
            // This is for the mutable attribute_helper.
            #[allow(unused_mut)]
            fn attributes() -> Vec<(String, $crate::attribute::BaseAttribute)> {
                use $crate::attribute::VertexComponent;
                use $crate::attribute::BaseAttribute;
                use ::std::mem::{size_of,transmute};

                let zeroed_struct = unsafe { ::std::mem::zeroed::<$struct_name>() };

                // This struct exists to avoid macro hygiene; only fields of this struct may
                // be overridden by user.
                struct AttributeHelper<'a> {
                    name: &'a str,
                    normalized: bool,
                }

                let mut attributes = vec![];
                $(attributes.push({
                    use $crate::attribute::$field_type as FieldType;
                    let mut attribute_helper = AttributeHelper {
                        name: stringify!($field_name),
                        normalized: false,
                    };

                    // Macro hygiene would make a new variable binding, made with
                    // "let $key = $value;", be invisible for the code within the macro.
                    // Using $key as *field name* avoids this.
                    $( $(attribute_helper.$key = $value;)+ )*

                    let size = <FieldType as VertexComponent>::component_count();
                    let attribute_type = <FieldType as VertexComponent>::attribute_type();
                    let stride = size_of::<$struct_name>() as u32;

                    // Scary!
                    let struct_start: *const $struct_name = &zeroed_struct;
                    let struct_start = unsafe { transmute::<_, usize>(struct_start) };
                    let field_start: *const FieldType = &zeroed_struct.$field_name;
                    let field_start = unsafe { transmute::<_, usize>(field_start) };
                    assert!(field_start >= struct_start);
                    let offset = (field_start - struct_start) as u32;

                    let attribute = BaseAttribute {
                        size: size,
                        attribute_type: attribute_type,
                        normalized: attribute_helper.normalized,
                        stride: stride,
                        offset: offset,
                    };
                    (attribute_helper.name.into(), attribute)
                });)+

                ::std::mem::forget(zeroed_struct);

                attributes
            }
        }
    );
}
