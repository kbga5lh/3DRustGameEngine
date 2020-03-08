use crate::game_engine::vector3::Vector3;

pub fn view_matrix(position: Vector3, direction: Vector3, up: Vector3) -> [[f32; 4]; 4] {
    let front = direction.normalized();
    let right = up.cross(front).normalized();
    let up = front.cross(right);

    let position = Vector3::new(
        (-position).dot(right),
        (-position).dot(up),
        (-position).dot(front));

    [
        [right.x, up.x, front.x, 0.0],
        [right.y, up.y, front.y, 0.0],
        [right.z, up.z, front.z, 0.0],
        [position.x, position.y, position.z, 1.0],
    ]
}

pub fn perspective_matrix(frame_size: (u32, u32), fov: f32, zfar: f32, znear: f32) -> [[f32; 4]; 4] {
    let aspect_ratio = frame_size.1 as f32 / frame_size.0 as f32;
    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
        [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
    ]
}