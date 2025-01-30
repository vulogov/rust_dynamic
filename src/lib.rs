#[allow(dead_code)]

pub mod applicative;
pub mod attr;
pub mod bind;
pub mod bincode;
pub mod carcdr;
pub mod cast;
pub mod cast_json_to_value;
pub mod cast_value_to_json;
pub mod cast_matrix;
pub mod wrap_json;
pub mod cast_generic;
pub mod conv;
pub mod conv_inner;
pub mod ctx;
pub mod ctxapplicative;
pub mod create;
pub mod create_generic;
pub mod create_special;
pub mod create_special_float;
pub mod create_list;
pub mod create_matrix;
pub mod create_map;
pub mod create_curry;
pub mod create_complex;
pub mod create_metrics;
pub mod checker;
pub mod display;
pub mod dup;
pub mod q;
pub mod func;
pub mod export;
pub mod message;
pub mod types;
pub mod error;
pub mod push;
pub mod push_inplace;
pub mod pull;
pub mod pop;
pub mod hash;
pub mod eq;
pub mod ord;
pub mod map;
pub mod metric;
pub mod math;
pub mod reduce;
pub mod len;
pub mod create_lambda;
pub mod id;
pub mod iter;
pub mod json;
pub mod set;
pub mod get;
pub mod has_key;
pub mod tags;
pub mod timestamp;
pub mod value;
pub mod value_types;
pub mod tensor;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}
