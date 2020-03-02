use wavefront_obj::obj;

#[derive(Copy, Clone)]
pub struct VertexPN {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
}

impl std::cmp::PartialEq<(&obj::Vertex, &obj::Vertex)> for VertexPN {
    fn eq(&self, other: &(&obj::Vertex, &obj::Vertex)) -> bool {
        self.position.0 == other.0.x as f32 &&
        self.position.1 == other.0.y as f32 &&
        self.position.2 == other.0.z as f32 &&
        
        self.normal.0 == other.1.x as f32 &&
        self.normal.1 == other.1.y as f32 &&
        self.normal.2 == other.1.z as f32
    }
}

impl std::cmp::PartialEq for VertexPN {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position &&
        self.normal == other.normal
    }
}

implement_vertex!(VertexPN, position, normal);