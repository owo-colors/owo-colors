
/*  XXX
    You know, naming is hard :)
    Fell free to re-name or re-locate this as you see fit (or just tell me, and I wil do it)
    The name this module "kind of" clashes with the `styles` module, but I don't know a better name.
*/

use core::fmt;
use crate::{Color, colors, DynColor, DynStylesColor, OwoColorize};

const CLOSINGS: [&str; 12] = [
    "",
    "\x1b[0m",
    "\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m",
    "\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m\x1b[0m",
];

// XXX Should we better use the plural `Effects`? I prefer `Effect` but I think you used the plural versions so far (for example `DynColors`)
pub enum Effect {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    BlinkFast,
    Reversed,
    Hidden,
    Strikethrough,

    // XXX would a `unset_effects` method that accepts the above variants and removing the variants below (and the "unset" functionality in `effects`) be a nicer API?
    BoldOff,
    DimmedOff,
    ItalicOff,
    UnderlineOff,
    BlinkOff,
    BlinkFastOff,
    ReversedOff,
    HiddenOff,
    StrikethroughOff,
}

macro_rules! color_methods {
    ($(
        #[$fg_meta:meta] #[$bg_meta:meta] $color:ident $fg_method:ident $bg_method:ident
    ),* $(,)?) => {
        $(
            #[$fg_meta]
            #[inline(always)]
            pub fn $fg_method(mut self) -> Self {
                self.fg = Some(DynStylesColor::Ansi(colors::$color::ANSI_FG));
                self
            }

            #[$fg_meta]
            #[inline(always)]
            pub fn $bg_method(mut self) -> Self {
                self.bg = Some(DynStylesColor::Ansi(colors::$color::ANSI_BG));
                self
            }
         )*
    };
}

