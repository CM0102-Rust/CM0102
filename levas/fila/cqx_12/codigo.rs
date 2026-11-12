// ---------- batch %%LEVA%%: the UEFA Cup club list; the domestic cup semi-final losers in the UEFA seeding; the Mercosur records (uefa_cup.cpp, uefa_seeding.cpp, conmebol_merc.cpp) ----------

/// Rule: the UEFA Cup's club list: of one round (kind 1: round count − 8, kind 2: − 7, kind 3: − 4) the two clubs of
/// each tie, in tie order; otherwise (or with no such round) every entrant of the team table.
///
/// `UefaCup::collect_club_list(&out, kind)` (virtual +0x58). The list (`exe::crt_malloc`(4 × `cup::TEAM_COUNT`): push;
/// add esp, 4 -> cdecl; out of memory: the error box at line 0x2d6, `out` = 0 and 0). Kind (byte) 1, 2 or 3: the round
/// r = `cup::ROUND_COUNT` − 8, 7 or 4 (words); if r > −1, for each of the round's ties (the word +0x1a of its record in
/// `cup::ROUND_TABLE`, `cup_round::SIZE` bytes; the tie k at [`cup::TIE_TABLES`][r] + 0x35 k) the home team (word +0)
/// and then the away team (word +2), when not −1, as clubs of the team table (`cup::TEAM_TABLE`, 6-byte rows), with no
/// bound check (as the original). Otherwise every non-null club of the team table (`cup::TEAM_COUNT`, re-read each
/// turn), while the count is below the table size. `out` = the list; returns the count in the low word (the high word
/// is `out`'s, as the original's eax).
/// Original: `UefaCup::collect_club_list` (0x903120, uefa_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn uefa_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let cup = Cup(this);
    let n = cup.team_count();
    let v = exe::crt_malloc(((n as i32) << 2) as u32);
    if v == 0 {
        show_error_at(0xaa19e0, 0x2d6);
        write_usize(out, 0);
        return 0; // simplification: the original returns the box's eax with ax = 0; the caller reads the word
    }
    let mut count = 0i16;
    let round = match kind as u8 {
        1 => Some(cup.round_count().wrapping_sub(8)),
        2 => Some(cup.round_count().wrapping_sub(7)),
        3 => Some(cup.round_count().wrapping_sub(4)),
        _ => None,
    };
    let push = |c: usize, count: &mut i16| {
        write_usize(v + 4 * *count as usize, c);
        *count = count.wrapping_add(1);
    };
    match round {
        Some(r) if r > -1 => {
            covered!("cqx_12 uefa cup round clubs");
            let r = r as usize;
            let mut k = 0i16;
            while k < read_i16(cup.round_table().wrapping_add(r * cup_round::SIZE) + 0x1a) {
                let tie = read_usize(cup.tie_tables() + 4 * r).wrapping_add(0x35 * k as usize);
                let home = read_i16(tie);
                if home != -1 {
                    push(read_usize(cup.team_table().wrapping_add(6 * home as isize as usize)), &mut count);
                }
                let away = read_i16(tie + 2);
                if away != -1 { // cqx12: the away club of the tie
                    push(read_usize(cup.team_table().wrapping_add(6 * away as isize as usize)), &mut count);
                }
                k += 1;
            }
        }
        _ => {
            let mut i = 0i16;
            while i < cup.team_count() {
                let c = read_usize(cup.team_table() + 6 * i as usize);
                if c != 0 && count < n {
                    push(c, &mut count);
                }
                i += 1;
            }
        }
    }
    write_usize(out, v);
    (out as u32 & 0xffff_0000) | count as u16 as u32
}

/// Comparer for `uefa_cup_collect_club_list`: side by side (it only allocates its own list): the original with the
/// caller's `out`, the copy with its own; the counts (the low word) and the clubs of the two lists must be equal; the
/// copy's list is freed.
unsafe extern "thiscall" fn compare_uefa_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%)(this, out, kind);
    let mut mine = 0usize;
    let c = uefa_cup_collect_club_list(this, &mut mine as *mut usize as usize, kind);
    let theirs = read_usize(out);
    let n = (r as u16 as i16).max(0) as usize;
    let same = r as u16 == c as u16 && (theirs == 0) == (mine == 0) && (theirs == 0 || (0..n).all(|k| read_usize(theirs + 4 * k) == read_usize(mine + 4 * k)));
    if mine != 0 {
        exe::crt_free(mine);
    }
    SCOREBOARD.record(same, || format!("uefa_cup_collect_club_list({:#x}, {}): original {} copy {}", this, kind as u8, r as u16, c as u16));
    r
}

