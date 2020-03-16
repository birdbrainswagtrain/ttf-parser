/*!
A high-level, safe, zero-allocation TrueType font parser.

Can be used as Rust and as C library.

## Features

- A high-level API, for people who doesn't know how TrueType works internally.
  Basically, no direct access to font tables.
- A [C API](./c-api).
- Zero heap allocations.
- Zero unsafe.
- Zero required dependencies. Logging is enabled by default.
- `no_std` compatible.
- Fast. Set the *Performance* section.
- Stateless. No mutable methods.
- Simple and maintainable code (no magic numbers).

## Safety

- The library must not panic. Any panic considered as a critical bug and should be reported.
- The library forbids the unsafe code.
- No heap allocations, so crash due to OOM is not possible.
- All recursive methods have a depth limit.
- Technically, should use less than 64KiB of stack in worst case scenario.
- Most of arithmetic operations are checked.

## Supported TrueType features

- (`cmap`) Character to glyph index mapping using [glyph_index()] method.
  <br/>All subtable formats except Mixed Coverage (8) are supported.
- (`cmap`) Character variation to glyph index mapping using [glyph_variation_index()] method.
- (`glyf`) Glyph outlining using [outline_glyph()] method.
- (`hmtx`) Retrieving glyph's horizontal metrics using [glyph_hor_advance()] and [glyph_hor_side_bearing()] methods.
- (`vmtx`) Retrieving glyph's vertical metrics using [glyph_ver_advance()] and [glyph_ver_side_bearing()] methods.
- (`kern`) Retrieving glyphs pair kerning using [glyphs_kerning()] method.
- (`maxp`) Retrieving total number of glyphs using [number_of_glyphs()] method.
- (`name`) Listing all name records using [names()] method.
- (`name`) Retrieving font's family name using [family_name()] method.
- (`name`) Retrieving font's PostScript name using [post_script_name()] method.
- (`post`) Retrieving font's underline metrics using [underline_metrics()] method.
- (`post`) Retrieving glyph's name using [glyph_name()] method.
- (`head`) Retrieving font's units per EM value using [units_per_em()] method.
- (`hhea`) Retrieving generic font info using: [ascender()], [descender()], [height()]
  and [line_gap()] methods.

[glyph_index()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_index
[glyph_variation_index()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_variation_index
[outline_glyph()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.outline_glyph
[glyph_hor_advance()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_hor_advance
[glyph_hor_side_bearing()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_hor_side_bearing
[glyph_ver_advance()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_ver_advance
[glyph_ver_side_bearing()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_ver_side_bearing
[glyphs_kerning()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyphs_kerning
[number_of_glyphs()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.number_of_glyphs
[names()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.names
[family_name()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.family_name
[post_script_name()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.post_script_name
[underline_metrics()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.underline_metrics
[glyph_name()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_name
[units_per_em()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.units_per_em
[ascender()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.ascender
[descender()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.descender
[height()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.height
[line_gap()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.line_gap

## Supported OpenType features

- (`CFF `) Glyph outlining using [outline_glyph()] method.
- (`CFF2`) Variable glyph outlining using [outline_variable_glyph()] method.
- (`gvar`) Variable glyph outlining using [outline_variable_glyph()] method.
- (`OS/2`) Retrieving font's kind using [is_regular()], [is_italic()],
  [is_bold()] and [is_oblique()] methods.
- (`OS/2`) Retrieving font's weight using [weight()] method.
- (`OS/2`) Retrieving font's width using [width()] method.
- (`OS/2`) Retrieving font's X height using [x_height()] method.
- (`OS/2`) Retrieving font's strikeout metrics using [strikeout_metrics()] method.
- (`OS/2`) Retrieving font's subscript metrics using [subscript_metrics()] method.
- (`OS/2`) Retrieving font's superscript metrics using [superscript_metrics()] method.
- (`GDEF`) Retrieving glyph's class using [glyph_class()] method.
- (`GDEF`) Retrieving glyph's mark attachment class using [glyph_mark_attachment_class()] method.
- (`GDEF`) Checking that glyph is a mark using [is_mark_glyph()] method.
- (`avar`) Variation coordinates normalization using [map_variation_coordinates()] method.
- (`fvar`) Variation axis parsing using [variation_axes()] method.
- (`VORG`) Retrieving glyph's vertical origin using [glyph_y_origin()] method.
- (`MVAR`) Retrieving font's metrics variation using [metrics_variation()] method.
- (`HVAR`) Retrieving glyph's variation offset for horizontal advance using [glyph_hor_advance_variation()] method.
- (`HVAR`) Retrieving glyph's variation offset for horizontal side bearing using [glyph_hor_side_bearing_variation()] method.
- (`VVAR`) Retrieving glyph's variation offset for vertical advance using [glyph_ver_advance_variation()] method.
- (`VVAR`) Retrieving glyph's variation offset for vertical side bearing using [glyph_ver_side_bearing_variation()] method.

[is_regular()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.is_regular
[is_italic()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.is_italic
[is_bold()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.is_bold
[is_oblique()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.is_oblique
[weight()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.weight
[width()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.width
[x_height()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.x_height
[strikeout_metrics()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.strikeout_metrics
[subscript_metrics()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.subscript_metrics
[superscript_metrics()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.superscript_metrics
[glyph_class()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_class
[glyph_mark_attachment_class()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_mark_attachment_class
[is_mark_glyph()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.is_mark_glyph
[outline_variable_glyph()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.outline_variable_glyph
[map_variation_coordinates()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.map_variation_coordinates
[variation_axes()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.variation_axis
[glyph_y_origin()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_y_origin
[metrics_variation()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.metrics_variation
[glyph_hor_advance_variation()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_hor_advance_variation
[glyph_hor_side_bearing_variation()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_hor_side_bearing_variation
[glyph_ver_advance_variation()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_ver_advance_variation
[glyph_ver_side_bearing_variation()]: https://docs.rs/ttf-parser/0.4.0/ttf_parser/struct.Font.html#method.glyph_ver_side_bearing_variation

## Error handling

`ttf-parser` is designed to parse well-formed fonts, so it does not have an `Error` enum.
It doesn't mean that it will crash or panic on malformed fonts, only that the
error handling will boil down to `Option::None`. So you will not get a detailed cause of an error.
By doing so we can simplify an API quite a lot since otherwise, we will have to use
`Result<Option<T>, Error>`.

Some methods may print warnings, when the `logging` feature is enabled.

## Methods' computational complexity

TrueType fonts designed for fast querying, so most of the methods are very fast.
The main exception is glyph outlining. Glyphs can be stored using two different methods:
using [Glyph Data](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf) format
and [Compact Font Format](http://wwwimages.adobe.com/content/dam/Adobe/en/devnet/font/pdfs/5176.CFF.pdf) (pdf).
The first one is fairly simple which makes it faster to process.
The second one is basically a tiny language with a stack-based VM, which makes it way harder to process.

The [benchmark](./benches/outline/) tests how long it takes to outline all glyphs in the font.

```text
ttf_parser_outline_glyf     853957 ns
freetype_outline_glyf      1250442 ns

ttf_parser_outline_gvar     984885 ns
freetype_outline_gvar      1443903 ns

ttf_parser_outline_cff     1371693 ns
freetype_outline_cff       5856448 ns
```

**Note:** FreeType is surprisingly slow, so I'm worried that I've messed something up.

And here are some methods benchmarks:

```text
test outline_glyph_276_from_cff  ... bench:         877 ns/iter (+/- 265)
test outline_glyph_276_from_cff2 ... bench:         779 ns/iter (+/- 122)
test from_data_otf_cff2          ... bench:         675 ns/iter (+/- 8)
test outline_glyph_276_from_glyf ... bench:         623 ns/iter (+/- 77)
test from_data_otf_cff           ... bench:         562 ns/iter (+/- 7)
test outline_glyph_8_from_cff2   ... bench:         531 ns/iter (+/- 118)
test outline_glyph_8_from_cff    ... bench:         322 ns/iter (+/- 7)
test from_data_ttf               ... bench:         313 ns/iter (+/- 4)
test outline_glyph_8_from_glyf   ... bench:         285 ns/iter (+/- 10)
test glyph_name_276              ... bench:         214 ns/iter (+/- 3)
test family_name                 ... bench:         170 ns/iter (+/- 12)
test glyph_index_u41             ... bench:          16 ns/iter (+/- 0)
test glyph_name_8                ... bench:           1 ns/iter (+/- 0)
test underline_metrics           ... bench:         0.5 ns/iter (+/- 0)
test units_per_em                ... bench:         0.5 ns/iter (+/- 0)
test strikeout_metrics           ... bench:         0.5 ns/iter (+/- 0)
test ascender                    ... bench:         0.2 ns/iter (+/- 0)
test hor_advance                 ... bench:         0.2 ns/iter (+/- 0)
test hor_side_bearing            ... bench:         0.2 ns/iter (+/- 0)
test subscript_metrics           ... bench:         0.2 ns/iter (+/- 0)
test width                       ... bench:         0.2 ns/iter (+/- 0)
test x_height                    ... bench:         0.2 ns/iter (+/- 0)
```

`family_name` is expensive, because it allocates a `String` and the original data
is stored as UTF-16 BE.

`glyph_name_8` is faster that `glyph_name_276`, because for glyph indexes lower than 258
we are using predefined names, so no parsing is involved.
*/