macro_rules! style_methods {
    ($(#[$meta:meta] $name:ident $ty:ident),* $(,)?) => {
        $(
            #[$meta]
            #[inline(always)]
            pub fn $name(mut self) -> Self {
                self.$name = true;
                self
            }
         )*
    };
}

pub struct Styled<T> {
    target: T,
    style: Style,
}

impl<T> Styled<T> {
    fn new(target: T, style: Style) -> Self {
        Self {target, style}
    }
}

// XXX Is it correct here to derive `Copy` (because it only contains `&static str` and `bool` values), or would it still make sense to work with references, instead of `copy`ing `Style`?
#[derive(Default, Copy, Clone)]
pub struct Style {
    fg: Option<DynStylesColor>,
    bg: Option<DynStylesColor>,
    bold: bool,
    dimmed: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    blink_fast: bool,
    reversed: bool,
    hidden: bool,
    strikethrough: bool,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_to<T>(&self, target: T) -> Styled<T> {
        Styled::new(target, *self)
    }

    /// Set the foreground color generically
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, colors::*};
    ///
    /// println!("{}", "red foreground".fg::<Red>());
    /// ```
    #[inline(always)]
    pub fn fg<C: Color>(mut self) -> Self {
        self.fg = Some(DynStylesColor::Ansi(C::ANSI_FG));
        self
    }

    /// Set the background color generically.
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, colors::*};
    ///
    /// println!("{}", "black background".bg::<Black>());
    /// ```
    #[inline(always)]
    pub fn bg<C: Color>(mut self) -> Self {
        self.fg = Some(DynStylesColor::Ansi(C::ANSI_BG));
        self
    }

    color_methods! {
        /// Change the foreground color to black 
        /// Change the background color to black
        Black    black    on_black,
        /// Change the foreground color to red
        /// Change the background color to red
        Red      red      on_red,
        /// Change the foreground color to green
        /// Change the background color to green
        Green    green    on_green,
        /// Change the foreground color to yellow
        /// Change the background color to yellow
        Yellow   yellow   on_yellow,
        /// Change the foreground color to blue
        /// Change the background color to blue
        Blue     blue     on_blue,
        /// Change the foreground color to magenta
        /// Change the background color to magenta
        Magenta  magenta  on_magenta,
        /// Change the foreground color to purple
        /// Change the background color to purple
        Magenta  purple   on_purple,
        /// Change the foreground color to cyan
        /// Change the background color to cyan
        Cyan     cyan     on_cyan,
        /// Change the foreground color to white
        /// Change the background color to white
        White    white    on_white,

        /// Change the foreground color to bright black 
        /// Change the background color to bright black
        BrightBlack    bright_black    on_bright_black,
        /// Change the foreground color to bright red
        /// Change the background color to bright red
        BrightRed      bright_red      on_bright_red,
        /// Change the foreground color to bright green
        /// Change the background color to bright green
        BrightGreen    bright_green    on_bright_green,
        /// Change the foreground color to bright yellow
        /// Change the background color to bright yellow
        BrightYellow   bright_yellow   on_bright_yellow,
        /// Change the foreground color to bright blue
        /// Change the background color to bright blue
        BrightBlue     bright_blue     on_bright_blue,
        /// Change the foreground color to bright magenta
        /// Change the background color to bright magenta
        BrightMagenta  bright_magenta  on_bright_magenta,
        /// Change the foreground color to bright purple
        /// Change the background color to bright purple
        BrightMagenta  bright_purple   on_bright_purple,
        /// Change the foreground color to bright cyan
        /// Change the background color to bright cyan
        BrightCyan     bright_cyan     on_bright_cyan,
        /// Change the foreground color to bright white
        /// Change the background color to bright white
        BrightWhite    bright_white    on_bright_white,
    }

    style_methods! {
        /// Make the text bold
        bold BOLD,
        /// Make the text dim
        dimmed DIMMED,
        /// Make the text italicized
        italic ITALIC,
        /// Make the text italicized
        underline UNDERLINED,
        /// Make the text blink
        blink BLINK,
        /// Make the text blink (but fast!)
        blink_fast BLINK_FAST,
        /// Swap the foreground and background colors
        reversed REVERSED,
        /// Hide the text
        hidden HIDDEN,
        /// Cross out the text
        strikethrough STRIKETHROUGH,
    }

    #[inline(always)]
    pub fn effects(mut self, effects: &[Effect]) -> Self {
        use Effect::*;
        for e in effects {
            match e {
                Bold                => self.bold           = true,
                Dimmed              => self.dimmed         = true,
                Italic              => self.italic         = true,
                Underline           => self.underline      = true,
                Blink               => self.blink          = true,
                BlinkFast           => self.blink_fast     = true,
                Reversed            => self.reversed       = true,
                Hidden              => self.hidden         = true,
                Strikethrough       => self.strikethrough  = true,

                BoldOff             => self.bold           = false,
                DimmedOff           => self.dimmed         = false,
                ItalicOff           => self.italic         = false,
                UnderlineOff        => self.underline      = false,
                BlinkOff            => self.blink          = false,
                BlinkFastOff        => self.blink_fast     = false,
                ReversedOff         => self.reversed       = false,
                HiddenOff           => self.hidden         = false,
                StrikethroughOff    => self.strikethrough  = false,
            }
        }
        self
    }

    /// Set the foreground color at runtime. Only use if you do not know which color will be used at
    /// compile-time. If the color is constant, use either [`OwoColorize::fg`](OwoColorize::fg) or
    /// a color-specific method, such as [`OwoColorize::green`](OwoColorize::green),
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, AnsiColors};
    ///
    /// fn main() {
    ///     println!("{}", "green".color(AnsiColors::Green));
    /// }
    /// ```
    #[inline(always)]
    pub fn color<Color: DynColor>(mut self, color: Color) -> Self {
        self.fg = Some(color.get_fg());
        self
    }

    /// Set the background color at runtime. Only use if you do not know what color to use at
    /// compile-time. If the color is constant, use either [`OwoColorize::bg`](OwoColorize::bg) or
    /// a color-specific method, such as [`OwoColorize::on_yellow`](OwoColorize::on_yellow),
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, AnsiColors};
    ///
    /// fn main() {
    ///     println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));
    /// }
    /// ```
    #[inline(always)]
    pub fn on_color<Color: DynColor>(mut self, color: Color) -> Self {
        self.bg = Some(color.get_bg());
        self
    }

    // XXX It seems, for the next two methods `unsafe` Rust is needed, but I don't know `unsafe` at the moment. Can you implement this?

    /*
    /// Set the foreground color to a specific RGB value.
    ///
    /// **Requires**: nightly and the `custom` feature.
    ///
    /// If nightly is not preferable for you, use [`OwoColorize::truecolor`](OwoColorize::truecolor)
    #[cfg(feature = "custom")]
    pub fn fg_rgb<'a, const R: u8, const G: u8, const B: u8>(
        &'a self,
    ) -> FgColorDisplay<'a, colors::CustomColor<R, G, B>, Self> {
        FgColorDisplay(self, PhantomData)
    }

    /// Set the background color to a specific RGB value.
    ///
    /// **Requires**: nightly and the `custom` feature.
    ///
    /// If nightly is not preferable for you, use [`OwoColorize::on_truecolor`](OwoColorize::on_truecolor)
    #[cfg(feature = "custom")]
    pub fn bg_rgb<'a, const R: u8, const G: u8, const B: u8>(
        &'a self,
    ) -> BgColorDisplay<'a, colors::CustomColor<R, G, B>, Self> {
        BgColorDisplay(self, PhantomData)
    }
    */

    /// Sets the foreground color to an RGB value.
    #[inline(always)]
    pub fn truecolor(mut self, r: u8, g: u8, b: u8) -> Self {
        self.fg = Some(DynStylesColor::Rgb(r, g, b));
        self
    }

    /// Sets the background color to an RGB value.
    #[inline(always)]
    pub fn on_truecolor(mut self, r: u8, g: u8, b: u8) -> Self {
        self.bg = Some(DynStylesColor::Rgb(r, g, b));
        self
    }

}

