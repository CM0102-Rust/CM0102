// ---------- sx10_7: the formation's slots and pitch coordinates (formation.cpp, tactics screens) ----------

/// "Formation vs {}<%s - club name>{}": the name of a formation built from a match.
const FORMATION_VS_CLUB_NAME: usize = 0x9cc3b8;
/// Formation::is_slot_coordinate_customised (thiscall, ret 0xc).
const FN_FORMATION_IS_SLOT_COORDINATE_CUSTOMISED: usize = 0x5b4b70;
/// The line bits of a general position code, goalkeeper to forward (the original's local word table, ended by 0xffff).
const FORMATION_LINE_BITS: [u16; 7] = [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40];
/// The largest pitch coordinate across (x 0..0x1a) and along (y 0..0x23).
const FORMATION_MAX_X: i16 = 0x1a;
const FORMATION_MAX_Y: i16 = 0x23;

/// A slot argument (its low byte, signed) is 0..10.
fn formation_slot_ok(slot: u32) -> bool {
    (0..=10).contains(&(slot as u8 as i8))
}

/// A phase argument (its low byte, signed) is 0 (without the ball) or 1 (with it).
fn formation_stage_ok(stage: u32) -> bool {
    (0..=1).contains(&(stage as u8 as i8))
}

/// A ball zone argument: band 0..2 in the low word, height 0..3 in the high word (both signed).
fn formation_zone_ok(zone: u32) -> bool {
    (0..=2).contains(&(zone as u16 as i16)) && (0..=3).contains(&((zone >> 16) as u16 as i16))
}

/// The general position code of a slot (`formation_fields::SLOT_CELLS`).
unsafe fn formation_general(this: usize, slot: usize) -> u16 {
    read_u16(this + formation_fields::SLOT_CELLS + 2 * slot)
}

/// Gives a slot the general position `position` the way the original does it inline: the mask redone
/// (`Formation::update_shape_code`), the slot's display order 1 and 0 on every other slot with the same position.
unsafe fn formation_take_position(this: usize, slot: usize, position: u16) {
    write_u16(this + formation_fields::SLOT_CELLS + 2 * slot, position);
    exe::formation_update_shape_code(this);
    write_u8(this + formation_fields::DISPLAY_ORDER + slot, 1);
    for k in 0..11usize {
        if k != slot && formation_general(this, k) == position {
            write_u8(this + formation_fields::DISPLAY_ORDER + k, 0);
        }
    }
}

