use std::convert::TryFrom;


// FIXME There must be a better to do this in Rust, i.e. with templates
pub fn from_slice_to_four_u8(slice: &[u8]) -> [u8;4] {
    return <[u8; 4]>::try_from(slice).unwrap();
}

pub fn from_slice_to_three_u8(slice: &[u8]) -> [u8;3] {
    return <[u8; 3]>::try_from(slice).unwrap();
}


pub fn from_slice_to_two_u8(slice: &[u8]) -> [u8;2] {
    return <[u8; 2]>::try_from(slice).unwrap();
}

pub fn from_slice_to_eight_u8(slice: &[u8]) -> [u8;8] {
    return <[u8; 8]>::try_from(slice).unwrap();
}

pub fn from_slice_to_sixteen_u8(slice: &[u8]) -> [u8;16] {
    return <[u8; 16]>::try_from(slice).unwrap();
}

pub fn from_slice_to_thirtytwo_u8(slice: &[u8]) -> [u8;32] {
    return <[u8; 32]>::try_from(slice).unwrap();
}

pub fn bytes_to_codepoints(slice: &[u8]) -> String {
    unsafe { [char::from_u32_unchecked(4608 + slice[0] as u32),
              char::from_u32_unchecked(4608 + slice[1] as u32)].iter().collect()
    }
}


// Cf. https://stackoverflow.com/a/36676814/289466

pub fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) |
    ((array[1] as u32) << 16) |
    ((array[2] as u32) <<  8) |
    ((array[3] as u32) <<  0)
}

pub fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) |
    ((array[1] as u32) <<  8) |
    ((array[2] as u32) << 16) |
    ((array[3] as u32) << 24)
}

pub fn as_u64_be(array: &[u8; 8]) -> u64 {
    ((array[0] as u64) << 56) |
    ((array[1] as u64) << 48) |
    ((array[2] as u64) << 40) |
    ((array[3] as u64) << 32) |
    ((array[4] as u64) << 24) |
    ((array[5] as u64) << 16) |
    ((array[6] as u64) <<  8) |
    ((array[7] as u64) <<  0)
}

pub fn as_u64_le(array: &[u8; 8]) -> u64 {
    ((array[0] as u64) <<  0) |
    ((array[1] as u64) <<  8) |
    ((array[2] as u64) << 16) |
    ((array[3] as u64) << 24) |
    ((array[4] as u64) << 32) |
    ((array[5] as u64) << 40) |
    ((array[6] as u64) << 48) |
    ((array[7] as u64) << 56)
}
