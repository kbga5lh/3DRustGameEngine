#[macro_use]
extern crate glium;
extern crate wavefront_obj;

use std::vec;

fn main() {
    #![allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let o = wavefront_obj::obj::parse(std::fs::read_to_string("assets/cube.obj").unwrap()).unwrap();
    let o = &o.objects[0];
    
    let positions = {
        let mut result = vec::Vec::new();
        for v in &o.vertices {
            result.push(Vertex { position: (v.x as f32, v.y as f32, v.z as f32) });
        }
        result
    };

    let normals = {
        let mut result = vec::Vec::new();
        for v in &o.normals {
            result.push(Normal { normal: (v.x as f32, v.y as f32, v.z as f32) });
        }
        result
    };

    println!("{:?}", positions.len());
    println!("{:?}", normals.len());

    let mut t: glium::index::PrimitiveType = glium::index::PrimitiveType::LinesList;

    let i = &o.geometry[0].shapes;
    let indices = {
        let mut result = vec::Vec::new();
        for v in i {
            match v.primitive {
                wavefront_obj::obj::Primitive::Triangle(x, y, z) => {
                    result.push(x.0 as u16);
                    result.push(y.0 as u16);
                    result.push(z.0 as u16);
                    t = glium::index::PrimitiveType::TrianglesList;
                },
                wavefront_obj::obj::Primitive::Line(x, y) => {
                    result.push(x.0 as u16);
                    result.push(y.0 as u16);
                    t = glium::index::PrimitiveType::LinesList;
                },
                wavefront_obj::obj::Primitive::Point(x) => {
                    result.push(x.0 as u16);
                    t = glium::index::PrimitiveType::Points;
                }
            }
        }
        result
    };

    println!("{:?}", indices.len());

    let positions = glium::VertexBuffer::new(&display, &positions).unwrap();
    let normals = glium::VertexBuffer::new(&display, &normals).unwrap();
    let indices = glium::IndexBuffer::new(&display, t, &indices).unwrap();

    let vertex_shader_src = std::fs::read_to_string("assets/vertex_shader.glsl").unwrap();
    let fragment_shader_src = std::fs::read_to_string("assets/fragment_shader.glsl").unwrap();

    let program = glium::Program::from_source(&display, &vertex_shader_src.to_string(), &fragment_shader_src.to_string(),
                                              None).unwrap();

    let mut angle: f32 = 0.0;
    let speed: f32 = 0.01;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

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

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let scale = 1.0f32;
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

        let light = [1.4, 0.4, 0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };
        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                    &params).unwrap();
        target.finish().unwrap();

        angle += speed;
    });
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

// fn correct_input(in_vertices: vec::Vec<Vertex>, in_normals: vec::Vec<Normal>) -> (vec::Vec<Vertex>, vec::Vec<Normal>, vec::Vec<u16>) {
//     let mut out_vertices = vec::Vec::new();
//     let mut out_normals = vec::Vec::new();
//     let mut out_indices = vec::Vec::new();

//     for i in 0..in_vertices.len() {
//         let mut found = false;
//         let mut result: u16 = 0;
//         for j in 0..out_vertices.len() {
//             if out_vertices[j] == in_vertices[i] {
//                 result = j as u16;
//                 found = true;
//                 break;
//             }
//         }

//         if found {
//             out_indices.push(result);
//         }
//         else {
//             out_vertices.push(value: T)

//             out_indices.push(out_vertices.len() - 1);
//         }
//     }
//     (out_vertices, out_normals, out_indices)
// }

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