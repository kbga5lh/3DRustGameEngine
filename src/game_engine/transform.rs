use crate::game_engine::basis::Basis;
use crate::game_engine::vector3::Vector3;

pub struct Transform {
    basis: Basis,
    origin: Vector3,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            basis: Basis::unit_matrix(),
            origin: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn scale(&mut self, scale: &Vector3) {
        self.basis.scale(scale);
        self.origin *= scale;
    }

    pub fn form_matrix(&self) -> [[f32; 4]; 4] {
        let basis_elements = self.basis.get_elements();
        [
            [basis_elements[0].x, basis_elements[0].y, basis_elements[0].z, 0.0],
            [basis_elements[1].x, basis_elements[1].y, basis_elements[1].z, 0.0],
            [basis_elements[2].x, basis_elements[2].y, basis_elements[2].z, 0.0],
            [self.origin.x, self.origin.y, self.origin.z, 1.0],
        ]
    }
}