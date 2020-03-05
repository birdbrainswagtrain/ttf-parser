// https://docs.microsoft.com/en-us/typography/opentype/spec/fvar

use core::num::NonZeroU16;

use crate::{Font, Tag};
use crate::parser::{Stream, Offset16, Offset, LazyArray16, LazyArrayIter};
use crate::raw::fvar as raw;


/// A [variation axis](https://docs.microsoft.com/en-us/typography/opentype/spec/fvar#variationaxisrecord).
#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct VariationAxis {
    pub tag: Tag,
    pub min_value: f32,
    pub default_value: f32,
    pub max_value: f32,
    /// Axis name in `name` table.
    pub name_id: u16,
    pub hidden: bool,
}


#[derive(Clone, Copy)]
pub(crate) struct Table<'a> {
    axes: LazyArray16<'a, raw::VariationAxisRecord>,
}

impl<'a> Table<'a> {
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let version: u32 = s.read()?;
        if version != 0x00010000 {
            return None;
        }

        let axes_array_offset: Offset16 = s.read()?;
        s.skip::<u16>(); // reserved
        let axis_count: u16 = s.read()?;

        // 'If axisCount is zero, then the font is not functional as a variable font,
        // and must be treated as a non-variable font;
        // any variation-specific tables or data is ignored.'
        let axis_count = NonZeroU16::new(axis_count)?;

        let mut s = Stream::new_at(data, axes_array_offset.to_usize());
        let axes = s.read_array(axis_count.get())?;

        Some(Table { axes })
    }
}


#[allow(missing_debug_implementations)]
#[derive(Clone, Copy, Default)]
pub struct VariationAxes<'a> {
    iter: LazyArrayIter<'a, raw::VariationAxisRecord, u16>,
}

impl<'a> Iterator for VariationAxes<'a> {
    type Item = VariationAxis;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let record = self.iter.next()?;

        let default_value = record.default_value();
        let min_value = core::cmp::min(default_value, record.min_value());
        let max_value = core::cmp::max(default_value, record.max_value());

        Some(VariationAxis {
            tag: record.axis_tag(),
            min_value: min_value as f32 / 65536.0,
            default_value: default_value as f32 / 65536.0,
            max_value: max_value as f32 / 65536.0,
            name_id: record.axis_name_id(),
            hidden: (record.flags() >> 3) & 1 == 1,
        })
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }
}


impl<'a> Font<'a> {
    /// Returns an iterator over variation axes.
    pub fn variation_axes(&self) -> VariationAxes {
        self.fvar.map(|fvar| VariationAxes { iter: fvar.axes.into_iter() })
            .unwrap_or_default()
    }
}
