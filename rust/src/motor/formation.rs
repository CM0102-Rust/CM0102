use super::*;

/// Returns the tactical grid cell code of formation slot 0..10 (word array +0x12d, or the second set at +0x14e when
/// `segunda` is set). Out-of-range slots go to the original routine (error box).
pub(crate) unsafe extern "thiscall" fn slot_code(this: usize, slot: i8, second: u32) -> u16 {
    if !(0..=10).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, u32) -> u16>(IDX_SLOT_CODE)(this, slot, second);
    }
    let base = if second != 0 { 0x14e } else { 0x12d };
    read_u16(this + base + 2 * slot as usize)
}

/// Returns a grammatical attribute of a competition name [inference: its gender]: from the stage name id in the low
/// word of `b` (or else of `a`) via the comp_util name routine `exe::comp_util_get_stage_name_gender`; without either, the byte +0x37 (when `tipo` is
/// 1) or +0x52 of the competition record; with no record, 1.
pub(crate) unsafe extern "C" fn competition_criterion(record: usize, a: u32, b: u32, kind: u8) -> u8 {
    if b as u16 != 0 {
        return exe::comp_util_get_stage_name_gender(b, 0);
    }
    if a as u16 != 0 {
        return exe::comp_util_get_stage_name_gender(a, 0);
    }
    if record == 0 {
        return 1;
    }
    read_u8(record + if kind == 1 { 0x37 } else { 0x52 })
}

/// With attribute masking enabled and a valid person, resets that person's knowledge level (9-byte record) in each of
/// the 16 knowledge maps of `this` that exist and are active (+4), so every manager forgets him.
pub(crate) unsafe extern "thiscall" fn forget_person(this: usize, person: usize) {
    let valid = || read_i32(person) < read_i32(G_STAFF_COUNT) && *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0;
    if person == 0 || !valid() {
        return;
    }
    for k in 0..16usize {
        let q = read_usize(this + 4 * k);
        if q != 0 && *((q + 4) as *const u32) != 0 && valid() {
            write_u8(read_usize(q).wrapping_add((read_i32(person) as usize).wrapping_mul(9)), 0);
        }
    }
}

/// Fog of war: returns the knowledge map of manager `tecnico` (pointer table indexed backwards from the staff count:
/// [this + (staff_count - id) x 4 - 4]); 0 with no manager.
pub(crate) unsafe extern "thiscall" fn manager_table(this: usize, manager: usize) -> usize {
    if manager == 0 {
        return 0;
    }
    let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)) as usize;
    read_usize(this.wrapping_add(k.wrapping_mul(4)).wrapping_sub(4))
}

/// Splits the position mask into sector and side (`exe::formation_split_position_code`), finds the first of the 11 formation slots (+0x12d) of that
/// sector flagged 0x200 and returns 0 if none; with no player list returns 1, otherwise 1 if any of the 20 players
/// (0x1be-byte records, slot in +0x19, or +0x1a when +0x19 is 0xfe) occupies that slot.
pub(crate) unsafe extern "thiscall" fn sector_slot_taken(this: usize, mask: u32, players: usize) -> u32 {
    let (mut setor, mut lado) = (0u16, 0u16);
    exe::formation_split_position_code(mask, &mut setor, &mut lado);
    let home = |k: usize| read_u16(this + formation_fields::SLOT_CELLS + 2 * k);
    let k = match (0..11usize).find(|&k| home(k) & setor != 0 && home(k) & (slot_cells::CENTRE as u16) != 0) {
        Some(k) => k as u8,
        None => return 0,
    };
    if players == 0 {
        return 1;
    }
    (0..20usize).any(|i| {
        let p = players + match_player::POSITION + i * 0x1be;
        let c = read_u8(p);
        c == k || (c == 0xfe && *((p + 1) as *const u8) == k)
    }) as u32
}

/// Formation: returns the slot (0..10) whose 16-bit position code among the 11 at +0x12d equals `codigo`, or 0xff if
/// none.
pub(crate) unsafe extern "thiscall" fn slot_of_code(this: usize, code: u16) -> u32 {
    (0..11u32).find(|&k| read_u16(this + formation_fields::SLOT_CELLS + 2 * k as usize) == code).unwrap_or(0xff)
}

/// marks (`formation_fields::MARKING_TYPES`) with 4 and 8 become 2; assigned records
/// (`formation_fields::DESIGNATED_PEOPLE`) go back to −1; returns 1
/// Original: `clear_assigned` (0x5b5960, formation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn clear_assigned(this: usize) -> u32 {
    for k in 0..11usize {
        let b = (this + formation_fields::MARKING_TYPES + k) as *mut u8;
        if *b & 4 != 0 && *b & 8 != 0 {
            *b = 2;
        }
        write_i32(this + formation_fields::DESIGNATED_PEOPLE + 4 * k, -1);
    }
    1
}

/// `formation_fields::FORMATION_MASK` = OR of the bits of the 11 slots (`exe::formation_position_to_shape_bit`), with
/// 11 masks swapped for 4 reference formations
/// Original: `formation_mask` (0x5b59a0, formation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn formation_mask(this: usize) -> u32 {
    let m = (this + formation_fields::FORMATION_MASK) as *mut u32;
    core::ptr::write_unaligned(m, 0);
    for k in 0..11usize {
        let c = read_i16(this + formation_fields::SLOT_CELLS + 2 * k);
        let v = core::ptr::read_unaligned(m) | exe::formation_position_to_shape_bit(c);
        core::ptr::write_unaligned(m, v);
    }
    let v = core::ptr::read_unaligned(m);
    let swap = match v {
        0x101176c1 | 0x101b26c1 => 0x101f06c1,
        0x38115381 | 0x3a242381 => 0x381b0381,
        0x280e07c1 | 0x091a07c1 | 0x2884ab81 | 0x280aab81 | 0x288c07c1 | 0x281507c1 => 0x280e8b81,
        0x288a8b81 => 0x281b2381,
        _ => v,
    };
    core::ptr::write_unaligned(m, swap);
    v
}

/// Formation: returns the person record designated for position `posicao` (index at +0x5bf + 4 x position into the
/// person table at 0xae23bc, 110 bytes each), or 0 when the index is negative.
pub(crate) unsafe extern "thiscall" fn designated_for_position(this: usize, position: i8) -> usize {
    let i = *((this as isize + (formation_fields::DESIGNATED_PEOPLE as isize) + 4 * position as isize) as *const i32);
    if i > -1 {
        (*(G_STAFF_TABLE as *const usize)).wrapping_add(staff::SIZE * i as usize)
    } else {
        0
    }
}

