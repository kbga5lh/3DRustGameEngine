use glium::Program;

use std::rc::Rc;

use crate::game_engine::color::Color;

#[derive(Clone)]
pub struct Material {
    pub albedo: Color,
    pub shader: Rc<Program>,
}