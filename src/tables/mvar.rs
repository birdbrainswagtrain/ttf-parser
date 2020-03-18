// https://docs.microsoft.com/en-us/typography/opentype/spec/mvar

use crate::{Tag, NormalizedCoord};
use crate::parser::{Stream, Offset, Offset16, Offset32, LazyArray16};
use crate::raw::mvar as raw;

#[derive(Clone, Copy)]
pub struct Table<'a> {
    data: &'a [u8],
    variation_store_offset: Offset16,
    records: LazyArray16<'a, raw::ValueRecord>,
}

impl<'a> Table<'a> {
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version: u32 = s.read()?;
        if version != 0x00010000 {
            return None;
        }

        s.skip::<u16>(); // reserved
        let value_record_size: u16 = s.read()?;

        if value_record_size as usize != raw::ValueRecord::SIZE {
            return None;
        }

        let count: u16 = s.read()?;
        if count == 0 {
            return None;
        }

        let variation_store_offset = s.read::<Option<Offset16>>()??;
        let records = s.read_array16::<raw::ValueRecord>(count)?;

        Some(Table {
            data,
            variation_store_offset,
            records,
        })
    }
}

pub(crate) fn metrics_offset(
    table: &Table,
    tag: Tag,
    coordinates: &[NormalizedCoord],
) -> Option<f32> {
    let (_, record) = table.records.binary_search_by(|r| r.value_tag().cmp(&tag))?;
    let mut s2 = Stream::new_at(table.data, table.variation_store_offset.to_usize());
    parse_item_variation_store(
        record.delta_set_outer_index(), record.delta_set_inner_index(), coordinates, &mut s2,
    )
}

// TODO: merge with var_store
pub(crate) fn parse_item_variation_store(
    outer_index: u16,
    inner_index: u16,
    coordinates: &[NormalizedCoord],
    s: &mut Stream,
) -> Option<f32> {
    let orig = s.clone();

    let format: u16 = s.read()?;
    if format != 1 {
        return None;
    }

    let variation_region_list_offset: Offset32 = s.read()?;
    let count: u16 = s.read()?;
    let item_variation_data_offsets = s.read_array16::<Offset32>(count)?;

    let var_data_offset = item_variation_data_offsets.get(outer_index)?;
    let mut s = orig.clone();
    s.advance(var_data_offset.to_usize());

    let mut region_s = orig.clone();
    region_s.advance(variation_region_list_offset.to_usize());

    parse_item_variation_data(inner_index, coordinates, &mut s, region_s)
}

fn parse_item_variation_data(
    inner_index: u16,
    coordinates: &[NormalizedCoord],
    s: &mut Stream,
    region_s: Stream,
) -> Option<f32> {
    let item_count: u16 = s.read()?;
    if inner_index >= item_count {
        return None;
    }

    let short_delta_count: u16 = s.read()?;
    let region_index_count: u16 = s.read()?;
    let region_indexes = s.read_array16::<u16>(region_index_count)?;
    s.advance(usize::from(inner_index).checked_mul(
        usize::from(short_delta_count) + usize::from(region_index_count))?);

    let mut delta = 0.0;
    let mut i = 0;
    while i < short_delta_count {
        let idx = region_indexes.get(i)?;
        delta += f32::from(s.read::<i16>()?) * evaluate_region(idx, coordinates, region_s)?;
        i += 1;
    }

    while i < region_index_count {
        let idx = region_indexes.get(i)?;
        delta += f32::from(s.read::<i8>()?) * evaluate_region(idx, coordinates, region_s)?;
        i += 1;
    }

    Some(delta)
}

fn evaluate_region(
    index: u16,
    coordinates: &[NormalizedCoord],
    mut s: Stream,
) -> Option<f32> {
    let axis_count: u16 = s.read()?;
    s.skip::<u16>(); // region_count
    s.advance(usize::from(index)
        .checked_mul(usize::from(axis_count))?
        .checked_mul(raw::RegionAxisCoordinatesRecord::SIZE)?);

    let mut v = 1.0;
    for i in 0..axis_count {
        let record: raw::RegionAxisCoordinatesRecord = s.read()?;
        let coord = coordinates.get(usize::from(i)).cloned().unwrap_or_default();
        let factor = record.evaluate_axis(coord.get());
        if factor == 0.0 {
            return Some(0.0);
        }

        v *= factor;
    }

    Some(v)
}