#![doc(html_root_url = "https://docs.rs/ttf-parser/0.4.0")]

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(feature = "std")]
use std::string::String;

use core::fmt;
use core::num::NonZeroU16;

macro_rules! try_opt_or {
    ($value:expr, $ret:expr) => {
        match $value {
            Some(v) => v,
            None => return $ret,
        }
    };
}

#[cfg(feature = "logging")]
macro_rules! warn {
    ($($arg:tt)+) => (
        log::log!(log::Level::Warn, $($arg)+);
    )
}

#[cfg(not(feature = "logging"))]
macro_rules! warn {
    ($($arg:tt)+) => () // do nothing
}

mod avar;
mod cff2;
mod cff;
mod cmap;
mod fvar;
mod gdef;
mod ggg;
mod glyf;
mod gpos;
mod gsub;
mod gvar;
mod hmtx;
mod hvar;
mod kern;
mod loca;
mod maxp;
mod mvar;
mod name;
mod os2;
mod parser;
mod post;
mod raw;
mod vorg;
mod var_store;

#[cfg(feature = "std")]
mod writer;

use parser::{Stream, SafeStream, Offset};
pub use fvar::{VariationAxes, VariationAxis};
pub use gdef::GlyphClass;
pub use ggg::*;
pub use gpos::PositioningTable;
pub use gsub::SubstitutionTable;
pub use name::*;
pub use os2::*;
pub use parser::{FromData, ArraySize, LazyArray, LazyArray16, LazyArray32, LazyArrayIter};


/// A type-safe wrapper for glyph ID.
#[repr(C)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct GlyphId(pub u16);

impl FromData for GlyphId {
    #[inline]
    fn parse(data: &[u8]) -> Self {
        let mut s = SafeStream::new(data);
        GlyphId(s.read())
    }
}

impl Default for GlyphId {
    fn default() -> Self {
        GlyphId(0)
    }
}


/// A 4-byte tag.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(pub u32);

impl Tag {
    /// Creates a `Tag` from bytes.
    pub const fn from_bytes(bytes: &[u8; 4]) -> Self {
        Tag(((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) |
            ((bytes[2] as u32) << 8) | (bytes[3] as u32))
    }

    /// Creates a `Tag` from bytes.
    ///
    /// In case of empty data will return `Tag` set to 0.
    ///
    /// When `bytes` are shorter than 4, will set missing bytes to ` `.
    ///
    /// Data after first 4 bytes is ignored.
    pub fn from_bytes_lossy(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Tag::from_bytes(&[0, 0, 0, 0]);
        }

        let mut iter = bytes.iter().cloned().chain(core::iter::repeat(b' '));
        Tag::from_bytes(&[
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ])
    }

    /// Returns tag as 4-element byte array.
    pub const fn to_bytes(self) -> [u8; 4] {
        [
            (self.0 >> 24 & 0xff) as u8,
            (self.0 >> 16 & 0xff) as u8,
            (self.0 >> 8 & 0xff) as u8,
            (self.0 >> 0 & 0xff) as u8,
        ]
    }

    /// Returns tag as 4-element byte array.
    pub const fn to_chars(self) -> [char; 4] {
        [
            (self.0 >> 24 & 0xff) as u8 as char,
            (self.0 >> 16 & 0xff) as u8 as char,
            (self.0 >> 8 & 0xff) as u8 as char,
            (self.0 >> 0 & 0xff) as u8 as char,
        ]
    }

    /// Returns tag for a default script.
    pub const fn default_script() -> Self {
        Tag::from_bytes(b"DFLT")
    }

    /// Returns tag for a default language.
    pub const fn default_language() -> Self {
        Tag::from_bytes(b"dflt")
    }

    /// Checks if tag is null / `[0, 0, 0, 0]`.
    pub const fn is_null(&self) -> bool {
        self.0 == 0
    }

