use crate::game_engine::vector3::Vector3;

pub fn view_matrix(position: Vector3, direction: Vector3, up: Vector3) -> [[f32; 4]; 4] {
    let pos_norm = direction.normalized();

    let s = up.cross(pos_norm);
    let s_norm = s.normalized();

    let u = pos_norm.cross(s_norm);

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

pub fn perspective_matrix(frame_size: (u32, u32), fov: f32, zfar: f32, znear: f32) -> [[f32; 4]; 4] {
    let aspect_ratio = frame_size.1 as f32 / frame_size.0 as f32;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}