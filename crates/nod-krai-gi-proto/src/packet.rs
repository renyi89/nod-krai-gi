use crate::{packet_head::PacketHead, Protobuf, ProtobufDecodeError};
use byteorder::{ByteOrder, BE};

pub struct NetPacket<Proto> {
    pub head: PacketHead,
    pub body: Proto,
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("head magic mismatch")]
    HeadMagicMismatch,
    #[error("tail magic mismatch")]
    TailMagicMismatch,
    #[error("input buffer is less than overhead, len: {0}, overhead: {1}")]
    InputLessThanOverhead(usize, usize),
    #[error("out of bounds ({0}/{1})")]
    OutOfBounds(usize, usize),
    #[error("failed to decode PacketHead: {0}")]
    HeadDecode(ProtobufDecodeError),
    #[error("failed to decode body: {0}")]
    BodyDecode(ProtobufDecodeError),
}

#[derive(thiserror::Error, Debug)]
pub enum ProtocolConversionError {
    #[error("failed to decode: {0}")]
    Decode(#[from] ProtobufDecodeError),
    #[error("normal proto for cmd_id: {0} not found")]
    NotFound(u16),
    #[error("normal proto for arg_type: {0:?} not found")]
    NotFoundCombatArgument(crate::normal::CombatTypeArgument),
}

const OVERHEAD: usize = 12;
const HEAD_MAGIC: [u8; 2] = 0x4567_u16.to_be_bytes();
const TAIL_MAGIC: [u8; 2] = 0x89AB_u16.to_be_bytes();

pub fn read_cmd_id(buf: &[u8]) -> Result<u16, DecodeError> {
    if buf.len() < OVERHEAD {
        return Err(DecodeError::InputLessThanOverhead(buf.len(), OVERHEAD));
    }

    (buf[0..2] == HEAD_MAGIC)
        .then_some(BE::read_u16(&buf[2..4]))
        .ok_or(DecodeError::HeadMagicMismatch)
}

pub fn decode_head(buf: &[u8]) -> Option<PacketHead> {
    is_well_formed(buf)
        .then_some(PacketHead::decode(&buf[10..][..BE::read_u16(&buf[4..6]) as usize]).ok())
        .flatten()
}

pub fn is_well_formed(buf: &[u8]) -> bool {
    buf.len() >= OVERHEAD
        && buf[0..2] == HEAD_MAGIC
        && OVERHEAD + BE::read_u16(&buf[4..6]) as usize + BE::read_u32(&buf[6..10]) as usize
            >= buf.len()
        && buf[10 + BE::read_u16(&buf[4..6]) as usize + BE::read_u32(&buf[6..10]) as usize..][..2]
            == TAIL_MAGIC
}
