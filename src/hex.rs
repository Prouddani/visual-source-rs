use std::{fmt::Display, ops::{Deref, DerefMut}};

#[derive(Clone, Copy, Debug)]
pub struct Hex(pub f64);
impl Hex {
    pub fn from_hex(src: &str) -> Option<Self> {
        let (int_part, frac_part) = src.split_once('.').unwrap_or((src, ""));

        let mut decimal = 0.0;

        // integer part
        for c in int_part.chars() {
            let digit = c.to_digit(16)? as f64; // HERE
            decimal = decimal * 16.0 + digit;
        }

        // fractional part
        let mut hex_factor = 1.0 / 16.0;

        for c in frac_part.chars() {
            let digit = c.to_digit(16)? as f64; // HERE
            decimal += digit * hex_factor;
            hex_factor /= 16.0;
        }

        Some(Self(decimal))
    }
}
impl From<f64> for Hex {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl Into<f64> for Hex {
    fn into(self) -> f64 {
        self.0
    }
}
impl Deref for Hex {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Hex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x = self.0;
        
        let sign = if x < 0.0 {
            x = -x;
            "-"
        } else {
            ""
        };

        let int_part = x.trunc() as u64;
        let mut frac = x.fract();

        let mut out = format!("{}{:X}", sign, int_part);

        // no fractional part
        if frac == 0.0 {
            return write!(f, "{}", out);
        }

        out.push('.');

        // generate fractional digits
        let mut seen = 0;
        while frac != 0.0 && seen < 16 {
            frac *= 16.0; // we essentially convert a float digit into a hex digit
            let digit = frac.trunc() as u8;
            out.push(std::char::from_digit(digit as u32, 16)
                .unwrap()
                .to_ascii_uppercase());
            frac -= digit as f64;
            seen += 1;
        }

        write!(f, "{}", out)
    }
}