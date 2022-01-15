use std::borrow::Cow;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Lines;

use crate::colorformat::ColorFormat;
use crate::document::{Document, Unchecked};
use crate::face::FaceRef;
use crate::vertex::{Vertex, VertexRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    kind: ErrorKind,
    line_index: usize,
    message: Option<Cow<'static, str>>,
}

impl Error {
    pub fn new(kind: ErrorKind, line_index: usize, message: Option<Cow<'static, str>>) -> Self {
        Self {
            kind,
            line_index,
            message,
        }
    }

    pub fn with_message<M: Into<Cow<'static, str>>, O: Into<Option<M>>>(
        kind: ErrorKind,
        line_index: usize,
        message: O,
    ) -> Self {
        Self {
            kind,
            line_index,
            message: message.into().map(|inner| inner.into()),
        }
    }

    pub fn without_message(kind: ErrorKind, line_index: usize) -> Self {
        Self {
            kind,
            line_index,
            message: None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let Some(msg) = &self.message {
            write!(f, "{} @ ln:{} - {}", self.kind, self.line_index + 1, msg)
        } else {
            write!(f, "{} @ ln:{}", self.kind, self.line_index + 1,)
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Empty,
    Missing,
    Invalid,
    InvalidMagic,
    InvalidCounts,
    InvalidVertex,
    InvalidColor,
    InvalidFace,
    LimitExceeded,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, f)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct OffLines<'a> {
    lines: Enumerate<Lines<'a>>,
}

impl<'a> OffLines<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            lines: s.lines().enumerate(),
        }
    }
}

impl<'a> Iterator for OffLines<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        for (line_index, mut line) in self.lines.by_ref() {
            if let Some(comment_index) = line.find('#') {
                line = &line[..comment_index];
            }

            // Trim after removing comments to prevent the following `Hello # World` => `Hello `
            // (should be `Hello`)
            line = line.trim();

            if !line.is_empty() {
                return Some((line_index, line));
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub vertex_count: usize,
    pub face_count: usize,
    pub face_vertex_count: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            vertex_count: 2048,
            face_count: 4096,
            face_vertex_count: 128,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserOptions<C> {
    pub color_format: C,
    pub limits: Limits,
}

impl Default for ParserOptions<crate::colorformat::Any> {
    fn default() -> Self {
        Self {
            color_format: crate::colorformat::Any,
            limits: Default::default(),
        }
    }
}

pub struct OffParser<'a, C> {
    #[allow(unused)]
    options: ParserOptions<C>,
    lines: Peekable<OffLines<'a>>,
}

impl<'a> OffParser<'a, crate::colorformat::Any> {
    pub fn new<S: AsRef<str>>(s: &'a S) -> Self {
        let lines = OffLines::new(s.as_ref()).peekable();

        Self {
            lines,
            options: Default::default(),
        }
    }
}

