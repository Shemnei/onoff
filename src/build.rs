use std::marker::PhantomData;

use crate::color::Color;
use crate::document::{Checked, Document};
use crate::face::FaceRef;
use crate::vertex::{Vertex, VertexRef};

#[derive(Default, Debug, Clone)]
pub struct CheckedDocumentBuilder {
	vertices: Vec<Vertex>,
	face_refs: Vec<FaceRef>,
	edge_count: Option<u64>,
}

impl CheckedDocumentBuilder {
	#[must_use]
	pub fn add_vertex(mut self, vertex: Vertex) -> Self {
		self.vertices.push(vertex);
		self
	}

	#[must_use]
	pub fn add_face(
		mut self,
		vertices: Vec<Vertex>,
		color: Option<Color>,
	) -> Self {
		let vertex_refs = vertices
			.into_iter()
			.map(|v| {
				let index = self.vertices.len();
				self.vertices.push(v);
				VertexRef(index)
			})
			.collect();

		self.face_refs.push(FaceRef { vertex_refs, color });

		self
	}

	#[must_use]
	pub fn set_edge_count(mut self, edge_count: Option<u64>) -> Self {
		self.edge_count = edge_count;
		self
	}

	#[must_use]
	pub fn finish(self) -> Document<Checked> {
		// TODO: minimize vertices (compare with bits?)
		Document {
			vertices: self.vertices,
			face_refs: self.face_refs,
			edge_count: self.edge_count,
			_marker: PhantomData,
		}
	}
}

impl Document<Checked> {
	pub fn build() -> CheckedDocumentBuilder {
		CheckedDocumentBuilder::default()
	}
}
