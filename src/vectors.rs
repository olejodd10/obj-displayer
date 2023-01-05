type Vector = [f32; 3];

pub fn negate(v: Vector) -> Vector {
    let [x,y,z] = v;
    [-x,-y,-z]
}

pub fn scale(v: Vector, c: f32) -> Vector {
    let [x,y,z] = v;
    [c*x,c*y,c*z]
}

pub fn add(v1: Vector, v2: Vector) -> Vector {
    let [x1,y1,z1] = v1;
    let [x2,y2,z2] = v2;
    [x1+x2,y1+y2,z1+z2]
}

pub fn dot_product(v1: Vector, v2: Vector) -> f32 {
    let [x1,y1,z1] = v1;
    let [x2,y2,z2] = v2;
    x1*x2 + y1*y2 + z1*z2
}

pub fn cross_product(v1: Vector, v2: Vector) -> Vector {
    let [x1,y1,z1] = v1;
    let [x2,y2,z2] = v2;
    [y1*z2-z1*y2, x1*z2-z1*x2, x1*y2-y1*x2]
}

pub fn normalize(v: Vector) -> Vector {
    let len = dot_product(v,v).sqrt();
    scale(v, 1.0/len)
}

pub fn angle(v1: Vector, v2: Vector) -> f32 {
    dot_product(normalize(v1), normalize(v2)).acos()
}

pub fn increment(v: Vector, dir: Vector, units: f32) -> Vector {
    let dir = normalize(dir);
    add(v,scale(dir, units))
}

pub fn rotate(v: Vector, axis: Vector, angle: f32) -> Vector {
    let [x,y,z] = v;
    let [u,v,w] = normalize(axis);
    let (sin, cos) = (angle.sin(), angle.cos());
    let k = 1.0-cos;
    [x*(cos+u*u*k) + y*(u*v*k-w*sin) + z*(u*w*k+v*sin),
     x*(v*u*k+w*sin) + y*(cos+v*v*k) + z*(v*w*k-u*sin),
     x*(w*u*k-v*sin) + y*(w*v*k+u*sin) + z*(cos+w*w*k)]
}