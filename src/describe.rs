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


// FIXME There must be a better to do this in Rust, i.e. with templates
fn from_slice_to_four_u8(slice: &[u8]) -> [u8;4] {
    return <[u8; 4]>::try_from(slice).unwrap();
}

fn from_slice_to_three_u8(slice: &[u8]) -> [u8;3] {
    return <[u8; 3]>::try_from(slice).unwrap();
}


fn from_slice_to_two_u8(slice: &[u8]) -> [u8;2] {
    return <[u8; 2]>::try_from(slice).unwrap();
}

fn from_slice_to_sixteen_u8(slice: &[u8]) -> [u8;16] {
    return <[u8; 16]>::try_from(slice).unwrap();
}


fn bytes_to_codepoints(slice: &[u8]) -> String {
    unsafe { [char::from_u32_unchecked(4608 + slice[0] as u32),
              char::from_u32_unchecked(4608 + slice[1] as u32)].iter().collect()
    }
}

fn describe_inner_light(inner_light: u8) -> &'static str {
    match inner_light {
        0..20 => "Ignorant",
        200..240 => "Intelligent",
        240..255 => "Enlightened",
        255 => "Genius",
        _ => ""
    }
}

fn describe_presence(presence: u8) -> &'static str {
    match presence {
        0 => "Invisible",
        1..10 => "Ghost",
        10..40 => "Shadow",
        40..60 => "Shimmering",
        230..255 => "Rock",
        255 => "Marble",
        _ => ""
    }
}

fn describe_charm(charm: u8) -> &'static str {
    match charm {
        0..5 => "Brutal",
        5..15 => "Unfriendly",
        190..230 => "Frendly",
        230..250 => "Charming",
        250..255 => "Charismatic",
        _ => ""
    }
}

fn describe_strangeness(strangeness: u8) -> &'static str {
    match strangeness {
        0..10 => "Practical",
        200..240 => "Strange",
        240..255 => "Weird",
        _ => ""
    }
}

fn describe_beauty(beauty: u8) -> &'static str {
    match beauty {
        0..10 => "Ugly",
        10..20 => "Unattractive",
        200..230 => "Attractive",
        230..250 => "Beautiful",
        250..255 => "Exquisite",
        _ => ""
    }
}

fn describe_truth(truth: u8) -> &'static str {
    match truth {
        0..5 => "Lying",
        5..20 => "Dishonest",
        220..250 => "Honest",
        250..255 => "Oracular",
        _ => ""
    }
}

fn describe_magic(magic: u8) -> &'static str {
    match magic {
        0..20 => "Clumsy",
        210..250 => "Magical",
        250..255 => "Legendary",
        255 => "Mythical",
        _ => ""
    }
}

fn print_virtue(name: &str, value:u8, description: &str) {
    print!("{}: {}%", name, value as f32 * 100.0 / 200.0);
    if description != "" {
        println!("  ({})", description);
    } else {
        println!();
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


    // Strength
    let high_bytes = from_slice_to_sixteen_u8(&b[0..16]);
    let low_bytes = from_slice_to_sixteen_u8(&b[16..32]);
    let strength = u128::from_be_bytes(high_bytes).count_ones() +
        u128::from_be_bytes(low_bytes).count_ones();

    let inner_light = describe_inner_light(virtues.inner_light);
    let presence = describe_presence(virtues.presence);
    let charm = describe_charm(virtues.charm);
    let strangeness = describe_strangeness(virtues.strangeness);
    let beauty = describe_beauty(virtues.beauty);
    let truth = describe_truth(virtues.truth);
    let magic = describe_magic(virtues.magic);

    let virtues_description = vec![inner_light, presence, charm, strangeness, beauty, truth, magic];
    let personality = virtues_description.iter().filter(|&s| s.len() > 0)
        .cloned()
        .collect::<Vec<&str>>()
        .join(", ");
    if personality.len() == 0 {
        println!("Unremarkable.");
    } else {
        println!("{}.", personality);
    }
    println!();


    print!("Strength: {}", strength);
    if strength > 140 {
        println!("  (Powerful)");
    } else {
        println!();
    }

    print_virtue("Inner Light", virtues.inner_light, inner_light);

    // Colour
    println!("Colour: {}", "■■■■".truecolor(virtues.colour[0],
                                            virtues.colour[1],
                                            virtues.colour[2]));


    print_virtue("Presence", virtues.presence, presence);
    print_virtue("Charm", virtues.charm, charm);
    print_virtue("Strangeness", virtues.strangeness, strangeness);
    print_virtue("Beauty", virtues.beauty, beauty);
    print_virtue("Truth", virtues.truth, truth);
    print_virtue("Magic", virtues.magic, magic);

    println!("Special Powers: {:?}", virtues.special_powers);
    println!("Manifestation: {}", virtues.manifestation);
    println!("Arcana: {}", virtues.arcana);
    println!("Cabala: {}", virtues.cabala);
    println!("Maturity: {}", virtues.maturity);
    println!("Sigil: {}", bytes_to_codepoints(&virtues.sigil.to_be_bytes()));

    Ok(())
}
