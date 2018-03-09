#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Block {
    Void = 0,
    Rock = 1,
    Dirt = 2,
}
