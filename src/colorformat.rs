use crate::color::{Color, ColorError};

pub trait ColorFormat {
    fn try_parse<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Color, ColorError>;
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Any;

impl ColorFormat for Any {
    fn try_parse<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Color, ColorError> {
        // (u8/f32) (u8/f32) (u8/f32) (u8/f32)?

        macro_rules! try_parse_convert_both {
            ( $name:literal : $value:expr ) => {{
                let value = $value.ok_or_else(|| {
                    ColorError::FailedToParse(format!("Element missing (`{}`)", $name))
                })?;

                if let Ok(byte) = value.parse::<u8>() {
                    Ok(byte)
                } else if let Ok(float) = value.parse::<f32>() {
                    if (0.0..=1.0).contains(&float) {
                        Ok((float * 255.0) as u8)
                    } else {
                        Err(ColorError::FailedToParse(format!(
                            "Invalid value for element `{}` (range: 0.0 - 1.0; got: {})",
                            $name, float
                        )))
                    }
                } else {
                    Err(ColorError::FailedToParse(format!(
                        "Element could not be parsed (`{}`)",
                        $name
                    )))
                }
            }};
        }

        let red = try_parse_convert_both!("red": iter.next())?;
        let green = try_parse_convert_both!("green": iter.next())?;
        let blue = try_parse_convert_both!("blue": iter.next())?;
        let alpha = try_parse_convert_both!("alpha": Some(iter.next().unwrap_or("255")))?;

        Ok(Color::new(red, green, blue, alpha))
    }
}

macro_rules! try_parse_convert {
    ( $name:literal : $value:expr ) => {{
        let value = $value
            .ok_or_else(|| ColorError::FailedToParse(format!("Element missing (`{}`)", $name)))?;

        if let Ok(byte) = value.parse() {
            Ok(byte)
        } else {
            Err(ColorError::FailedToParse(format!(
                "Element could not be parsed (`{}`)",
                $name
            )))
        }
    }};
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct RgbU8;

impl ColorFormat for RgbU8 {
    fn try_parse<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Color, ColorError> {
        let red = try_parse_convert!("red": iter.next())?;
        let green = try_parse_convert!("green": iter.next())?;
        let blue = try_parse_convert!("blue": iter.next())?;

        Ok(Color::new(red, green, blue, 255))
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct RgbaU8;

impl ColorFormat for RgbaU8 {
    fn try_parse<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Color, ColorError> {
        let red = try_parse_convert!("red": iter.next())?;
        let green = try_parse_convert!("green": iter.next())?;
        let blue = try_parse_convert!("blue": iter.next())?;
        let alpha = try_parse_convert!("alpha": iter.next())?;

        Ok(Color::new(red, green, blue, alpha))
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct RgbF32;

impl ColorFormat for RgbF32 {
    fn try_parse<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Color, ColorError> {
        let red = try_parse_convert!("red": iter.next())?;
        let green = try_parse_convert!("green": iter.next())?;
        let blue = try_parse_convert!("blue": iter.next())?;

        Color::try_from_f32(red, green, blue, 1.0)
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct RgbaF32;

impl ColorFormat for RgbaF32 {
    fn try_parse<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Color, ColorError> {
        let red = try_parse_convert!("red": iter.next())?;
        let green = try_parse_convert!("green": iter.next())?;
        let blue = try_parse_convert!("blue": iter.next())?;
        let alpha = try_parse_convert!("alpha": iter.next())?;

        Color::try_from_f32(red, green, blue, alpha)
    }
}
