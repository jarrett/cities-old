use std::num::Float;

pub fn assert_eq_f32(a: f32, b: f32) {
    assert!((a - b).abs() < 0.00001, "Expected {} but got {}", a, b);
}