extern crate hex;

use colored::*;

use bit_vec::BitVec;

use crate::util;

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

fn describe_inner_light(inner_light: u8) -> &'static str {
    match inner_light {
        0..20 => "Ignorant",
        201..240 => "Intelligent",
        241..255 => "Enlightened",
        255 => "Genius",
        _ => ""
    }
}

fn describe_presence(presence: u8) -> &'static str {
    match presence {
        0 => "Invisible",
        1..5 => "Ghostly",
        5..20 => "Shadowy",
        210..250 => "Practical",
        220..255 => "Shimmering",
        _ => ""
    }
}

fn describe_charm(charm: u8) -> &'static str {
    match charm {
        0..5 => "Brutal",
        5..15 => "Unfriendly",
        230..250 => "Frendly",
        250..255 => "Charming",
        _ => ""
    }
}

fn describe_strangeness(strangeness: u8) -> &'static str {
    match strangeness {
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

    // Strength
    let high_bytes = util::from_slice_to_sixteen_u8(&b[0..16]);
    let low_bytes = util::from_slice_to_sixteen_u8(&b[16..32]);
    let strength = u128::from_be_bytes(high_bytes).count_ones() +
        u128::from_be_bytes(low_bytes).count_ones();

    print!("Strength: {}", strength);
    if strength > 140 {
        println!("  (Powerful)");
    } else {
        println!();
    }

    let virtues = Virtues {
        identity: BitVec::from_bytes(&b[1..3]),
        inner_light: b[3],
        colour: util::from_slice_to_three_u8(&b[4..7]),
        presence: b[7],
        charm: b[8],
        strangeness: b[9],
        beauty: b[10],
        truth: b[11],
        magic: b[12],
        special_powers: BitVec::from_bytes(&b[13..16]),
        manifestation: u32::from_be_bytes(util::from_slice_to_four_u8(&b[16..20])),
        arcana: u32::from_be_bytes(util::from_slice_to_four_u8(&b[20..24])),
        cabala: u32::from_be_bytes(util::from_slice_to_four_u8(&b[24..28])),
        maturity: u16::from_be_bytes(util::from_slice_to_two_u8(&b[28..30])),
        sigil: u16::from_be_bytes(util::from_slice_to_two_u8(&b[30..32]))
    };

    let inner_light = describe_inner_light(virtues.inner_light);
    print_virtue("Inner Light", virtues.inner_light, inner_light);

    // Colour
    println!("Colour: {}", "■■■■".truecolor(virtues.colour[0],
                                            virtues.colour[1],
                                            virtues.colour[2]));


    // Presence
    let presence = describe_presence(virtues.presence);
    print_virtue("Presence", virtues.presence, presence);

    // Charm
    let charm = describe_charm(virtues.charm);
    print_virtue("Charm", virtues.charm, charm);

    // Strangeness
    let strangeness = describe_strangeness(virtues.strangeness);
    print_virtue("Strangeness", virtues.strangeness, strangeness);

    // Beauty
    let beauty = describe_beauty(virtues.beauty);
    print_virtue("Beauty", virtues.beauty, beauty);

    // Truth
    let truth = describe_truth(virtues.truth);
    print_virtue("Truth", virtues.truth, truth);

    // Magic
    let magic = describe_magic(virtues.magic);
    print_virtue("Magic", virtues.magic, magic);

    println!("Special Powers: {:?}", virtues.special_powers);
    println!("Manifestation: {}", virtues.manifestation);
    println!("Arcana: {}", virtues.arcana);
    println!("Cabala: {}", virtues.cabala);
    println!("Maturity: {}", virtues.maturity);
    println!("Sigil: {}", util::bytes_to_codepoints(&virtues.sigil.to_be_bytes()));

    Ok(())
}
