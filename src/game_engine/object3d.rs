extern crate wavefront_obj;
extern crate glium;

use crate::game_engine::vertex_types::{Vertex, Normal};
use wavefront_obj::obj;
use std::vec;
use crate::game_engine::vector::Vector;

pub struct Object3D {
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub normal_buffer: glium::VertexBuffer<Normal>,
    pub index_buffer: glium::IndexBuffer<u16>,
    
    pub draw_type: glium::index::PrimitiveType,

    pub local_position: [[f32; 4]; 4],
    pub scale: [[f32; 4]; 4],
}

impl Object3D {
    pub fn new(model: &obj::Object, display: &glium::Display) -> Object3D {
        let raw_positions: Vec<Vertex> = model.vertices.iter()
        .map(|v| Vertex { position: (v.x as f32, v.y as f32, v.z as f32) })
        .collect();
        
        let raw_normals: Vec<Normal> = model.normals.iter()
        .map(|n| Normal { normal: (n.x as f32, n.y as f32, n.z as f32) })
        .collect();

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
        let normal_buffer = glium::VertexBuffer::new(display, &result.1).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, draw_type, &result.2).unwrap();
        
        Object3D {
            vertex_buffer: vertex_buffer,
            normal_buffer: normal_buffer,
            index_buffer: index_buffer,

            draw_type: draw_type,

            local_position: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]],
            scale: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]],
        }
    }

    pub fn set_scale(&mut self, scale: Vector) {
        self.scale[0][0] = scale.x;
        self.scale[1][1] = scale.y;
        self.scale[2][2] = scale.z;
    }

    pub fn set_position(&mut self, position: Vector) {
        self.local_position[3][0] = position.x;
        self.local_position[3][1] = position.y;
        self.local_position[3][2] = position.z;
    }

    pub fn model_matrix(&self) -> [[f32; 4]; 4] {
        Object3D::dot(&self.scale, &self.local_position)
    }

    fn correct_input(raw_positions: &Vec<Vertex>, raw_normals: &Vec<Normal>, raw_indices: &Vec<obj::VTNIndex>)
        -> (Vec<Vertex>, Vec<Normal>, Vec<u16>) {

        let mut positions = Vec::<Vertex>::new();
        let mut normals = Vec::<Normal>::new();
        let mut indices = Vec::<u16>::new();
        
        for i in 0..raw_indices.len() {
            let vertex = raw_positions[raw_indices[i].0];
            let normal = raw_normals[raw_indices[i].2.unwrap()];

            let found_index = Object3D::find_same_vertex(vertex, normal, &positions, &normals);
            match found_index {
                Some(v) => {
                    indices.push(v as u16);
                },
                None => {
                    positions.push(vertex);
                    normals.push(normal);
                    indices.push((positions.len() - 1) as u16);
                }
            }
        }

        (positions, normals, indices)
    }

    // return index if vertex found
    fn find_same_vertex(vertex: Vertex, normal: Normal, positions: &Vec<Vertex>, normals: &Vec<Normal>) -> Option<u16> {
        for i in 0..positions.len() {
            if vertex == positions[i] && normal == normals[i] {
                return Some(i as u16);
            }
        }
        None
    }

    pub fn dot(first: &[[f32; 4]; 4], second: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let mut result = [[0 as f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += first[i][k] * second[k][j];
                }
            }
        }
        result
    }
}