#[macro_use]
extern crate glium;
extern crate wavefront_obj;

use glium::{glutin, Surface};
use glutin::event;
use wavefront_obj::obj;

use std::fs;
use std::rc::Rc;

mod game_engine;

use game_engine::object3d::Object3D;
use game_engine::math;
use game_engine::vector3::Vector3;
use game_engine::renderer::Renderer;
use game_engine::mesh::Mesh;
use game_engine::material::Material;
use game_engine::color::Color;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let s = event_loop.available_monitors().next().unwrap();
    let wb = glutin::window::WindowBuilder::new().with_fullscreen(Some(glutin::window::Fullscreen::Borderless(s)));
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = fs::read_to_string("assets/shaders/vertex_shader.glsl").unwrap();
    let fragment_shader_src = fs::read_to_string("assets/shaders/fragment_shader.glsl").unwrap();

    let program = Rc::new(glium::Program::from_source(&display,
        &vertex_shader_src.to_string(),
        &fragment_shader_src.to_string(), None).unwrap());

    // objects

    let mut board = {
        let object = {
            let source = fs::read_to_string("assets/models/board.obj").unwrap();
            obj::parse(source).unwrap()
        };
        let mut mesh = Mesh::new(&object.objects[0], &display);

        let material1 = Material {
            albedo: Color::new(0.075, 0.04, 0.01, 1.0),
            shader: program.clone(),
        };
        let material2 = Material {
            albedo: Color::new(1.0, 1.0, 1.0, 1.0),
            shader: program.clone(),
        };
        mesh.materials = vec!(material1.clone(), material2, material1);
        Object3D::new(mesh)
    };
    board.mesh.transform.scale(Vector3::fill(0.1));
    board.mesh.transform.translate(Vector3::new(0.0, -0.1, 0.0));

    let mut rook = {
        let object = {
            let source = fs::read_to_string("assets/models/rook.obj").unwrap();
            obj::parse(source).unwrap()
        };
        let mut mesh = Mesh::new(&object.objects[0], &display);
        mesh.materials = vec!(Material {
            albedo: Color::new(0.7, 0.3, 0.1, 1.0),
            shader: program.clone(),
        });
        Object3D::new(mesh)
    };
    rook.mesh.transform.scale(Vector3::fill(0.1));

    // variables

    let mut angle: f32 = 0.0;
    let speed: f32 = 0.5;

    let mut movement_buttons = [false; 6];
    let mut view_pos = Vector3::new(0.0, 1.5, -2.0);

    // =======================
    // ====== loop ===========
    // =======================

    event_loop.run(move |event, _, control_flow| {

        let frame_time = std::time::Instant::now();

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                event::WindowEvent::KeyboardInput { input: e, .. } => {
                    match e {
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::W), state: event::ElementState::Pressed, .. } => movement_buttons[0] = true,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::S), state: event::ElementState::Pressed, .. } => movement_buttons[1] = true,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::A), state: event::ElementState::Pressed, .. } => movement_buttons[2] = true,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::D), state: event::ElementState::Pressed, .. } => movement_buttons[3] = true,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::LShift), state: event::ElementState::Pressed, .. } => movement_buttons[4] = true,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Space), state: event::ElementState::Pressed, .. } => movement_buttons[5] = true,

                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::W), state: event::ElementState::Released, .. } => movement_buttons[0] = false,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::S), state: event::ElementState::Released, .. } => movement_buttons[1] = false,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::A), state: event::ElementState::Released, .. } => movement_buttons[2] = false,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::D), state: event::ElementState::Released, .. } => movement_buttons[3] = false,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::LShift), state: event::ElementState::Released, .. } => movement_buttons[4] = false,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Space), state: event::ElementState::Released, .. } => movement_buttons[5] = false,
                        _ => (),
                    }
                },
                _ => return,
            },
            event::Event::NewEvents(cause) => match cause {
                event::StartCause::Init => (),
                event::StartCause::Poll => (),
                _ => return,
            },
            _ => return,
        }

        // draw

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let mut movement_direction = Vector3::new(0.0, 0.0, 0.0);
        if movement_buttons[0] { movement_direction.z += 1.0 };
        if movement_buttons[1] { movement_direction.z -= 1.0 };
        if movement_buttons[2] { movement_direction.x -= 1.0 };
        if movement_buttons[3] { movement_direction.x += 1.0 };
        if movement_buttons[4] { movement_direction.y -= 1.0 };
        if movement_buttons[5] { movement_direction.y += 1.0 };
        if movement_direction.magnitude() > 0.0 {
            movement_direction.normalize();
        }

        println!("{:?}", movement_direction);
        view_pos += movement_direction * frame_time.elapsed().as_secs_f32() * 50.0;
        let view = math::view_matrix(
            view_pos,
            Vector3::new(0.0, -1.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0));

        let frame_size = display.get_framebuffer_dimensions();
        let mut renderer = Renderer::new(&display,
            [0.0, 0.0, 0.0],
            [1.4, 0.4, -0.7f32],
            view,
            math::perspective_matrix(frame_size, 3.141592 / 3.0, 1024.0, 0.1),
            params,
        );

        renderer.clear(Color::new(0.02, 0.02, 0.02, 1.0));
        renderer.draw(&rook.mesh);
        renderer.draw(&board.mesh);
        renderer.show();

        // update

        rook.mesh.transform.set_position(Vector3::new(angle.sin(), 0.0, angle.cos())
            * rook.mesh.transform.get_scale().z * 3.0);
        angle += speed * frame_time.elapsed().as_secs_f32();
    });
}