/// Builds a formation from one side of a match record, named after the opposing club.
///
/// `Formation::Formation(record, side)`: no record: the formation.cpp 0xb5 box and the record is left as it was (not
/// even cleared: a defect of the original). Otherwise the name is "Formation vs <club>" with the short name of the other
/// side's club (side 0 gives the away club, `match_record::AWAY_CLUB_ID`; any other the home club) and its gender twice
/// (`langlib_sprintf`); the password and creator empty and the display order zeroed; each of the 11 slots gets the
/// side's general position (the word at side block +0x154 + 2 × slot) and then its extra position (+0x16a + 2 × slot)
/// through `Formation::set_slot_position`; the team instructions from the side block +0x180; then, as in the default
/// constructor, each player's instructions 0x15552221 with 0xa, marking types 0x12, no designated people, the position
/// lookup rebuilt, no password, not saved and changed = 1. The rest of the record is not touched. Returns `this`.
/// Used by the tactics screens to show the opposition's formation of a match.
/// Original: `Formation::Formation` (0x5b2930, formation.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn formation_new_vs_club(this: usize, record: usize, side: u32) -> usize {
    if record == 0 {
        formation_error_box(0xb5);
        return this;
    }
    let side = side as u8 as i8;
    let opponent = club_at(read_i32(record + if side == 0 { match_record::AWAY_CLUB_ID } else { match_record::HOME_CLUB_ID }) as isize);
    let gender = opponent.short_gender_name() as i8 as i32;
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    langlib(this, FORMATION_VS_CLUB_NAME, gender, gender, opponent.addr() + club::SHORT_NAME);
    write_u8(this + formation_fields::PASSWORD, 0);
    write_u8(this + formation_fields::CREATOR, 0);
    core::ptr::write_bytes((this + formation_fields::DISPLAY_ORDER) as *mut u8, 0, 11);
    let block = (record as isize + side as isize * match_record::SIDE_SIZE as isize) as usize;
    for slot in 0..11usize {
        // the side block's general (+0x154) and extra (+0x16a) position words of each slot
        exe::formation_set_slot_position(this, slot as u32, read_u16(block + 0x154 + 2 * slot) as u32, 0); // sx10_7: the side's general positions
        exe::formation_set_slot_position(this, slot as u32, read_u16(block + 0x16a + 2 * slot) as u32, 1);
    }
    write_u32(this + formation_fields::FLAGS, read_u32(block + 0x180)); // the side block's team instructions
    for k in 0..11usize {
        write_u64(this + formation_fields::PLAYER_INSTRUCTIONS + 8 * k, 0x0000_000a_1555_2221);
        write_u8(this + formation_fields::MARKING_TYPES + k, 0x12);
        write_i32(this + formation_fields::DESIGNATED_PEOPLE + 4 * k, -1);
    }
    exe::formation_build_position_lookup(this);
    write_u32(this + formation_fields::HAS_PASSWORD, 0);
    write_u32(this + 0x1887, 0); // saved [inference]
    write_u32(this + formation_fields::CHANGED_FLAG, 1);
    covered!("sx10_7 formation vs club");
    this
}

/// Swaps two slots completely.
///
/// `Formation::swap_slots(a, b)`: a slot outside 0..10: the formation.cpp box (0x1df for `a`, 0x1e7 for `b`) and 0.
/// The general positions are exchanged one at a time, each only if it differs: `a` takes `b`'s (then the display order
/// of `a` is 1 and 0 on every other slot with that position, `b` included) and `b` takes `a`'s old one (display order 1,
/// 0 on the others with it), each redoing the mask (`Formation::update_shape_code`). Then the extra positions, the 24
/// pitch coordinates of each slot (both phases, every zone), the player instructions, the marking types and the
/// designated people are exchanged (`utils_swap_bytes`). Changed = 0. Returns 1.
/// Original: `Formation::swap_slots` (0x5b2f30, formation.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn formation_swap_slots(this: usize, a: u32, b: u32) -> u32 {
    if !formation_slot_ok(a) {
        formation_error_box(0x1df);
        return 0;
    }
    if !formation_slot_ok(b) {
        formation_error_box(0x1e7);
        return 0;
    }
    let (a, b) = (a as u8 as usize, b as u8 as usize);
    let old_a = formation_general(this, a);
    if old_a != formation_general(this, b) {
        formation_take_position(this, a, formation_general(this, b)); // sx10_7: a takes b's position
    }
    if formation_general(this, b) != old_a {
        formation_take_position(this, b, old_a);
    }
    let swap = |offset: usize, size: usize| exe::utils_swap_bytes((this + offset + size * a) as *mut i32, (this + offset + size * b) as *mut i32, size as u32);
    swap(formation_fields::EXTRA_SLOT_CELLS, 2);
    swap(formation_fields::SLOT_COORDINATES, 0x60); // ponytail: one swap of the 0x60 bytes; the original makes 24 of 4
    swap(formation_fields::PLAYER_INSTRUCTIONS, 8);
    swap(formation_fields::MARKING_TYPES, 1);
    swap(formation_fields::DESIGNATED_PEOPLE, 4);
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    covered!("sx10_7 swap slots");
    1
}

