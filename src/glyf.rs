// https://docs.microsoft.com/en-us/typography/opentype/spec/glyf

// This module is a heavily modified version of https://github.com/raphlinus/font-rs

use core::num::NonZeroU16;

use crate::{Font, GlyphId, OutlineBuilder, Rect, BBox};
use crate::parser::{Stream, F2DOT14, FromData, LazyArrayIter};

pub(crate) struct Builder<'a> {
    pub builder: &'a mut dyn OutlineBuilder,
    pub transform: Transform,
    pub is_default_ts: bool, // `bool` is faster than `Option` or `is_default`.
    pub bbox: Option<BBox>, // Used only by `gvar`.
    pub first_oncurve: Option<Point>,
    pub first_offcurve: Option<Point>,
    pub last_offcurve: Option<Point>,
}

impl<'a> Builder<'a> {
    #[inline]
    pub fn new(transform: Transform, bbox: Option<BBox>, builder: &'a mut dyn OutlineBuilder) -> Self {
        Builder {
            builder,
            transform,
            is_default_ts: transform.is_default(),
            bbox,
            first_oncurve: None,
            first_offcurve: None,
            last_offcurve: None,
        }
    }

    #[inline]
    fn move_to(&mut self, mut x: f32, mut y: f32) {
        if !self.is_default_ts {
            self.transform.apply_to(&mut x, &mut y);
        }

        if let Some(ref mut bbox) = self.bbox {
            bbox.extend_by(x, y);
        }

        self.builder.move_to(x, y);
    }

    #[inline]
    fn line_to(&mut self, mut x: f32, mut y: f32) {
        if !self.is_default_ts {
            self.transform.apply_to(&mut x, &mut y);
        }

        if let Some(ref mut bbox) = self.bbox {
            bbox.extend_by(x, y);
        }

        self.builder.line_to(x, y);
    }

    #[inline]
    fn quad_to(&mut self, mut x1: f32, mut y1: f32, mut x: f32, mut y: f32) {
        if !self.is_default_ts {
            self.transform.apply_to(&mut x1, &mut y1);
            self.transform.apply_to(&mut x, &mut y);
        }

        if let Some(ref mut bbox) = self.bbox {
            bbox.extend_by(x1, y1);
            bbox.extend_by(x, y);
        }

        self.builder.quad_to(x1, y1, x, y);
    }

    #[inline]
    fn close(&mut self) {
        self.builder.close();
    }

    // Useful links:
    //
    // - https://developer.apple.com/fonts/TrueType-Reference-Manual/RM01/Chap1.html
    // - https://stackoverflow.com/a/20772557
    pub fn push_point(&mut self, x: f32, y: f32, on_curve_point: bool, last_point: bool) {
        let p = Point { x, y };
        if self.first_oncurve.is_none() {
            if on_curve_point {
                self.first_oncurve = Some(p);
                self.move_to(p.x, p.y);
            } else {
                if let Some(offcurve) = self.first_offcurve {
                    let mid = offcurve.lerp(p, 0.5);
                    self.first_oncurve = Some(mid);
                    self.last_offcurve = Some(p);
                    self.move_to(mid.x, mid.y);
                } else {
                    self.first_offcurve = Some(p);
                }
            }
        } else {
            match (self.last_offcurve, on_curve_point) {
                (Some(offcurve), true) => {
                    self.last_offcurve = None;
                    self.quad_to(offcurve.x, offcurve.y, p.x, p.y);
                }
                (Some(offcurve), false) => {
                    self.last_offcurve = Some(p);
                    let mid = offcurve.lerp(p, 0.5);
                    self.quad_to(offcurve.x, offcurve.y, mid.x, mid.y);
                }
                (None, true) => {
                    self.line_to(p.x, p.y);
                }
                (None, false) => {
                    self.last_offcurve = Some(p);
                }
            }
        }

        if last_point {
            self.finish_contour();
        }
    }