    /// Returns tag value as `u32` number.
    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    /// Converts tag to lowercase.
    pub fn to_lowercase(&self) -> Self {
        let b = self.to_bytes();
        Tag::from_bytes(&[
            b[0].to_ascii_lowercase(),
            b[1].to_ascii_lowercase(),
            b[2].to_ascii_lowercase(),
            b[3].to_ascii_lowercase(),
        ])
    }

    /// Converts tag to uppercase.
    pub fn to_uppercase(&self) -> Self {
        let b = self.to_bytes();
        Tag::from_bytes(&[
            b[0].to_ascii_uppercase(),
            b[1].to_ascii_uppercase(),
            b[2].to_ascii_uppercase(),
            b[3].to_ascii_uppercase(),
        ])
    }
}

impl core::fmt::Debug for Tag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Tag({})", self)
    }
}

impl core::fmt::Display for Tag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let b = self.to_chars();
        write!(
            f,
            "{}{}{}{}",
            b.get(0).unwrap_or(&' '),
            b.get(1).unwrap_or(&' '),
            b.get(2).unwrap_or(&' '),
            b.get(3).unwrap_or(&' ')
        )
    }
}

impl FromData for Tag {
    #[inline]
    fn parse(data: &[u8]) -> Self {
        Tag(u32::parse(data))
    }
}



/// A line metrics.
///
/// Used for underline and strikeout.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LineMetrics {
    /// Line position.
    pub position: i16,

    /// Line thickness.
    pub thickness: i16,
}


/// A rectangle.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Rect {
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
}


#[derive(Clone, Copy, Debug)]
pub(crate) struct BBox {
    x_min: f32,
    y_min: f32,
    x_max: f32,
    y_max: f32,
}

impl BBox {
    #[inline]
    fn new() -> Self {
        BBox {
            x_min: core::f32::MAX,
            y_min: core::f32::MAX,
            x_max: core::f32::MIN,
            y_max: core::f32::MIN,
        }
    }

    #[inline]
    fn is_default(&self) -> bool {
        self.x_min == core::f32::MAX &&
        self.y_min == core::f32::MAX &&
        self.x_max == core::f32::MIN &&
        self.y_max == core::f32::MIN
    }

    #[inline]
    fn extend_by(&mut self, x: f32, y: f32) {
        self.x_min = self.x_min.min(x);
        self.y_min = self.y_min.min(y);
        self.x_max = self.x_max.max(x);
        self.y_max = self.y_max.max(y);
    }

    #[inline]
    fn to_rect(&self) -> Option<Rect> {
        #[inline]
        fn try_f32_to_i16(n: f32) -> Option<i16> {
            // There is no i16::try_from(f32) so we have to write one ourselves.
            if n >= core::i16::MIN as f32 && n <= core::i16::MAX as f32 {
                Some(n as i16)
            } else {
                None
            }
        }

        Some(Rect {
            x_min: try_f32_to_i16(self.x_min)?,
            y_min: try_f32_to_i16(self.y_min)?,
            x_max: try_f32_to_i16(self.x_max)?,
            y_max: try_f32_to_i16(self.y_max)?,
        })
    }
}


/// A trait for glyph outline construction.
pub trait OutlineBuilder {
    /// Appends a MoveTo segment.
    ///
    /// Start of a contour.
    fn move_to(&mut self, x: f32, y: f32);

    /// Appends a LineTo segment.
    fn line_to(&mut self, x: f32, y: f32);

    /// Appends a QuadTo segment.
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32);

    /// Appends a CurveTo segment.
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32);

    /// Appends a ClosePath segment.
    ///
    /// End of a contour.
    fn close(&mut self);
}


struct DummyOutline;
impl OutlineBuilder for DummyOutline {
    fn move_to(&mut self, _: f32, _: f32) {}
    fn line_to(&mut self, _: f32, _: f32) {}
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {}
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {}
    fn close(&mut self) {}
}


#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IndexToLocationFormat {
    Short,
    Long,
}


/// A table name.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum TableName {
    AxisVariations = 0,
    CharacterToGlyphIndexMapping,
    CompactFontFormat,
    CompactFontFormat2,
    FontVariations,
    GlyphData,
    GlyphDefinition,
    GlyphPositioning,
    GlyphSubstitution,
    GlyphVariations,
    Header,
    HorizontalHeader,
    HorizontalMetrics,
    HorizontalMetricsVariations,
    IndexToLocation,
    Kerning,
    MaximumProfile,
    MetricsVariations,
    Naming,
    PostScript,
    VerticalHeader,
    VerticalMetrics,
    VerticalMetricsVariations,
    VerticalOrigin,
    WindowsMetrics,
}


/// A font data handle.
#[derive(Clone)]
pub struct Font<'a> {
    avar: Option<&'a [u8]>,
    cff_: Option<cff::Metadata<'a>>,
    cff2: Option<cff2::Metadata<'a>>,
    cmap: Option<cmap::Table<'a>>,
    fvar: Option<fvar::Table<'a>>,
    gdef: Option<gdef::Table<'a>>,
    glyf: Option<&'a [u8]>,
    gpos: Option<ggg::GsubGposTable<'a>>,
    gsub: Option<ggg::GsubGposTable<'a>>,
    gvar: Option<gvar::Table<'a>>,
    head: raw::head::Table<'a>,
    hhea: raw::hhea::Table<'a>,
    hmtx: Option<hmtx::Table<'a>>,
    hvar: Option<&'a [u8]>,
    kern: Option<&'a [u8]>,
    loca: Option<loca::Table<'a>>,
    mvar: Option<&'a [u8]>,
    name: Option<name::Names<'a>>,
    os_2: Option<os2::Table<'a>>,
    post: Option<post::Table<'a>>,
    vhea: Option<raw::vhea::Table<'a>>,
    vmtx: Option<hmtx::Table<'a>>,
    vorg: Option<vorg::Table<'a>>,
    vvar: Option<&'a [u8]>,
    number_of_glyphs: NonZeroU16,
}

