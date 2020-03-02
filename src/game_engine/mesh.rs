use crate::game_engine::vertex_types::VertexPN;
use crate::game_engine::material::Material;

pub struct Mesh {
    pub vertex_buffer: glium::VertexBuffer<VertexPN>,
    pub index_buffer: glium::IndexBuffer<u16>,  
    pub draw_type: glium::index::PrimitiveType,
    pub material: Material,
}