/// Formation: sets one instruction bit `mascara` in the 64-bit instruction mask of position `posicao` (+0x188b + 8 x
/// position), first clearing the mutually exclusive bits of its group, and clears the changed flag (+0x187f). Returns
/// 1; position outside 0..=10 or unknown bit: clears the mouse-button global and returns 0 (the original also shows an
/// error box).
pub(crate) unsafe extern "thiscall" fn set_square(this: usize, position: i8, mask: u64) -> u32 {
    let group: u64 = match mask {
        0x1 | 0x2 | 0x4 | 0x8 | 0x10 => 0x1f,
        0x20 | 0x40 | 0x80 | 0x100 => 0x1e0,
        0x200 | 0x400 | 0x800 | 0x1000 => 0x1e00,
        0x2000 | 0x4000 | 0x8000 => 0xe000,
        0x10000 | 0x20000 | 0x4000_0000 => 0x4003_0000,
        0x40000 | 0x80000 | 0x8000_0000 => 0x800c_0000,
        0x10_0000 | 0x20_0000 => 0x30_0000,
        0x40_0000 | 0x80_0000 => 0xc0_0000,
        0x100_0000 | 0x200_0000 => 0x300_0000,
        0x400_0000 | 0x800_0000 => 0xc00_0000,
        0x1000_0000 | 0x2000_0000 => 0x3000_0000,
        0x1_0000_0000 | 0x2_0000_0000 => 0x3_0000_0000,
        0x4_0000_0000 | 0x8_0000_0000 => 0xc_0000_0000,
        _ => 0,
    };
    if !(0..=10).contains(&position) || group == 0 {
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
        return 0;
    }
    let home = (this + 0x188b + 8 * position as usize) as *mut u64;
    *home = (*home & !group) | mask;
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    1
}

/// Marks the standout players of side `lado` in the side's formation (instruction bit 0x800000): each active outfield
/// player scores 10 x (three person attributes +0x43, +0x33, +0x27) plus current form (+0x4d), adjusted by role
/// (defenders down, attacking roles up); those above 8500 are flagged. Goalkeepers are skipped.
pub(crate) unsafe extern "thiscall" fn mark_stars(this: usize, side: i8) {
    for k in 0..20 {
        let j = Engine(this).player(side, k);
        if j.position() < 0 {
            continue;
        }
        let pd = j.player();
        let sum = 10 * (pd.vision() as i32 + pd.movement() as i32 + pd.finishing() as i32);
        let mut v = ftol(sum as f64 + j.stamina() as f64);
        // simplification: the original calls 0x717410 on every test; it only reads the player
        let r = match_player_position(j);
        if r & 1 != 0 {
            continue;
        }
        if r & 2 != 0 {
            v += 500;
        } else if r & 4 != 0 {
            v -= if r & 0x800 != 0 || r & 0x80 != 0 { 500 } else { 1000 };
        } else if r & 8 != 0 {
            if r & 0x800 == 0 && r & 0x80 == 0 {
                v -= 300;
            }
        } else if r & 0x10 != 0 {
            v += 1000;
        } else if r & 0x20 != 0 {
            v += 2000;
        }
        if v as i16 > 8500 {
            exe::formation_set_player_instruction((this as isize + (engine::TACTICS as isize) + (formation_fields::SIZE as isize) * side as isize) as usize, j.position(), player_instructions::STANDOUT_PLAYER as u64);
        }
    }
}

/// Builds the player's strong-position mask from the position ratings and the sides rated 15; returns only `ax`.
/// (cdecl) The mask of the strong positions of player `p` from the position ratings (signed bytes): from 15, going down
/// to 10, the first level at which any rating passes gives the sector bits (`player_sheet::GOALKEEPER` 1,
/// `player_sheet::SWEEPER` 2, `player_sheet::DEFENDER` 4, `player_sheet::DEFENSIVE_MIDFIELDER` 8 or, without it,
/// `player_sheet::ATTACKING_MIDFIELDER` 0x20 or `player_sheet::MIDFIELDER` 0x10, `player_sheet::ATTACKER` 0x40); the
/// sides (`player_sheet::RIGHT_SIDE` 0x800, `player_sheet::LEFT_SIDE` 0x80, `player_sheet::CENTRAL` 0x200) only count
/// with a rating of 15 or more (the original repeats the same test at each level). Returns only `ax` (the callers use
/// 16 bits). Null `p`: as in the original (error box and 0).
/// Original: `position_mask` (0x5b7d20, formation.cpp, cdecl).
pub(crate) unsafe extern "C" fn position_mask(p: usize) -> u16 {
    if p == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_POSITION_MASK)(p);
    }
    let mut m = 0u16;
    let mut level = 15i8;
    loop {
        if read_i8(p + player_sheet::GOALKEEPER) >= level {
            m |= 1;
        }
        if read_i8(p + player_sheet::DEFENDER) >= level {
            m |= 4;
        }
        if read_i8(p + player_sheet::SWEEPER) >= level {
            m |= 2;
        }
        if read_i8(p + player_sheet::DEFENSIVE_MIDFIELDER) >= level {
            m |= 8;
        } else if read_i8(p + player_sheet::ATTACKING_MIDFIELDER) >= level {
            m |= 0x20;
        } else if read_i8(p + player_sheet::MIDFIELDER) >= level {
            m |= 0x10;
        }
        if read_i8(p + player_sheet::ATTACKER) >= level {
            m |= 0x40;
        }
        level -= 1;
        if level < 10 || m != 0 {
            break;
        }
    }
    if read_i8(p + player_sheet::RIGHT_SIDE) >= 15 {
        m |= 0x800;
    }
    if read_i8(p + player_sheet::LEFT_SIDE) >= 15 {
        m |= 0x80;
    }
    if read_i8(p + player_sheet::CENTRAL) >= 15 {
        m |= 0x200;
    }
    m
}

