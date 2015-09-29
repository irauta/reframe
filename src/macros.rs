
macro_rules! vertex_type {
    (
        pub struct $struct_name:ident {
            $(pub $field_name:ident : $field_type:ident),+
        }
    ) => (
        #[repr(C,packed)]
        #[derive(Debug,Clone,Copy)]
        pub struct $struct_name {
            $(pub $field_name: $crate::attribute::$field_type),+
        }
    );
}

/* macro_rules! provide_attributes {
    ($struct_name:ident : $($field_name:ident),+) => (
        impl
    )
} */

vertex_type!(pub struct Simple {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoords: Vec2
});
