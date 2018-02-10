// See mod.rs for how the UI structs fit together.
pub trait Widget {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn w(&self) -> i32;
    fn h(&self) -> i32;
    fn texture_data(&self) -> Vec<u8>;
}