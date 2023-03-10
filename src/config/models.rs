pub const NUM_MODELS: usize = 3;

pub const OBJ_PATHS: [&str; NUM_MODELS] = [
    "models/low_poly_tree/low_poly_tree/Lowpoly_tree_sample.obj",
    "models/Santa_Claus_v1_L2.123cb4fe07b2-55dd-461b-9f27-42ccf1b3f3f5/12165_Santa_Claus_v1_l2.obj",
    "models/container/Container.obj"
];

pub const ORIENTATIONS: [[f32; 4]; NUM_MODELS] = [
    [0.0, 1.0, 0.0, 0.0],
    [3.14, 0.0, 1.0, 1.0],
    [3.14/4.0, 0.0, 1.0, 0.0],
];

pub const OFFSETS: [[f32; 3]; NUM_MODELS] = [
    [5.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],
    [-10.0, 0.0, 0.0],
];

pub const SHRINKS: [f32; NUM_MODELS] = [
    1.0,
    20.0,
    30.0,
];
