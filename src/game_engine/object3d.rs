extern crate wavefront_obj;
extern crate glium;

use wavefront_obj::obj;
use crate::game_engine::vertex_types::VertexPN;
use crate::game_engine::transform::Transform;

pub struct Object3D {
    pub vertex_buffer: glium::VertexBuffer<VertexPN>,
    pub index_buffer: glium::IndexBuffer<u16>,
    
    pub draw_type: glium::index::PrimitiveType,

    pub transform: Transform,

    pub color: [f32; 3],
}

impl Object3D {
    pub fn new(model: &obj::Object, display: &glium::Display) -> Object3D {
        let raw_positions = &model.vertices;
        let raw_normals = &model.normals;

        let mut draw_type = glium::index::PrimitiveType::TrianglesList;

        let mut indices = Vec::new();
        for shapes in &model.geometry {
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
            indices.append(&mut raw_indices);
        }
        
        let result = Object3D::correct_input(&raw_positions, &raw_normals, &indices);

        let vertex_buffer = glium::VertexBuffer::new(display, &result.0).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, draw_type, &result.1).unwrap();
        
        Object3D {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,

            draw_type: draw_type,

            transform: Transform::new(),

            color: [1.0, 1.0, 1.0],
        }
    }

    fn correct_input(raw_positions: &Vec<obj::Vertex>, raw_normals: &Vec<obj::Vertex>, raw_indices: &Vec<obj::VTNIndex>)
        -> (Vec<VertexPN>, Vec<u16>) {

        let mut vertices = Vec::<VertexPN>::new();
        let mut indices = Vec::<u16>::new();
        
        for i in 0..raw_indices.len() {
            let vertex = raw_positions[raw_indices[i].0];
            let normal = raw_normals[raw_indices[i].2.unwrap()];

            let found_index = Object3D::find_same_vertex(&vertex, &normal, &vertices);
            match found_index {
                Some(v) => {
                    indices.push(v as u16);
                },
                None => {
                    vertices.push(VertexPN { position: (vertex.x as f32, vertex.y as f32, vertex.z as f32),
                        normal: (normal.x as f32, normal.y as f32, normal.z as f32) });
                    indices.push((vertices.len() - 1) as u16);
                }
            }
        }
        (vertices, indices)
    }

    // return index if vertex found
    fn find_same_vertex(vertex: &obj::Vertex, normal: &obj::Vertex, vertices: &Vec<VertexPN>) -> Option<u16> {
        for i in 0..vertices.len() {
            if vertices[i] == (vertex, normal) {
                return Some(i as u16);
            }
        }
        None
    }
}