/// Rule: the two losing semi-finalists of a European nation's domestic cup are kept in the UEFA seeding (in the cup's
/// slot, or the first free of 50), for the UEFA Cup places of the next season; a cup of another continent is left out.
///
/// `UefaSeeding::record_cup_semi_final_losers(comp_id, club_a, club_b)`. The competition (`comp_record_of_key`'s
/// arithmetic: [`G_CLUB_COMP_TABLE`] + 0x6b × id) must have a continent (`competition::CONTINENT`) and it must be
/// Europe ([`KEY_CONTINENT_EUROPE`]); otherwise nothing. Among the 50 slots of 12 bytes at +0x440 (the cup id, then the
/// two clubs' dwords), the one with this id, or else the first with −1; none: the error box at line 0x6b2. The slot
/// gets the id, `club_a` (+4) and `club_b` (+8).
/// Original: `UefaSeeding::record_cup_semi_final_losers` (0x905790, uefa_seeding.cpp, thiscall, ret 0xc).
unsafe extern "thiscall" fn uefa_seeding_record_cup_semi_final_losers(this: usize, comp_id: u32, club_a: u32, club_b: u32) {
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b));
    let continent = read_usize(comp + competition::CONTINENT);
    if continent == 0 || read_u32(continent) != read_u32(KEY_CONTINENT_EUROPE) {
        return;
    }
    let mut slot: i8 = -1;
    for k in 0..0x32i8 {
        let e = read_u32(this + 0x440 + 12 * k as usize);
        if e == comp_id {
            slot = k;
            break;
        }
        if slot == -1 && e == u32::MAX {
            slot = k;
        }
    }
    if slot == -1 {
        return show_error_at(0xaa1a1c, 0x6b2);
    }
    covered!("cqx_12 semi-final losers");
    let e = this + 0x440 + 12 * slot as usize;
    write_u32(e, comp_id);
    write_u32(e + 4, club_a); // cqx12: the first semi-final loser
    write_u32(e + 8, club_b);
}

/// Comparer for `uefa_seeding_record_cup_semi_final_losers`, side by side over the same object: the original first,
/// then the copy, which finds the cup's slot the original filled and writes it again; the 50 slots after the copy must
/// equal the original's.
unsafe extern "thiscall" fn compare_uefa_seeding_record_cup_semi_final_losers(this: usize, comp_id: u32, club_a: u32, club_b: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(%%I1%%)(this, comp_id, club_a, club_b);
    let original = crate::comparison::MemorySnapshot::read(&[(this + 0x440, 0x258)]);
    uefa_seeding_record_cup_semi_final_losers(this, comp_id, club_a, club_b);
    let copy = crate::comparison::MemorySnapshot::read(&[(this + 0x440, 0x258)]);
    SCOREBOARD.record(copy == original, || format!("uefa_seeding_record_cup_semi_final_losers({}, {:#x}, {:#x}): the slots differ", comp_id, club_a, club_b));
}

/// Rule: the Mercosur records: in the groups (stages below 4) the qualified club gets the record of round 0x78 (weight
/// 0x28); in the knockout (stage 4) elimination and qualification weigh by round (50 to 100 over the 2 rounds) and the
/// winner and runner-up get the final records.
///
/// `ConmebolMerc::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44). Stage
/// (signed byte) below 4: outcome 6, the qualification (`european_cup_add_qualified_record`: 0, 0x78, 0x28), returns 0;
/// otherwise −1. Stage 4, weight = `scale_by_round`(0x32, 0x64, the fixture's `round_number()`, 2): 0xff, the
/// elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification with `round_prefix()` and the name word +0x6f of the fixture's round in the round table of stage 4
/// ([`cup::STAGES`][4] + `cup::ROUND_TABLE`; the high 16 bits come from a small product of the round: zero); 1, the
/// winner (0x86a800); 2, the runner-up (0x86b0b0); returns 0. Above 4: 0.
/// Original: `ConmebolMerc::update_staff_records_for_status` (0x4c5960, conmebol_merc.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn conmebol_merc_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 4 {
        if d != 6 {
            return u32::MAX;
        }
        covered!("cqx_12 mercosur group qualified");
        european_cup_add_qualified_record(club, comp, 0, 0x78, 0x28);
    } else if f == 4 {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                let knockout = read_usize(read_usize(this + cup::STAGES) + 0x10);
                let name = read_u16(read_usize(knockout + cup::ROUND_TABLE).wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            _ => {}
        }
    }
    0
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: the return (−1
// in the groups for an outcome other than 6, otherwise 0).
unsafe extern "thiscall" fn compare_conmebol_merc_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if f < 4 && d != 6 { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I2%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("conmebol_merc_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}
