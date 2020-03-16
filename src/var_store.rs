//! Implementation of Item Variation Store
//!
//! https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#item-variation-store

use crate::NormalizedCoord;
use crate::parser::{Stream, LazyArray16};

#[derive(Clone, Copy)]
pub struct VariationRegionList<'a> {
    axis_count: u16,
    regions: LazyArray16<'a, crate::raw::mvar::RegionAxisCoordinatesRecord>,
}

impl<'a> VariationRegionList<'a> {
    pub fn evaluate_region(
        &self,
        index: u16,
        coordinates: &[NormalizedCoord],
    ) -> f32 {
        let mut v = 1.0;
        for (i, coord) in coordinates.iter().enumerate() {
            let region = match self.regions.get(index * self.axis_count + i as u16) {
                Some(r) => r,
                None => return 0.0,
            };

            let factor = region.evaluate_axis(coord.get());
            if factor == 0.0 {
                return 0.0;
            }

            v *= factor;
        }

        v
    }
}

impl crate::raw::mvar::RegionAxisCoordinatesRecord {
    pub fn evaluate_axis(&self, coord: i16) -> f32 {
        let start = self.start_coord();
        let peak = self.peak_coord();
        let end = self.end_coord();

        if start > peak || peak > end {
            return 1.0;
        }

        if start < 0 && end > 0 && peak != 0 {
            return 1.0;
        }

        if peak == 0 || coord == peak {
            return 1.0;
        }

        if coord <= start || end <= coord {
            return 0.0;
        }

        if coord < peak {
            (coord - start) as f32 / (peak - start) as f32
        } else {
            (end - coord) as f32 / (end - peak) as f32
        }
    }
}

#[derive(Clone, Copy)]
pub struct ItemVariationStore<'a> {
    data: &'a [u8],
    data_offsets: LazyArray16<'a, u32>,
    pub regions: VariationRegionList<'a>,
}

impl<'a> Default for ItemVariationStore<'a> {
    #[inline]
    fn default() -> Self {
        ItemVariationStore {
            data: &[],
            data_offsets: LazyArray16::new(&[]),
            regions: VariationRegionList {
                axis_count: 0,
                regions: LazyArray16::new(&[]),
            }
        }
    }
}

impl<'a> ItemVariationStore<'a> {
    pub fn new(mut s: Stream) -> Option<ItemVariationStore> {
        let data = s.tail()?;

        let mut regions_s = s.clone();
        let format: u16 = s.read()?;
        if format != 1 {
            return None;
        }

        let region_list_offset: u32 = s.read()?;
        let offsets = s.read_array16::<u32>()?;

        let regions = {
            regions_s.advance(region_list_offset);
            // TODO: should be the same as in `fvar`
            let axis_count = regions_s.read::<u16>()?;
            let count = regions_s.read::<u16>()?;
            let total = count.checked_mul(axis_count)?;
            VariationRegionList {
                axis_count,
                regions: regions_s.read_array(total)?,
            }
        };

        Some(ItemVariationStore { data, data_offsets: offsets, regions })
    }

    pub fn region_indices(&self, index: u16) -> Option<LazyArray16<u16>> {
        // Offsets in bytes from the start of the item variation store
        // to each item variation data subtable.
        let offset = self.data_offsets.get(index)?;
        let mut s = Stream::new_at(self.data, offset as usize);
        s.skip::<u16>(); // item_count
        s.skip::<u16>(); // short_delta_count
        s.read_array16()
    }
}