/// Returns the pitch cell (column 0..26, row 0..35) of a position by attack and band.
///
/// (cdecl) The pitch cell (`out[0]` = column 0..26, `out[1]` = row 0..35) of position `mask` (split by `exe::formation_split_position_code` into
/// sector and side), with `attack` 0/1 and `band` = low word 0..2 (left, centre, right) and high word 0..3 (the height
/// in the formation). The row comes from the sector (tables of 4 by height; in attack, the full-backs 0x80/0x800 move
/// up 1, or 1/4 in sectors 2 and 4); the column, from the side (tables of 5); then the nudges (of 1 or 5) in the bands
/// and the limits. Arguments out of range: as in the original (error box and 0). Returns 1.
/// Original: `pitch_cell` (0x5b7e60, formation.cpp, cdecl).
pub(crate) unsafe extern "C" fn pitch_cell(mask: u32, attack: i8, band: u32, out: *mut i16) -> u32 {
    let low = band as u16 as i16;
    let high = (band >> 16) as u16 as i16;
    if !(0..=1).contains(&attack) || !(0..=2).contains(&low) || !(0..=3).contains(&high) {
        return original_fn::<unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32>(IDX_PITCH_CELL)(mask, attack, band, out);
    }
    let (mut setor, mut lado) = (0u16, 0u16);
    exe::formation_split_position_code(mask, &mut setor, &mut lado);
    let a = high as usize;
    let throw_in = lado & 0x80 != 0 || lado & 0x800 != 0;
    let at = attack != 0;
    // the row (output[1])
    let row: i16 = if setor & 1 != 0 {
        if at { [0, 2, 4, 6][a] } else { [0, 1, 2, 3][a] }
    } else if setor & 2 != 0 {
        if !at {
            [1, 3, 5, 9][a]
        } else {
            let v = [2, 4, 13, 15][a];
            if throw_in { v + if high != 3 { 1 } else { 4 } } else { v }
        }
    } else if setor & 4 != 0 {
        if !at {
            [2, 7, 9, 13][a]
        } else {
            let v = [4, 9, 17, 17][a];
            if throw_in { v + if high != 3 { 1 } else { 4 } } else { v }
        }
    } else {
        let (sem, com): ([i16; 4], [i16; 4]) = if setor & 8 != 0 {
            ([4, 9, 13, 18], [7, 11, 20, 22])
        } else if setor & 0x10 != 0 {
            ([6, 11, 16, 22], [9, 14, 23, 26])
        } else if setor & 0x20 != 0 {
            ([9, 13, 19, 26], [12, 19, 27, 30])
        } else {
            ([13, 15, 23, 31], [15, 24, 31, 34])
        };
        if !at {
            sem[a]
        } else if throw_in {
            com[a] + 1
        } else {
            com[a]
        }
    };
    *out.add(1) = row;
    // the column (saida[0]), by side
    let l = if lado & 0x80 != 0 {
        0
    } else if lado & 0x100 != 0 {
        1
    } else if lado & 0x200 != 0 {
        2
    } else if lado & 0x400 != 0 {
        3
    } else {
        4
    };
    let table: [i16; 5] = if setor & 1 != 0 {
        [13; 5]
    } else if setor & 6 != 0 || setor & 0x38 != 0 {
        match (at, high == 0, setor & 6 != 0) {
            (false, true, _) => [9, 11, 13, 15, 17],
            (false, false, _) => [7, 11, 13, 15, 19],
            (true, true, true) => [6, 11, 13, 15, 20],
            (true, _, _) => [3, 10, 13, 16, 23],
        }
    } else if !at {
        [7, 11, 13, 15, 19]
    } else if high == 0 || high == 1 {
        [2, 8, 13, 18, 24]
    } else if high == 2 {
        [3, 10, 13, 16, 23]
    } else {
        [6, 11, 13, 15, 20]
    };
    *out = table[l];
    // the cushions: with `perto`, one step in the band; without, five (except the goalkeeper)
    let near = (setor & 1 != 0 && high > 0)
        || (setor & 6 != 0 && high == 0)
        || (setor & 0x40 != 0 && lado & 0x700 != 0 && high == 3 && attack == 1);
    let step_towards = if near { 1 } else if setor & 1 == 0 { 5 } else { 0 };
    if step_towards != 0 {
        let v = *out as i32;
        if low == 0 {
            *out = (v - step_towards).clamp(2, 0x11) as i16;
        } else if low == 2 {
            *out = (v + step_towards).clamp(9, 0x18) as i16;
        }
    }
    *out = (*out).clamp(0, 0x1a);
    *out.add(1) = (*out.add(1)).clamp(0, 0x23);
    1
}

/// Checks the 11 formation slots for conflicts: 0 if any slot with an open side is also a striker's, midfielder's,
/// defensive or attacking midfielder's; returns only `al`.
///
/// (thiscall) The 11 slots of formation `this` without conflict: returns 0 if any slot with an open side (0x880 in cell
/// +0x12d or +0x14e, see `exe::formation_get_slot_position`) is also a striker's (0x40, in both), a midfielder's (0x20 in +0x12d and in +0x14e
/// through `exe::formation_get_slot_position`) or a defensive midfielder's and attacking midfielder's through `exe::formation_get_slot_position` (8 and 0x10); otherwise 1.
/// Returns only `al`.
/// Original: `slots_without_conflict` (0x5b8960, formation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn slots_without_conflict(this: usize) -> u8 {
    for i in 0..=10u32 {
        let a = read_u16(this + (0x12d + 2 * i as usize));
        let b = read_u16(this + (0x14e + 2 * i as usize));
        if a & 0x880 != 0 && a & 0x40 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && b & 0x40 != 0 {
            return 0;
        }
        if a & 0x880 != 0 && a & 0x20 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && exe::formation_get_slot_position(this, i, 1) & 0x20 != 0 {
            return 0;
        }
        if exe::formation_get_slot_position(this, i, 0) & 0x880 != 0 && exe::formation_get_slot_position(this, i, 0) & 8 != 0 {
            return 0;
        }
        if exe::formation_get_slot_position(this, i, 0) & 0x880 != 0 && exe::formation_get_slot_position(this, i, 0) & 0x10 != 0 {
            return 0;
        }
        if exe::formation_get_slot_position(this, i, 1) & 0x880 != 0 && exe::formation_get_slot_position(this, i, 1) & 0x10 != 0 {
            return 0;
        }
    }
    1
}

compare_value!(compare_slots_without_conflict, slots_without_conflict, IDX_SLOTS_WITHOUT_CONFLICT, () -> u8);