    fn finish_contour(&mut self) {
        if let (Some(offcurve1), Some(offcurve2)) = (self.first_offcurve, self.last_offcurve) {
            self.last_offcurve = None;
            let mid = offcurve2.lerp(offcurve1, 0.5);
            self.quad_to(offcurve2.x, offcurve2.y, mid.x, mid.y);
        }

        if let (Some(p), Some(offcurve1)) = (self.first_oncurve, self.first_offcurve) {
            self.quad_to(offcurve1.x, offcurve1.y, p.x, p.y);
        } else if let (Some(p), Some(offcurve2)) = (self.first_oncurve, self.last_offcurve) {
            self.quad_to(offcurve2.x, offcurve2.y, p.x, p.y);
        } else if let Some(p) = self.first_oncurve {
            self.line_to(p.x, p.y);
        }

        self.close();

        self.first_oncurve = None;
        self.first_offcurve = None;
        self.last_offcurve = None;

        self.close();
    }
}


// https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#simple-glyph-description
#[derive(Clone, Copy)]
pub struct SimpleGlyphFlags(pub u8);

impl SimpleGlyphFlags {
    #[inline] pub fn on_curve_point(self) -> bool { self.0 & 0x01 != 0 }
    #[inline] pub fn x_short(self) -> bool { self.0 & 0x02 != 0 }
    #[inline] pub fn y_short(self) -> bool { self.0 & 0x04 != 0 }
    #[inline] pub fn repeat_flag(self) -> bool { self.0 & 0x08 != 0 }
    #[inline] pub fn x_is_same_or_positive_short(self) -> bool { self.0 & 0x10 != 0 }
    #[inline] pub fn y_is_same_or_positive_short(self) -> bool { self.0 & 0x20 != 0 }
}

impl FromData for SimpleGlyphFlags {
    #[inline]
    fn parse(data: &[u8]) -> Self {
        SimpleGlyphFlags(data[0])
    }
}


// https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#composite-glyph-description
#[derive(Clone, Copy)]
pub struct CompositeGlyphFlags(u16);

impl CompositeGlyphFlags {
    #[inline] pub fn arg_1_and_2_are_words(self) -> bool { self.0 & 0x0001 != 0 }
    #[inline] pub fn args_are_xy_values(self) -> bool { self.0 & 0x0002 != 0 }
    #[inline] pub fn we_have_a_scale(self) -> bool { self.0 & 0x0008 != 0 }
    #[inline] pub fn more_components(self) -> bool { self.0 & 0x0020 != 0 }
    #[inline] pub fn we_have_an_x_and_y_scale(self) -> bool { self.0 & 0x0040 != 0 }
    #[inline] pub fn we_have_a_two_by_two(self) -> bool { self.0 & 0x0080 != 0 }
}

impl FromData for CompositeGlyphFlags {
    #[inline]
    fn parse(data: &[u8]) -> Self {
        CompositeGlyphFlags(u16::parse(data))
    }
}


#[inline]
pub fn f32_bound(min: f32, val: f32, max: f32) -> f32 {
    debug_assert!(min.is_finite());
    debug_assert!(val.is_finite());
    debug_assert!(max.is_finite());

    if val > max {
        return max;
    } else if val < min {
        return min;
    }

    val
}

// It's not defined in the spec, so we are using our own value.
pub const MAX_COMPONENTS: u8 = 32;

impl<'a> Font<'a> {
    pub(crate) fn glyf_glyph_outline(
        &self,
        glyph_id: GlyphId,
        builder: &mut dyn OutlineBuilder,
    ) -> Option<Rect> {
        let mut b = Builder::new(Transform::default(), None, builder);
        let glyph_data = self.glyph_data(glyph_id)?;
        self.outline_impl(glyph_data, 0, &mut b)
    }