/// The sort key of a general position code: its line (goalkeeper 0, sweeper 10, defender 20, defensive midfielder 30,
/// midfielder 40, attacking midfielder 50, forward 60) plus its side (left 0, right 1, centre left 2, centre and the
/// goalkeeper 3, centre right 4). A code without a line (formation.cpp 0xe50) or without a side (0xe79) shows the box
/// and gives 0.
unsafe fn formation_position_order(code: u16) -> i16 {
    let line = match FORMATION_LINE_BITS.iter().position(|&bit| code & bit != 0) {
        Some(k) => 10 * k as i16,
        None => {
            formation_error_box(0xe50);
            return 0;
        }
    };
    if code & 0x80 != 0 {
        line
    } else if code & 0x100 != 0 {
        line + 2
    } else if code & 0x200 != 0 || code & 1 != 0 {
        line + 3
    } else if code & 0x400 != 0 {
        line + 4
    } else if code & 0x800 != 0 {
        line + 1
    } else {
        formation_error_box(0xe79);
        0
    }
}

/// Lists the 11 slots from the goalkeeper forwards, left to right within each line.
///
/// `Formation::get_slots_in_position_order(out, size)`: no buffer: the formation.cpp 0x221 box and 0; a size below 11:
/// the 0x229 box and 0. Then 11 times the slot not listed yet with the smallest sort key of its general position (line
/// × 10 plus side: left, right, centre left, centre, centre right; the first one on a tie) goes into `out`. Returns 1.
/// Original: `Formation::get_slots_in_position_order` (0x5b31b0, formation.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn formation_slots_in_position_order(this: usize, out: usize, size: u32) -> u32 {
    if out == 0 {
        formation_error_box(0x221);
        return 0;
    }
    if (size as i32) < 11 {
        formation_error_box(0x229);
        return 0;
    }
    let mut listed = [false; 11];
    for k in 0..11usize {
        let mut best: i16 = 1000;
        for slot in 0..11usize {
            if listed[slot] {
                continue;
            }
            let key = formation_position_order(formation_general(this, slot));
            if key < best { // sx10_7: the smallest key first
                best = key;
                write_u8(out + k, slot as u8);
            }
        }
        listed[read_u8(out + k) as usize] = true;
    }
    1
}

/// Sets the pitch coordinates of all 11 slots for one phase and ball zone.
///
/// `Formation::set_zone_coordinates(phase, zone, coordinates)`: phase outside 0..1: the formation.cpp 0x383 box and 0;
/// zone outside band 0..2, height 0..3: 0x38c; no array: 0x394. Each slot k gets coordinate k of the array through
/// `Formation::set_slot_coordinate` (which also picks its positions). Changed = 0. Returns 1.
/// Original: `Formation::set_zone_coordinates` (0x5b3de0, formation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn formation_set_zone_coordinates(this: usize, stage: u32, zone: u32, coordinates: usize) -> u32 {
    let line = if !formation_stage_ok(stage) { 0x383 } else if !formation_zone_ok(zone) { 0x38c } else if coordinates == 0 { 0x394 } else { 0 };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    for slot in 0..11usize {
        exe::formation_set_slot_coordinate(this, slot as i32, stage as i32, zone, read_u32(coordinates + 4 * slot));
    }
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    covered!("sx10_7 set zone coordinates");
    1
}

/// Copies the pitch coordinates of all 11 slots for one phase and ball zone.
///
/// `Formation::get_zone_coordinates(phase, zone, out)`: phase outside 0..1: the formation.cpp 0x3b7 box and 0; zone
/// outside band 0..2, height 0..3: 0x3c0; no array: 0x3c8. Coordinate k of `out` is slot k's
/// (`Formation::get_slot_coordinate`). Returns 1.
/// Original: `Formation::get_zone_coordinates` (0x5b3f90, formation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn formation_get_zone_coordinates(this: usize, stage: u32, zone: u32, out: usize) -> u32 {
    let line = if !formation_stage_ok(stage) { 0x3b7 } else if !formation_zone_ok(zone) { 0x3c0 } else if out == 0 { 0x3c8 } else { 0 };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    for slot in 0..11usize {
        exe::formation_get_slot_coordinate(this, slot as i8, stage as i8, zone, (out + 4 * slot) as *mut u32);
    }
    1
}

