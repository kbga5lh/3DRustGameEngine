use wavefront_obj::obj;

use std::rc::Rc;

use crate::game_engine::vertex_types::VertexPN;
use crate::game_engine::transform::Transform;
use crate::game_engine::mesh::Mesh;
use crate::game_engine::material::Material;

pub struct Object3D {
    pub mesh: Mesh,
    pub transform: Transform,
}

impl Object3D {
    pub fn new(mesh: Mesh) -> Object3D {        
        Object3D {
            mesh: mesh,
            transform: Transform::new(),
        }
    }
}