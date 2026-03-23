use modular_bitfield_msb::{BitfieldSpecifier, bitfield, specifiers::*};

#[bitfield(bytes = 12)]
#[derive(BitfieldSpecifier, Debug)]
pub struct Header {
    pub id: B16,
    pub qr: bool,
    pub opcode: B4,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: B3,
    pub rcode: B4,
    pub qdcount: B16,
    pub ancount: B16,
    pub nscount: B16,
    pub arcount: B16,
}