/// Sets the pitch coordinates of every slot in every ball zone of one phase.
///
/// `Formation::set_phase_coordinates(phase, coordinates)`: phase outside 0..1: the formation.cpp 0x3e9 box and 0; no
/// array: 0x3f1. The array runs band 0..2, then height 0..3, then slot 0..10 (132 coordinates), each set through
/// `Formation::set_slot_coordinate`. Changed = 0. Returns 1.
/// Original: `Formation::set_phase_coordinates` (0x5b4120, formation.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn formation_set_phase_coordinates(this: usize, stage: u32, coordinates: usize) -> u32 {
    let line = if !formation_stage_ok(stage) { 0x3e9 } else if coordinates == 0 { 0x3f1 } else { 0 };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    let mut next = coordinates;
    for band in 0..3u32 {
        for height in 0..4u32 {
            for slot in 0..11i32 {
                exe::formation_set_slot_coordinate(this, slot, stage as i32, band | height << 16, read_u32(next));
                next += 4;
            }
        }
    }
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    covered!("sx10_7 set phase coordinates");
    1
}

/// Copies the pitch coordinates of every slot in every ball zone of one phase.
///
/// `Formation::get_phase_coordinates(phase, out)`: phase outside 0..1: the formation.cpp 0x41f box and 0; no array:
/// 0x427. Fills `out` band 0..2, then height 0..3, then slot 0..10 (`Formation::get_slot_coordinate`). Returns 1.
/// Original: `Formation::get_phase_coordinates` (0x5b42a0, formation.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn formation_get_phase_coordinates(this: usize, stage: u32, out: usize) -> u32 {
    let line = if !formation_stage_ok(stage) { 0x41f } else if out == 0 { 0x427 } else { 0 };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    let mut next = out;
    for band in 0..3u32 {
        for height in 0..4u32 {
            for slot in 0..11i8 {
                exe::formation_get_slot_coordinate(this, slot, stage as i8, band | height << 16, next as *mut u32);
                next += 4;
            }
        }
    }
    1
}

/// A pitch point argument: x 0..0x1a in the low word, y 0..0x23 in the high word (both signed).
fn formation_point_ok(point: u32) -> bool {
    (0..=FORMATION_MAX_X).contains(&(point as u16 as i16)) && (0..=FORMATION_MAX_Y).contains(&((point >> 16) as u16 as i16))
}

/// Finds the slot standing on a pitch point in one phase and ball zone.
///
/// `Formation::find_slot_at_coordinate(phase, zone, point)`: phase outside 0..1: the formation.cpp 0x453 box and 0;
/// zone outside band 0..2, height 0..3: 0x45c; a point outside x 0..0x1a, y 0..0x23: 0x465. Otherwise the first slot
/// whose coordinate (`Formation::get_slot_coordinate`) equals the point, or 0xff. Only `al` is the result.
/// Original: `Formation::find_slot_at_coordinate` (0x5b4410, formation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn formation_find_slot_at_coordinate(this: usize, stage: u32, zone: u32, point: u32) -> u8 {
    let line = if !formation_stage_ok(stage) { 0x453 } else if !formation_zone_ok(zone) { 0x45c } else if !formation_point_ok(point) { 0x465 } else { 0 };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    let mut at = 0u32;
    for slot in 0..11i8 {
        exe::formation_get_slot_coordinate(this, slot, stage as i8, zone, &mut at);
        if at == point { // sx10_7: the slot on the point
            return slot as u8;
        }
    }
    0xff
}

