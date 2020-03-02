pub struct Basis {
    elements: [[f32; 4]; 4],
}

impl Basis {
    pub fn unit_matrix() -> Basis {
        Basis {
            elements: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn get_elements(&self) -> &[[f32; 4]; 4] {
        &self.elements
    }
}