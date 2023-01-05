#[macro_use]
extern crate glium;

extern crate image;

mod matrices;
mod vectors;
mod material;
mod shaders;
mod vertex;
mod config; 

use std::path::PathBuf;
use std::collections::HashMap;
use std::rc::Rc;

// Heavily inspired by the glium tutorial
// Switch to storing Rc<StoredMaterial> if materials are reused (unlikely)

fn main() {
    use glium::{glutin::{self, event::VirtualKeyCode}, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut programs: HashMap<material::MaterialClass, Rc<glium::Program>> = Default::default(); // Programs will be the same between models, so no need for array here
    let mut models: [[[f32; 4]; 4]; config::NUM_MODELS] = Default::default();
    let mut n_objects: [usize; config::NUM_MODELS] = Default::default();
    let mut n_groups: [Vec<usize>; config::NUM_MODELS] = Default::default();

    let mut model_vertices: [Vec<Vec<glium::VertexBuffer<_>>>; config::NUM_MODELS] = Default::default();
    let mut model_materials: [Vec<Vec<(material::MaterialClass, Option<material::StoredMaterial>)>>; config::NUM_MODELS] = Default::default(); 
    let mut model_programs: [Vec<Vec<Rc<glium::Program>>>; config::NUM_MODELS] = Default::default();

    let mut camera_pos = config::CAMERA_POS_INIT;
    let mut camera_dir = config::CAMERA_DIR_INIT;
    for i in 0..config::NUM_MODELS {
        // Store model matrix
        // *In a dynamic world these model matrices would be continuously calculated based on time step and translational and rotational velocities
        models[i] = matrices::model(config::ORIENTATIONS[i], config::OFFSETS[i], config::SHRINKS[i]);

        let obj_path = PathBuf::from(config::OBJ_PATHS[i]);
        let mut obj = obj::Obj::load(obj_path).expect("Error parsing file");
        if let Err(err) = obj.load_mtls() {
            println!("WARNING: Error while loading mtls: {:?}", err);
        }
        
        let positions = obj.data.position;
        let tex_coords = obj.data.texture;
        let normals = obj.data.normal;
        
        n_objects[i] = obj.data.objects.len();
        for object in obj.data.objects {
            
            let mut object_vertices = Vec::new();
            let mut object_materials = Vec::new();
            let mut object_programs = Vec::new();

            n_groups[i].push(object.groups.len());
            for group in object.groups {    
                // Store vertices
                let mut _group_vertices = Vec::new();
                for polygon in group.polys {
                    let polygon = polygon.0;
                    if polygon.len() == 3 {
                        _group_vertices.push(vertex::unwrap_indices(polygon[0], &positions, &tex_coords, &normals));
                        _group_vertices.push(vertex::unwrap_indices(polygon[1], &positions, &tex_coords, &normals));
                        _group_vertices.push(vertex::unwrap_indices(polygon[2], &positions, &tex_coords, &normals));
                    } else if polygon.len() == 4 {
                        _group_vertices.push(vertex::unwrap_indices(polygon[0], &positions, &tex_coords, &normals));
                        _group_vertices.push(vertex::unwrap_indices(polygon[1], &positions, &tex_coords, &normals));
                        _group_vertices.push(vertex::unwrap_indices(polygon[2], &positions, &tex_coords, &normals));
                        _group_vertices.push(vertex::unwrap_indices(polygon[2], &positions, &tex_coords, &normals));
                        _group_vertices.push(vertex::unwrap_indices(polygon[3], &positions, &tex_coords, &normals));
                        _group_vertices.push(vertex::unwrap_indices(polygon[0], &positions, &tex_coords, &normals));
                    } else {
                        panic!("Unknown polygon length");
                    }
                }
                object_vertices.push(glium::VertexBuffer::new(&display, &_group_vertices).unwrap());
                
                // Generate if needed, then store suitable program
                let material_class = material::classify(&group.material);
                if programs.get(&material_class).is_none() {
                    let fragment_shader = match material_class {
                        material::MaterialClass::None => shaders::DEFAULT_FRAGMENT_SHADER_SRC,
                        material::MaterialClass::Simple => shaders::SIMPLE_FRAGMENT_SHADER_SRC,
                        material::MaterialClass::OnlyDiffuse => shaders::ONLY_DIFFUSE_FRAGMENT_SHADER_SRC,
                        material::MaterialClass::Complete => shaders::COMPLETE_FRAGMENT_SHADER_SRC,
                    };
                    programs.insert(material_class, Rc::new(glium::Program::from_source(&display, shaders::VERTEX_SHADER_SRC, fragment_shader, None).unwrap()));
                }
                object_programs.push(programs.get(&material_class).unwrap().clone());

                // Store material
                let stored_material = group.material.and_then(|obj_mtl| if let obj::ObjMaterial::Mtl(mtl) = obj_mtl { 
                    Some(material::StoredMaterial::from(&mtl, &display, &obj.path))
                } else { 
                    None 
                });
                object_materials.push((material_class, stored_material));

            }
            model_vertices[i].push(object_vertices);
            model_materials[i].push(object_materials);
            model_programs[i].push(object_programs);
        }   
    }
            
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: config::DEPTH_TEST,
            write: true,
            .. Default::default()
        },
        blend: glium::draw_parameters::Blend {
            color: config::BLENDING_FUNCTION,
            alpha: config::BLENDING_FUNCTION,
            .. Default::default()
        },
        .. Default::default()
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(keycode) => match keycode {
                        VirtualKeyCode::W => camera_pos = vectors::increment(camera_pos, camera_dir, config::MOVEMENT_SPEED),
                        VirtualKeyCode::A => camera_pos = vectors::increment(camera_pos, vectors::negate(vectors::cross_product(config::CAMERA_UP, camera_dir)), config::MOVEMENT_SPEED),
                        VirtualKeyCode::S => camera_pos = vectors::increment(camera_pos, vectors::negate(camera_dir), config::MOVEMENT_SPEED),
                        VirtualKeyCode::D => camera_pos = vectors::increment(camera_pos, vectors::cross_product(config::CAMERA_UP, camera_dir), config::MOVEMENT_SPEED),
                        VirtualKeyCode::Space => camera_pos = vectors::increment(camera_pos, config::CAMERA_UP, config::MOVEMENT_SPEED),
                        VirtualKeyCode::LShift => camera_pos = vectors::increment(camera_pos, vectors::negate(config::CAMERA_UP), config::MOVEMENT_SPEED),
                        VirtualKeyCode::Left => camera_dir = vectors::rotate(camera_dir, config::CAMERA_UP, -config::PANNING_SPEED),
                        VirtualKeyCode::Right => camera_dir = vectors::rotate(camera_dir, config::CAMERA_UP, config::PANNING_SPEED),
                        VirtualKeyCode::Up if vectors::angle(camera_dir, config::CAMERA_UP) > config::ANGLE_LIMIT => {
                            let left = vectors::negate(vectors::cross_product(config::CAMERA_UP, camera_dir));
                            camera_dir = vectors::rotate(camera_dir, left, config::PANNING_SPEED);
                        },
                        VirtualKeyCode::Down if vectors::angle(camera_dir, vectors::negate(config::CAMERA_UP)) > config::ANGLE_LIMIT => {
                            let right = vectors::cross_product(config::CAMERA_UP, camera_dir);
                            camera_dir = vectors::rotate(camera_dir, right, config::PANNING_SPEED);
                        },
                        _ => return,
                    },
                    _ => return,
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
    
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color_and_depth((config::BACKGROUND_COLOR[0], config::BACKGROUND_COLOR[1], config::BACKGROUND_COLOR[2], config::BACKGROUND_ALPHA), config::BACKGROUND_DEPTH); 

        let (width, height) = target.get_dimensions();

        let view = matrices::view(&camera_pos, &camera_dir, &config::CAMERA_UP);
        let perspective = matrices::perspective(width, height);

        for i in 0..config::NUM_MODELS { 
            for j in 0..n_objects[i] { 
                for k in 0..n_groups[i][j] {
                    //Draws every group in every object in every model
                    let material_class = model_materials[i][j][k].0;
                    let mtl = model_materials[i][j][k].1.as_ref();
                    match material_class {
                        material::MaterialClass::None => {
                            target.draw(&model_vertices[i][j][k], glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), model_programs[i][j][k].as_ref(),
                            &uniform! { model: models[i], view: view, perspective: perspective, u_light: config::LIGHT },
                            &params).unwrap();
                        },
                        material::MaterialClass::Simple => {
                            let mtl = mtl.unwrap();
                            target.draw(&model_vertices[i][j][k], glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), model_programs[i][j][k].as_ref(),
                            &uniform! { model: models[i], view: view, perspective: perspective, u_light: config::LIGHT, ambient_color: mtl.ka, diffuse_color: mtl.kd, specular_color: mtl.ks, alpha: mtl.alpha },
                            &params).unwrap();
                        },
                        material::MaterialClass::OnlyDiffuse => {
                            let mtl = mtl.unwrap();
                            target.draw(&model_vertices[i][j][k], glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), model_programs[i][j][k].as_ref(),
                            &uniform! { model: models[i], view: view, perspective: perspective, u_light: config::LIGHT, specular_color: mtl.ks, alpha: mtl.alpha, ambient_tex: mtl.map_ka.as_ref().unwrap(), diffuse_tex: mtl.map_kd.as_ref().unwrap() },
                            &params).unwrap();
                        },
                        material::MaterialClass::Complete => {
                            let mtl = mtl.unwrap();
                            target.draw(&model_vertices[i][j][k], glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), model_programs[i][j][k].as_ref(),
                            &uniform! { model: models[i], view: view, perspective: perspective, u_light: config::LIGHT, alpha: mtl.alpha, ambient_tex: mtl.map_ka.as_ref().unwrap(), diffuse_tex: mtl.map_kd.as_ref().unwrap(), specular_tex: mtl.map_ks.as_ref().unwrap() },
                            &params).unwrap();
                        },
                    }
                }
            }
        }
        target.finish().unwrap();
    });
} 