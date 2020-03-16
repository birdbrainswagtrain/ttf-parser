// https://docs.microsoft.com/en-us/typography/opentype/spec/avar

use core::convert::TryFrom;

use crate::parser::{Stream, LazyArray16, F2DOT14};
use crate::raw::avar as raw;

pub fn map_variation_coordinates(data: &[u8], coordinates: &mut [F2DOT14]) -> Option<()> {
    let mut s = Stream::new(data);

    let version: u32 = s.read()?;
    if version != 0x00010000 {
        return None;
    }

    s.skip::<u16>(); // reserved
    // TODO: check that `axisCount` is the same as in `fvar`?
    let axis_count = s.read::<u16>()? as usize;
    if axis_count != coordinates.len() {
        return None;
    }

    for i in 0..axis_count {
        let map = s.read_array16::<raw::AxisValueMapRecord>()?;
        coordinates[i] = F2DOT14(map_value(&map, coordinates[i].0));
    }

    Some(())
}

fn map_value(map: &LazyArray16<raw::AxisValueMapRecord>, value: i16) -> i16 {
    // This code is based on harfbuzz implementation.

    if map.len() == 0 {
        return value;
    } else if map.len() == 1 {
        let record = map.at(0);
        return value - record.from_coordinate() + record.to_coordinate();
    }

    let record_0 = map.at(0);
    if value <= record_0.from_coordinate() {
        return value - record_0.from_coordinate() + record_0.to_coordinate();
    }

    let mut i = 1;
    while i < map.len() && value > map.at(i).from_coordinate() {
        i += 1;
    }

    if i == map.len() {
        i -= 1;
    }

    let record_i = map.at(i);
    if value >= record_i.from_coordinate() {
        return value - record_i.from_coordinate() + record_i.to_coordinate();
    }

    let record_prev = map.at(i - 1);
    if record_prev.from_coordinate() == record_i.from_coordinate() {
        return record_prev.to_coordinate();
    }

    let denom = record_i.from_coordinate() as i32 - record_prev.from_coordinate() as i32;
    let value = record_prev.to_coordinate() as i32 +
        ((record_i.to_coordinate() as i32 - record_prev.to_coordinate() as i32) *
            (value as i32 - record_prev.from_coordinate() as i32) + denom / 2) / denom;
    i16::try_from(value).unwrap_or(0)
}
