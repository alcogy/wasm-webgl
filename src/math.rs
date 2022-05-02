pub fn look_at(eye: [f32; 3], target: [f32; 3]) -> [f32; 16] {
    let up: [f32; 3] = [0.0, 1.0, 0.0];
    
    let n = normalize(subtraction(eye, target));
    let u = normalize(cross_product(up, n));
    let v = normalize(cross_product(n, u));

    let tx = -1.0 * dot_product(u, eye);
    let ty = -1.0 * dot_product(v, eye);
    let tz = -1.0 * dot_product(n, eye);

    [ u[0], v[0], n[0], 0.0,
      u[1], v[1], n[1], 0.0,
      u[2], v[2], n[2], 0.0,
        tx,   ty,   tz, 1.0 ]
}

pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let f = 1.0 / (fov / 2.0).tan();
    let inv = 1.0 / (near - far);

    [ f / aspect, 0.0, 0.0, 0.0,
      0.0, f, 0.0, 0.0,
      0.0, 0.0, (near + far) * inv, -1.0,
      0.0, 0.0, near * far * inv * 2.0, 0.0, ]    
}

pub fn addition(vec1: [f32; 3], vec2: [f32; 3]) -> [f32; 3] {
    [(vec1[0] + vec2[0]), (vec1[1] + vec2[1]), (vec1[2] + vec2[2])]
}

pub fn subtraction(vec1: [f32; 3], vec2: [f32; 3]) -> [f32; 3] {
    [(vec1[0] - vec2[0]), (vec1[1] - vec2[1]), (vec1[2] - vec2[2])]
}

pub fn dot_product(vec1: [f32; 3], vec2: [f32; 3]) -> f32 {
    (vec1[0] * vec2[0]) + (vec1[1] * vec2[1]) + (vec1[2] * vec2[2])
}

pub fn cross_product(vec1: [f32; 3], vec2: [f32; 3]) -> [f32; 3] {
    [(vec1[1] * vec2[2]) - (vec1[2] * vec2[1]),
     (vec1[2] * vec2[0]) - (vec1[0] * vec2[2]),
     (vec1[0] * vec2[1]) - (vec1[1] * vec2[0])]
}

pub fn normalize(vec: [f32; 3]) -> [f32; 3] {
    let length = ((vec[0] * vec[0]) + (vec[1] * vec[1]) + (vec[2] * vec[2])).sqrt() as f32;
    [vec[0] / length, vec[1] / length, vec[2] / length]
}

pub fn invert(vec: [f32; 3]) -> [f32; 3] {
    [(vec[0] * -1.0), (vec[1] * -1.0), (vec[2] * -1.0)]
}

pub fn identity() -> [f32; 16] {
    [ 1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0, ]
}

pub fn rotate_x(angle: f32) -> [f32; 16] {
    [ 1.0, 0.0, 0.0, 0.0, 
      0.0,  angle.cos(), angle.sin(), 0.0, 
      0.0, -angle.sin(), angle.cos(), 0.0, 
      0.0, 0.0, 0.0, 1.0, ]
}

pub fn rotate_y(angle: f32) -> [f32; 16] {
    [ angle.cos(), 0.0, -angle.sin(), 0.0, 
      0.0, 1.0, 0.0, 0.0, 
      angle.sin(), 0.0, angle.cos(), 0.0, 
      0.0, 0.0, 0.0, 1.0, ]
}

pub fn rotate_z(angle: f32) -> [f32; 16] {
    [ angle.cos(), angle.sin(), 0.0, 0.0, 
     -angle.sin(), angle.cos(), 0.0, 0.0, 
      0.0, 0.0, 1.0, 0.0, 
      0.0, 0.0, 0.0, 1.0, ]
}