/// Stores the slot coordinate for the phase and ball zone and picks the general and extra positions closest to the
/// slot's 12 coordinates. `Formation::set_slot_coordinate(home, stage, band, coord)` (cell, phase, band, coord): writes
/// the coordinate (dword: x 0..0x1a, y 0..0x23) of the cell for the phase and the ball band
/// (`formation_fields::SLOT_COORDINATES`, 11 × 2 × 3 × 4 dwords) and, for each phase, finds the positions of table
/// 0x9cc368 (words up to 0xffff) closest to the cell's 12 coordinates in that phase (|Δx·9/27| + |Δy·12/36| against
/// each position's cell through `exe::formation_get_default_coordinate`; doubled for positions with bit 2; ties go into
/// the phase's list, up to 11 contiguous at [esp+0x64]). The general position (`formation_fields::SLOT_CELLS`) stays if
/// it is in one of the lists; otherwise it becomes the first of the phase with the smallest distance
/// (`exe::formation_update_shape_code` redoes the mask, the cell gets the flag `formation_fields::DISPLAY_ORDER` and
/// the other cells with the same position lose it). The extra position (`formation_fields::EXTRA_SLOT_CELLS`) comes
/// from the other phase's list: the same position, or one with the same side (`exe::formation_split_position_code`), or
/// the first, or, with two, the middle of the two's bits. Zeroes `formation_fields::CHANGED_FLAG`. Arguments out of
/// range: as in the original (box and 0). Returns 1.
/// Original: `Formation::set_slot_coordinate` (0x5b3560, formation.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn set_slot_coordinate(this: usize, home: u32, stage: u32, band: u32, coord: u32) -> u32 {
    let (s, f) = (home as u8 as i8, stage as u8 as i8);
    let (zx, zy) = (band as u16 as i16, (band >> 16) as u16 as i16);
    let (cx, cy) = (coord as u16 as i16, (coord >> 16) as u16 as i16);
    if !(0..=10).contains(&s) || !(0..=1).contains(&f) || !(0..=2).contains(&zx) || !(0..=3).contains(&zy)
        || !(0..=0x1a).contains(&cx) || !(0..=0x23).contains(&cy)
    {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(IDX_SET_SLOT_COORDINATE)(this, home, stage, band, coord);
    }
    let su = s as usize;
    let cel = this + formation_fields::SLOT_COORDINATES + 4 * (zy as usize + 4 * (zx as usize + 3 * (f as usize + 2 * su)));
    covered!("cpu_11 coordinate recorded");
    write_u32(cel, coord); // cpu11: a coordenada
    // the output of 0x5b7e60 is the coord argument itself in the original (a single slot, for the whole function)
    let mut out: [i16; 2] = [cx, cy];
    // simplification: the lists of the two stages stay contiguous as at [esp+0x64]; indexes beyond 0xea do not happen
    let mut cand = [0u16; 0xea];
    let mut count = [0i8; 2]; // uninitialized in the original; the first position always zeroes it
    let mut best = [0i32; 2];
    let mut last_dy: i32 = 0; // [esp+0x60], read only by the indices outside the table at the end
    for p in 0..2usize {
        best[p] = 0x7fffffff;
        let mut list = 0x9cc368usize;
        while read_i16(list) != -1 {
            let pos = read_i16(list) as u16;
            let base = this + formation_fields::SLOT_COORDINATES + su * 0x60 + p * 0x30;
            let mut dist: i32 = 0;
            for zx in 0..3u32 {
                for zy in 0..4u32 {
                    // the mask goes with the band height in the upper 16 bits (the original’s ecx); 0x5b7e60 does not read them
                    exe::formation_get_default_coordinate(pos as u32 | zy << 16, p as i8, zx | zy << 16, out.as_mut_ptr());
                    let c = base + 4 * (zx as usize * 4 + zy as usize);
                    let dy = ((read_i16(c + 2) as i32 * 12) / 36 - (out[1] as i32 * 12) / 36).wrapping_abs();
                    let dx = ((read_i16(c) as i32 * 9) / 27 - (out[0] as i32 * 9) / 27).wrapping_abs();
                    last_dy = dy;
                    dist = dx.wrapping_add(dist).wrapping_add(dy);
                }
            }
            if pos & 2 != 0 {
                covered!("cpu_11 distance doubled");
                dist = dist.wrapping_add(dist); // cpu11: o dobro
            }
            if dist <= best[p] { // cpu11: the tie goes into the list
                if dist < best[p] {
                    count[p] = 0;
                } else {
                    covered!("cpu_11 tie in the list");
                }
                best[p] = dist;
                cand[(count[p] as i32 + p as i32 * 11) as usize] = pos;
                count[p] = count[p].wrapping_add(1);
            }
            list += 2;
        }
    }
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    let general = this + formation_fields::SLOT_CELLS + su * 2;
    let mut sf: i8 = (best[0] > best[1]) as i8;
    let mut si: i8 = 0;
    for q in 0..2i8 {
        let mut a: i8 = 0;
        while a < count[q as usize] {
            if cand[(a as i32 + q as i32 * 11) as usize] == read_i16(general) as u16 {
                sf = q;
                si = a;
                break;
            }
            a += 1;
        }
    }
    let new = cand[(sf as i32 * 11 + si as i32) as usize];
    if read_i16(general) as u16 != new {
        write_u16(general, new);
        exe::formation_update_shape_code(this);
        write_u8(this + formation_fields::DISPLAY_ORDER + su, 1);
        for k in 0..11i8 {
            if k != s && read_i16(this + formation_fields::SLOT_CELLS + k as usize * 2) as u16 == new {
                write_u8(this + formation_fields::DISPLAY_ORDER + k as usize, 0);
            }
        }
    }
    let extra = this + formation_fields::EXTRA_SLOT_CELLS + su * 2;
    let o = (1 - sf as i32) * 11; // the start of the other stage’s list
    let in_the = count[(1 - sf) as usize];
    let current = read_i16(general) as u16;
    for a in 0..in_the.max(0) as i32 {
        if cand[(o + a) as usize] == current {
            write_u16(extra, cand[(o + a) as usize]);
            return 1;
        }
    }
    // simplification: the outputs of 0x5b73a0 are uninitialized in the original if it fails (position without a side); here 0
    let (mut setor, mut lado_c, mut lado_a) = (0u16, 0u16, 0u16);
    for b in 0..in_the.max(0) as i32 {
        exe::formation_split_position_code(cand[(o + b) as usize] as u32, &mut setor, &mut lado_c);
        exe::formation_split_position_code(read_i16(general) as u16 as u32, &mut setor, &mut lado_a);
        if lado_c == lado_a {
            write_u16(extra, cand[(o + b) as usize]);
            return 1;
        }
    }
    if in_the != 2 {
        write_u16(extra, cand[o as usize]);
        return 1;
    }
    // two in the other stage: the middle of the sector bits (1..0x40) and side bits (0x80..0x800) of both. The frame words
    // from [esp+0x38]: the sides table and 0xffff, the band (3) and 0, the best-distance pointer (uninitialized
    // reproducible: 0), this, the sector table and 0xffff, the last |Δy| and the lists.
    let (c0, c1) = (cand[o as usize], cand[(o + 1) as usize]);
    let mut q = [0u16; 0x100];
    q[..6].copy_from_slice(&[0x80, 0x100, 0x200, 0x400, 0x800, 0xffff]);
    q[6] = 3;
    q[10] = this as u16;
    q[11] = (this >> 16) as u16;
    q[12..20].copy_from_slice(&[1, 2, 4, 8, 0x10, 0x20, 0x40, 0xffff]);
    q[20] = last_dy as u16;
    q[21] = (last_dy >> 16) as u16;
    q[22..].copy_from_slice(&cand[..0xea]);
    let mut v1a: i8 = count[0]; // [esp+0x1a]
    let mut v12: i8 = in_the; // [esp+0x12]
    for k in 0..7usize {
        if q[12 + k] & c0 != 0 {
            v1a = k as i8;
        }
        if q[12 + k] & c1 != 0 {
            v12 = k as i8;
        }
    }
    let mut v13: i8 = si; // [esp+0x13]
    let mut dl: i8 = v1a;
    for k in 0..5usize {
        if q[k] & c0 != 0 {
            v13 = k as i8;
        }
        if q[k] & c1 != 0 {
            dl = k as i8;
        }
    }
    let ib = (v13 as i32 + dl as i32) / 2;
    let ia = (v12 as i32 + v1a as i32) / 2;
    write_u16(extra, q[ib as usize & 0xff] | q[(12 + ia) as usize & 0xff]);
    1
}

