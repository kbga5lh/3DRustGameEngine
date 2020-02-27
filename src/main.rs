#[macro_use]
extern crate glium;
extern crate wavefront_obj;

use glium::{glutin, Surface};
use wavefront_obj::obj;
use std::fs;
use std::vec;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let object = {
        let source = fs::read_to_string("assets/heart.obj").unwrap();
        obj::parse(source).unwrap()
    };
    if object.objects.len() < 1 {
        panic!("Can't find any object in .obj file");
    }
    let object = &object.objects[0];
    

    let raw_positions: Vec<Vertex> = object.vertices.iter()
        .map(|v| Vertex { position: (v.x as f32, v.y as f32, v.z as f32) })
        .collect();

    let raw_normals: Vec<Normal> = object.normals.iter()
        .map(|n| Normal { normal: (n.x as f32, n.y as f32, n.z as f32) })
        .collect();


    let mut draw_type = glium::index::PrimitiveType::TrianglesList;

    let raw_indices = &object.geometry[0].shapes;
    let raw_indices = raw_indices.iter().map(|i|
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
    }).collect::<Vec<Vec<obj::VTNIndex>>>().concat();

    let result = correct_input(&raw_positions, &raw_normals, &raw_indices);

    let positions = glium::VertexBuffer::new(&display, &result.0).unwrap();
    let normals = glium::VertexBuffer::new(&display, &result.1).unwrap();
    let indices = glium::IndexBuffer::new(&display, draw_type, &result.2).unwrap();

    let vertex_shader_src = fs::read_to_string("assets/vertex_shader.glsl").unwrap();
    let fragment_shader_src = fs::read_to_string("assets/fragment_shader.glsl").unwrap();

    let program = glium::Program::from_source(&display, &vertex_shader_src.to_string(),
        &fragment_shader_src.to_string(), None).unwrap();

    let mut angle: f32 = 0.0;
    let speed: f32 = 0.01;
    let light = [1.4, 0.4, 0.7f32];

    // =======================
    // ====== loop ===========
    // =======================

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // event handling

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        // draw

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let scale = 50.0f32;
        let model = [
            [scale * angle.cos(), 0.0, scale * angle.sin(), 0.0],
            [0.0, scale, 0.0, 0.0],
            [scale * -angle.sin(), 0.0, scale * angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let view = view_matrix(&[0.0, 0.0, -5.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };
        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                    &params).unwrap();
        target.finish().unwrap();

        // update

        angle += speed;
    });
}

fn correct_input(raw_positions: &Vec<Vertex>, raw_normals: &Vec<Normal>, raw_indices: &Vec<obj::VTNIndex>)
    -> (Vec<Vertex>, Vec<Normal>, Vec<u16>) {

    let mut positions = Vec::<Vertex>::new();
    let mut normals = Vec::<Normal>::new();
    let mut indices = Vec::<u16>::new();
    
    for i in 0..raw_indices.len() {
        let vertex = raw_positions[raw_indices[i].0];
        let normal = raw_normals[raw_indices[i].2.unwrap()];

        let found_index = find_same_vertex(vertex, normal, &positions, &normals);
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

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}

impl std::cmp::PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

impl std::cmp::PartialEq for Normal {
    fn eq(&self, other: &Self) -> bool {
        self.normal == other.normal
    }
}