impl<'a> Font<'a> {
    /// Creates a `Font` object from a raw data.
    ///
    /// You can set `index` for font collections.
    /// For simple `ttf` fonts set `index` to 0.
    ///
    /// This method will do some parsing and sanitization, so it's a bit expensive.
    ///
    /// Required tables: `head`, `hhea` and `maxp`.
    ///
    /// If an optional table has an invalid data it will be skipped.
    pub fn from_data(data: &'a [u8], index: u32) -> Option<Self> {
        let table_data = if let Some(n) = fonts_in_collection(data) {
            if index < n {
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#ttc-header
                const OFFSET_32_SIZE: usize = 4;
                let offset = raw::TTCHeader::SIZE + OFFSET_32_SIZE * index as usize;
                let font_offset: u32 = Stream::read_at(data, offset)?;
                data.get(font_offset as usize .. data.len())?
            } else {
                return None;
            }
        } else {
            data
        };

        // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#organization-of-an-opentype-font
        const OFFSET_TABLE_SIZE: usize = 12;
        if data.len() < OFFSET_TABLE_SIZE {
            return None;
        }

        // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#organization-of-an-opentype-font
        const SFNT_VERSION_TRUE_TYPE: u32 = 0x00010000;
        const SFNT_VERSION_OPEN_TYPE: u32 = 0x4F54544F;

        let mut s = Stream::new(table_data);

        let sfnt_version: u32 = s.read()?;
        if sfnt_version != SFNT_VERSION_TRUE_TYPE && sfnt_version != SFNT_VERSION_OPEN_TYPE {
            return None;
        }

        let num_tables: u16 = s.read()?;
        s.advance(6u32); // searchRange (u16) + entrySelector (u16) + rangeShift (u16)
        let tables = s.read_array::<raw::TableRecord, u16>(num_tables)?;

        let mut cff_ = None;
        let mut cff2 = None;
        let mut gdef = None;
        let mut gpos = None;
        let mut gsub = None;
        let mut hvar = None;
        let mut gvar = None;
        let mut mvar = None;
        let mut os_2 = None;
        let mut vorg = None;
        let mut vvar = None;
        let mut avar = None;
        let mut cmap = None;
        let mut fvar = None;
        let mut glyf = None;
        let mut head = None;
        let mut hhea = None;
        let mut hmtx = None;
        let mut kern = None;
        let mut loca = None;
        let mut maxp = None;
        let mut name = None;
        let mut post = None;
        let mut vhea = None;
        let mut vmtx = None;
        for table in tables {
            let offset = table.offset().to_usize();
            let length = table.length() as usize;
            let range = offset..(offset + length);

            // It's way faster to compare `[u8; 4]` with `&[u8]`
            // rather than `&[u8]` with `&[u8]`.
            match &table.table_tag().to_bytes() {
                b"CFF " => cff_ = data.get(range).and_then(|data| cff::parse_metadata(data)),
                b"CFF2" => cff2 = data.get(range).and_then(|data| cff2::parse_metadata(data)),
                b"GDEF" => gdef = data.get(range).and_then(|data| gdef::Table::parse(data)),
                b"GPOS" => gpos = data.get(range).and_then(|data| ggg::GsubGposTable::parse(data)),
                b"GSUB" => gsub = data.get(range).and_then(|data| ggg::GsubGposTable::parse(data)),
                b"HVAR" => hvar = data.get(range),
                b"MVAR" => mvar = data.get(range),
                b"OS/2" => os_2 = data.get(range).and_then(|data| os2::Table::parse(data)),
                b"VORG" => vorg = data.get(range).and_then(|data| vorg::Table::parse(data)),
                b"VVAR" => vvar = data.get(range),
                b"avar" => avar = data.get(range),
                b"cmap" => cmap = data.get(range).and_then(|data| cmap::Table::parse(data)),
                b"fvar" => fvar = data.get(range).and_then(|data| fvar::Table::parse(data)),
                b"glyf" => glyf = data.get(range),
                b"gvar" => gvar = data.get(range).and_then(|data| gvar::Table::parse(data)),
                b"head" => head = data.get(range).and_then(|data| raw::head::Table::parse(data)),
                b"hhea" => hhea = data.get(range).and_then(|data| raw::hhea::Table::parse(data)),
                b"hmtx" => hmtx = data.get(range),
                b"kern" => kern = data.get(range),
                b"loca" => loca = data.get(range),
                b"maxp" => maxp = data.get(range).and_then(|data| maxp::parse(data)),
                b"name" => name = data.get(range).and_then(|data| name::parse(data)),
                b"post" => post = data.get(range).and_then(|data| post::Table::parse(data)),
                b"vhea" => vhea = data.get(range).and_then(|data| raw::vhea::Table::parse(data)),
                b"vmtx" => vmtx = data.get(range),
                _ => {}
            }
        }

        // Check for mandatory tables.
        let head = head?;
        let hhea = hhea?;
        let maxp = maxp?;
        let number_of_glyphs = maxp.number_of_glyphs;

        let mut font = Font {
            avar,
            cff_,
            cff2,
            cmap,
            fvar,
            gdef,
            glyf,
            gvar,
            gpos,
            gsub,
            head,
            hhea,
            hmtx: None,
            hvar,
            kern,
            loca: None,
            mvar,
            name,
            os_2,
            post,
            vhea,
            vmtx: None,
            vorg,
            vvar,
            number_of_glyphs,
        };

        if let Some(data) = hmtx {
            if let Some(number_of_h_metrics) = font.hhea.number_of_h_metrics() {
                font.hmtx = hmtx::Table::parse(data, number_of_h_metrics, font.number_of_glyphs);
            }
        }

        if let (Some(vhea), Some(data)) = (font.vhea, vmtx) {
            if let Some(number_of_v_metrics) = vhea.num_of_long_ver_metrics() {
                font.vmtx = hmtx::Table::parse(data, number_of_v_metrics, font.number_of_glyphs);
            }
        }

        if let Some(data) = loca {
            if let Some(format) = font.index_to_location_format() {
                font.loca = loca::Table::parse(data, font.number_of_glyphs, format);
            }
        }

        Some(font)
    }

    /// Checks that font has a specified table.
    ///
    /// Will return `true` only for tables that were successfully parsed.
    #[inline]
    pub fn has_table(&self, name: TableName) -> bool {
        match name {
            TableName::Header                       => true,
            TableName::HorizontalHeader             => true,
            TableName::MaximumProfile               => true,
            TableName::AxisVariations               => self.avar.is_some(),
            TableName::CharacterToGlyphIndexMapping => self.cmap.is_some(),
            TableName::CompactFontFormat            => self.cff_.is_some(),
            TableName::CompactFontFormat2           => self.cff2.is_some(),
            TableName::FontVariations               => self.fvar.is_some(),
            TableName::GlyphData                    => self.glyf.is_some(),
            TableName::GlyphDefinition              => self.gdef.is_some(),
            TableName::GlyphPositioning             => self.gpos.is_some(),
            TableName::GlyphSubstitution            => self.gsub.is_some(),
            TableName::GlyphVariations              => self.gvar.is_some(),
            TableName::HorizontalMetrics            => self.hmtx.is_some(),
            TableName::HorizontalMetricsVariations  => self.hvar.is_some(),
            TableName::IndexToLocation              => self.loca.is_some(),
            TableName::Kerning                      => self.kern.is_some(),
            TableName::MetricsVariations            => self.mvar.is_some(),
            TableName::Naming                       => self.name.is_some(),
            TableName::PostScript                   => self.post.is_some(),
            TableName::VerticalHeader               => self.vhea.is_some(),
            TableName::VerticalMetrics              => self.vmtx.is_some(),
            TableName::VerticalMetricsVariations    => self.vvar.is_some(),
            TableName::VerticalOrigin               => self.vorg.is_some(),
            TableName::WindowsMetrics               => self.os_2.is_some(),
        }
    }

