use wavefront_obj::obj;

use std::vec;

use crate::game_engine::vertex_types::VertexPN;
use crate::game_engine::material::Material;
use crate::game_engine::transform::Transform;

pub struct Mesh {
    pub transform: Transform,
    pub vertex_buffer: glium::VertexBuffer<VertexPN>,
    pub index_buffers: Vec<glium::IndexBuffer<u16>>,  
    pub draw_type: glium::index::PrimitiveType,
    pub materials: Vec<Material>,
}

impl Mesh {
    pub fn new(object: &obj::Object, display: &glium::Display) -> Mesh {
        let raw_positions = &object.vertices;
        let raw_normals = &object.normals;

        let mut draw_type = glium::index::PrimitiveType::TrianglesList;

        let mut indices_vec = Vec::new();
        for shapes in &object.geometry {
            let raw_indices = &shapes.shapes;
            let mut raw_indices = raw_indices.iter().map(|i|
                match i.primitive {
                    obj::Primitive::Triangle(v1, v2, v3) => {
                        draw_type = glium::index::PrimitiveType::TrianglesList;
                        vec!(v1, v2, v3)
                    },
                    obj::Primitive::Line(v1, v2) => {
                        draw_type = glium::index::PrimitiveType::LinesList;
                        vec!(v1, v2)
                    },
                    obj::Primitive::Point(v1) => {
                        draw_type = glium::index::PrimitiveType::Points;
                        vec!(v1)
                    }
                }
            ).collect::<Vec<Vec<obj::VTNIndex>>>().concat();
            indices_vec.push(raw_indices);
        }
        
        let result = Mesh::correct_input(&raw_positions, &raw_normals, &indices_vec);

        let vertex_buffer = glium::VertexBuffer::new(display, &result.0).unwrap();
        let mut index_buffers = Vec::new();
        for indices in result.1 {
            index_buffers.push(glium::IndexBuffer::new(display, draw_type, &indices).unwrap());
        }

        Mesh {
            transform: Transform::new(),
            vertex_buffer: vertex_buffer,
            index_buffers: index_buffers,
            draw_type: draw_type,
            materials: Vec::new(),
        }
    }

    // returns vertices and indices
    fn correct_input(raw_positions: &[obj::Vertex], raw_normals: &[obj::Vertex], raw_indices: &Vec<Vec<obj::VTNIndex>>)
        -> (Vec<VertexPN>, Vec<Vec<u16>>) {

        let mut vertices = Vec::<VertexPN>::new();
        let mut indices = Vec::<Vec<u16>>::new();
        
        for ind in raw_indices {
            let mut inner_indices = Vec::<u16>::new(); 
            for i in 0..ind.len() {
                let vertex = raw_positions[ind[i].0];
                let normal = raw_normals[ind[i].2.unwrap()];

                let found_index = Mesh::find_same_vertex(vertex, normal, &vertices);
                match found_index {
                    Some(v) => {
                        inner_indices.push(v as u16);
                    },
                    None => {
                        vertices.push(VertexPN { position: (vertex.x as f32, vertex.y as f32, vertex.z as f32),
                            normal: (normal.x as f32, normal.y as f32, normal.z as f32) });
                        inner_indices.push((vertices.len() - 1) as u16);
                    }
                }
            }
            indices.push(inner_indices);
        }
        (vertices, indices)
    }

    // return index if vertex found
    fn find_same_vertex(vertex: obj::Vertex, normal: obj::Vertex, vertices: &[VertexPN]) -> Option<u16> {
        for i in 0..vertices.len() {
            if vertices[i] == (vertex, normal) {
                return Some(i as u16);
            }
        }
        None
    }
}