    pub(crate) fn glyf_glyph_bbox(&self, glyph_id: GlyphId) -> Option<Rect> {
        let glyph_data = self.glyph_data(glyph_id)?;
        let mut s = Stream::new(glyph_data);
        s.skip::<i16>(); // number_of_contours
        // It's faster to parse the rect directly, instead of using `FromData`.
        Some(Rect {
            x_min: s.read()?,
            y_min: s.read()?,
            x_max: s.read()?,
            y_max: s.read()?,
        })
    }

    pub(crate) fn glyph_data(&self, glyph_id: GlyphId) -> Option<&[u8]> {
        let range = self.glyph_range(glyph_id)?;
        let data = self.glyf?;
        data.get(range)
    }

    fn outline_impl(
        &self,
        data: &[u8],
        depth: u8,
        builder: &mut Builder,
    ) -> Option<Rect> {
        if depth >= MAX_COMPONENTS {
            warn!("Recursion detected in the 'glyf' table.");
            return None;
        }

        let mut s = Stream::new(data);
        let number_of_contours: i16 = s.read()?;
        // It's faster to parse the rect directly, instead of using `FromData`.
        let rect = Rect {
            x_min: s.read()?,
            y_min: s.read()?,
            x_max: s.read()?,
            y_max: s.read()?,
        };

        if number_of_contours > 0 {
            // Simple glyph.
            let number_of_contours = NonZeroU16::new(number_of_contours as u16)?;
            for point in parse_simple_outline(s.tail()?, number_of_contours)? {
                builder.push_point(point.x as f32, point.y as f32, point.on_curve_point, point.last_point);
            }
        } else if number_of_contours < 0 {
            // Composite glyph.
            for comp in CompositeGlyphIter::new(s.tail()?) {
                if let Some(glyph_data) = self.glyph_data(comp.glyph_id) {
                    let transform = Transform::combine(builder.transform, comp.transform);
                    let mut b = Builder::new(transform, builder.bbox, builder.builder);
                    self.outline_impl(glyph_data, depth + 1, &mut b)?;
                }
            }
        } else {
            // An empty glyph.
            return None;
        }

        Some(rect)
    }
}

#[inline(never)]
pub fn parse_simple_outline(
    glyph_data: &[u8],
    number_of_contours: NonZeroU16,
) -> Option<GlyphPoints> {
    let mut s = Stream::new(glyph_data);
    let endpoints = s.read_array::<u16, u16>(number_of_contours.get())?;

    let points_total = endpoints.last()?.checked_add(1)?;

    // Skip instructions byte code.
    let instructions_len: u16 = s.read()?;
    s.advance(instructions_len);

    let flags_offset = s.offset();
    let x_coords_len = resolve_x_coords_len(&mut s, points_total)?;
    let x_coords_offset = s.offset();
    let y_coords_offset = x_coords_offset + x_coords_len as usize;

    let mut endpoints = endpoints.into_iter();
    let contour_points_left = endpoints.next()?;

    Some(GlyphPoints {
        endpoints: endpoints.into_iter(),
        flags: Stream::new(glyph_data.get(flags_offset..x_coords_offset)?),
        x_coords: Stream::new(glyph_data.get(x_coords_offset..y_coords_offset)?),
        y_coords: Stream::new(glyph_data.get(y_coords_offset..glyph_data.len())?),
        points_left: points_total,
        last_point_index: 0,
        contour_points_left,
        current_contour: 0,
        flag_repeats: 0,
        last_flags: SimpleGlyphFlags(0),
        x: 0,
        y: 0,
    })
}

/// Resolves the X coordinates length.
///
/// The length depends on *Simple Glyph Flags*, so we have to process them all to find it.
pub fn resolve_x_coords_len(
    s: &mut Stream,
    points_total: u16,
) -> Option<u16> {
    let mut flags_left = points_total;
    let mut x_coords_len = 0u16;
    while flags_left > 0 {
        let flags: SimpleGlyphFlags = s.read()?;

        // The number of times a glyph point repeats.
        let repeats = if flags.repeat_flag() {
            let repeats: u8 = s.read()?;
            repeats as u16 + 1
        } else {
            1
        };

        if flags.x_short() {
            // Coordinate is 1 byte long.
            x_coords_len = x_coords_len.checked_add(repeats)?;
        } else if !flags.x_is_same_or_positive_short() {
            // Coordinate is 2 bytes long.
            x_coords_len = x_coords_len.checked_add(repeats * 2)?;
        }

        // Check for overflow.
        // Do not use checked_sub, because it's very slow for some reasons.
        if repeats <= flags_left {
            flags_left -= repeats;
        } else {
            return None;
        }
    }

    Some(x_coords_len)
}


