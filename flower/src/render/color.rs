pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    /// Attempt to create a color from a CSS-style hex string.
    ///
    /// This will accept strings in the following formats, *with or without*
    /// the leading `#`:
    ///
    /// - `rrggbb`
    /// - `rrggbbaa`
    /// - `rbg`
    /// - `rbga`
    ///
    /// This method returns a [`ColorParseError`] if the color cannot be parsed.
    pub const fn from_hex_str(hex: &str) -> Result<Color, ColorParseError> {
        // can't use `map()` in a const function
        match get_4bit_hex_channels(hex) {
            Ok(channels) => Ok(color_from_4bit_hex(channels)),
            Err(e) => Err(e),
        }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Self {
            r,
            g,
            b,
            a,
        }
    }
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Self {
            r,
            g,
            b,
            a: 0xff,
        }
    }

    pub fn rgba_gl_vec4(&self) -> String {
        format!("vec4({},{},{},{});", self.r as f32 / 255., self.g as f32 / 255., self.b as f32 / 255., self.a as f32 / 255.)
    }
}

const fn color_from_4bit_hex(components: [u8; 8]) -> Color {
    let [r0, r1, g0, g1, b0, b1, a0, a1] = components;
    Color::rgba(r0 << 4 | r1, g0 << 4 | g1, b0 << 4 | b1, a0 << 4 | a1)
}

/// Errors that can occur when parsing a hex color.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorParseError {
    /// The input string has an incorrect length
    WrongSize(usize),
    /// A byte in the input string is not in one of the ranges `0..=9`,
    /// `a..=f`, or `A..=F`.
    #[allow(missing_docs)]
    NotHex { idx: usize, byte: u8 },
}

const fn hex_from_ascii_byte(b: u8) -> Result<u8, u8> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        _ => Err(b),
    }
}

const fn get_4bit_hex_channels(hex_str: &str) -> Result<[u8; 8], ColorParseError> {
    let mut four_bit_channels = match hex_str.as_bytes() {
        &[b'#', r, g, b] | &[r, g, b] => [r, r, g, g, b, b, b'f', b'f'],
        &[b'#', r, g, b, a] | &[r, g, b, a] => [r, r, g, g, b, b, a, a],
        &[b'#', r0, r1, g0, g1, b0, b1] | &[r0, r1, g0, g1, b0, b1] => {
            [r0, r1, g0, g1, b0, b1, b'f', b'f']
        }
        &[b'#', r0, r1, g0, g1, b0, b1, a0, a1] | &[r0, r1, g0, g1, b0, b1, a0, a1] => {
            [r0, r1, g0, g1, b0, b1, a0, a1]
        }
        other => return Err(ColorParseError::WrongSize(other.len())),
    };

    // convert to hex in-place
    // this is written without a for loop to satisfy `const`
    let mut i = 0;
    while i < four_bit_channels.len() {
        let ascii = four_bit_channels[i];
        let as_hex = match hex_from_ascii_byte(ascii) {
            Ok(hex) => hex,
            Err(byte) => return Err(ColorParseError::NotHex { idx: i, byte }),
        };
        four_bit_channels[i] = as_hex;
        i += 1;
    }
    Ok(four_bit_channels)
}