/// Finds another slot with the same general position and sets the output when its flag is lower.
/// `Formation::check_duplicate_position(home, out)` (cell, *out): zeroes the output; the first other cell with the same
/// general position (`formation_fields::SLOT_CELLS`) decides: with a flag (`formation_fields::DISPLAY_ORDER`, signed
/// byte) lower than the cell's, output 1 and returns 1; otherwise returns 0. With no duplicate, 1. Cell outside 0..10:
/// as in the original (box and 0).
/// Original: `Formation::check_duplicate_position` (0x5b3460, formation.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn check_duplicate_position(this: usize, home: u32, out: *mut u32) -> u32 {
    let s = home as u8 as i8;
    if !(0..=10).contains(&s) {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(IDX_CHECK_DUPLICATE_POSITION)(this, home, out);
    }
    *out = 0;
    let su = s as usize;
    for k in 0..11usize {
        if k != su && read_i16(this + formation_fields::SLOT_CELLS + k * 2) == read_i16(this + formation_fields::SLOT_CELLS + su * 2) {
            if read_i8(this + formation_fields::DISPLAY_ORDER + k) < read_i8(this + formation_fields::DISPLAY_ORDER + su) {
                *out = 1;
                return 1;
            }
            return 0;
        }
    }
    1
}

/// Sets one marking type of the slot (bits 0..2 or 3..4 of byte `formation_fields::MARKING_TYPES`) and clears
/// `formation_fields::CHANGED_FLAG`. `Formation::set_marking_type(home, mark)` (cell, marking): markings 1, 2 and 4
/// replace bits 0..2 of the cell's byte `formation_fields::MARKING_TYPES`; 8 and 0x10, bits 3..4; it zeroes
/// `formation_fields::CHANGED_FLAG` and returns 1. Any other marking: as in the original (box and 0). The cell is not
/// checked.
/// Original: `Formation::set_marking_type` (0x5b57d0, formation.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_marking_type(this: usize, home: u32, mark: u32) -> u32 {
    let m = mark as u8;
    let limpa = match m {
        1 | 2 | 4 => 0xf8u8,
        8 | 0x10 => 0xe7,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_MARKING_TYPE)(this, home, mark),
    };
    let b = (this as isize + home as u8 as i8 as isize + (formation_fields::MARKING_TYPES as isize)) as *mut u8;
    *b = *b & limpa | m;
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    1
}

/// Formation memory regions the three slot setters write: positions/marks/coordinates (+0x12d..+0x584), marking types
/// (+0x5b4) and the mask (+0x187b).
pub(crate) fn formation_regions(this: usize) -> [(usize, usize); 3] {
    [(this + formation_fields::SLOT_CELLS, 0x584 - 0x12d), (this + formation_fields::MARKING_TYPES, 11), (this + formation_fields::FORMATION_MASK, 8)]
}

/// Expands each formation slot's positions into the pitch cell tables and their mirrored copy. (thiscall) The
/// formation's cell map. For each of the 22 lines k (11 cells × 2 phases) and 3 × 4 block (a18, a14) with the position
/// (i = 12k + 4·a18 + a14) at `formation_fields::SLOT_COORDINATES` (column, word) and
/// `formation_fields::SLOT_COORDINATES` + 2 (row), the 3 × 3 cells of the block get the column × 9 / 27 in the low
/// nibble and the row × 12 / 36 in the high nibble of table `formation_fields::POSITION_LOOKUP` (12 per row, 9 rows per
/// k); the mirror at `formation_fields::POSITION_LOOKUP` + 0x9b3 (row and column inverted) receives (0xb0 − row × 16)
/// with (0xf8 − column) & 0xf. Returns 1.
/// Original: `Formation::build_position_lookup` (0x5b7160, formation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn build_position_lookup(this: usize) -> u32 {
    for k in 0..22i32 {
        for a18 in 0..3i32 {
            for a14 in 0..4i32 {
                let i = (12 * k + 4 * a18 + a14) as usize;
                for y in 3 * a18..3 * a18 + 3 {
                    for x in 3 * a14..3 * a14 + 3 {
                        let column = read_i16(this + i * 4 + formation_fields::SLOT_COORDINATES) as i32;
                        let c = (column * 9 / 27) as u8;
                        let p = (this as isize + (formation_fields::POSITION_LOOKUP as isize) + ((9 * k + y) * 12 + x) as isize) as *mut u8;
                        *p = (*p & 0xf0) | (c & 0xf);
                        let row = read_i16(this + i * 4 + formation_fields::SLOT_COORDINATES + 2) as i32;
                        let d = ((row * 12 / 36) as u8) << 4;
                        *p = (c & 0xf) | d;
                        let q = (this as isize + (formation_fields::POSITION_LOOKUP as isize) + 0x9b3 + ((9 * k - y) * 12 - x) as isize) as *mut u8;
                        *q = 0xb0u8.wrapping_sub(d) ^ (0xf8u8.wrapping_sub(c) & 0xf); // espelho
                    }
                }
            }
        }
    }
    1
}

