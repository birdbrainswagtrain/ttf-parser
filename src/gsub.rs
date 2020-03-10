// https://docs.microsoft.com/en-us/typography/opentype/spec/gsub

use crate::ggg::*;

/// A reference to a [Glyph Substitution Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub).
#[derive(Clone, Copy)]
pub struct SubstitutionTable<'a> {
    pub(crate) table: GsubGposTable<'a>,
}

impl<'a> GlyphPosSubTable for SubstitutionTable<'a> {
    fn scripts(&self) -> Scripts {
        self.table.script
    }

    fn features(&self) -> Features {
        self.table.features
    }

    fn lookups(&self) -> Lookups {
        self.table.lookups
    }

    fn feature_variations(&self) -> FeatureVariations {
        self.table.feature_variations
    }
}

impl core::fmt::Debug for SubstitutionTable<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SubstitutionTable()")
    }
}