#[derive(Clone, Copy)]
pub struct Transform {
    pub a: f32, pub b: f32, pub c: f32,
    pub d: f32, pub e: f32, pub f: f32,
}

impl Transform {
    #[inline]
    pub fn new_translate(tx: f32, ty: f32) -> Self {
        Transform { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: tx, f: ty }
    }

    #[inline]
    pub fn combine(ts1: Self, ts2: Self) -> Self {
        Transform {
            a: ts1.a * ts2.a + ts1.c * ts2.b,
            b: ts1.b * ts2.a + ts1.d * ts2.b,
            c: ts1.a * ts2.c + ts1.c * ts2.d,
            d: ts1.b * ts2.c + ts1.d * ts2.d,
            e: ts1.a * ts2.e + ts1.c * ts2.f + ts1.e,
            f: ts1.b * ts2.e + ts1.d * ts2.f + ts1.f,
        }
    }

    #[inline]
    pub fn apply_to(&self, x: &mut f32, y: &mut f32) {
        let tx = *x;
        let ty = *y;
        *x = self.a * tx + self.c * ty + self.e;
        *y = self.b * tx + self.d * ty + self.f;
    }

    #[inline]
    pub fn is_default(&self) -> bool {
        // A direct float comparison is fine in our case.
           self.a == 1.0
        && self.b == 0.0
        && self.c == 0.0
        && self.d == 1.0
        && self.e == 0.0
        && self.f == 0.0
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Transform { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 }
    }
}

impl core::fmt::Debug for Transform {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Transform({} {} {} {} {} {})", self.a, self.b, self.c, self.d, self.e, self.f)
    }
}


pub struct CompositeGlyphInfo {
    pub glyph_id: GlyphId,
    pub transform: Transform,
    pub flags: CompositeGlyphFlags,
}


#[derive(Clone)]
pub struct CompositeGlyphIter<'a> {
    stream: Stream<'a>,
}

impl<'a> CompositeGlyphIter<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        CompositeGlyphIter { stream: Stream::new(data) }
    }
}

impl<'a> Iterator for CompositeGlyphIter<'a> {
    type Item = CompositeGlyphInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stream.at_end() {
            return None;
        }

        let flags: CompositeGlyphFlags = self.stream.read()?;
        let glyph_id: GlyphId = self.stream.read()?;

        let mut ts = Transform::default();

        if flags.args_are_xy_values() {
            if flags.arg_1_and_2_are_words() {
                ts.e = self.stream.read::<i16>()? as f32;
                ts.f = self.stream.read::<i16>()? as f32;
            } else {
                ts.e = self.stream.read::<i8>()? as f32;
                ts.f = self.stream.read::<i8>()? as f32;
            }
        }

        if flags.we_have_a_two_by_two() {
            ts.a = self.stream.read::<F2DOT14>()?.to_float();
            ts.b = self.stream.read::<F2DOT14>()?.to_float();
            ts.c = self.stream.read::<F2DOT14>()?.to_float();
            ts.d = self.stream.read::<F2DOT14>()?.to_float();
        } else if flags.we_have_an_x_and_y_scale() {
            ts.a = self.stream.read::<F2DOT14>()?.to_float();
            ts.d = self.stream.read::<F2DOT14>()?.to_float();
        } else if flags.we_have_a_scale() {
            // 'If the bit WE_HAVE_A_SCALE is set, the scale value is read in 2.14 format.
            // The value can be between -2 to almost +2.'
            ts.a = f32_bound(-2.0, self.stream.read::<F2DOT14>()?.to_float(), 2.0);
            ts.d = ts.a;
        }