/// Record (42735 bytes in [this]) of the match the human manager (or the on-screen one) is watching: Ok(0) without
/// human, Err when the index is out of range.
pub(crate) unsafe fn formation_watched_match_record(this: usize, human: usize) -> Result<usize, ()> {
    let h = if human != 0 {
        human
    } else {
        *((0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize) as *const usize)
    };
    if h == 0 {
        return Ok(0);
    }
    let i = (*(h as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
    if i < 0 || i > (*((this + 4) as *const i32)).wrapping_sub(1) {
        return Err(());
    }
    Ok((*(this as *const usize)).wrapping_add((i as usize).wrapping_mul(42735)))
}

/// Returns the stored cell of the slot by attack and band. The stored cell of slot `slot` 0..10 with `attack` 0/1 and
/// `band` (low word 0..2, high word 0..3): the dword `formation_fields::SLOT_COORDINATES` + 4×(high + 4×(low +
/// 3×(attack + 2×slot))) goes to `*out`. Returns 1. Out of range: as in the original (error box).
/// Original: `stored_cell` (0x5b3c70, formation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn stored_cell(t: usize, slot: i8, attack: i8, band: u32, out: *mut u32) -> u32 {
    let low = band as u16 as i16;
    let high = (band >> 16) as u16 as i16;
    if !(0..=10).contains(&slot) || !(0..=1).contains(&attack) || !(0..=2).contains(&low) || !(0..=3).contains(&high) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(IDX_STORED_CELL)(t, slot, attack, band, out);
    }
    let i = high as usize + 4 * (low as usize + 3 * (attack as usize + 2 * slot as usize));
    *out = read_u32(t + formation_fields::SLOT_COORDINATES + 4 * i);
    1
}

/// Sets a formation flag in `formation_fields::FLAGS`, clearing the others of the same group, and zeroes
/// `formation_fields::CHANGED_FLAG`. Sets flag `m` in `formation_fields::FLAGS`, first clearing those of the same group
/// (1/2/4/8; 0x10/0x20/0x40; 0x80/0x100; 0x200/0x400; 0x800/0x1000; 0x2000/0x4000; 0x8000/0x10000/0x20000) and zeroes
/// `formation_fields::CHANGED_FLAG`. Returns 1. Flag outside the groups: as in the original (error box).
/// Original: `set_formation_flag` (0x5b50d0, formation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_formation_flag(t: usize, m: u32) -> u32 {
    let group: u32 = match m {
        1 | 2 | 4 | 8 => 0xf,
        0x10 | 0x20 | 0x40 => 0x70,
        0x80 | 0x100 => 0x180,
        0x200 | 0x400 => 0x600,
        0x800 | 0x1000 => 0x1800,
        0x2000 | 0x4000 => 0x6000,
        0x8000 | 0x10000 | 0x20000 => 0x38000,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_FORMATION_FLAG)(t, m),
    };
    let marks = (t + formation_fields::FLAGS) as *mut u32;
    let v = core::ptr::read_unaligned(marks) & !group | m;
    write_u32(t + formation_fields::CHANGED_FLAG, 0);
    core::ptr::write_unaligned(marks, v);
    1
}

