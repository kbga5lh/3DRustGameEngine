use crate::game_engine::vector3::Vector3;

pub struct Basis {
    elements: [Vector3; 3],
}

impl Basis {
    pub fn new() -> Basis {
        Basis {
            elements: [
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            ],
        }
    }

    pub fn scale(&mut self, scale: Vector3) {
        self.elements[0] *= scale.x;
        self.elements[1] *= scale.y;
        self.elements[2] *= scale.z;
    }

    pub fn get_scale(&self) -> Vector3 {
        Vector3::new(
            Vector3::new(self.elements[0].x, self.elements[1].x, self.elements[2].x).length(),
            Vector3::new(self.elements[0].y, self.elements[1].y, self.elements[2].y).length(),
            Vector3::new(self.elements[0].z, self.elements[1].z, self.elements[2].z).length()
        )
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        
    }

    pub fn get_elements(&self) -> &[Vector3; 3] {
        &self.elements
    }
}