impl<'a, C> OffParser<'a, C>
where
    C: ColorFormat,
{
    pub fn new_with_options<S: AsRef<str>>(s: &'a S, options: ParserOptions<C>) -> Self {
        let lines = OffLines::new(s.as_ref()).peekable();

        Self { lines, options }
    }

    pub fn try_parse(mut self) -> Result<Document<Unchecked>> {
        let _ = self.try_consume_magic()?;
        let (vertex_count, face_count, edge_count) = self.try_consume_counts()?;

        if vertex_count > self.options.limits.vertex_count {
            return Err(Error::with_message(
                ErrorKind::LimitExceeded,
                0, // TODO: save last line index
                format!(
                    "Vertex count exceeds limit (limit: {}, requested: {})",
                    self.options.limits.vertex_count, vertex_count
                ),
            ));
        }

        if face_count > self.options.limits.face_count {
            return Err(Error::with_message(
                ErrorKind::LimitExceeded,
                0, // TODO: save last line index
                format!(
                    "Face count exceeds limit (limit: {}, requested: {})",
                    self.options.limits.face_count, face_count
                ),
            ));
        }

        let vertices = self.try_consume_vertices(vertex_count)?;
        let faces = self.try_consume_faces(face_count, vertex_count)?;

        if let Some((line_index, _)) = self.lines.next() {
            Err(Error::with_message(
                ErrorKind::Invalid,
                line_index,
                "Unexpected lines after OFF definition",
            ))
        } else {
            Ok(Document::new(vertices, faces, edge_count))
        }
    }

    fn try_consume_magic(&mut self) -> Result<()> {
        let (line_index, line) = self
            .lines
            .peek()
            .ok_or_else(|| Error::without_message(ErrorKind::Empty, 0))?;

        if let Some(suffix) = line.strip_prefix("OFF") {
            if suffix.is_empty() {
                // valid magic
                // consume peeked item
                let _ = self.lines.next().expect("Next item not present");
            } else {
                // trailing characters; invalid magic
                return Err(Error::with_message(
                    ErrorKind::InvalidMagic,
                    *line_index,
                    "Trailing characters after magic",
                ));
            }
        }

        Ok(())
    }

    fn try_consume_counts(&mut self) -> Result<(usize, usize, Option<u64>)> {
        let (line_index, line) = self.lines.next().ok_or_else(|| {
            Error::with_message(
                ErrorKind::Missing,
                0,
                "No counts for vertices, faces and edges present",
            )
        })?;

        let counts = line
            .split_whitespace()
            .map(|w| w.parse::<usize>())
            // Take one more than we expect/want so that we can check bellow
            // if we got the expected amount or more.
            .take(4)
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|err| {
                Error::with_message(
                    ErrorKind::InvalidCounts,
                    line_index,
                    format!("Failed to parse count as number ({})", err),
                )
            })?;

        match counts[..] {
            [vertices, faces, edges] => Ok((vertices, faces, Some(edges as u64))),
            [vertices, faces] => Ok((vertices, faces, None)),
            _ => Err(Error::with_message(
                ErrorKind::InvalidCounts,
                line_index,
                format!(
                    "Invalid number of counts given (expected: 2-3, actual: {})",
                    counts.len()
                ),
            )),
        }
    }

    fn try_consume_vertices(&mut self, vertex_count: usize) -> Result<Vec<Vertex>> {
        (0..vertex_count)
            .map(|_| self.try_consume_vertex())
            .collect()
    }

    fn try_consume_vertex(&mut self) -> Result<Vertex> {
        let (line_index, line) = self
            .lines
            .next()
            .ok_or_else(|| Error::with_message(ErrorKind::Missing, 0, "Expected vertex"))?;

        let coords = line
            .split_whitespace()
            .map(|w| w.parse::<f32>())
            // Take one more than we expect/want so that we can check bellow
            // if we got the expected amount or more.
            .take(4)
            .collect::<Result<Vec<f32>, _>>()
            .map_err(|err| {
                Error::with_message(
                    ErrorKind::InvalidVertex,
                    line_index,
                    format!("Failed to parse coordinate as number ({})", err),
                )
            })?;

        if let [x, y, z] = coords[..] {
            Ok(Vertex::new(x, y, z))
        } else {
            Err(Error::with_message(
                ErrorKind::InvalidVertex,
                line_index,
                format!(
                    "Invalid number of coordinates given (expected: 3, actual: {})",
                    coords.len()
                ),
            ))
        }
    }

    fn try_consume_faces(
        &mut self,
        face_count: usize,
        vertex_count: usize,
    ) -> Result<Vec<FaceRef>> {
        (0..face_count)
            .map(|_| self.try_consume_face(vertex_count))
            .collect()
    }

    fn try_consume_face(&mut self, vertex_count: usize) -> Result<FaceRef> {
        let (line_index, line) = self
            .lines
            .next()
            .ok_or_else(|| Error::with_message(ErrorKind::Missing, 0, "Expected face"))?;

        let mut words = line.split_whitespace();

        let vertex_index_count = words
            .next()
            .ok_or_else(|| {
                Error::with_message(
                    ErrorKind::InvalidFace,
                    line_index,
                    "Expected number of vertices",
                )
            })?
            .parse::<usize>()
            .map_err(|err| {
                Error::with_message(
                    ErrorKind::InvalidFace,
                    line_index,
                    format!("Failed to parse vertex count as number ({})", err),
                )
            })?;

        if vertex_index_count > self.options.limits.face_vertex_count {
            return Err(Error::with_message(
                ErrorKind::LimitExceeded,
                line_index,
                format!(
                    "Vertex count of face exceeds limit (limit: {}, requested: {})",
                    self.options.limits.face_vertex_count, vertex_index_count
                ),
            ));
        }

        let mut vertex_indexes = Vec::with_capacity(vertex_index_count);

        for i in 0..vertex_index_count {
            let vertex_index = words
                .next()
                .ok_or_else(|| {
                    Error::with_message(
                        ErrorKind::InvalidFace,
                        line_index,
                        format!("Expected vertex index ({}/{})", i, vertex_index_count),
                    )
                })?
                .parse::<usize>()
                .map_err(|err| {
                    Error::with_message(
                        ErrorKind::InvalidFace,
                        line_index,
                        format!(
                            "Failed to parse vertex index as number ({}/{}; {})",
                            i, vertex_index_count, err
                        ),
                    )
                })?;

            if vertex_index >= vertex_count {
                return Err(Error::with_message(
                    ErrorKind::InvalidFace,
                    line_index,
                    format!(
                        "Vertex index out of bounds ({}/{})",
                        vertex_index, vertex_count
                    ),
                ));
            }

            vertex_indexes.push(VertexRef(vertex_index));
        }

        // Check for color
        let mut words = words.peekable();

        let color = if words.peek().is_some() {
            Some(C::try_parse(&mut words).map_err(|err| {
                Error::with_message(ErrorKind::InvalidColor, line_index, err.to_string())
            })?)
        } else {
            None
        };

        if words.next().is_some() {
            Err(Error::with_message(
                ErrorKind::Invalid,
                line_index,
                "Found elements after color definition",
            ))
        } else {
            Ok(FaceRef {
                vertex_refs: vertex_indexes,
                color,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::colorformat::{RgbU8, RgbaF32};

    use super::*;

    #[test]
    fn wiki() {
        let content = r#"OFF
# cube.off
# A cube
 
8 6 12
 1.0  0.0 1.4142
 0.0  1.0 1.4142
-1.0  0.0 1.4142
 0.0 -1.0 1.4142
 1.0  0.0 0.0
 0.0  1.0 0.0
-1.0  0.0 0.0
 0.0 -1.0 0.0
4  0 1 2 3  255 0 0 #red
4  7 4 0 3  0 255 0 #green
4  4 5 1 0  0 0 255 #blue
4  5 6 2 1  0 255 0 
4  3 2 6 7  0 0 255
4  6 5 4 7  255 0 0"#;

        let options = ParserOptions {
            color_format: RgbU8,
            limits: Default::default(),
        };
        let parser = OffParser::new_with_options(&content, options);
        let document = parser.try_parse().unwrap();

        println!("{:#?}", document);
    }

    #[test]
    fn spec_example() {
        let content = r#"
OFF
#
#  cube.off
#  A cube.
#  There is extra RGBA color information specified for the faces.
#
8 6 12
  1.632993   0.000000   1.154701
  0.000000   1.632993   1.154701
 -1.632993   0.000000   1.154701
  0.000000  -1.632993   1.154701
  1.632993   0.000000  -1.154701
  0.000000   1.632993  -1.154701
 -1.632993   0.000000  -1.154701
  0.000000  -1.632993  -1.154701
  4  0 1 2 3  1.000 0.000 0.000 0.75
  4  7 4 0 3  0.300 0.400 0.000 0.75
  4  4 5 1 0  0.200 0.500 0.100 0.75
  4  5 6 2 1  0.100 0.600 0.200 0.75
  4  3 2 6 7  0.000 0.700 0.300 0.75
  4  6 5 4 7  0.000 1.000 0.000 0.75
"#;

        let options = ParserOptions {
            color_format: RgbaF32,
            limits: Default::default(),
        };
        let parser = OffParser::new_with_options(&content, options);
        let document = parser.try_parse().unwrap();

        println!("{:#?}", document);

        let checked = document.validate().unwrap();

        for face in checked.face_iter() {
            println!("F: {:?}", face);
        }
    }

    #[test]
    fn parse_resources() {
        for res in std::fs::read_dir("resources").unwrap() {
            let res = res.expect("Failed to get resources");
            let content = std::fs::read_to_string(res.path())
                .expect(&format!("Failed to read: {}", res.path().display()));

            let parser = OffParser::new(&content);
            let _ = parser
                .try_parse()
                .expect(&format!("Failed to parse: {}", res.path().display()));
        }
    }
}
