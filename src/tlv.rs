use crate::{SafeRead, SafeWrite};
use eyre::Result;

/// TLV field order
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Fields {
    Tag,
    Length,
    Checksum,
    Value,
    None,
}

pub enum Endianess {
    Big,
    Little,
}

pub struct TlvType {
    /// Order of fields. If not all fields are present, fill with Fields::None
    pub fields: [Fields; 4],

    /// Endianess of length and checksum fields
    pub endianess: Endianess,

    /// Does length field cover TYPE field
    pub length_includes_type: bool,

    /// Does length field cover checksum field
    pub length_includes_csum: bool,

    /// Does checksum cover TYPE field
    pub csum_includes_type: bool,

    /// Does checksum cover length field
    pub csum_includes_length: bool,
}

/// PNG chunk descriptor
pub const TLV_PNG: TlvType = TlvType {
    fields:               [Fields::Length, Fields::Tag, Fields::Value, Fields::Checksum],
    endianess:            Endianess::Big,
    length_includes_type: false,
    length_includes_csum: false,
    csum_includes_type:   true,
    csum_includes_length: false,
};

pub struct Chunk {
    typ:      [u8; 4],
    checksum: Option<u32>,
    data:     Vec<u8>,
}

pub fn read<SR>(src: &mut SR, typ: &TlvType) -> Result<Chunk>
where
    SR: SafeRead,
{
    unimplemented!();
}

impl Chunk {
    pub fn write<SW>(&self, dst: &mut SW, typ: &TlvType) -> Result<()>
    where
        SW: SafeWrite,
    {
        unimplemented!();
    }
}
