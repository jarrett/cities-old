// Types.
pub type Vector3d = vecmath_lib::Vector3<f32>;
pub type Matrix4d = vecmath_lib::Matrix4<f32>;

// Vector re-exports.
pub use vecmath_lib::vec3_add;
pub use vecmath_lib::vec3_sub;
pub use vecmath_lib::vec3_scale;
pub use vecmath_lib::vec3_normalized;

// Matrix re-exports.
pub use vecmath_lib::mat4_id;
pub use vecmath_lib::col_mat4_mul as mat4_mul;