
use ::uniform::{IVec2,IVec3,IVec4,UVec2,UVec3,UVec4,Vec2,Vec3,Vec4};

pub const ALIGNMENT_1: u32 = 1 * 4;
pub const ALIGNMENT_2: u32 = 2 * 4;
pub const ALIGNMENT_3: u32 = 4 * 4;
pub const ALIGNMENT_4: u32 = 4 * 4;
pub const ALIGNMENT_STRUCT: u32 = ALIGNMENT_4;
pub const ALIGNMENT_ARRAY: u32 = ALIGNMENT_4;

pub trait Std140 : Sized {
    fn alignment() -> u32;
    fn aligned_size() -> u32;
    fn from_buffer(buffer: &mut [u8]) -> &mut Self {
        let size_of = ::std::mem::size_of::<Self>();
        // Will panic is there's not enough data, keeping us "safe"
        let buffer = &mut buffer[0..size_of];
        let ptr = buffer.as_mut_ptr() as *mut Self;
        unsafe { &mut *ptr }
    }
}

impl Std140 for i32 {
    fn alignment() -> u32 { ALIGNMENT_1 }
    fn aligned_size() -> u32 { ALIGNMENT_1 }
}
impl Std140 for u32 {
    fn alignment() -> u32 { ALIGNMENT_1 }
    fn aligned_size() -> u32 { ALIGNMENT_1 }
}
impl Std140 for f32 {
    fn alignment() -> u32 { ALIGNMENT_1 }
    fn aligned_size() -> u32 { ALIGNMENT_1 }
}
/*impl Std140 for bool {
    fn alignment() -> u32 { 4 }
}*/
impl Std140 for IVec2 {
    fn alignment() -> u32 { ALIGNMENT_2 }
    fn aligned_size() -> u32 { ALIGNMENT_2 }
}
impl Std140 for IVec3 {
    fn alignment() -> u32 { ALIGNMENT_3 }
    fn aligned_size() -> u32 { ALIGNMENT_3 }
}
impl Std140 for IVec4 {
    fn alignment() -> u32 { ALIGNMENT_4 }
    fn aligned_size() -> u32 { ALIGNMENT_4 }
}
impl Std140 for UVec2 {
    fn alignment() -> u32 { ALIGNMENT_2 }
    fn aligned_size() -> u32 { ALIGNMENT_2 }
}
impl Std140 for UVec3 {
    fn alignment() -> u32 { ALIGNMENT_3 }
    fn aligned_size() -> u32 { ALIGNMENT_3 }
}
impl Std140 for UVec4 {
    fn alignment() -> u32 { ALIGNMENT_4 }
    fn aligned_size() -> u32 { ALIGNMENT_4 }
}
impl Std140 for Vec2 {
    fn alignment() -> u32 { ALIGNMENT_2 }
    fn aligned_size() -> u32 { ALIGNMENT_2 }
}
impl Std140 for Vec3 {
    fn alignment() -> u32 { ALIGNMENT_3 }
    fn aligned_size() -> u32 { ALIGNMENT_3 }
}
impl Std140 for Vec4 {
    fn alignment() -> u32 { ALIGNMENT_4 }
    fn aligned_size() -> u32 { ALIGNMENT_4 }
}


macro_rules! std140 {
    // The main matcher doesn't really look into the content of the struct definition.
    // The "fields" won't end up being actual fields after all.
    (pub struct $struct_name:ident {
        $($fields:tt)+
    }) => (
        pub struct $struct_name<'a> {
            buffer: &'a mut[u8],
        }

        impl<'a> $struct_name<'a> {
            pub fn new(buffer: &'a mut [u8]) -> $struct_name<'a> {
                $struct_name {
                    buffer: buffer
                }
            }

            pub fn dump_meta() {
                println!("{:#?}", std140_layout!( { $($fields)+ } ))
            }

            std140!( fields { $($fields)+ } { $($fields)+ } );

        }

        impl<'a> $crate::std140::Std140 for $struct_name<'a> {
            fn alignment() -> u32 {
                $crate::std140::ALIGNMENT_STRUCT
            }

            fn aligned_size() -> u32 {
                use $crate::std140::ALIGNMENT_STRUCT;
                use $crate::uniform::align_up_to;
                let meta = std140_layout!( { $($fields)+ } );
                align_up_to(meta.size, ALIGNMENT_STRUCT)
            }
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
    ( array { $field_name:ident : [ $field_type:ty ; $array_size:expr ] } { $($all_fields:tt)+ }) => (
        pub fn $field_name(index: usize) {
            #![allow(unused_variables)]
            let meta = std140_layout!( { $($all_fields)+ } );
            let array_field = stringify!($field_name:ident : [ $field_type:ty ; $array_size:expr ]);
        }
    );

    // Regular fields.
    ( field { $field_name:ident : $field_type:ty  } { $($all_fields:tt)+ }) => (
        pub fn $field_name() {
            #![allow(unused_variables)]
            let meta = std140_layout!( { $($all_fields)+ } );
            let field = stringify!($field_name:ident : $field_type:ty);
        }
    );
}

// Helper macro, not meant to be invoked directly. Returns a FieldInfo struct that describes the
// layout and size of the type, based on the fields of the struct.
macro_rules! std140_layout {
    ( { $($fields:tt)+ } ) => ({
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
        std140_layout!( field_struct { $($fields)+ } );
        let mut meta = <MetaStruct as Default>::default();
        std140_layout!( fill_meta(meta) { $($fields)+ } );
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
        std140_layout!( fill_array_meta($meta) { $field_name : [ $field_type ; $array_len ] })
        std140_layout!( fill_meta($meta) { $($rest)+ } )
    );
    // Array field base case.
    ( fill_meta($meta:ident) {
            pub $field_name:ident : [ $field_type:ty ; $array_len:expr ]
        }
    ) => (
        std140_layout!( fill_array_meta($meta) { $field_name : [ $field_type ; $array_len ] } );
    );

    // Regular field matcher, recursive case.
    ( fill_meta($meta:ident) {
            pub $field_name:ident : $field_type:ty, $($rest:tt)+
        }
    ) => (
        std140_layout!( fill_field_meta($meta) { $field_name : $field_type  })
        std140_layout!( fill_meta($meta) { $($rest)+ } )
    );
    // Regular field base case.
    ( fill_meta($meta:ident) {
            pub $field_name:ident : $field_type:ty
        }
    ) => (
        std140_layout!( fill_field_meta($meta) { $field_name : $field_type  } );
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
