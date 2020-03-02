use crate::game_engine::vector3::Vector3;

pub struct Basis {
    elements: [Vector3; 3],
}

impl Basis {
    pub fn unit_matrix() -> Basis {
        Basis {
            elements: [
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            ],
        }
    }

    pub fn scale(&mut self, scale: &Vector3) {
        self.elements[0] *= scale.x;
        self.elements[1] *= scale.x;
        self.elements[2] *= scale.x;
    }

    pub fn get_elements(&self) -> &[Vector3; 3] {
        &self.elements
    }
}