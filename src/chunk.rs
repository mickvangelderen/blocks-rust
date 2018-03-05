use block::Block;

pub const CHUNK_SIDE_BLOCKS: usize = 32;
pub const CHUNK_TOTAL_BLOCKS: usize = CHUNK_SIDE_BLOCKS * CHUNK_SIDE_BLOCKS * CHUNK_SIDE_BLOCKS;

pub struct Chunk {
    pub blocks: [Block; CHUNK_TOTAL_BLOCKS],
}

impl Chunk {
    #[inline]
    pub fn block_index(x: usize, y: usize, z: usize) -> usize {
        (z * CHUNK_SIDE_BLOCKS + y) * CHUNK_SIDE_BLOCKS + x
    }

    pub fn block_at(&self, x: usize, y: usize, z: usize) -> &Block {
        &self.blocks[Self::block_index(x, y, z)]
    }

    pub fn block_at_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Block {
        &mut self.blocks[Self::block_index(x, y, z)]
    }
}
