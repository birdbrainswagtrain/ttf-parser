use crate::parser::{Stream, LazyArray16, FromData};

#[derive(Clone)]
struct Color{
    b: u8,
    g: u8,
    r: u8,
    a: u8
}

impl FromData for Color {
    const SIZE: usize = 4;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self{
            b: s.read()?,
            g: s.read()?,
            r: s.read()?,
            a: s.read()?,
        })
    }
}

/// Only implements version 0. Does not support palette background types or labels.
#[derive(Clone)]
pub struct Table<'a>{
    color_indices: LazyArray16<'a,u16>,
    colors: LazyArray16<'a,Color>,
    colors_per_palette: u16
}

pub(crate) fn parse(data: &[u8]) -> Option<Table> {
    let mut s = Stream::new(data);

    let _version: u16 = s.read()?;

    let colors_per_palette: u16 = s.read()?;
    let palette_count: u16 = s.read()?;
    let color_count: u16 = s.read()?;
    let color_offset: u32 = s.read()?;

    let color_indices: LazyArray16<u16> = s.read_array16(palette_count)?;

    Some(Table{
        color_indices,
        colors: Stream::new_at(data, color_offset as usize)?.read_array16(color_count)?,
        colors_per_palette
    })
}
