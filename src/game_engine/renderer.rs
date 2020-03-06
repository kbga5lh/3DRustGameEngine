use glium::{Display, Frame, Surface};

use crate::game_engine::mesh::Mesh;
use crate::game_engine::transform::Transform;

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

    pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.target.clear_color_and_depth((r, g, b, a), 1.0);
    }

    pub fn show(&mut self) {
        self.target.set_finish();
    }

    pub fn draw(&mut self, mesh: &Mesh) {
        self.target.draw(&mesh.vertex_buffer, &mesh.index_buffer, &mesh.material.shader,
            &uniform! { model: mesh.transform.form_matrix(),
                view: self.view_matrix,
                perspective: self.perspective_matrix,
                u_light: self.light_position,
                u_color: mesh.material.albedo },
            &self.params).unwrap();
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