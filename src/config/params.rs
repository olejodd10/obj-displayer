pub const LIGHT: [f32; 3] = [-1.0, 0.4, 0.9];

pub const BACKGROUND_COLOR: [f32; 3] = [1.0, 1.0, 1.0];
pub const BACKGROUND_ALPHA: f32 = 1.0;
pub const BACKGROUND_DEPTH: f32 = 1.0; // The value that the depth buffer is reset to, important for backface culling

// Backface culling 
pub const DEPTH_TEST: glium::draw_parameters::DepthTest = glium::draw_parameters::DepthTest::IfLess;

// This is a common blending function https://learnopengl.com/Advanced-OpenGL/Blending
// https://docs.rs/glium/latest/glium/draw_parameters/enum.BlendingFunction.html
// Used for both color and alpha blending
pub const BLENDING_FUNCTION: glium::draw_parameters::BlendingFunction = glium::draw_parameters::BlendingFunction::Addition { 
    source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha, 
    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha, 
};