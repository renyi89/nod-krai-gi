use nod_krai_gi_encryption::xor::MhyXorpad;

pub fn xor_packet(
    session_xorpad: Option<&MhyXorpad>,
    initial_xorpad: Option<&MhyXorpad>,
    buf: &mut [u8],
) {
    match (session_xorpad, initial_xorpad) {
        (Some(xorpad), _) => xorpad.xor(buf),
        (None, Some(xorpad)) => xorpad.xor(buf),
        (_, _) => (),
    }
}
