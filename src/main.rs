#[macro_use]
extern crate glium;
extern crate wavefront_obj;

use glium::{glutin, Surface};
use glutin::event;
use wavefront_obj::obj;
use std::fs;
use std::vec;

mod game_engine;

use game_engine::object3d::Object3D;
use game_engine::vertex_types::{Vertex, Normal};
use game_engine::vector::Vector;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut heart = {
        let object = {
            let source = fs::read_to_string("assets/heart.obj").unwrap();
            obj::parse(source).unwrap()
        };
        Object3D::new(&object.objects[0], &display)
    };

    heart.set_scale(Vector::new(50.0, 50.0, 50.0));
    heart.set_position(Vector::new(-1.5, 0.0, 0.0));

    let mut cube = {
        let object = {
            let source = fs::read_to_string("assets/black_bishop.obj").unwrap();
            obj::parse(source).unwrap()
        };
        Object3D::new(&object.objects[0], &display)
    };

    cube.set_scale(Vector::new(1.0, 1.0, 1.0));
    cube.set_position(Vector::new(1.5, -1.0, 0.0));

    let vertex_shader_src = fs::read_to_string("assets/vertex_shader.glsl").unwrap();
    let fragment_shader_src = fs::read_to_string("assets/fragment_shader.glsl").unwrap();

    let program = glium::Program::from_source(&display, &vertex_shader_src.to_string(),
        &fragment_shader_src.to_string(), None).unwrap();

    let mut angle: f32 = 0.0;
    let speed: f32 = 0.5;

    // =======================
    // ====== loop ===========
    // =======================

    event_loop.run(move |event, _, control_flow| {
        let frame_time = std::time::Instant::now();

        // event handling

        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            event::Event::NewEvents(cause) => match cause {
                event::StartCause::ResumeTimeReached { .. } => (),
                event::StartCause::Init => (),
                event::StartCause::Poll => (),
                _ => return,
            },
            _ => return,
        }

        // draw

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let light_position = [0.0, 3.0, -1.0 as f32];

        let camera_position = [-5.0 * angle.sin(), 0.0, -5.0 * angle.cos()];
        let view = view_matrix(&camera_position, &[-camera_position[0], -camera_position[1], -camera_position[2]], &[0.0, 1.0, 0.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar: f32 = 1024.0;
            let znear: f32 = 0.1;

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
        target.draw((&heart.vertex_buffer, &heart.normal_buffer), &heart.index_buffer, &program,
                    &uniform! { model: heart.model_matrix(),
                        view: view, perspective: perspective, u_light: light_position },
                    &params).unwrap();

        target.draw((&cube.vertex_buffer, &cube.normal_buffer), &cube.index_buffer, &program,
                    &uniform! { model: cube.model_matrix(),
                        view: view, perspective: perspective, u_light: light_position },
                    &params).unwrap();

        target.finish().unwrap();

        // update

        angle += speed * frame_time.elapsed().as_secs_f32();
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