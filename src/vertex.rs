#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vertex {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vertex {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexRef(pub(crate) usize);

impl VertexRef {
	pub(crate) unsafe fn resolve_unchecked(self, items: &[Vertex]) -> &Vertex {
		items.get_unchecked(self.0)
	}
}