/// Exchanges the pitch coordinates of two slots in one phase and ball zone.
///
/// `Formation::swap_slot_coordinates(a, b, phase, zone)`: `a` outside 0..10: the formation.cpp 0x48b box and 0; `b`:
/// 0x493; phase outside 0..1: 0x49b; zone outside band 0..2, height 0..3: 0x4a4. Reads both coordinates
/// (`Formation::get_slot_coordinate`), sets `a` to `b`'s and then `b` to `a`'s (`Formation::set_slot_coordinate`, which
/// picks their positions again). Changed = 0. Returns 1.
/// Original: `Formation::swap_slot_coordinates` (0x5b45b0, formation.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn formation_swap_slot_coordinates(this: usize, a: u32, b: u32, stage: u32, zone: u32) -> u32 {
    let line = if !formation_slot_ok(a) {
        0x48b
    } else if !formation_slot_ok(b) {
        0x493
    } else if !formation_stage_ok(stage) {
        0x49b
    } else if !formation_zone_ok(zone) {
        0x4a4
    } else {
        0
    };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    let (mut at_a, mut at_b) = (0u32, 0u32);
    exe::formation_get_slot_coordinate(this, a as i8, stage as i8, zone, &mut at_a);
    exe::formation_get_slot_coordinate(this, b as i8, stage as i8, zone, &mut at_b);
    exe::formation_set_slot_coordinate(this, a as i32, stage as i32, zone, at_b); // sx10_7: a gets b's point
    exe::formation_set_slot_coordinate(this, b as i32, stage as i32, zone, at_a);
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    covered!("sx10_7 swap slot coordinates");
    1
}

/// Copies one ball zone's coordinates to the opposite flank, mirrored left to right.
///
/// `Formation::mirror_zone_coordinates(phase, zone)`: phase outside 0..1: the formation.cpp 0x4c9 box and 0; zone
/// outside band 0..2, height 0..3: 0x4d2. Reads the 11 coordinates of the zone and mirrors each across (x becomes 0x1a −
/// x). Then, line by line (goalkeeper to forward) among the slots of that line not yet paired, the leftmost and the
/// rightmost (smallest and largest mirrored x; the first one on a tie) exchange their coordinates and are paired, again
/// until a line has fewer than two left (or both ends are the same slot): so each left player keeps the left side.
/// Every slot then gets its coordinate in the zone of the opposite band (2 − band, same height) through
/// `Formation::set_slot_coordinate`. Changed = 0. Returns 1.
/// Original: `Formation::mirror_zone_coordinates` (0x5b47a0, formation.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn formation_mirror_zone_coordinates(this: usize, stage: u32, zone: u32) -> u32 {
    let line = if !formation_stage_ok(stage) { 0x4c9 } else if !formation_zone_ok(zone) { 0x4d2 } else { 0 };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    let mut points = [0u32; 11];
    for slot in 0..11usize {
        exe::formation_get_slot_coordinate(this, slot as i8, stage as i8, zone, &mut points[slot]);
        let x = FORMATION_MAX_X.wrapping_sub(points[slot] as u16 as i16) as u16; // sx10_7: mirrored across
        points[slot] = (points[slot] & 0xffff_0000) | x as u32;
    }
    let mut paired = [false; 11];
    for &bit in FORMATION_LINE_BITS.iter() {
        loop {
            let (mut min_x, mut max_x) = (FORMATION_MAX_X + 1, -1i16);
            let (mut left, mut right) = (0usize, 0usize);
            for slot in 0..11usize {
                if paired[slot] || formation_general(this, slot) & bit == 0 {
                    continue;
                }
                let x = points[slot] as u16 as i16;
                if x < min_x {
                    min_x = x;
                    left = slot;
                }
                if x > max_x {
                    max_x = x;
                    right = slot;
                }
            }
            if min_x > FORMATION_MAX_X || max_x < 0 || left == right {
                break;
            }
            points.swap(left, right); // sx10_7: the two ends of the line swap
            paired[left] = true;
            paired[right] = true;
        }
    }
    let opposite = (zone & 0xffff_0000) | (2u16.wrapping_sub(zone as u16)) as u32;
    for slot in 0..11usize {
        exe::formation_set_slot_coordinate(this, slot as i32, stage as i32, opposite, points[slot]);
    }
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    covered!("sx10_7 mirror zone");
    1
}

