use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorError {
	FromF32(String),
	FailedToParse(String),
}

impl fmt::Display for ColorError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::FromF32(msg) => write!(f, "Failed to convert: {}", msg),
			Self::FailedToParse(msg) => write!(f, "Failed to parse: {}", msg),
		}
	}
}

impl std::error::Error for ColorError {}

pub type Result<T, E = ColorError> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Color {
	pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Self { r: red, g: green, b: blue, a: alpha }
	}

	pub fn try_from_f32(
		red: f32,
		green: f32,
		blue: f32,
		alpha: f32,
	) -> Result<Self> {
		macro_rules! check_convert_value {
			( $value:expr ) => {
				if (0.0..=1.0).contains(&$value) {
					($value * 255.) as u8
				} else {
					return Err(ColorError::FromF32(format!(
						"Invalid value for element `{}` (range: 0.0 - 1.0; \
			 got: {})",
						stringify!($value),
						$value
					)));
				}
			};
		}

		let red = check_convert_value!(red);
		let green = check_convert_value!(green);
		let blue = check_convert_value!(blue);
		let alpha = check_convert_value!(alpha);

		Ok(Self::new(red, green, blue, alpha))
	}
}
