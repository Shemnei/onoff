use crate::color::Color;
use crate::vertex::{Vertex, VertexRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FaceRef {
	pub(crate) vertex_refs: Vec<VertexRef>,
	pub(crate) color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFaceRef<'a> {
	pub vertices: Vec<&'a Vertex>,
	pub color: Option<&'a Color>,
}
