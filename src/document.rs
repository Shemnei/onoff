use std::fmt;
use std::marker::PhantomData;

use crate::face::FaceRef;
use crate::vertex::Vertex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(String);

impl fmt::Display for ValidationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Failed to validate document: {}", self.0)
	}
}

mod sealed {
	pub trait Sealed {}
}

pub trait State: sealed::Sealed {}

#[derive(Debug)]
pub struct Unchecked;
impl sealed::Sealed for Unchecked {}
impl State for Unchecked {}

#[derive(Debug)]
pub struct Checked;
impl sealed::Sealed for Checked {}
impl State for Checked {}

#[derive(Debug, Clone, PartialEq)]
pub struct Document<S> {
	pub(crate) vertices: Vec<Vertex>,
	pub(crate) face_refs: Vec<FaceRef>,
	pub(crate) edge_count: Option<u64>,
	pub(crate) _marker: PhantomData<S>,
}

impl<S> Document<S> {
	pub fn vertices(&self) -> &[Vertex] {
		&self.vertices
	}

	pub fn face_refs(&self) -> &[FaceRef] {
		&self.face_refs
	}
}

impl Document<Unchecked> {
	pub fn new(
		vertices: Vec<Vertex>,
		face_refs: Vec<FaceRef>,
		edge_count: Option<u64>,
	) -> Self {
		Self { vertices, face_refs, edge_count, _marker: PhantomData }
	}

	pub fn validate(self) -> Result<Document<Checked>, ValidationError> {
		let mut vertex_ref_iter =
			self.face_refs.iter().flat_map(|fr| &fr.vertex_refs);

		if let Some(invalid_vertex_index) =
			vertex_ref_iter.find(|vr| vr.0 >= self.vertices.len())
		{
			Err(ValidationError(format!(
				"No vertex present for index `{}`",
				invalid_vertex_index.0
			)))
		} else {
			Ok(Document {
				vertices: self.vertices,
				face_refs: self.face_refs,
				edge_count: self.edge_count,
				_marker: PhantomData,
			})
		}
	}
}
