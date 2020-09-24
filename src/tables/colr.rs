use crate::parser::{Stream, LazyArray16, FromData};

#[derive(Clone)]
struct BaseGlyph{
    ref_glyph_id: u16,
    layer_index: u16,
    layer_count: u16
}

#[derive(Clone)]
struct Layer{
    glyph_id: u16,
    palette_index: u16
}

impl FromData for BaseGlyph {
    const SIZE: usize = 6;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self{
            ref_glyph_id: s.read()?,
            layer_index: s.read()?,
            layer_count: s.read()?
        })
    }
}

impl FromData for Layer {
    const SIZE: usize = 4;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self{
            glyph_id: s.read()?,
            palette_index: s.read()?
        })
    }
}

#[derive(Clone)]
pub struct Table<'a>{
    base_glyphs: LazyArray16<'a,BaseGlyph>,
    layers: LazyArray16<'a,Layer>
}

pub(crate) fn parse(data: &[u8]) -> Option<Table> {
    let mut s = Stream::new(data);

    let _version: u16 = s.read()?;

    let base_glyphs_count: u16 = s.read()?;
    let base_glyphs_offset: u32 = s.read()?;
    let layers_offset: u32 = s.read()?;
    let layers_count: u16 = s.read()?;

    Some(Table{
        base_glyphs: Stream::new_at(data, base_glyphs_offset as usize)?.read_array16(base_glyphs_count)?,
        layers: Stream::new_at(data, layers_offset as usize)?.read_array16(layers_count)?,
    })
}
