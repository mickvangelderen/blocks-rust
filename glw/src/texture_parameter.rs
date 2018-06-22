pub trait TextureParameterI32Value {
    fn as_i32(&self) -> i32;
}

pub trait TextureParameterI32Key {
    type Value: TextureParameterI32Value;

    fn as_u32(&self) -> u32;
}
