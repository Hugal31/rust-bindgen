/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_LittleArray {
    pub a: [::std::os::raw::c_int; 32usize],
}
impl ::std::clone::Clone for Struct_LittleArray {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_LittleArray() {
    assert_eq!(::std::mem::size_of::<Struct_LittleArray>() , 128usize);
    assert_eq!(::std::mem::align_of::<Struct_LittleArray>() , 4usize);
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_BigArray {
    pub a: [::std::os::raw::c_int; 33usize],
}
impl ::std::clone::Clone for Struct_BigArray {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_BigArray() {
    assert_eq!(::std::mem::size_of::<Struct_BigArray>() , 132usize);
    assert_eq!(::std::mem::align_of::<Struct_BigArray>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_WithLittleArray {
    pub a: Struct_LittleArray,
}
impl ::std::clone::Clone for Struct_WithLittleArray {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_WithLittleArray() {
    assert_eq!(::std::mem::size_of::<Struct_WithLittleArray>() , 128usize);
    assert_eq!(::std::mem::align_of::<Struct_WithLittleArray>() , 4usize);
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_WithBigArray {
    pub a: Struct_BigArray,
}
impl ::std::clone::Clone for Struct_WithBigArray {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_WithBigArray() {
    assert_eq!(::std::mem::size_of::<Struct_WithBigArray>() , 132usize);
    assert_eq!(::std::mem::align_of::<Struct_WithBigArray>() , 4usize);
}
