
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

        impl $crate::traits::NamedBaseAttributes for $struct_name {
            // This is for the mutable attribute_helper.
            #[allow(unused_mut)]
            fn attributes() -> Vec<(String, $crate::attribute::BaseAttribute)> {
                use $crate::traits::VertexComponent;
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
