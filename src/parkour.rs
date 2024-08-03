use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use valence::{
    entity::{Look, Position},
    math::{DVec3, Vec3},
    BlockPos, ChunkLayer, ChunkPos,
};

use crate::{
    jump::{two_by_two_to_block_pos, Jump},
    port::Port,
};

pub enum PlayerStateUpdate {
    OnCourse,
    Skipped,
    OnPastPlatform,
    TeleportedBack,
    Paused,
}

fn shuffled_options(start: Port, layer: &ChunkLayer) -> Vec<Jump> {
    let mut possible = start.possible_next_jumps(layer);
    possible.shuffle(&mut SmallRng::from_entropy());

    possible
}

fn valid_jump(jump: &Jump, previous: &Vec<Jump>, layer: &mut ChunkLayer) -> bool {
    !jump.too_close(layer)
        && !jump.too_close_to_jumps(
            previous
                .iter()
                .rev()
                .skip(2)
                .take(15)
                .collect::<Vec<&Jump>>(),
            4,
        )
        && !jump.too_close_to_jumps(
            previous
                .iter()
                .rev()
                .skip(1)
                .take(2)
                .collect::<Vec<&Jump>>(),
            2,
        )
        && !jump.too_high()
}

fn build_jump_tree(tree: &mut Vec<Jump>, layer: &mut ChunkLayer, depth: u32) -> bool {
    for new_jump in shuffled_options(tree.last().unwrap().endpoint, layer) {
        if !valid_jump(&new_jump, tree, layer) {
            continue;
        }

        tree.push(new_jump);

        if depth == 0 || build_jump_tree(tree, layer, depth - 1) {
            return true;
        }

        tree.pop();
    }
    false
}

pub struct ParkourCourse {
    jumps: Vec<Jump>,
    generated_end: bool,
}

impl ParkourCourse {
    pub fn new(start: BlockPos, layer: &ChunkLayer) -> Self {
        let mut jumps = Vec::new();
        jumps.push(two_by_two_to_block_pos(start, layer));
        Self {
            jumps,
            generated_end: false,
        }
    }

    pub fn done(&self) -> bool {
        self.generated_end
    }

    pub fn get_start(&self) -> DVec3 {
        self.jumps.first().unwrap().endpoint.center()
    }

    pub fn spawn_platform(&mut self, layer: &mut ChunkLayer) -> bool {
        let foresight = 2;

        // Check whether all chunks surrounding the last jump are loaded
        let last_jump = self.jumps.last().unwrap();
        for x in -1..2 {
            for z in -1..2 {
                let pos = BlockPos::new(
                    last_jump.endpoint.center().x as i32,
                    0,
                    last_jump.endpoint.center().z as i32,
                );
                let mut chunk = ChunkPos::from(pos);
                chunk.x += x;
                chunk.z += z;

                if layer.chunk(pos).is_none() {
                    return false;
                }
            }
        }

        // If we can't create a new jump, we're done
        let jump = build_jump_tree(&mut self.jumps, layer, foresight);
        if !jump {
            // We're done :3
            build_jump_tree(&mut self.jumps, layer, foresight - 1);

            // Switch the last one to finish portal mode
            self.jumps.last_mut().unwrap().set_finish_portal();

            // Set the generated end flag
            self.generated_end = true;

            // Now, we can just build all of these
            for jump in self.jumps.iter().rev() {
                jump.build(layer);
            }

            return false;
        }

        // Pop the last jumps, as they're the result of foresight
        for _ in 0..foresight {
            self.jumps.pop();
        }

        // Spawn the Nth to last jump
        if self.jumps.len() > foresight as usize {
            self.jumps[self.jumps.len() - foresight as usize - 1].build(layer);
        }
        true
    }

    pub fn len(&self) -> i32 {
        self.jumps.len() as i32
    }

    pub fn respawn_course(&mut self, layer: &mut ChunkLayer) {
        for jump in self.jumps.iter().rev().skip(3) {
            jump.build(layer);
        }
    }
}

pub struct PlayerOnCourse {
    last_platform: i32,
    last_valid_position: DVec3,
    last_valid_look: Vec3,
    paused: bool,
}

impl PlayerOnCourse {
    pub fn new(start: BlockPos) -> Self {
        Self {
            last_platform: 0,
            last_valid_position: DVec3::new(
                start.x as f64 + 1.0,
                start.y as f64 + 1.0,
                start.z as f64 + 1.0,
            ),
            last_valid_look: Vec3::new(1.0, 0.0, 0.0),
            paused: false,
        }
    }

    pub fn current_platform(&self) -> i32 {
        self.last_platform
    }

    pub fn platforms_left(&self, course: &ParkourCourse) -> i32 {
        course.len() - self.last_platform
    }

    pub fn update_player_state(
        &mut self,
        course: &ParkourCourse,
        player_pos: &mut Position,
        player_look: &mut Look,
    ) -> PlayerStateUpdate {
        if self.paused {
            return PlayerStateUpdate::Paused;
        }

        // Find the platform the player is on
        for (i, jump) in course.jumps.iter().enumerate() {
            if jump.endpoint.on_platform(player_pos.get()) {
                if i < self.last_platform as usize {
                    return PlayerStateUpdate::OnPastPlatform;
                } else if i == self.last_platform as usize {
                    return PlayerStateUpdate::OnCourse;
                }

                if i > self.last_platform as usize + 8 {
                    self.last_platform = i as i32;
                    self.last_valid_position = player_pos.get();
                    self.last_valid_look = player_look.vec();
                    return PlayerStateUpdate::Skipped;
                }

                self.last_platform = i as i32;
                self.last_valid_position = player_pos.get();
                self.last_valid_look = player_look.vec();
                return PlayerStateUpdate::OnCourse;
            }
        }

        // Detect whether player is in the air
        // Considered on course if any future platform is reachable
        for platform in course.jumps.iter().skip(self.last_platform as usize) {
            if platform.endpoint.is_reachable_from(player_pos.get()) {
                return PlayerStateUpdate::OnCourse;
            }
        }

        // Player is off course
        self.to_last_checkpoint(player_pos, player_look);
        return PlayerStateUpdate::TeleportedBack;
    }

    pub fn to_last_checkpoint(&self, player_pos: &mut Position, player_look: &mut Look) {
        player_pos.set(self.last_valid_position);
        player_look.set_vec(self.last_valid_look);
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }
}
