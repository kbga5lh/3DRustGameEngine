use crate::vertex_types::{Vertex, Normal};

pub struct Object3D {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    normal_buffer: glium::VertexBuffer<Normal>,
    index_buffer: glium::IndexBuffer<u16>,
}