use std::fmt::{Debug, Display};
use std::mem;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
}

impl Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fg = if let Some(fg) = self.fg {
            format!("\\x1b[38;5;{}m", fg.0)
        } else {
            "\\x1b[39m".to_string()
        };

        let bg = if let Some(bg) = self.bg {
            format!("\\x1b[48;5;{}m", bg.0)
        } else {
            "\\x1b[49m".to_string()
        };

        write!(f, "{}{}", fg, bg)
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fg = if let Some(fg) = self.fg {
            format!("\x1b[38;5;{}m", fg.0)
        } else {
            "\x1b[39m".to_string()
        };

        let bg = if let Some(bg) = self.bg {
            format!("\x1b[48;5;{}m", bg.0)
        } else {
            "\x1b[49m".to_string()
        };

        write!(f, "{}{}", fg, bg)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color(u8);

impl Color {
    pub fn new(ansi: AnsiColor) -> Self {
        Self(ansi.into())
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(AnsiColor::rgb(r, g, b))
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnsiColor {
    Black = 0,
    DarkRed = 1,
    DarkGreen = 2,
    DarkYellow = 3,
    DarkBlue = 4,
    DarkMagenta = 5,
    DarkCyan = 6,
    Gray = 7,
    DarkGray = 8,
    Red = 9,
    Green = 10,
    Yellow = 11,
    Blue = 12,
    Magenta = 13,
    Cyan = 14,
    White = 15,
    ColorCube(U6, U6, U6) = 16,
    Grayscale(U24) = 17,
}

impl AnsiColor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        let rq = Self::quantize(r);
        let gq = Self::quantize(g);
        let bq = Self::quantize(b);

        let gray = (r as u16 * 30 + g as u16 * 59 + b as u16 * 11) / 100;
        let gi = if gray < 8 {
            0
        } else if gray > 248 {
            23
        } else {
            ((gray - 8) / 10) as u8
        };

        let (rc, gc, bc) = (Self::unround(rq), Self::unround(gq), Self::unround(bq));
        let gv = 8 + 10 * gi as u16;

        let cd = Self::dist2(r, g, b, rc, gc, bc);
        let gd = Self::dist2(r, g, b, gv as u8, gv as u8, gv as u8);

        let cube = Self::colorcube_unchecked(rq, gq, bq);
        let gray = Self::grayscale_unchecked(gi);

        if gd < cd { gray } else { cube }
    }

    #[inline]
    fn quantize(v: u8) -> u8 {
        ((v as u16 * 5 + 127) / 255) as u8
    }

    #[inline]
    fn unround(v: u8) -> u8 {
        [0, 95, 135, 175, 215, 255][v as usize]
    }

    #[inline]
    fn dist2(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> u32 {
        let dr = r1 as i32 - r2 as i32;
        let dg = g1 as i32 - g2 as i32;
        let db = b1 as i32 - b2 as i32;
        (dr * dr + dg * dg + db * db) as u32
    }

    pub fn colorcube(r: u8, g: u8, b: u8) -> Option<Self> {
        Some(Self::ColorCube(U6::new(r)?, U6::new(g)?, U6::new(b)?))
    }

    pub fn colorcube_unchecked(r: u8, g: u8, b: u8) -> Self {
        Self::ColorCube(
            U6::new_unchecked(r),
            U6::new_unchecked(g),
            U6::new_unchecked(b),
        )
    }

    pub fn grayscale(scale: u8) -> Option<Self> {
        Some(Self::Grayscale(U24::new(scale)?))
    }

    pub fn grayscale_unchecked(scale: u8) -> Self {
        Self::Grayscale(U24::new_unchecked(scale))
    }
}

impl From<AnsiColor> for u8 {
    fn from(value: AnsiColor) -> Self {
        match value {
            AnsiColor::ColorCube(r, g, b) => 16 + 36 * r.0 + 6 * g.0 + b.0,
            AnsiColor::Grayscale(scale) => 232 + scale.0,
            _ => (unsafe { mem::transmute::<AnsiColor, u32>(value) }) as u8,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct U6(u8);

impl U6 {
    pub const fn new(v: u8) -> Option<Self> {
        if v < 6 { Some(Self(v)) } else { None }
    }

    pub const fn new_unchecked(v: u8) -> Self {
        assert!(v < 6);
        Self(v)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct U24(u8);

impl U24 {
    pub const fn new(v: u8) -> Option<Self> {
        if v < 24 { Some(Self(v)) } else { None }
    }

    pub const fn new_unchecked(v: u8) -> Self {
        assert!(v < 24);
        Self(v)
    }
}
