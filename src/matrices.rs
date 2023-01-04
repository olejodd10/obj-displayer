
// Sånn som jeg forstår det så er det ikke noe å tjene på quaternions når vi må ha en matrise uansett.
// Quaternions tillater en raskere måte å rotere enn matriser, men her må vi ha en matrise uansett
// Derfor bruker jeg axis-angle-matrisen her https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle
pub fn model(orientation: [f32; 4], offset: [f32; 3], shrinking: f32) -> [[f32; 4]; 4] {
    let [theta, ux, uy, uz] = orientation;
    let len = (ux*ux+uy*uy+uz*uz).sqrt();
    let (ux,uy,uz) = (ux/len, uy/len, uz/len);
    let (sin, cos) = (theta.sin(), theta.cos());
    let k = 1.0-cos;
    [
        [cos+ux*ux*k,    uy*ux*k+uz*sin, uz*ux*k-uy*sin, 0.0],
        [ux*uy*k-uz*sin, cos+uy*uy*k,    uz*uy*k+ux*sin, 0.0],
        [ux*uz*k+uy*sin, uy*uz*k-ux*sin, cos+uz*uz*k,    0.0],
        [shrinking*offset[0], shrinking*offset[1], shrinking*offset[2], shrinking],
    ]
} 

pub fn view(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

pub fn perspective(width: u32, height: u32) -> [[f32; 4]; 4] {
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}