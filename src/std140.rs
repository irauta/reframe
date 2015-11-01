
pub const ALIGNMENT_1: u32 = 1 * 4;
pub const ALIGNMENT_2: u32 = 2 * 4;
pub const ALIGNMENT_3: u32 = 4 * 4; // Three-component types are aligned like four-component types!
pub const ALIGNMENT_4: u32 = 4 * 4;
pub const ALIGNMENT_STRUCT: u32 = ALIGNMENT_4;
pub const ALIGNMENT_ARRAY: u32 = ALIGNMENT_4;

pub trait Std140 {
    fn alignment() -> u32;
    fn aligned_size() -> u32;
}

/// This module exists only because the macro defined inside it shouldn't leak
/// to library users.
mod impl_traits {
    use uniform::{IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
    use super::*;
    macro_rules! simple_std140_impl {
        ($uniform_type:ty : $alignment:expr) => (
            impl Std140 for $uniform_type {
                fn alignment() -> u32 { $alignment }
                fn aligned_size() -> u32 { $alignment }
            }
        )
    }
    simple_std140_impl!(i32 : ALIGNMENT_1);
    simple_std140_impl!(u32 : ALIGNMENT_1);
    simple_std140_impl!(f32 : ALIGNMENT_1);
    simple_std140_impl!(bool : ALIGNMENT_1);

    // Missing: BVec{2,3,4}

    simple_std140_impl!(IVec2 : ALIGNMENT_2);
    simple_std140_impl!(IVec3 : ALIGNMENT_3);
    simple_std140_impl!(IVec4 : ALIGNMENT_4);

    simple_std140_impl!(UVec2 : ALIGNMENT_2);
    simple_std140_impl!(UVec3 : ALIGNMENT_3);
    simple_std140_impl!(UVec4 : ALIGNMENT_4);

    simple_std140_impl!(Vec2 : ALIGNMENT_2);
    simple_std140_impl!(Vec3 : ALIGNMENT_3);
    simple_std140_impl!(Vec4 : ALIGNMENT_4);
}


#[macro_export]
macro_rules! std140 {
    // The main matcher doesn't really look into the content of the struct definition.
    // The "fields" won't end up being actual fields after all.
    (pub struct $struct_name:ident {
        $($fields:tt)+
    }) => (
        pub struct $struct_name<'a> {
            buffer: &'a mut[u8],
        }

        impl<'a> $crate::std140::Std140 for $struct_name<'a> {
            fn alignment() -> u32 {
                $crate::std140::ALIGNMENT_STRUCT
            }

            fn aligned_size() -> u32 {
                use $crate::std140::ALIGNMENT_STRUCT;
                use $crate::uniform::align_up_to;
                let meta = std140!( layout { $($fields)+ } );
                align_up_to(meta.size, ALIGNMENT_STRUCT)
            }
        }

        impl<'a> $crate::uniform::MapBytesMut<'a> for $struct_name<'a> {
            type UniformType = $struct_name<'a>;
            type LayoutInfoType = ();
            fn map_bytes_mut(
                buffer: &'a mut [u8],
                _: Self::LayoutInfoType
            ) -> Self::UniformType where Self::UniformType: 'a {
                $struct_name {
                    buffer: buffer,
                }
            }
        }

        impl<'a> $struct_name<'a> {
            pub fn new(buffer: &'a mut [u8]) -> $crate::ReframeResult<$struct_name<'a>> {
                let meta = std140!( layout { $($fields)+ } );
                if meta.size as usize > buffer.len() {
                    Err($crate::ReframeError::TooSmallBufferError)
                } else {
                    Ok($struct_name {
                        buffer: buffer
                    })
                }
            }

            pub fn dump_meta() {
                println!("{:#?}", std140!( layout { $($fields)+ } ))
            }

            std140!( fields { $($fields)+ } { $($fields)+ } );

        }
    );

    // Array field matcher, recursive case.
    ( fields {
            pub $field_name:ident : [ $field_type:ty ; $array_size:expr ],
            $($rest:tt)+
        } { $($all_fields:tt)+ }
    ) => (
        std140!( array { $field_name : [ $field_type ; $array_size ] } { $($all_fields)+ });
        std140!( fields { $($rest)+ } { $($all_fields)+ } );
    );
    // Array field base case.
    ( fields {
            pub $field_name:ident : [ $field_type:ty ; $array_size:expr ]
        } { $($all_fields:tt)+ }
    ) => (
        std140!( array { $field_name : [ $field_type ; $array_size ] } { $($all_fields)+ });
    );

    // Regular field matcher, recursive case.
    ( fields {
            pub $field_name:ident : $field_type:ty, $($rest:tt)+
        } { $($all_fields:tt)+ }
    ) => (
        std140!( field { $field_name : $field_type  } { $($all_fields)+ });
        std140!( fields { $($rest)+ } { $($all_fields)+ } );
    );
    // Regular field base case.
    ( fields {
            pub $field_name:ident : $field_type:ty
        } { $($all_fields:tt)+ }
    ) => (
        std140!( field { $field_name : $field_type  } { $($all_fields)+ });
    );

    // The matchers that actually produce code:

    // Array field.
    ( array {
            $field_name:ident : [ $field_type:ty ; $array_size:expr ]
        } { $($all_fields:tt)+ }
    ) => (
        pub fn $field_name(&mut self, index: usize)
                -> <$field_type as $crate::uniform::MapBytesMut>::UniformType {
            use $crate::std140::ALIGNMENT_4;
            use $crate::uniform::align_up_to;
            let meta = std140!( layout { $($all_fields)+ } );
            let offset = meta.fields.$field_name.start as usize;
            let element_size = align_up_to(meta.fields.$field_name.size, ALIGNMENT_4) as usize;
            if index >= $array_size {
                panic!("Uniform array access out of bounds!");
            }
            let offset = offset + element_size * index;
            let mut buffer = &mut self.buffer[offset..];
            <$field_type as $crate::uniform::MapBytesMut>::map_bytes_mut(buffer, ())
        }
    );

    // Regular field.
    ( field { $field_name:ident : $field_type:ty  } { $($all_fields:tt)+ }) => (
        pub fn $field_name(&mut self)
                -> <$field_type as $crate::uniform::MapBytesMut>::UniformType {
            let meta = std140!( layout { $($all_fields)+ } );
            let offset = meta.fields.$field_name.start as usize;
            let mut buffer = &mut self.buffer[offset..];
            <$field_type as $crate::uniform::MapBytesMut>::map_bytes_mut(buffer, ())
        }
    );

    // The matchers below used to belong to a separate macro that produces std140 layout struct,
    // but was combined into one to make macro_use(std140) work properly.

    ( layout { $($fields:tt)+ } ) => ({
        use $crate::uniform::*;
        use $crate::std140::*;
        #[derive(Default,Debug)]
        struct FieldInfo {
            start: u32,
            size: u32,
        }
        #[derive(Default,Debug)]
        struct MetaStruct {
            fields: FieldStruct,
            size: u32,
        }
        std140!( field_struct { $($fields)+ } );
        let mut meta = <MetaStruct as Default>::default();
        std140!( fill_meta(meta) { $($fields)+ } );
        meta
    });

    ( field_struct { $(pub $field_name:ident : $field_type:ty),+ } ) => (
        #[derive(Default,Debug)]
        struct FieldStruct {
            $($field_name : FieldInfo),+
        }
    );

    // Array field matcher, recursive case.
    ( fill_meta($meta:ident) {
            pub $field_name:ident : [ $field_type:ty ; $array_len:expr ],
            $($rest:tt)+
        }
    ) => (
        std140!( fill_array_meta($meta) { $field_name : [ $field_type ; $array_len ] })
        std140!( fill_meta($meta) { $($rest)+ } )
    );
    // Array field base case.
    ( fill_meta($meta:ident) {
            pub $field_name:ident : [ $field_type:ty ; $array_len:expr ]
        }
    ) => (
        std140!( fill_array_meta($meta) { $field_name : [ $field_type ; $array_len ] } );
    );

    // Regular field matcher, recursive case.
    ( fill_meta($meta:ident) {
            pub $field_name:ident : $field_type:ty, $($rest:tt)+
        }
    ) => (
        std140!( fill_field_meta($meta) { $field_name : $field_type  })
        std140!( fill_meta($meta) { $($rest)+ } )
    );
    // Regular field base case.
    ( fill_meta($meta:ident) {
            pub $field_name:ident : $field_type:ty
        }
    ) => (
        std140!( fill_field_meta($meta) { $field_name : $field_type  } );
    );

    // The matchers that actually produce code:

    // Array field.
    ( fill_array_meta($meta:ident) {
            $field_name:ident : [ $field_type:ty ; $array_len:expr ]
        } ) => ({
        let prev_end = $meta.size;
        let base_alignment = align_up_to(<$field_type as Std140>::alignment(), ALIGNMENT_ARRAY);
        let start = align_up_to(prev_end, base_alignment);
        let array_size = base_alignment * $array_len;
        $meta.fields.$field_name.start = start;
        $meta.fields.$field_name.size = array_size;
        $meta.size = start + array_size;
    });

    // Regular field.
    ( fill_field_meta($meta:ident) { $field_name:ident : $field_type:ty  } ) => ({
        let prev_end = $meta.size;
        let start = align_up_to(prev_end, <$field_type as Std140>::alignment());
        let size = <$field_type as Std140>::aligned_size();
        $meta.fields.$field_name.start = start;
        $meta.fields.$field_name.size = size;
        $meta.size = start + size;
    });

}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use uniform::{Vec2, Vec3, Vec4};
    std140!(pub struct Foo140 {
        pub a: f32,
        pub b: [Vec4; 4],
        pub c: Vec3,
        pub d: Vec2
    });

}
