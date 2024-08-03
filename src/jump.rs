use valence::{BlockPos, BlockState, ChunkLayer};

use crate::{block_chooser::choose_block, port::Port};

#[derive(Clone)]
pub struct JumpBlock {
    pub pos: BlockPos,
    pub state: BlockState,
}

#[derive(Clone)]
pub struct Jump {
    pub endpoint: Port,
    blocks: Vec<JumpBlock>,
}

impl Jump {
    pub fn build(&self, layer: &mut ChunkLayer) {
        for block in &self.blocks {
            layer.set_block(block.pos, block.state);
        }
    }

    pub fn despawn(&self, layer: &mut ChunkLayer) {
        for block in &self.blocks {
            layer.set_block(block.pos, BlockState::AIR);
        }
    }

    pub fn set_finish_portal(&mut self) {
        match &mut self.endpoint {
            Port::TwoByTwo(pos) => {
                self.blocks.clear();
                // Create an end portal frame
                for x in -1..3 {
                    for z in -1..3 {
                        if x == -1 || x == 2 {
                            if z == -1 || z == 2 {
                                continue;
                            }
                        }
                        self.blocks.push(JumpBlock {
                            pos: BlockPos::new(pos.x + x, pos.y, pos.z + z),
                            state: BlockState::END_PORTAL_FRAME,
                        });
                    }
                }
                for x in 0..2 {
                    for z in 0..2 {
                        self.blocks.push(JumpBlock {
                            pos: BlockPos::new(pos.x + x, pos.y, pos.z + z),
                            state: BlockState::END_PORTAL,
                        });
                    }
                }
            }
        }
    }

    pub fn too_close(&self, layer: &ChunkLayer) -> bool {
        // We find the bounding box of the jump,
        // and then make sure no blocks are within M blocks of it

        let radius = 4;

        let mut min = BlockPos::new(255, 255, 255);
        let mut max = BlockPos::new(0, 0, 0);

        for block in &self.blocks {
            let pos = block.pos;
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);
            min.z = min.z.min(pos.z);
            max.x = max.x.max(pos.x);
            max.y = max.y.max(pos.y);
            max.z = max.z.max(pos.z);
        }

        for x in min.x - radius..max.x + radius {
            for y in min.y - radius..max.y + radius {
                for z in min.z - radius..max.z + radius {
                    let pos = BlockPos::new(x, y, z);
                    let block = layer.block(pos);
                    if let Some(block) = block {
                        if block.state != BlockState::AIR {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn too_close_to_jumps(&self, jumps: Vec<&Jump>, radius: i32) -> bool {
        let blocks = jumps
            .iter()
            .map(|jump| {
                jump.blocks
                    .iter()
                    .map(|block| block.pos)
                    .collect::<Vec<BlockPos>>()
            })
            .flatten()
            .collect::<Vec<BlockPos>>();

        let mut min = BlockPos::new(255, 255, 255);
        let mut max = BlockPos::new(0, 0, 0);

        for block in &self.blocks {
            let pos = block.pos;
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);
            min.z = min.z.min(pos.z);
            max.x = max.x.max(pos.x);
            max.y = max.y.max(pos.y);
            max.z = max.z.max(pos.z);
        }

        for pos in blocks {
            if pos.x < min.x - radius || pos.x > max.x + radius {
                continue;
            }
            if pos.y < min.y - radius || pos.y > max.y + radius {
                continue;
            }
            if pos.z < min.z - radius || pos.z > max.z + radius {
                continue;
            }

            return true;
        }
        false
    }

    pub fn too_high(&self) -> bool {
        for block in &self.blocks {
            if block.pos.y > 255 {
                return true;
            }
        }
        false
    }
}

pub fn two_by_two_to_block_pos(pos: BlockPos, layer: &ChunkLayer) -> Jump {
    let biome = layer.biome(pos);
    let state = choose_block(biome);

    let mut blocks = Vec::new();

    for x in 0..2 {
        for z in 0..2 {
            blocks.push(JumpBlock {
                pos: BlockPos::new(pos.x + x, pos.y, pos.z + z),
                state,
            });
        }
    }

    let end = Port::TwoByTwo(pos);
    Jump {
        endpoint: end,
        blocks,
    }
}