    /// Returns an iterator over [Name Records].
    ///
    /// An iterator can be empty.
    ///
    /// [Name Records]: https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records
    #[inline]
    pub fn names(&self) -> Names {
        self.name.unwrap_or_default()
    }

    /// Returns font's family name.
    ///
    /// *Typographic Family* is preferred over *Family*.
    ///
    /// Note that font can have multiple names. You can use [`names()`] to list them all.
    ///
    /// [`names()`]: #method.names
    #[cfg(feature = "std")]
    #[inline]
    pub fn family_name(&self) -> Option<String> {
        let mut idx = None;
        let mut iter = self.names();
        for (i, name) in iter.enumerate() {
            if name.name_id() == name_id::TYPOGRAPHIC_FAMILY && name.is_unicode() {
                // Break the loop as soon as we reached 'Typographic Family'.
                idx = Some(i);
                break;
            } else if name.name_id() == name_id::FAMILY && name.is_unicode() {
                idx = Some(i);
                // Do not break the loop since 'Typographic Family' can be set later
                // and it has a higher priority.
            }
        }

        iter.nth(idx?).and_then(|name| name.name_from_utf16_be())
    }

    /// Returns font's PostScript name.
    ///
    /// Note that font can have multiple names. You can use [`names()`] to list them all.
    ///
    /// [`names()`]: #method.names
    #[cfg(feature = "std")]
    #[inline]
    pub fn post_script_name(&self) -> Option<String> {
        self.names()
            .find(|name| name.name_id() == name_id::POST_SCRIPT_NAME && name.is_unicode())
            .and_then(|name| name.name_from_utf16_be())
    }

    /// Checks that font is marked as *Regular*.
    ///
    /// Returns `false` when OS/2 table is not present.
    #[inline]
    pub fn is_regular(&self) -> bool {
        try_opt_or!(self.os_2, false).is_regular()
    }

    /// Checks that font is marked as *Italic*.
    ///
    /// Returns `false` when OS/2 table is not present.
    #[inline]
    pub fn is_italic(&self) -> bool {
        try_opt_or!(self.os_2, false).is_italic()
    }

    /// Checks that font is marked as *Bold*.
    ///
    /// Returns `false` when OS/2 table is not present.
    #[inline]
    pub fn is_bold(&self) -> bool {
        try_opt_or!(self.os_2, false).is_bold()
    }

    /// Checks that font is marked as *Oblique*.
    ///
    /// Returns `false` when OS/2 table is not present or when its version is < 4.
    #[inline]
    pub fn is_oblique(&self) -> bool {
        try_opt_or!(self.os_2, false).is_oblique()
    }

    /// Checks if font is a variable font.
    #[inline]
    pub fn is_variable(&self) -> bool {
        // `fvar::Table::parse` already checked that `axisCount` is non-zero.
        self.fvar.is_some()
    }

    /// Parses font's weight.
    ///
    /// Returns `Weight::Normal` when OS/2 table is not present.
    #[inline]
    pub fn weight(&self) -> Weight {
        try_opt_or!(self.os_2, Weight::default()).weight()
    }

    /// Parses font's width.
    ///
    /// Returns `Width::Normal` when OS/2 table is not present or when value is invalid.
    #[inline]
    pub fn width(&self) -> Width {
        try_opt_or!(self.os_2, Width::default()).width()
    }

    /// Parses font's ascender value.
    #[inline]
    pub fn ascender(&self) -> i16 {
        if let Some(os_2) = self.os_2 {
            if os_2.is_use_typo_metrics() {
                return os_2.s_typo_ascender();
            }
        }

        self.hhea.ascender()
    }

    /// Parses font's descender value.
    #[inline]
    pub fn descender(&self) -> i16 {
        if let Some(os_2) = self.os_2 {
            if os_2.is_use_typo_metrics() {
                return os_2.s_typo_descender();
            }
        }

        self.hhea.descender()
    }

    /// Parses font's height.
    #[inline]
    pub fn height(&self) -> i16 {
        self.ascender() - self.descender()
    }

    /// Parses font's line gap.
    #[inline]
    pub fn line_gap(&self) -> i16 {
        if let Some(os_2) = self.os_2 {
            if os_2.is_use_typo_metrics() {
                return os_2.s_typo_line_gap();
            }
        }

        self.hhea.line_gap()
    }

    // TODO: should we automatically use the vhea?

    /// Parses font's vertical ascender value.
    ///
    /// Returns `None` when `vhea` table is not present.
    #[inline]
    pub fn vertical_ascender(&self) -> Option<i16> {
        self.vhea.map(|table| table.ascender())
    }

    /// Parses font's vertical descender value.
    ///
    /// Returns `None` when `vhea` table is not present.
    #[inline]
    pub fn vertical_descender(&self) -> Option<i16> {
        self.vhea.map(|table| table.descender())
    }

    /// Parses font's vertical height.
    ///
    /// Returns `None` when `vhea` table is not present.
    #[inline]
    pub fn vertical_height(&self) -> Option<i16> {
        Some(self.vertical_ascender()? - self.vertical_descender()?)
    }

    /// Parses font's vertical line gap.
    ///
    /// Returns `None` when `vhea` table is not present.
    #[inline]
    pub fn vertical_line_gap(&self) -> Option<i16> {
        self.vhea.map(|table| table.line_gap())
    }

    /// Parses glyphs index to location format.
    #[inline]
    pub(crate) fn index_to_location_format(&self) -> Option<IndexToLocationFormat> {
        match self.head.index_to_loc_format() {
            0 => Some(IndexToLocationFormat::Short),
            1 => Some(IndexToLocationFormat::Long),
            _ => None,
        }
    }

    /// Parses font's units per EM.
    ///
    /// Returns `None` when value is not in a 16..=16384 range.
    #[inline]
    pub fn units_per_em(&self) -> Option<u16> {
        let num = self.head.units_per_em();
        if num >= 16 && num <= 16384 {
            Some(num)
        } else {
            None
        }
    }