        if !flags.more_components() {
            // Finish the iterator even if stream still has some data.
            self.stream.jump_to_end();
        }

        Some(CompositeGlyphInfo {
            glyph_id,
            transform: ts,
            flags,
        })
    }
}


// Due to some optimization magic, using f32 instead of i16
// makes the code ~10% slower. At least on my machine.
// I guess it's due to the fact that with i16 the struct
// fits into the machine word.
#[derive(Clone, Copy, Default, Debug)]
pub struct GlyphPoint {
    pub x: i16,
    pub y: i16,
    /// Indicates that a point is a point on curve
    /// and not a control point.
    pub on_curve_point: bool,
    /// Indicates the last point of a contour.
    pub last_point: bool,
}


#[derive(Clone)]
pub struct GlyphPoints<'a> {
    endpoints: LazyArrayIter<'a, u16, u16>, // Each endpoint indicates a contour end.
    flags: Stream<'a>,
    x_coords: Stream<'a>,
    y_coords: Stream<'a>,
    pub points_left: u16, // Number of points left in the glyph.
    last_point_index: u16, // Number of parsed points.
    contour_points_left: u16, // Number of points left in the current contour.
    pub current_contour: u16,
    // Number of timer the `last_flags` should be used
    // before reading the next one from `flags`.
    flag_repeats: u8,
    last_flags: SimpleGlyphFlags,
    // Points stored as deltas, so we have to keep the previous one.
    x: i16,
    y: i16,
}

impl<'a> Iterator for GlyphPoints<'a> {
    type Item = GlyphPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.points_left == 0 {
            return None;
        }

        if self.flag_repeats == 0 {
            self.last_flags = self.flags.read()?;
            if self.last_flags.repeat_flag() {
                self.flag_repeats = self.flags.read()?;
            }
        } else {
            self.flag_repeats -= 1;
        }

        let x = match (self.last_flags.x_short(), self.last_flags.x_is_same_or_positive_short()) {
            (true, true) => {
                self.x_coords.read::<u8>()? as i16
            }
            (true, false) => {
                -(self.x_coords.read::<u8>()? as i16)
            }
            (false, true) => {
                // Keep previous coordinate.
                0
            }
            (false, false) => {
                self.x_coords.read()?
            }
        };
        self.x = self.x.wrapping_add(x);

        let y = match (self.last_flags.y_short(), self.last_flags.y_is_same_or_positive_short()) {
            (true, true) => {
                self.y_coords.read::<u8>()? as i16
            }
            (true, false) => {
                -(self.y_coords.read::<u8>()? as i16)
            }
            (false, true) => {
                // Keep previous coordinate.
                0
            }
            (false, false) => {
                self.y_coords.read()?
            }
        };
        self.y = self.y.wrapping_add(y);

        self.points_left -= 1;

        if self.last_point_index != core::u16::MAX {
            self.last_point_index += 1;
        }

        let last_point = self.contour_points_left == 0;
        if last_point {
            if self.points_left != 0 {
                loop {
                    let endpoint = self.endpoints.next()?;

                    // Endpoints are stored in an increasing order,
                    // and we need only the delta.
                    self.contour_points_left = endpoint.checked_sub(self.last_point_index)?;

                    // Contour must have at least 2 points.
                    if self.contour_points_left >= 2 {
                        // TODO: should we skip points too?
                        break;
                    }
                }
            }

            self.current_contour += 1;
        } else {
            self.contour_points_left -= 1;
        }

        Some(GlyphPoint {
            x: self.x,
            y: self.y,
            on_curve_point: self.last_flags.on_curve_point(),
            last_point,
        })
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[inline]
    pub fn lerp(self, other: Point, t: f32) -> Point {
        Point {
            x: self.x + t * (other.x - self.x),
            y: self.y + t * (other.y - self.y),
        }
    }
}
