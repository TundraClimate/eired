use std::fmt::{Debug, Display};
use std::mem;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    fg: Option<AnsiColor>,
    bg: Option<AnsiColor>,
}

impl Style {
    pub fn with_fg(color: AnsiColor) -> Self {
        Self::default().fg(color)
    }

    pub fn with_bg(color: AnsiColor) -> Self {
        Self::default().bg(color)
    }

    pub fn fg(mut self, color: AnsiColor) -> Self {
        self.fg = Some(color);

        self
    }

    pub fn bg(mut self, color: AnsiColor) -> Self {
        self.bg = Some(color);

        self
    }
}

impl Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fg = if let Some(fg) = self.fg {
            format!("\\x1b[38;5;{}m", fg)
        } else {
            "\\x1b[39m".to_string()
        };

        let bg = if let Some(bg) = self.bg {
            format!("\\x1b[48;5;{}m", bg)
        } else {
            "\\x1b[49m".to_string()
        };

        write!(f, "{}{}", fg, bg)
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fg = if let Some(fg) = self.fg {
            format!("\x1b[38;5;{}m", fg)
        } else {
            "\x1b[39m".to_string()
        };

        let bg = if let Some(bg) = self.bg {
            format!("\x1b[48;5;{}m", bg)
        } else {
            "\x1b[49m".to_string()
        };

        write!(f, "{}{}", fg, bg)
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
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

    pub fn is_colorcube(&self) -> bool {
        let num = u8::from(*self);

        232 > num && num > 15
    }

    pub fn is_grayscale(&self) -> bool {
        u8::from(*self) > 231
    }

    pub fn colorcube(r: u8, g: u8, b: u8) -> Option<Self> {
        (r < 6 && g < 6 && b < 6).then_some(Self::colorcube_unchecked(r, g, b))
    }

    pub fn colorcube_unchecked(r: u8, g: u8, b: u8) -> Self {
        debug_assert!(r < 6 && g < 6 && b < 6);
        unsafe { mem::transmute::<u8, Self>(16 + 36 * r + 6 * g + b) }
    }

    pub fn grayscale(scale: u8) -> Option<Self> {
        (scale < 24).then_some(Self::grayscale_unchecked(scale))
    }

    pub fn grayscale_unchecked(scale: u8) -> Self {
        debug_assert!(scale < 24);
        unsafe { mem::transmute::<u8, Self>(232 + scale) }
    }
}

impl From<u8> for AnsiColor {
    fn from(value: u8) -> Self {
        unsafe { mem::transmute::<u8, Self>(value) }
    }
}

impl From<AnsiColor> for u8 {
    fn from(value: AnsiColor) -> Self {
        value as Self
    }
}

impl Debug for AnsiColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", u8::from(*self))
    }
}

impl Display for AnsiColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

impl PartialEq for AnsiColor {
    fn eq(&self, other: &Self) -> bool {
        u8::from(*self) == u8::from(*other)
    }
}

impl Eq for AnsiColor {}