    /// Parses font's X height.
    ///
    /// Returns `None` when OS/2 table is not present or when its version is < 2.
    #[inline]
    pub fn x_height(&self) -> Option<i16> {
        self.os_2.and_then(|os_2| os_2.x_height())
    }

    /// Returns font's underline metrics.
    #[inline]
    pub fn underline_metrics(&self) -> Option<LineMetrics> {
        self.post.and_then(|post| post.underline_metrics())
    }

    /// Parses font's strikeout metrics.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn strikeout_metrics(&self) -> Option<LineMetrics> {
        self.os_2.and_then(|os_2| os_2.strikeout_metrics())
    }

    /// Parses font's subscript metrics.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn subscript_metrics(&self) -> Option<ScriptMetrics> {
        self.os_2.and_then(|os_2| os_2.subscript_metrics())
    }

    /// Parses font's superscript metrics.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn superscript_metrics(&self) -> Option<ScriptMetrics> {
        self.os_2.and_then(|os_2| os_2.superscript_metrics())
    }

    /// Parses metrics variation offset using
    /// [Metrics Variations Table](https://docs.microsoft.com/en-us/typography/opentype/spec/mvar).
    ///
    /// Note: coordinates should be converted from fixed point 2.14 to i16
    /// by multiplying each coordinate by 16384.
    ///
    /// Number of `coordinates` should be the same as number of variation axes in the font.
    ///
    /// Returns `None` when `MVAR` table is not present or invalid.
    pub fn metrics_variation(&self, tag: Tag, coordinates: &[i16]) -> Option<f32> {
        mvar::metrics_variation(self.mvar?, tag, coordinates)
    }

    /// Returns a total number of glyphs in the font.
    ///
    /// Never zero.
    ///
    /// The value was already parsed, so this function doesn't involve any parsing.
    #[inline]
    pub fn number_of_glyphs(&self) -> u16 {
        self.number_of_glyphs.get()
    }

    /// Returns an iterator over variation axes.
    #[inline]
    pub fn variation_axes(&self) -> VariationAxes {
        self.fvar.map(|fvar| fvar.axes()).unwrap_or_default()
    }

    /// Performs normalization mapping to variation coordinates
    /// using [Axis Variations Table](https://docs.microsoft.com/en-us/typography/opentype/spec/avar).
    ///
    /// Note: coordinates should be converted from fixed point 2.14 to i16
    /// by multiplying each coordinate by 16384.
    ///
    /// Number of `coordinates` should be the same as number of variation axes in the font.
    #[inline]
    pub fn map_variation_coordinates(&self, coordinates: &mut [i16]) -> Option<()> {
        avar::map_variation_coordinates(self.avar?, coordinates)
    }

    /// Resolves a Glyph ID for a code point.
    ///
    /// Returns `None` instead of `0` when glyph is not found.
    ///
    /// All subtable formats except Mixed Coverage (8) are supported.
    #[inline]
    pub fn glyph_index(&self, c: char) -> Option<GlyphId> {
        cmap::glyph_index(self.cmap.as_ref()?, c)
    }

    /// Resolves a variation of a Glyph ID from two code points.
    ///
    /// Implemented according to
    /// [Unicode Variation Sequences](
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences).
    ///
    /// Returns `None` instead of `0` when glyph is not found.
    #[inline]
    pub fn glyph_variation_index(&self, c: char, variation: char) -> Option<GlyphId> {
        cmap::glyph_variation_index(self.cmap.as_ref()?, c, variation)
    }

    /// Returns glyph's horizontal advance using
    /// [Horizontal Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx).
    #[inline]
    pub fn glyph_hor_advance(&self, glyph_id: GlyphId) -> Option<u16> {
        self.hmtx.and_then(|hmtx| hmtx.advance(glyph_id))
    }

    /// Parses glyph's variation offset for horizontal advance using
    /// [Horizontal Metrics Variations Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hvar).
    ///
    /// Note: coordinates should be converted from fixed point 2.14 to i16
    /// by multiplying each coordinate by 16384.
    ///
    /// Number of `coordinates` should be the same as number of variation axes in the font.
    ///
    /// Returns `None` when `HVAR` table is not present or invalid.
    pub fn glyph_hor_advance_variation(
        &self,
        glyph_id: GlyphId,
        coordinates: &[i16],
    ) -> Option<f32> {
        hvar::glyph_advance_variation(self.hvar?, glyph_id, coordinates)
    }

    /// Returns glyph's horizontal side bearing using
    /// [Horizontal Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx).
    #[inline]
    pub fn glyph_hor_side_bearing(&self, glyph_id: GlyphId) -> Option<i16> {
        self.hmtx.and_then(|hmtx| hmtx.side_bearing(glyph_id))
    }

    /// Parses glyph's variation offset for horizontal side bearing using
    /// [Horizontal Metrics Variations Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hvar).
    ///
    /// Note: coordinates should be converted from fixed point 2.14 to i16
    /// by multiplying each coordinate by 16384.
    ///
    /// Number of `coordinates` should be the same as number of variation axes in the font.
    ///
    /// Returns `None` when `HVAR` table is not present or invalid.
    pub fn glyph_hor_side_bearing_variation(
        &self,
        glyph_id: GlyphId,
        coordinates: &[i16],
    ) -> Option<f32> {
        hvar::glyph_side_bearing_variation(self.hvar?, glyph_id, coordinates)
    }

    /// Returns glyph's vertical advance using
    /// [Vertical Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vmtx).
    #[inline]
    pub fn glyph_ver_advance(&self, glyph_id: GlyphId) -> Option<u16> {
        self.vmtx.and_then(|vmtx| vmtx.advance(glyph_id))
    }

    /// Parses glyph's variation offset for vertical advance using
    /// [Vertical Metrics Variations Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vvar).
    ///
    /// Note: coordinates should be converted from fixed point 2.14 to i16
    /// by multiplying each coordinate by 16384.
    ///
    /// Number of `coordinates` should be the same as number of variation axes in the font.
    ///
    /// Returns `None` when `VVAR` table is not present or invalid.
    pub fn glyph_ver_advance_variation(
        &self,
        glyph_id: GlyphId,
        coordinates: &[i16],
    ) -> Option<f32> {
        crate::hvar::glyph_advance_variation(self.vvar?, glyph_id, coordinates)
    }

    /// Returns glyph's vertical side bearing using
    /// [Vertical Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vmtx).
    #[inline]
    pub fn glyph_ver_side_bearing(&self, glyph_id: GlyphId) -> Option<i16> {
        self.vmtx.and_then(|vmtx| vmtx.side_bearing(glyph_id))
    }

    /// Parses glyph's variation offset for vertical side bearing using
    /// [Vertical Metrics Variations Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vvar).
    ///
    /// Note: coordinates should be converted from fixed point 2.14 to i16
    /// by multiplying each coordinate by 16384.
    ///
    /// Number of `coordinates` should be the same as number of variation axes in the font.
    ///
    /// Returns `None` when `VVAR` table is not present or invalid.
    pub fn glyph_ver_side_bearing_variation(
        &self,
        glyph_id: GlyphId,
        coordinates: &[i16],
    ) -> Option<f32> {
        crate::hvar::glyph_side_bearing_variation(self.vvar?, glyph_id, coordinates)
    }

    /// Returns a vertical origin of a glyph according to
    /// [Vertical Origin Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg).
    pub fn glyph_y_origin(&self, glyph_id: GlyphId) -> Option<i16> {
        self.vorg.and_then(|vorg| vorg.glyph_y_origin(glyph_id))
    }

    /// Returns glyph's name.
    ///
    /// Uses the `post` table as a source.
    ///
    /// Returns `None` when no name is associated with a `glyph`.
    #[inline]
    pub fn glyph_name(&self, glyph_id: GlyphId) -> Option<&str> {
        self.post.and_then(|post| post.glyph_name(glyph_id))
    }

    /// Checks that font has
    /// [Glyph Class Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#glyph-class-definition-table).
    pub fn has_glyph_classes(&self) -> bool {
        self.glyph_class(GlyphId(0)).is_some()
    }

    /// Parses glyph's class according to
    /// [Glyph Class Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#glyph-class-definition-table).
    ///
    /// Returns `None` when *Glyph Class Definition Table* is not set
    /// or glyph class is not set or invalid.
    pub fn glyph_class(&self, glyph_id: GlyphId) -> Option<GlyphClass> {
        self.gdef.and_then(|gdef| gdef.glyph_class(glyph_id))
    }

    /// Parses glyph's mark attachment class according to
    /// [Mark Attachment Class Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-attachment-class-definition-table).
    ///
    /// All glyphs not assigned to a class fall into Class 0.
    pub fn glyph_mark_attachment_class(&self, glyph_id: GlyphId) -> Class {
        try_opt_or!(self.gdef, Class(0)).glyph_mark_attachment_class(glyph_id)
    }

    /// Checks that glyph is a mark according to
    /// [Mark Glyph Sets Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-glyph-sets-table).
    ///
    /// `set_index` allows checking a specific glyph coverage set.
    /// Otherwise all sets will be checked.
    ///
    /// Returns `Ok(false)` when *Mark Glyph Sets Table* is not set.
    #[inline]
    pub fn is_mark_glyph(&self, glyph_id: GlyphId, set_index: Option<u16>) -> bool {
        try_opt_or!(self.gdef, false).is_mark_glyph(glyph_id, set_index)
    }

    /// Returns a reference to a [Glyph Positioning Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos).
    pub fn positioning_table(&self) -> Option<PositioningTable<'a>> {
        self.gpos.map(|table| PositioningTable { table })
    }

    /// Returns a reference to a [Glyph Substitution Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub).
    pub fn substitution_table(&self) -> Option<SubstitutionTable<'a>> {
        self.gsub.map(|table| SubstitutionTable { table })
    }

    /// Returns a glyphs pair kerning.
    ///
    /// Only a horizontal kerning is supported.
    pub fn glyphs_kerning(&self, glyph_id1: GlyphId, glyph_id2: GlyphId) -> Option<i16> {
        kern::glyphs_kerning(self.kern?, glyph_id1, glyph_id2)
    }

    /// Outlines a glyph and returns its tight bounding box.
    ///
    /// **Warning**: since `ttf-parser` is a pull parser,
    /// `OutlineBuilder` will emit segments even when outline is partially malformed.
    /// You must check `outline_glyph()` result before using
    /// `OutlineBuilder`'s output.
    ///
    /// This method supports `glyf`, `gvar`, `CFF` and `CFF2` tables.
    ///
    /// Returns `None` when glyph has no outline.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fmt::Write;
    /// use ttf_parser;
    ///
    /// struct Builder(String);
    ///
    /// impl ttf_parser::OutlineBuilder for Builder {
    ///     fn move_to(&mut self, x: f32, y: f32) {
    ///         write!(&mut self.0, "M {} {} ", x, y).unwrap();
    ///     }
    ///
    ///     fn line_to(&mut self, x: f32, y: f32) {
    ///         write!(&mut self.0, "L {} {} ", x, y).unwrap();
    ///     }
    ///
    ///     fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    ///         write!(&mut self.0, "Q {} {} {} {} ", x1, y1, x, y).unwrap();
    ///     }
    ///
    ///     fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    ///         write!(&mut self.0, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap();
    ///     }
    ///
    ///     fn close(&mut self) {
    ///         write!(&mut self.0, "Z ").unwrap();
    ///     }
    /// }
    ///
    /// let data = std::fs::read("tests/fonts/glyphs.ttf").unwrap();
    /// let font = ttf_parser::Font::from_data(&data, 0).unwrap();
    /// let mut builder = Builder(String::new());
    /// let bbox = font.outline_glyph(ttf_parser::GlyphId(0), &mut builder).unwrap();
    /// assert_eq!(builder.0, "M 50 0 L 50 750 L 450 750 L 450 0 L 50 0 Z ");
    /// assert_eq!(bbox, ttf_parser::Rect { x_min: 50, y_min: 0, x_max: 450, y_max: 750 });
    /// ```
    #[inline]
    pub fn outline_glyph(
        &self,
        glyph_id: GlyphId,
        builder: &mut dyn OutlineBuilder,
    ) -> Option<Rect> {
        if let Some(glyf_table) = self.glyf {
            return glyf::outline(self.loca?, glyf_table, glyph_id, builder);
        }

        if let Some(ref metadata) = self.cff_ {
            return cff::outline(metadata, glyph_id, builder);
        }

        // TODO: use default coords
        // if let Some(ref metadata) = self.cff2 {
        //     return cff2::outline(metadata, glyph_id, builder);
        // }

        None
    }

    /// Outlines a variable glyph and returns its tight bounding box.
    ///
    /// Note: coordinates should be converted from fixed point 2.14 to i16
    /// by multiplying each coordinate by 16384.
    ///
    /// Number of `coordinates` should be the same as number of variation axes in the font.
    ///
    /// **Warning**: since `ttf-parser` is a pull parser,
    /// `OutlineBuilder` will emit segments even when outline is partially malformed.
    /// You must check `outline_variable_glyph()` result before using
    /// `OutlineBuilder`'s output.
    ///
    /// This method supports `glyf` + `gvar` and `CFF2` tables.
    ///
    /// Returns `None` when glyph has no outline or when font is not variable.
    #[inline]
    pub fn outline_variable_glyph(
        &self,
        glyph_id: GlyphId,
        coordinates: &[i16],
        builder: &mut dyn OutlineBuilder,
    ) -> Option<Rect> {
        if let Some(ref gvar_table) = self.gvar {
            return gvar::outline_variable(self.loca?, self.glyf?, gvar_table,
                                          coordinates, glyph_id, builder);
        }

        if let Some(ref metadata) = self.cff2 {
            return cff2::outline(metadata, coordinates, glyph_id, builder);
        }

        None
    }

    /// Returns a tight glyph bounding box.
    ///
    /// Note that this method's performance depends on a table type the current font is using.
    /// In case of a `glyf` table, it's basically free, since this table stores
    /// bounding box separately. In case of `CFF` we should actually outline
    /// a glyph and then calculate its bounding box. So if you need an outline and
    /// a bounding box and you have an OpenType font (which uses CFF)
    /// then prefer `outline_glyph()` method.
    #[inline]
    pub fn glyph_bounding_box(&self, glyph_id: GlyphId) -> Option<Rect> {
        if let Some(glyf_table) = self.glyf {
            return glyf::glyph_bbox(self.loca?, glyf_table, glyph_id);
        }

        if let Some(ref metadata) = self.cff_ {
            return cff::outline(metadata, glyph_id, &mut DummyOutline);
        }

        // TODO: use default coords
        // if let Some(ref metadata) = self.cff2 {
        //     return cff2::outline(metadata, glyph_id, &mut DummyOutline);
        // }

        None
    }

    /// Returns a tight bounding box for a variable glyph.
    ///
    /// This is just a `outline_variable_glyph()` shorthand, since we have to outline
    /// the glyph in case of a variable font to get its bounding box.
    #[inline]
    pub fn variable_glyph_bounding_box(
        &self,
        glyph_id: GlyphId,
        coordinates: &[i16],
    ) -> Option<Rect> {
        if self.gvar.is_some() {
            return gvar::outline_variable(self.loca?, self.glyf?, self.gvar.as_ref()?,
                                          coordinates, glyph_id, &mut DummyOutline);
        }

        if let Some(ref metadata) = self.cff2 {
            return cff2::outline(metadata, coordinates, glyph_id, &mut DummyOutline);
        }

        None
    }
}