/// Reads a formation from a .pct file, fixes the instructions of old versions, redoes the positions of the 11 slots
/// from the coordinates and builds the coordinate map. `Formation::load_from_disk(path, quiet, dimmed)`, from the
/// creation of the tactics (0x893dc0: the AI formations and the predefined ones, 45 in a career) and from the tactics
/// screens. Null path (line 0x8d8) or a file that does not open (0x8e0): 0 (without `quiet`, the box). It reads the
/// identifier (4 bytes; byte-swapped if the file's marker `exe::utils_read_file_format_marker` is not the native one;
/// minus 123456789 if obfuscated (`dimmed`)): outside 0x98ec54..0x98ec5e "not a formation" (0x8f7), below 0x98ec59 "old
/// version" (0x900). Then, in order: the password (`formation_fields::PASSWORD`, 50 bytes, each byte inverted, 0xff
/// stays), the creator (`formation_fields::CREATOR`, 200), the mask (`formation_fields::FORMATION_MASK`, from 0x98ec5a;
/// before that 0), the 11 general positions (`formation_fields::SLOT_CELLS`, words) and the mask redone
/// (`exe::formation_update_shape_code`), the display order (`formation_fields::DISPLAY_ORDER`, 11 bytes, from 0x98ec5c;
/// before that zeros), the 11 extra positions (`formation_fields::EXTRA_SLOT_CELLS`), the cells
/// (`formation_fields::SLOT_COORDINATES`, 0x420 bytes = 0x210 words), in versions 0x98ec59/0x98ec5a 0x108 bytes
/// discarded, the team instructions (`formation_fields::FLAGS`, dword), the player instructions (below 0x98ec5e 11
/// dwords, each with 10 alongside, at +0x188b every 8; after that 0x58 bytes at +0x188b) and the marking types
/// (`formation_fields::MARKING_TYPES`, 11 bytes); the last three, if they fail, only show the box. The team
/// instructions get 0x2000 (without 0x4000) and, below 0x98ec5d, 0x20000. The 11 slots redo their positions from the
/// stored cell (`exe::formation_get_slot_coordinate` and `exe::formation_set_slot_coordinate` with phase 0 and band 0);
/// the name (+0) is the file's (_splitpath `exe::crt_strtok`); the cell map (`exe::formation_build_position_lookup`);
/// `formation_fields::CHANGED_FLAG` = 1, `formation_fields::HAS_PASSWORD` = password not empty, +0x1887 = 1. Closes and
/// returns 1. Non-native files swap the byte order (`exe::swap_bytes_16_array` words, `exe::swap_bytes_32_array`
/// dwords, `exe::swap_bytes_64_array` qwords).
/// Original: `Formation::load_from_disk` (0x5b5d10, formation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn load_from_disk(this: usize, path: usize, quiet: u32, dimmed: u32) -> u32 {
    if path == 0 {
        if quiet == 0 {
            formation_error_box(0x8d8);
        }
        return 0;
    }
    let f = exe::crt_fopen(path, 0x98f928);
    if f == 0 {
        if quiet == 0 {
            formation_error_box(0x8e0);
        }
        return 0;
    }
    let nativo = exe::utils_read_file_format_marker(f) != 0xf1ff_fff1;
    let mut id_ = [0u8; 4];
    if crt_fread_one(id_.as_mut_ptr() as usize, 4, f) != 1 {
        return fail_read(quiet, 0x8ea, f);
    }
    let mut id = u32::from_le_bytes(id_);
    if !nativo {
        id = swap_bytes_u32(id);
    }
    if dimmed != 0 {
        id = id.wrapping_sub(0x75bcd15);
    }
    let id = id as i32;
    if !(0x98ec54..=0x98ec5e).contains(&id) {
        return fail_read(quiet, 0x8f7, f);
    }
    if id < 0x98ec59 {
        return fail_read(quiet, 0x900, f);
    }
    let senha = this + formation_fields::PASSWORD;
    if crt_fread_one(senha, 0x32, f) != 1 {
        return fail_read(quiet, 0x909, f);
    }
    let strlen = |a: usize| (0..).take_while(|&k| read_u8(a + k) != 0).count();
    if strlen(senha) != 0 {
        let mut k = 0usize;
        loop {
            let b = read_u8(senha + k);
            write_u8(senha + k, if b == 0xff { 0xff } else { !b }); // cqg8: a senha invertida
            k += 1;
            if k >= strlen(senha) {
                break;
            }
        }
    }
    if crt_fread_one(this + formation_fields::CREATOR, 200, f) != 1 {
        return fail_read(quiet, 0x915, f);
    }
    if id >= 0x98ec5a {
        if crt_fread_one(this + formation_fields::FORMATION_MASK, 4, f) != 1 {
            return fail_read(quiet, 0x920, f);
        }
        if !nativo {
            write_u32(this + formation_fields::FORMATION_MASK, swap_bytes_u32(read_u32(this + formation_fields::FORMATION_MASK)));
        }
    } else {
        write_u32(this + formation_fields::FORMATION_MASK, 0);
    }
    if crt_fread_one(this + formation_fields::SLOT_CELLS, 0x16, f) != 1 {
        return fail_read(quiet, 0x930, f);
    }
    if !nativo {
        exe::swap_bytes_16_array(this + formation_fields::SLOT_CELLS, 0xb);
    }
    exe::formation_update_shape_code(this);
    if id >= 0x98ec5c {
        if crt_fread_one(this + formation_fields::DISPLAY_ORDER, 0xb, f) != 1 {
            return fail_read(quiet, 0x940, f);
        }
    } else {
        core::ptr::write_bytes((this + formation_fields::DISPLAY_ORDER) as *mut u8, 0, 0xb);
    }
    if crt_fread_one(this + formation_fields::EXTRA_SLOT_CELLS, 0x16, f) != 1 {
        return fail_read(quiet, 0x94f, f);
    }
    if !nativo {
        exe::swap_bytes_16_array(this + formation_fields::EXTRA_SLOT_CELLS, 0xb);
    }
    if crt_fread_one(this + formation_fields::SLOT_COORDINATES, 0x420, f) != 1 {
        return fail_read(quiet, 0x959, f);
    }
    if !nativo {
        exe::swap_bytes_16_array(this + formation_fields::SLOT_COORDINATES, 0x210);
    }
    if id == 0x98ec59 || id == 0x98ec5a {
        let mut descarte = [0u8; 0x108];
        if crt_fread_one(descarte.as_mut_ptr() as usize, 0x108, f) != 1 {
            return fail_read(quiet, 0x968, f);
        }
        if !nativo {
            exe::swap_bytes_16_array(descarte.as_mut_ptr() as usize, 0x84);
        }
    }
    let team = this + formation_fields::FLAGS;
    if crt_fread_one(team, 4, f) == 1 {
        if !nativo {
            write_u32(team, swap_bytes_u32(read_u32(team)));
        }
    } else if quiet == 0 {
        formation_error_box(0x971);
    }
    if id < 0x98ec5e {
        // simplification: the original copies the local array even when the read fails (stack leftover in a truncated file);
        // the copy starts with zeros
        let mut v = [0u32; 11];
        if crt_fread_one(v.as_mut_ptr() as usize, 0x2c, f) == 1 {
            if !nativo {
                exe::swap_bytes_32_array(v.as_mut_ptr() as usize, 0xb);
            }
        } else if quiet == 0 {
            formation_error_box(0x97b);
        }
        for (k, x) in v.iter().enumerate() {
            write_u32(this + 0x188b + 8 * k, *x);
            write_u32(this + 0x188b + 8 * k + 4, 0xa); // cqg8: the default instruction of the old versions
        }
        covered!("cqg_8 old instructions");
    } else if crt_fread_one(this + 0x188b, 0x58, f) == 1 {
        if !nativo {
            exe::swap_bytes_64_array(this + 0x188b, 0xb);
        }
    } else if quiet == 0 {
        formation_error_box(0x988);
    }
    if crt_fread_one(this + formation_fields::MARKING_TYPES, 0xb, f) != 1 && quiet == 0 {
        formation_error_box(0x98f);
    }
    let mut v = read_u32(team);
    if v & 0x4000 == 0 {
        v |= 0x2000; // cqg8: instruction 0x2000 without 0x4000
        write_u32(team, v);
        covered!("cqg_8 instruction 0x2000");
    }
    if id < 0x98ec5d {
        write_u32(team, read_u32(team) | 0x20000);
    }
    let mut c = 0u32;
    for vaga in 0..0xbi32 {
        exe::formation_get_slot_coordinate(this, vaga as i8, 0, 0, &mut c); // cqg8: the stored home side of stage 0
        covered!("cqg_8 positions redone");
        exe::formation_set_slot_coordinate(this, vaga, 0, 0, c); // cqg8: the positions redone by home side
    }
    exe::crt_strtok(path, 0, 0, (this) as *mut u8, (0) as *mut u8);
    exe::formation_build_position_lookup(this);
    write_u32(this + formation_fields::CHANGED_FLAG, 1);
    write_u32(this + formation_fields::HAS_PASSWORD, (read_u8(this + formation_fields::PASSWORD) != 0) as u32);
    write_u32(this + 0x1887, 1);
    cash_crt_fclose(f);
    1
}

