pub const CAMERA_POS_INIT: [f32; 3] = [0.0, 10.0, -40.0];
pub const CAMERA_DIR_INIT: [f32; 3] = [0.0, 0.0, 10.0];
pub const CAMERA_UP: [f32; 3] = [0.0, 1.0, 0.0];

pub const ANGLE_LIMIT: f32 = 0.05; // The smallest legal angle between camera dir and camera up

pub const MOVEMENT_SPEED: f32 = 0.4; // Meters per step
pub const PANNING_SPEED: f32 = 0.03; // Radians per step