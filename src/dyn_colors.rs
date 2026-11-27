#[allow(unused_imports)]
use crate::{
    AnsiColors, BgDynColorDisplay, CssColors, DynColor, FgDynColorDisplay, Rgb, XtermColors,
};
use core::fmt;

/// An enum describing runtime-configurable colors
///
/// This can be displayed using [`FgDynColorDisplay`](FgDynColorDisplay) or [`BgDynColorDisplay`](BgDynColorDisplay),
/// allowing for multiple types of colors to be used at runtime.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DynColors {
    Ansi(AnsiColors),
    Css(CssColors),
    Xterm(XtermColors),
    Rgb(u8, u8, u8),
}

impl crate::private::Sealed for DynColors {}

impl DynColor for DynColors {
    fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynColors::Ansi(ansi) => ansi.fmt_ansi_fg(f),
            DynColors::Css(css) => css.fmt_ansi_fg(f),
            DynColors::Xterm(xterm) => xterm.fmt_ansi_fg(f),
            &DynColors::Rgb(r, g, b) => Rgb(r, g, b).fmt_ansi_fg(f),
        }
    }

    fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynColors::Ansi(ansi) => ansi.fmt_ansi_bg(f),
            DynColors::Css(css) => css.fmt_ansi_bg(f),
            DynColors::Xterm(xterm) => xterm.fmt_ansi_bg(f),
            &DynColors::Rgb(r, g, b) => Rgb(r, g, b).fmt_ansi_bg(f),
        }
    }

    fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynColors::Ansi(ansi) => ansi.fmt_raw_ansi_fg(f),
            DynColors::Css(css) => css.fmt_raw_ansi_fg(f),
            DynColors::Xterm(xterm) => xterm.fmt_raw_ansi_fg(f),
            &DynColors::Rgb(r, g, b) => Rgb(r, g, b).fmt_raw_ansi_fg(f),
        }
    }

    fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynColors::Ansi(ansi) => ansi.fmt_raw_ansi_bg(f),
            DynColors::Css(css) => css.fmt_raw_ansi_bg(f),
            DynColors::Xterm(xterm) => xterm.fmt_raw_ansi_bg(f),
            &DynColors::Rgb(r, g, b) => Rgb(r, g, b).fmt_raw_ansi_bg(f),
        }
    }

    #[doc(hidden)]
    fn get_dyncolors_fg(&self) -> crate::DynColors {
        *self
    }

    #[doc(hidden)]
    fn get_dyncolors_bg(&self) -> crate::DynColors {
        *self
    }
}

/// An error for when the color can not be parsed from a string at runtime
#[derive(Debug)]
pub struct ParseColorError;

fn decode_hex_byte(high: u8, low: u8) -> Result<u8, ParseColorError> {
    Ok((decode_hex_digit(high)? << 4) | decode_hex_digit(low)?)
}

fn decode_hex_digit(digit: u8) -> Result<u8, ParseColorError> {
    match digit {
        b'0'..=b'9' => Ok(digit - b'0'),
        b'a'..=b'f' => Ok(digit - b'a' + 10),
        b'A'..=b'F' => Ok(digit - b'A' + 10),
        _ => Err(ParseColorError),
    }
}

impl core::str::FromStr for DynColors {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.strip_prefix('#') {
            // Work with the raw bytes to avoid having to think about multibyte
            // UTF-8 (outside of rejecting it in decode_hex_byte).
            match *hex.as_bytes() {
                [r_high, r_low, g_high, g_low, b_high, b_low] => Ok(Self::Rgb(
                    decode_hex_byte(r_high, r_low)?,
                    decode_hex_byte(g_high, g_low)?,
                    decode_hex_byte(b_high, b_low)?,
                )),
                // TODO: 3-digit #rgb isn't supported yet.
                _ => Err(ParseColorError),
            }
        } else {
            let ansi = match s {
                "black" => AnsiColors::Black,
                "red" => AnsiColors::Red,
                "green" => AnsiColors::Green,
                "yellow" => AnsiColors::Yellow,
                "blue" => AnsiColors::Blue,
                "magenta" | "purple" => AnsiColors::Magenta,
                "cyan" => AnsiColors::Cyan,
                "white" => AnsiColors::White,
                "bright black" => AnsiColors::BrightBlack,
                "bright red" => AnsiColors::BrightRed,
                "bright green" => AnsiColors::BrightGreen,
                "bright yellow" => AnsiColors::BrightYellow,
                "bright blue" => AnsiColors::BrightBlue,
                "bright magenta" => AnsiColors::BrightMagenta,
                "bright cyan" => AnsiColors::BrightCyan,
                "bright white" => AnsiColors::BrightWhite,
                _ => return Err(ParseColorError),
            };

            Ok(Self::Ansi(ansi))
        }
    }
}