/// Checks the centre-backs for conflicts: 0 if any slot with an open side is a centre-back's or defensive midfielder's;
/// returns only `al`.
///
/// (thiscall) Like `slots_without_conflict`, for the centre-backs: returns 0 if any of the 11 slots with an open side (0x880) is a
/// centre-back's (4, in cell +0x12d or +0x14e) or a defensive midfielder's (8 in +0x12d; in +0x14e through `exe::formation_get_slot_position`);
/// otherwise 1. Returns only `al`.
/// Original: `centre_backs_without_conflict` (0x5b8dc0, formation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn centre_backs_without_conflict(this: usize) -> u8 {
    for i in 0..=10u32 {
        let a = read_u16(this + (0x12d + 2 * i as usize));
        let b = read_u16(this + (0x14e + 2 * i as usize));
        if a & 0x880 != 0 && a & (slot_cells::DEFENDER as u16) != 0 {
            return 0;
        }
        if b & 0x880 != 0 && b & 4 != 0 {
            return 0;
        }
        if a & 0x880 != 0 && a & (slot_cells::DEFENSIVE_MIDFIELDER as u16) != 0 {
            return 0;
        }
        if b & 0x880 != 0 && exe::formation_get_slot_position(this, i, 1) & 8 != 0 {
            return 0;
        }
    }
    1
}

/// Stores the general or extra position of the slot, the flags, and recomputes the pitch cells without and with the
/// ball (`exe::formation_get_default_coordinate`). `Formation::set_slot_position(home, position, extra)` (cell,
/// position, extra): without `extra`, the cell's general position (`formation_fields::SLOT_CELLS`, words) changes to
/// `position` (if it changes: the mask `exe::formation_update_shape_code`, the cell's flag
/// `formation_fields::DISPLAY_ORDER` = 1 and that of the other cells with the same position = 0); with or without it,
/// the extra position (`formation_fields::EXTRA_SLOT_CELLS`) = `position`. Then, for each band 0..2 and height 0..3
/// (the low and high word of `exe::formation_get_default_coordinate`'s argument), the pitch cells without and with the
/// ball (`formation_fields::SLOT_COORDINATES` and `formation_fields::SLOT_COORDINATES` + 0x30, + 4 × (height + 4 × band
/// + 24 × cell)): with general = extra, both from the general one; otherwise, by sectors
/// (`exe::formation_split_position_code`): an extra in a sector below the general's gives the extra without the ball
/// and the general with the ball; otherwise the reverse. Finally `formation_fields::CHANGED_FLAG` = 0. Returns 1. Cell
/// outside 0..10: as in the original ("Error" box and 0).
/// Original: `Formation::set_slot_position` (0x5b2ba0, formation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn set_slot_position(this: usize, home: u32, position: u32, extra: u32) -> u32 {
    let k = home as u8 as i8;
    if !(0..=10).contains(&k) {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_SET_SLOT_POSITION)(this, home, position, extra); // message box: original only
    }
    let s = k as usize;
    let p = position as u16 as i16;
    let general = this + formation_fields::SLOT_CELLS + 2 * s;
    let extra_of_home = this + formation_fields::EXTRA_SLOT_CELLS + 2 * s;
    if extra == 0 && read_i16(general) != p {
        write_i16(general, p);
        exe::formation_update_shape_code(this);
        write_u8(this + formation_fields::DISPLAY_ORDER + s, 1);
        for j in 0..11usize {
            if j != s && read_i16(this + formation_fields::SLOT_CELLS + 2 * j) == p {
                write_u8(this + formation_fields::DISPLAY_ORDER + j, 0); // the other square with the same position
            }
        }
    }
    write_i16(extra_of_home, p);
    // 0x5b7e60 and 0x5b73a0 only read the low word of the mask (the original pushes eax with whatever high part it has)
    // the outputs carry over from one iteration to the next, like the original’s locals (uninitialized only if 0x5b73a0 fails
    // right on the first one: zero here)
    let (mut setor_geral, mut setor_extra, mut lado, mut saida) = (0u16, 0u16, 0u16, 0u32);
    if read_i16(general) == read_i16(extra_of_home) {
        covered!("cpk_7 formation same position");
    } else {
        covered!("cpk_7 formation extra position");
    }
    for band in 0..3usize {
        for altura in 0..4usize {
            let arg = band as u32 | (altura as u32) << 16;
            let i = altura + 4 * (band + 6 * s);
            let (sem_bola, com_bola) = (this + formation_fields::SLOT_COORDINATES + 4 * i, this + formation_fields::SLOT_COORDINATES + 0x30 + 4 * i);
            let (g, e) = (read_i16(general) as u16 as u32, read_i16(extra_of_home) as u16 as u32);
            let (a, b) = if g == e {
                (g, g) // the same position with and without the ball
            } else {
                exe::formation_split_position_code(g, &mut setor_geral, &mut lado);
                exe::formation_split_position_code(e, &mut setor_extra, &mut lado);
                if (setor_extra as i16) < (setor_geral as i16) { // the rearmost extra
                    (e, g)
                } else {
                    (g, e)
                }
            };
            exe::formation_get_default_coordinate(a, (0) as u32 as i8, arg, (&mut saida) as *mut u32 as *mut i16);
            write_u32(sem_bola, saida);
            exe::formation_get_default_coordinate(b, (1) as u32 as i8, arg, (&mut saida) as *mut u32 as *mut i16);
            write_u32(com_bola, saida);
        }
    }
    write_u32(this + formation_fields::CHANGED_FLAG, 0);
    1
}

/// Formation: returns byte `i` of the per-position table at `formation_fields::MARKING_TYPES` [inference: the role
/// flags of position `i`; bit 1 marks the goalkeeper].
/// Original: `position_byte` (0x5b57c0, formation.cpp).
pub(crate) unsafe extern "thiscall" fn position_byte(this: usize, i: i8) -> u8 {
    *((this as isize + (formation_fields::MARKING_TYPES as isize) + i as isize) as *const u8)
}

