use glium::Program;

use std::rc::Rc;

pub struct Material {
    pub albedo: [f32; 3],
    pub shader: Rc<Program>,
}