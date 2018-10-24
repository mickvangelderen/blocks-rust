#![feature(optin_builtin_traits)]
#![feature(const_fn)]

extern crate core;
extern crate gl;

mod attribute_location;
mod buffer_target;
mod framebuffer_attachment;
mod framebuffer_target;
mod functions;
mod marker;
mod max_combined_texture_image_units;
mod names;
mod num;
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

// Can't auto sort bc macro import order important.
#[macro_use]
pub mod string;

pub use self::attribute_location::*;
pub use self::buffer_target::*;
pub use self::framebuffer_attachment::*;
pub use self::framebuffer_target::*;
pub use self::functions::*;
pub use self::marker::*;
pub use self::max_combined_texture_image_units::*;
pub use self::names::*;
pub use self::num::*;
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
