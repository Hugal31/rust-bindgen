/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
pub struct OtherOpaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_OtherOpaque() {
    assert_eq!(::std::mem::size_of::<OtherOpaque>() , 4usize);
    assert_eq!(::std::mem::align_of::<OtherOpaque>() , 4usize);
}
#[repr(C)]
pub struct Opaque;
#[repr(C)]
pub struct Struct_WithOpaquePtr {
    pub whatever: u64,
    pub other: u32,
    pub t: u32,
}
#[test]
fn bindgen_test_layout_Struct_WithOpaquePtr() {
    assert_eq!(::std::mem::size_of::<Struct_WithOpaquePtr>() , 16usize);
    assert_eq!(::std::mem::align_of::<Struct_WithOpaquePtr>() , 8usize);
}
