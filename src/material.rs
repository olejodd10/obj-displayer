use std::path::Path;
use std::io::Cursor;


pub fn classify(mtl: &Option<obj::ObjMaterial>) -> MaterialClass {
    if let Some(obj::ObjMaterial::Mtl(mtl)) = mtl {
        if mtl.map_kd.is_none() {
            MaterialClass::Simple
        } else {
            if mtl.map_ks.is_none() {
                MaterialClass::OnlyDiffuse
            } else {
                MaterialClass::Complete
            }
        }
    } else {
        MaterialClass::None
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum MaterialClass {
    None, // Objects without associated mtl file
    Simple, // Mtl only with simple colors, no maps/textures
    OnlyDiffuse,
    Complete,
}

pub fn load_texture<P: AsRef<Path>>(display: &glium::Display, p: P) -> glium::texture::SrgbTexture2d {
    let image = image::load(Cursor::new(std::fs::read(p.as_ref()).unwrap()),
                            image::ImageFormat::from_path(p).unwrap()).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(display, image).unwrap()
}

// The only point of this struct is so store a subset of obj::Material that has externally stored maps completely loaded
pub struct StoredMaterial {
    pub ka: [f32; 3],
    pub kd: [f32; 3],
    pub ks: [f32; 3],
    pub alpha: f32,
    pub map_ka: Option<glium::texture::SrgbTexture2d>,
    pub map_kd: Option<glium::texture::SrgbTexture2d>,
    pub map_ks: Option<glium::texture::SrgbTexture2d>,
}

impl StoredMaterial {
    pub fn from(mtl: &obj::Material, display: &glium::Display, dir: &Path) -> Self {
        StoredMaterial {
            ka: mtl.ka.unwrap_or([1.0, 1.0, 1.0]),
            kd: mtl.kd.unwrap_or([1.0, 1.0, 1.0]),
            ks: mtl.ks.unwrap_or([1.0, 1.0, 1.0]),
            alpha: mtl.d.or(mtl.tr.map(|tr| 1.0-tr)).unwrap_or(1.0),
            map_ka: mtl.map_ka.as_ref().map(|s| load_texture(display, dir.join(s))),
            map_kd: mtl.map_kd.as_ref().map(|s| load_texture(display, dir.join(s))),
            map_ks: mtl.map_ks.as_ref().map(|s| load_texture(display, dir.join(s))),
        }
    }
}