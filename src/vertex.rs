use obj::IndexTuple;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

// These field identifiers must match the shaders
implement_vertex!(Vertex, position, tex_coords, normal);

pub fn unwrap_indices(IndexTuple(i, j, k): IndexTuple, positions: &[[f32; 3]], _tex_coords: &[[f32; 2]], normals: &[[f32; 3]]) -> Vertex {
    let position = positions[i];
    let tex_coords = j.map(|j| _tex_coords[j]).unwrap_or([0.0, 0.0]);
    let normal = k.map(|k| normals[k]).unwrap_or([0.0, 0.0, 0.0]);
    Vertex { position, tex_coords, normal }
}