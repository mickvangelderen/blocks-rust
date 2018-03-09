use block::Block;
use cgmath::*;

pub const CHUNK_SIDE_BLOCKS: usize = 16;
pub const CHUNK_TOTAL_BLOCKS: usize = CHUNK_SIDE_BLOCKS * CHUNK_SIDE_BLOCKS * CHUNK_SIDE_BLOCKS;

pub struct Chunk {
    pub blocks: [Block; CHUNK_TOTAL_BLOCKS],
    pub position_indices: Vector3<u32>,
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

    pub fn blocks(&self) -> BlockIter {
        BlockIter::new(self)
    }

    pub fn position(&self) -> Vector3<f32> {
        self.position_indices.cast::<f32>().unwrap() * CHUNK_SIDE_BLOCKS as f32
    }
}

pub struct BlockIter<'a> {
    chunk: &'a Chunk,
    index: usize,
    position: Vector3<u32>,
    offset: Vector3<f32>,
}

impl<'a> BlockIter<'a> {
    fn new(chunk: &'a Chunk) -> Self {
        BlockIter {
            chunk,
            index: 0,
            position: Zero::zero(),
            offset: chunk
                .position()
                .sub_element_wise((CHUNK_SIDE_BLOCKS - 1) as f32 / 2.0),
        }
    }
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = (Vector3<f32>, Block);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < CHUNK_TOTAL_BLOCKS {
            let block = self.chunk.blocks[self.index];
            let position = self.offset + self.position.cast::<f32>().unwrap();
            self.index += 1;
            self.position.x += 1;
            if self.position.x >= CHUNK_SIDE_BLOCKS as u32 {
                self.position.x = 0;
                self.position.y += 1;
                if self.position.y >= CHUNK_SIDE_BLOCKS as u32 {
                    self.position.y = 0;
                    self.position.z += 1;
                    // Note: because index < CHUNK_TOTAL_BLOCKS, z
                    // should never become >= CHUNK_SIDE_BLOCKS.
                }
            }
            Some((position, block))
        } else {
            None
        }
    }
}
