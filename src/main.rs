/*
render block world
flying camera
paint blocks
 */

/*
x, y, z
 */

mod block;
mod chunk;

use block::Block;
use chunk::Chunk;
use chunk::CHUNK_TOTAL_BLOCKS;

fn main() {
    let mut chunk = Chunk {
        blocks: [Block::Void; CHUNK_TOTAL_BLOCKS],
    };

    for y in 0..32 {
        for x in 0..32 {
            *chunk.block_at_mut(x, y, 0) = Block::Rock;
        }
    }

    *chunk.block_at_mut(5, 10, 1) = Block::Rock;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
