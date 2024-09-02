use valence::{math::DVec3, BlockPos, ChunkLayer};

use crate::jump::{two_by_two_to_block_pos, Jump};

#[derive(Debug, Clone, Copy)]
pub enum Port {
    TwoByTwo(BlockPos),
    TwoByTwoPortal(BlockPos),
}

impl Port {
    pub fn on_platform(&self, pos: DVec3) -> bool {
        // Check if the position is on the platform
        match self {
            Port::TwoByTwo(platform) => {
                let x = platform.x as f64;
                let y = platform.y as f64;
                let z = platform.z as f64;

                if pos.x < x - 0.3 || pos.x > x + 2.3 {
                    return false;
                }

                if pos.y < y + 1.0 || pos.y >= y + 1.05 {
                    return false;
                }

                if pos.z < z - 0.3 || pos.z > z + 2.3 {
                    return false;
                }

                true
            }
            Port::TwoByTwoPortal(platform) => {
                let x = platform.x as f64;
                let y = platform.y as f64;
                let z = platform.z as f64;

                if pos.x < x - 0.3 || pos.x > x + 2.3 {
                    return false;
                }

                if pos.y < y || pos.y >= y + 0.5 {
                    return false;
                }

                if pos.z < z - 0.3 || pos.z > z + 2.3 {
                    return false;
                }

                true
            }
        }
    }

    pub fn is_reachable_from(&self, pos: DVec3) -> bool {
        // Check if the position is on the platform
        match self {
            Port::TwoByTwo(platform) => {
                let x = platform.x as f64;
                let y = platform.y as f64;
                let z = platform.z as f64;

                let distance_x = (pos.x - x - 1.0).abs();
                let distance_z = (pos.z - z - 1.0).abs();
                let height_above = pos.y - y + 1.0;

                return distance_x + distance_z <= height_above * 5.0;
            }
            Port::TwoByTwoPortal(platform) => {
                let x = platform.x as f64;
                let y = platform.y as f64;
                let z = platform.z as f64;

                let distance_x = (pos.x - x - 1.0).abs();
                let distance_z = (pos.z - z - 1.0).abs();
                let height_above = pos.y - y + 1.0;

                return distance_x + distance_z <= height_above * 5.0;
            }
        }
    }

    pub fn center(&self) -> DVec3 {
        match self {
            Port::TwoByTwo(platform) => {
                let x = platform.x as f64;
                let y = platform.y as f64;
                let z = platform.z as f64;

                DVec3::new(x + 1.0, y + 1.0, z + 1.0)
            }
            Port::TwoByTwoPortal(platform) => {
                let x = platform.x as f64;
                let y = platform.y as f64;
                let z = platform.z as f64;

                DVec3::new(x + 1.0, y + 1.0, z + 1.0)
            }
        }
    }

    pub fn possible_next_jumps(&self, layer: &ChunkLayer) -> Vec<Jump> {
        match self {
            Port::TwoByTwo(platform) => {
                vec![
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x + 5, platform.y, platform.z),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x, platform.y, platform.z + 5),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x - 5, platform.y, platform.z),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x, platform.y, platform.z - 5),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x + 4, platform.y + 1, platform.z),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x + 4, platform.y - 1, platform.z),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x - 4, platform.y + 1, platform.z),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x - 4, platform.y - 1, platform.z),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x, platform.y + 1, platform.z + 4),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x, platform.y - 1, platform.z + 4),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x, platform.y + 1, platform.z - 4),
                        layer,
                    ),
                    two_by_two_to_block_pos(
                        BlockPos::new(platform.x, platform.y - 1, platform.z - 4),
                        layer,
                    ),
                ]
            }
            Port::TwoByTwoPortal(_) => Vec::new(),
        }
    }
}
