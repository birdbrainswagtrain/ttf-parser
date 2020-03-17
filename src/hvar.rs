// https://docs.microsoft.com/en-us/typography/opentype/spec/hvar

use crate::{GlyphId, NormalizedCoord};
use crate::parser::{Stream, Offset, Offset32};

#[derive(Clone, Copy)]
pub struct Table<'a> {
    data: &'a [u8],
    variation_store_offset: Offset32,
    advance_width_mapping_offset: Option<Offset32>,
    lsb_mapping_offset: Option<Offset32>,
}

impl<'a> Table<'a> {
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version: u32 = s.read()?;
        if version != 0x00010000 {
            return None;
        }

        Some(Table {
            data,
            variation_store_offset: s.read()?,
            advance_width_mapping_offset: s.read()?,
            lsb_mapping_offset: s.read()?,
        })
    }
}


pub struct DeltaSetIndexMap<'a> {
    data: &'a [u8],
}

impl<'a> DeltaSetIndexMap<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        DeltaSetIndexMap { data }
    }

    #[inline]
    pub fn map(&self, glyph_id: GlyphId) -> Option<(u16, u16)> {
        let mut idx = glyph_id.0;

        let mut s = Stream::new(self.data);
        let entry_format: u16 = s.read()?;
        let map_count: u16 = s.read()?;

        if map_count == 0 {
            return None;
        }

        // 'If a given glyph ID is greater than mapCount-1, then the last entry is used.'
        if idx >= map_count {
            idx = map_count - 1;
        }

        let entry_size = ((entry_format >> 4) & 3) + 1;
        let inner_index_bit_count = ((entry_format & 0xF) + 1) as u32;

        s.advance(entry_size as u32 * idx as u32);

        let mut n = 0u32;
        for b in s.read_bytes(entry_size)? {
            n = (n << 8) + *b as u32;
        }

        let outer_index = n >> inner_index_bit_count;
        let inner_index = n & ((1 << inner_index_bit_count) - 1);
        Some((outer_index as u16, inner_index as u16))
    }
}

#[inline]
pub(crate) fn glyph_advance_offset(
    table: Table,
    glyph_id: GlyphId,
    coordinates: &[NormalizedCoord],
) -> Option<f32> {
    let (outer_idx, inner_idx) = if let Some(offset) = table.advance_width_mapping_offset {
        DeltaSetIndexMap::new(table.data.get(offset.to_usize()..)?).map(glyph_id)?
    } else {
        let outer_index = glyph_id.0 as u32 >> 16;
        let inner_index = glyph_id.0 as u32 & 0xFFFF;
        (outer_index as u16, inner_index as u16)
    };

    let mut s2 = Stream::new_at(table.data, table.variation_store_offset.to_usize());
    crate::mvar::parse_item_variation_store(outer_idx, inner_idx, coordinates, &mut s2)
}

#[inline]
pub(crate) fn glyph_side_bearing_offset(
    table: Table,
    glyph_id: GlyphId,
    coordinates: &[NormalizedCoord],
) -> Option<f32> {
    let set_data = table.data.get(table.lsb_mapping_offset?.to_usize()..)?;
    let (outer_idx, inner_idx) = DeltaSetIndexMap::new(set_data).map(glyph_id)?;

    let mut s2 = Stream::new_at(table.data, table.variation_store_offset.to_usize());
    crate::mvar::parse_item_variation_store(outer_idx, inner_idx, coordinates, &mut s2)
}