/// Whether any coordinate of a slot differs from the default its positions give.
///
/// `Formation::is_slot_customised(slot)`: slot outside 0..10: the formation.cpp 0x538 box and 0. Otherwise 1 as soon as
/// `Formation::is_slot_coordinate_customised` answers 1 for a phase (0..1), band (0..2) and height (0..3), else 0.
/// Original: `Formation::is_slot_customised` (0x5b4a60, formation.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn formation_is_slot_customised(this: usize, slot: u32) -> u32 {
    if !formation_slot_ok(slot) {
        formation_error_box(0x538);
        return 0;
    }
    let customised: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_FORMATION_IS_SLOT_COORDINATE_CUSTOMISED);
    for stage in 0..2u32 {
        for band in 0..3u32 {
            for height in 0..4u32 {
                if customised(this, slot, stage, band | height << 16) != 0 {
                    return 1;
                }
            }
        }
    }
    0
}

/// Whether a slot's coordinate in one phase and ball zone differs from the default its positions give.
///
/// `Formation::is_slot_coordinate_customised(slot, phase, zone)`: slot outside 0..10: the formation.cpp 0x567 box and
/// 0; phase outside 0..1: 0x56f; zone outside band 0..8, height 0..0xb: 0x578 (wider than the 3 × 4 zones: a defect of
/// the original, an out-of-range zone reads another slot's coordinates). The default coordinates come from the slot's
/// positions as `Formation::set_slot_position` lays them out (`formation_get_default_coordinate`): with the general and
/// extra positions equal, both phases from the general one; otherwise, by sectors (`formation_split_position_code`), an
/// extra in a sector below the general's gives the extra without the ball and the general with it, else the reverse.
/// The phase asked is compared with the stored coordinate (`formation_fields::SLOT_COORDINATES`, + 0x30 with the ball):
/// 1 if it differs, else 0.
/// Original: `Formation::is_slot_coordinate_customised` (0x5b4b70, formation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn formation_is_slot_coordinate_customised(this: usize, slot: u32, stage: u32, zone: u32) -> u32 {
    let (band, height) = (zone as u16 as i16, (zone >> 16) as u16 as i16);
    let line = if !formation_slot_ok(slot) {
        0x567
    } else if !formation_stage_ok(stage) {
        0x56f
    } else if !(0..=8).contains(&band) || !(0..=0xb).contains(&height) {
        0x578
    } else {
        0
    };
    if line != 0 {
        formation_error_box(line);
        return 0;
    }
    let (slot, stage) = (slot as u8 as usize, stage as u8);
    let general = formation_general(this, slot);
    let extra = read_u16(this + formation_fields::EXTRA_SLOT_CELLS + 2 * slot);
    let (without_ball, with_ball) = if general == extra {
        (general, general)
    } else {
        let (mut general_sector, mut extra_sector, mut side) = (0u16, 0u16, 0u16);
        exe::formation_split_position_code(general as u32, &mut general_sector, &mut side);
        exe::formation_split_position_code(extra as u32, &mut extra_sector, &mut side);
        if (extra_sector as i16) < (general_sector as i16) {
            (extra, general)
        } else {
            (general, extra)
        }
    };
    let stored = |phase: usize| {
        let index = height as isize + 4 * (band as isize + 6 * slot as isize);
        read_u32((this as isize + (formation_fields::SLOT_COORDINATES + 0x30 * phase) as isize + 4 * index) as usize)
    };
    let mut default = 0u32;
    exe::formation_get_default_coordinate(without_ball as u32, 0, zone, (&mut default) as *mut u32 as *mut i16);
    if stage == 0 && stored(0) != default { // sx10_7: the phase without the ball moved
        return 1;
    }
    exe::formation_get_default_coordinate(with_ball as u32, 1, zone, (&mut default) as *mut u32 as *mut i16);
    if stage == 1 && stored(1) != default {
        return 1;
    }
    0
}

