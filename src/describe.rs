extern crate hex;

use colored::*;

use std::convert::TryFrom;
use bit_vec::BitVec;

pub struct Virtues {
    pub identity: BitVec,
    pub inner_light: u8,
    pub colour: [u8;3],
    pub presence: u8,
    pub charm: u8,
    pub strangeness: u8,
    pub beauty: u8,
    pub truth: u8,
    pub magic: u8,
    pub special_powers: BitVec,
    pub manifestation: u32,
    pub arcana: u32,
    pub cabala: u32,
    pub maturity: u16,
    pub sigil: u16
}


fn from_slice_to_four_u8(slice: &[u8]) -> [u8;4] {
    return <[u8; 4]>::try_from(slice).unwrap();
}

fn from_slice_to_three_u8(slice: &[u8]) -> [u8;3] {
    return <[u8; 3]>::try_from(slice).unwrap();
}


fn from_slice_to_two_u8(slice: &[u8]) -> [u8;2] {
    return <[u8; 2]>::try_from(slice).unwrap();
}


fn bytes_to_codepoints(slice: &[u8]) -> String {
    unsafe { [char::from_u32_unchecked(4608 + slice[0] as u32),
              char::from_u32_unchecked(4608 + slice[1] as u32)].iter().collect()
    }
}

pub fn describe(hashdragon: String) -> Result<(), String> {
    if hashdragon.chars().count() != 64 {
        let msg = format!("hashdragon should be 64 chars, has {}.", hashdragon.chars().count());
        return Err(msg.to_string());
    }

    let b = hex::decode(hashdragon).expect("Decode failed.");
    if b[0] != 0xd4 {
        let msg = format!("First byte should be 0xd4, is {}", b[0]);
        return Err(msg.to_string());
    }

    let virtues = Virtues {
        identity: BitVec::from_bytes(&b[1..3]),
        inner_light: b[3],
        colour: from_slice_to_three_u8(&b[4..7]),
        presence: b[7],
        charm: b[8],
        strangeness: b[9],
        beauty: b[10],
        truth: b[11],
        magic: b[12],
        special_powers: BitVec::from_bytes(&b[13..16]),
        manifestation: u32::from_be_bytes(from_slice_to_four_u8(&b[16..20])),
        arcana: u32::from_be_bytes(from_slice_to_four_u8(&b[20..24])),
        cabala: u32::from_be_bytes(from_slice_to_four_u8(&b[24..28])),
        maturity: u16::from_be_bytes(from_slice_to_two_u8(&b[28..30])),
        sigil: u16::from_be_bytes(from_slice_to_two_u8(&b[30..32]))
    };

    //    println!("Identity: {:?}", virtues.identity.to_bytes());
    println!("Inner Light: {}%", virtues.inner_light as f32 * 100.0 / 200.0);
    println!("Colour: {}", "■■■■".truecolor(virtues.colour[0],
                                                 virtues.colour[1],
                                                 virtues.colour[2]));
    println!("Presence: {}%", virtues.presence as f32 * 100.0 / 200.0);
    println!("Charm: {}%", virtues.charm as f32 * 100.0 / 200.0);
    println!("Strangeness: {}%", virtues.strangeness as f32 * 100.0 / 200.0);
    println!("Beauty: {}%", virtues.beauty as f32 * 100.0 / 200.0);
    println!("Truth: {}%", virtues.truth as f32 * 100.0 / 200.0);
    println!("Magic: {}%", virtues.magic as f32 * 100.0 / 200.0);
    println!("Special Powers: {:?}", virtues.special_powers);
    println!("Manifestation: {}", virtues.manifestation);
    println!("Arcana: {}", virtues.arcana);
    println!("Cabala: {}", virtues.cabala);
    println!("Maturity: {}", virtues.maturity);
    println!("Sigil: {}", bytes_to_codepoints(&virtues.sigil.to_be_bytes()));

    Ok(())
}