impl fmt::Debug for Font<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Font()")
    }
}

/// Returns the number of fonts stored in a TrueType font collection.
///
/// Returns `None` if a provided data is not a TrueType font collection.
#[inline]
pub fn fonts_in_collection(data: &[u8]) -> Option<u32> {
    let table = raw::TTCHeader::new(data.get(0..raw::TTCHeader::SIZE)?);

    if &table.ttc_tag().to_bytes() != b"ttcf" {
        return None;
    }

    Some(table.num_fonts())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::writer;
    use writer::TtfType::*;

    #[test]
    fn empty_font() {
        assert!(Font::from_data(&[], 0).is_none());
    }

    #[test]
    fn incomplete_header() {
        let data = writer::convert(&[
            TrueTypeMagic,
            UInt16(0), // numTables
            UInt16(0), // searchRange
            UInt16(0), // entrySelector
            UInt16(0), // rangeShift
        ]);

        for i in 0..data.len() {
            assert!(Font::from_data(&data[0..i], 0).is_none());
        }
    }

    #[test]
    fn zero_tables() {
        let data = writer::convert(&[
            TrueTypeMagic,
            UInt16(0), // numTables
            UInt16(0), // searchRange
            UInt16(0), // entrySelector
            UInt16(0), // rangeShift
        ]);

        assert!(Font::from_data(&data, 0).is_none());
    }

    #[test]
    fn tables_count_overflow() {
        let data = writer::convert(&[
            TrueTypeMagic,
            UInt16(std::u16::MAX), // numTables
            UInt16(0), // searchRange
            UInt16(0), // entrySelector
            UInt16(0), // rangeShift
        ]);

        assert!(Font::from_data(&data, 0).is_none());
    }

    #[test]
    fn open_type_magic() {
        let data = writer::convert(&[
            OpenTypeMagic,
            UInt16(0), // numTables
            UInt16(0), // searchRange
            UInt16(0), // entrySelector
            UInt16(0), // rangeShift
        ]);

        assert!(Font::from_data(&data, 0).is_none());
    }

    #[test]
    fn unknown_magic() {
        let data = writer::convert(&[
            Raw(&[0xFF, 0xFF, 0xFF, 0xFF]),
            UInt16(0), // numTables
            UInt16(0), // searchRange
            UInt16(0), // entrySelector
            UInt16(0), // rangeShift
        ]);

        assert!(Font::from_data(&data, 0).is_none());
    }

    #[test]
    fn empty_font_collection() {
        let data = writer::convert(&[
            FontCollectionMagic,
            UInt16(1), // majorVersion
            UInt16(0), // minorVersion
            UInt32(0), // numFonts
        ]);

        assert_eq!(fonts_in_collection(&data), Some(0));
        assert!(Font::from_data(&data, 0).is_none());
    }

    #[test]
    fn font_collection_num_fonts_overflow() {
        let data = writer::convert(&[
            FontCollectionMagic,
            UInt16(1), // majorVersion
            UInt16(0), // minorVersion
            UInt32(std::u32::MAX), // numFonts
        ]);

        assert_eq!(fonts_in_collection(&data), Some(std::u32::MAX));
        assert!(Font::from_data(&data, 0).is_none());
    }

    #[test]
    fn font_index_overflow_1() {
        let data = writer::convert(&[
            FontCollectionMagic,
            UInt16(1), // majorVersion
            UInt16(0), // minorVersion
            UInt32(1), // numFonts
        ]);

        assert_eq!(fonts_in_collection(&data), Some(1));
        assert!(Font::from_data(&data, std::u32::MAX).is_none());
    }

    #[test]
    fn font_index_overflow_2() {
        let data = writer::convert(&[
            FontCollectionMagic,
            UInt16(1), // majorVersion
            UInt16(0), // minorVersion
            UInt32(std::u32::MAX), // numFonts
        ]);

        assert_eq!(fonts_in_collection(&data), Some(std::u32::MAX));
        assert!(Font::from_data(&data, std::u32::MAX - 1).is_none());
    }
}
