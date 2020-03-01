#[macro_use]
extern crate glium;
extern crate wavefront_obj;

use glium::{glutin, Surface};
use glutin::event;
use wavefront_obj::obj;
use std::fs;

mod game_engine;

use game_engine::object3d::Object3D;
use game_engine::vector::Vector;
use game_engine::math;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = fs::read_to_string("assets/shaders/vertex_shader.glsl").unwrap();
    let fragment_shader_src = fs::read_to_string("assets/shaders/fragment_shader.glsl").unwrap();

    let program = glium::Program::from_source(&display, &vertex_shader_src.to_string(),
        &fragment_shader_src.to_string(), None).unwrap();

    // objects

    let mut board = {
        let object = {
            let source = fs::read_to_string("assets/models/board.obj").unwrap();
            obj::parse(source).unwrap()
        };
        Object3D::new(&object.objects[0], &display)
    };
    board.color = [0.0, 1.0, 0.0];

    // variables

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
        let camera_position = [-15.0 * angle.sin(), 5.0, -15.0 * angle.cos()];
        let view = math::view_matrix(&camera_position, &[-camera_position[0], -camera_position[1], -camera_position[2]], &[0.0, 1.0, 0.0]);
        let perspective = math::perspective(target.get_dimensions().0, target.get_dimensions().1);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw((&board.vertex_buffer, &board.normal_buffer), &board.index_buffer, &program,
                    &uniform! { model: board.model_matrix(),
                        view: view, perspective: perspective, u_light: light_position,
                        u_color: board.color },
                    &params).unwrap();

        target.finish().unwrap();

        // update

        angle += speed * frame_time.elapsed().as_secs_f32();
    });
}