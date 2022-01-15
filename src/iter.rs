use crate::document::{Checked, Document};
use crate::face::ResolvedFaceRef;

pub struct FaceIter<'a> {
	document: &'a Document<Checked>,
	face_index: usize,
}

impl<'a> FaceIter<'a> {
	pub fn new(document: &'a Document<Checked>) -> Self {
		Self { document, face_index: 0 }
	}
}

impl<'a> Iterator for FaceIter<'a> {
	type Item = ResolvedFaceRef<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let face_ref = self.document.face_refs.get(self.face_index)?;
		self.face_index += 1;

		let vertices = face_ref
			.vertex_refs
			.iter()
			.map(|fr|
                // SAFETY: A document with state `Checked` (which is the only
                // one accepted for this iter) has already verified that all
                // references can be resolved. It is also immutable sot that
                // no modifications can be made after the check.
                unsafe {fr.resolve_unchecked(&self.document.vertices)})
			.collect();

		Some(ResolvedFaceRef { vertices, color: face_ref.color.as_ref() })
	}
}

impl Document<Checked> {
	pub fn face_iter(&self) -> FaceIter<'_> {
		FaceIter::new(self)
	}
}