macro_rules! text_effect_fmt {
    ($closings:ident, $style:ident, $formatter:ident, $(($attr:ident, $value:literal)),* $(,)?) => {
        $(if $style.$attr {
            $formatter.write_str($value)?;
            $closings += 1;
        })+
    }
}

macro_rules! impl_fmt {
    ($($trait:path),* $(,)?) => {
        $(
            impl<T: $trait> $trait for Styled<T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let s = &self.style;
                    let mut closings = 0;

                    // XXX Is my assumption correct, that it doesn't matter in what order ANSI escape sequences are applied? Because otherwise we need to store the order in which the user applies them (which would make the code more complex)

                    if let Some(fg) = s.fg {
                        match fg {
                            DynStylesColor::Ansi(ansi) => f.write_str(ansi)?,
                            DynStylesColor::Rgb(r, g, b) => 
                                f.write_fmt(format_args!("\x1b[38;2;{};{};{}m", r, g, b))?,
                        }
                        closings += 1;
                    }

                    if let Some(bg) = s.bg {
                        match bg {
                            DynStylesColor::Ansi(ansi) => f.write_str(ansi)?,
                            DynStylesColor::Rgb(r, g, b) => 
                                f.write_fmt(format_args!("\x1b[48;2;{};{};{}m", r, g, b))?,
                        }
                        closings += 1;
                    }

                    text_effect_fmt!{
                        closings, s, f,
                        (bold,          "\x1b[1m"),
                        (dimmed,        "\x1b[2m"),
                        (italic,        "\x1b[3m"),
                        (underline,     "\x1b[4m"),
                        (blink,         "\x1b[5m"),
                        (blink_fast,    "\x1b[6m"),
                        (reversed,      "\x1b[7m"),
                        (hidden,        "\x1b[8m"),
                        (strikethrough, "\x1b[9m")
                    }

                    <T as $trait>::fmt(&self.target, f)?;

                    if closings > 0 {
                        f.write_str(CLOSINGS[closings])?;
                    }

                    /* XXX
                    Let me know if the above optimization is silly, and we should use a for loop:

                    for _ in 0..closing {
                        f.write_str("\x1b[0m")
                    }
                    */

                    Ok(())
                }
            }
        )*
    };
}

impl_fmt! {
    fmt::Display,
    fmt::Debug,
    fmt::UpperHex,
    fmt::LowerHex,
    fmt::Binary,
    fmt::UpperExp,
    fmt::LowerExp,
    fmt::Octal,
    fmt::Pointer,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AnsiColors;

    #[test]
    fn test_it() {
        let style = Style::new()
            .bright_white()
            .on_blue()
            .bold()
            .dimmed()
            .italic()
            .underline()
            .blink()
            //.blink_fast()
            //.reversed()
            //.hidden()
            .strikethrough()
        ;
        let s = style.apply_to("TEST");
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "TEST");
    }

    #[test]
    fn test_effects() {
        use Effect::*;
        let style = Style::new().effects(&[
            Strikethrough,
            Underline,
        ]);

        let s = style.apply_to("TEST");
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "TEST");
    }

    #[test]
    fn test_color() {
        let style = Style::new()
            .color(AnsiColors::White)
            .on_color(AnsiColors::Black);

        let s = style.apply_to("TEST");
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "TEST");
    }

    #[test]
    fn test_truecolor() {
        let style = Style::new()
            .truecolor(255, 255, 255)
            .on_truecolor(0, 0, 0);

        let s = style.apply_to("TEST");
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "TEST");
    }

    #[test]
    fn test_string_reference() {
        let style = Style::new()
            .truecolor(255, 255, 255)
            .on_truecolor(0, 0, 0);

        let string = String::from("TEST");
        let s = style.apply_to(&string);
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "TEST");
    }

    #[test]
    fn test_owocolorize() {
        let style = Style::new()
            .bright_white()
            .on_blue();

        let s = "TEST".style(&style);
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "TEST");
    }
}