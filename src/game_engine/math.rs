use crate::game_engine::vector3::Vector3;

pub fn dot(first: &[[f32; 4]; 4], second: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0 as f32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += first[i][k] * second[k][j];
            }
        }
    }
    result
}

pub fn view_matrix(position: Vector3, direction: Vector3, up: Vector3) -> [[f32; 4]; 4] {
    let pos_norm = direction.normalized();

    let s = Vector3::new(up.y * pos_norm.z - up.z * pos_norm.y,
             up.z * pos_norm.x - up.x * pos_norm.z,
             up.x * pos_norm.y - up.y * pos_norm.x);

    let s_norm = s.normalized();

    let u = Vector3::new(pos_norm.y * s_norm.z - pos_norm.z * s_norm.y,
             pos_norm.z * s_norm.x - pos_norm.x * s_norm.z,
             pos_norm.x * s_norm.y - pos_norm.y * s_norm.x);

    let p = Vector3::new(
        (-position).dot(s_norm),
        (-position).dot(u),
        (-position).dot(pos_norm));

    [
        [s_norm.x, u.x, pos_norm.x, 0.0],
        [s_norm.y, u.y, pos_norm.y, 0.0],
        [s_norm.z, u.z, pos_norm.z, 0.0],
        [p.x, p.y, p.z, 1.0],
    ]
}

pub fn perspective(width: u32, height: u32, fov: f32, zfar: f32, znear: f32) -> [[f32; 4]; 4] {
    let aspect_ratio = height as f32 / width as f32;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}