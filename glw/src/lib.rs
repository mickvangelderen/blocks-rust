#![feature(optin_builtin_traits)]
#![feature(const_int_wrapping)]
#![feature(const_fn)]

extern crate core;
extern crate gl;

// Macros first.
#[macro_use]
mod name;
#[macro_use]
pub mod string;

mod array;
mod attribute_location;
mod buffer_target;
mod framebuffer_attachment;
mod framebuffer_status;
mod framebuffer_target;
mod functions;
mod get_programiv_param;
mod get_shaderiv_param;
mod marker;
mod max_combined_texture_image_units;
mod num;
mod param;
mod program;
mod shader;
mod shader_kind;
mod small_ref;
mod texture_filter;
mod texture_parameter;
mod texture_target;
mod texture_unit;
mod texture_wrap;
mod uniform_location;
mod viewport;

pub use self::attribute_location::*;
pub use self::buffer_target::*;
pub use self::framebuffer_attachment::*;
pub use self::framebuffer_status::*;
pub use self::framebuffer_target::*;
pub use self::functions::*;
pub use self::get_programiv_param::*;
pub use self::get_shaderiv_param::*;
pub use self::marker::*;
pub use self::max_combined_texture_image_units::*;
pub use self::name::*;
pub use self::num::*;
pub use self::param::*;
pub use self::program::*;
pub use self::shader::*;
pub use self::shader_kind::*;
pub use self::small_ref::*;
pub use self::string::*;
pub use self::texture_filter::*;
pub use self::texture_parameter::*;
pub use self::texture_target::*;
pub use self::texture_unit::*;
pub use self::texture_wrap::*;
pub use self::uniform_location::*;
pub use self::viewport::*;
