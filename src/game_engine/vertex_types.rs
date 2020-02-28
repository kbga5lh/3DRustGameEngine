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