/// The record regions a formation routine may change: the whole record.
fn formation_whole(this: usize) -> [(usize, usize); 1] {
    [(this, formation_fields::SIZE)]
}

/// Side by side over `regions` for a formation routine of the tactics screens, recorded under `name`.
unsafe fn formation_side_by_side<R: PartialEq + Copy + core::fmt::Debug>(name: &str, regions: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> R {
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(regions, original, rust);
    SCOREBOARD.record(same && r == n, || format!("{} {:?}/{:?} {:?}", name, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `formation_new_vs_club`: side by side over the record (a bad record shows the box: original only).
pub(crate) unsafe extern "thiscall" fn compare_formation_new_vs_club(this: usize, record: usize, side: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%);
    if record == 0 {
        return o(this, record, side);
    }
    formation_side_by_side(&format!("Formation vs club ({:#x}, side {})", this, side as u8), &formation_whole(this), || o(this, record, side), || formation_new_vs_club(this, record, side))
}

/// Comparer for `formation_swap_slots`.
pub(crate) unsafe extern "thiscall" fn compare_formation_swap_slots(this: usize, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I1%%);
    if !formation_slot_ok(a) || !formation_slot_ok(b) {
        return o(this, a, b);
    }
    formation_side_by_side(&format!("Formation::swap_slots({}, {})", a as u8, b as u8), &formation_whole(this), || o(this, a, b), || formation_swap_slots(this, a, b))
}

/// Comparer for `formation_slots_in_position_order`: the 11 bytes written.
pub(crate) unsafe extern "thiscall" fn compare_formation_slots_in_position_order(this: usize, out: usize, size: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    if out == 0 || (size as i32) < 11 {
        return o(this, out, size);
    }
    formation_side_by_side("Formation::get_slots_in_position_order", &[(out, 11)], || o(this, out, size), || formation_slots_in_position_order(this, out, size))
}

/// Comparer for `formation_set_zone_coordinates`.
pub(crate) unsafe extern "thiscall" fn compare_formation_set_zone_coordinates(this: usize, stage: u32, zone: u32, coordinates: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32>(%%I3%%);
    if !formation_stage_ok(stage) || !formation_zone_ok(zone) || coordinates == 0 {
        return o(this, stage, zone, coordinates);
    }
    formation_side_by_side(&format!("Formation::set_zone_coordinates({}, {:#x})", stage as u8, zone), &formation_whole(this), || o(this, stage, zone, coordinates), || formation_set_zone_coordinates(this, stage, zone, coordinates))
}

/// Comparer for `formation_get_zone_coordinates`: the 11 coordinates written.
pub(crate) unsafe extern "thiscall" fn compare_formation_get_zone_coordinates(this: usize, stage: u32, zone: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32>(%%I4%%);
    if !formation_stage_ok(stage) || !formation_zone_ok(zone) || out == 0 {
        return o(this, stage, zone, out);
    }
    formation_side_by_side(&format!("Formation::get_zone_coordinates({}, {:#x})", stage as u8, zone), &[(out, 44)], || o(this, stage, zone, out), || formation_get_zone_coordinates(this, stage, zone, out))
}

/// Comparer for `formation_set_phase_coordinates`.
pub(crate) unsafe extern "thiscall" fn compare_formation_set_phase_coordinates(this: usize, stage: u32, coordinates: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(%%I5%%);
    if !formation_stage_ok(stage) || coordinates == 0 {
        return o(this, stage, coordinates);
    }
    formation_side_by_side(&format!("Formation::set_phase_coordinates({})", stage as u8), &formation_whole(this), || o(this, stage, coordinates), || formation_set_phase_coordinates(this, stage, coordinates))
}

/// Comparer for `formation_get_phase_coordinates`: the 132 coordinates written.
pub(crate) unsafe extern "thiscall" fn compare_formation_get_phase_coordinates(this: usize, stage: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(%%I6%%);
    if !formation_stage_ok(stage) || out == 0 {
        return o(this, stage, out);
    }
    formation_side_by_side(&format!("Formation::get_phase_coordinates({})", stage as u8), &[(out, 132 * 4)], || o(this, stage, out), || formation_get_phase_coordinates(this, stage, out))
}

/// Comparer for `formation_find_slot_at_coordinate`: the slot found.
pub(crate) unsafe extern "thiscall" fn compare_formation_find_slot_at_coordinate(this: usize, stage: u32, zone: u32, point: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u8>(%%I7%%);
    let r = o(this, stage, zone, point);
    if !formation_stage_ok(stage) || !formation_zone_ok(zone) || !formation_point_ok(point) {
        return r;
    }
    let n = formation_find_slot_at_coordinate(this, stage, zone, point);
    SCOREBOARD.record(r == n, || format!("Formation::find_slot_at_coordinate({}, {:#x}, {:#x}) {}/{}", stage as u8, zone, point, r, n));
    r
}

/// Comparer for `formation_swap_slot_coordinates`.
pub(crate) unsafe extern "thiscall" fn compare_formation_swap_slot_coordinates(this: usize, a: u32, b: u32, stage: u32, zone: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(%%I8%%);
    if !formation_slot_ok(a) || !formation_slot_ok(b) || !formation_stage_ok(stage) || !formation_zone_ok(zone) {
        return o(this, a, b, stage, zone);
    }
    formation_side_by_side(&format!("Formation::swap_slot_coordinates({}, {}, {}, {:#x})", a as u8, b as u8, stage as u8, zone), &formation_whole(this), || o(this, a, b, stage, zone), || formation_swap_slot_coordinates(this, a, b, stage, zone))
}

/// Comparer for `formation_mirror_zone_coordinates`.
pub(crate) unsafe extern "thiscall" fn compare_formation_mirror_zone_coordinates(this: usize, stage: u32, zone: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I9%%);
    if !formation_stage_ok(stage) || !formation_zone_ok(zone) {
        return o(this, stage, zone);
    }
    formation_side_by_side(&format!("Formation::mirror_zone_coordinates({}, {:#x})", stage as u8, zone), &formation_whole(this), || o(this, stage, zone), || formation_mirror_zone_coordinates(this, stage, zone))
}

/// Comparer for `formation_is_slot_customised`: the answer.
pub(crate) unsafe extern "thiscall" fn compare_formation_is_slot_customised(this: usize, slot: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I10%%)(this, slot);
    if formation_slot_ok(slot) {
        let n = formation_is_slot_customised(this, slot);
        SCOREBOARD.record(r == n, || format!("Formation::is_slot_customised({}) {}/{}", slot as u8, r, n));
    }
    r
}

/// Comparer for `formation_is_slot_coordinate_customised`: the answer (a bad argument shows the box: original only).
pub(crate) unsafe extern "thiscall" fn compare_formation_is_slot_coordinate_customised(this: usize, slot: u32, stage: u32, zone: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I11%%)(this, slot, stage, zone);
    let (band, height) = (zone as u16 as i16, (zone >> 16) as u16 as i16);
    if formation_slot_ok(slot) && formation_stage_ok(stage) && (0..=8).contains(&band) && (0..=0xb).contains(&height) {
        let n = formation_is_slot_coordinate_customised(this, slot, stage, zone);
        SCOREBOARD.record(r == n, || format!("Formation::is_slot_coordinate_customised({}, {}, {:#x}) {}/{}", slot as u8, stage as u8, zone, r, n));
    }
    r
}
