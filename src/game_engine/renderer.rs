use glium::{Display, Frame, Surface};

use crate::game_engine::mesh::Mesh;
use crate::game_engine::transform::Transform;
use crate::game_engine::color::Color;

pub struct Renderer<'a, 'b> {
    pub display: &'b Display,
    pub camera_position: [f32; 3],
    pub light_position: [f32; 3],
    pub view_matrix: [[f32; 4]; 4],
    pub perspective_matrix: [[f32; 4]; 4],
    pub params: glium::DrawParameters<'a>,
    target: Frame,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(d: &'b Display,
        cp: [f32; 3],
        lp: [f32; 3],
        v: [[f32; 4]; 4],
        p: [[f32; 4]; 4],
        params: glium::DrawParameters<'a>
    ) -> Renderer<'a, 'b> {

        let t = d.draw();
        Renderer {
            display: d,
            camera_position: cp,
            light_position: lp,
            view_matrix: v,
            perspective_matrix: p,
            params: params,
            target: t,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.target.clear_color_and_depth((color.r, color.g, color.b, color.a), 1.0);
    }

    pub fn show(&mut self) {
        self.target.set_finish();
    }

    pub fn draw(&mut self, mesh: &Mesh) {
        for surface_i in 0..mesh.index_buffers.len() {
            if mesh.materials.len() <= surface_i {
                break;
            }
            let material = &mesh.materials[surface_i];
            self.target.draw(&mesh.vertex_buffer, &mesh.index_buffers[surface_i], &material.shader,
                &uniform! { model: mesh.transform.form_matrix(),
                    view: self.view_matrix,
                    perspective: self.perspective_matrix,
                    u_light: self.light_position,
                    u_color: material.albedo.as_array_rgb() },
                &self.params).unwrap();
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.target.get_dimensions()
    }
}

impl Drop for Renderer<'_, '_> {
    fn drop(&mut self) {
        std::mem::drop(&self.target);
    }
}