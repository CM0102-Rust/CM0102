// ---------- rx10_12: the Club World Championship records, seeding order and first group (world_club_champ.cpp) ----------

/// Rule: the Club World Championship records: in the group rounds (stage below 2) the club going through gets the
/// record of round 0x96 and the club going out the failed record (weight 0x19 both); in the knockout (stage 2) the
/// winner, the runner-up and the third get the final records.
///
/// `WorldClubChamp::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44).
/// Stage (signed byte) below 2: outcome 6, `european_cup_add_qualified_record`(club, the competition, 0, 0x96, 0x19);
/// 0xfe, `european_cup_add_failed_to_qualify_record`(club, the competition, 0x41c, 0x19); returns 0. Stage 2: outcome 1
/// the winner (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0), 3 the third (0x86b710), each with the fixture and
/// `final`, and 0; any other outcome returns −1 (unlike the Oceania championship). Stage above 2: 0.
/// Original: `WorldClubChamp::update_staff_records_for_status` (0x929c70, world_club_champ.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn world_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> i32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + league::COMPETITION);
    if f < 2 {
        if d == 6 {
            covered!("rx10_12 club world championship group qualified");
            european_cup_add_qualified_record(club, comp, 0, 0x96, 0x19); // rx10_12: round 0x96
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19);
        }
        return 0;
    }
    if f > 2 {
        return 0;
    }
    covered!("rx10_12 club world championship final records");
    match d {
        1 => cup_record_final_result(0x86a800, club, Fixture(game), final_),
        2 => cup_record_final_result(0x86b0b0, club, Fixture(game), final_),
        3 => cup_record_final_result(0x86b710, club, Fixture(game), final_),
        _ => return -1,
    }
    0
}

/// Comparer for `world_club_champ_update_staff_records_for_status`: only the original (the records write the history
/// and may draw), with the plan: −1 only for stage 2 with an outcome other than 1 to 3, else 0.
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> i32>(%%I0%%)(this, club, outcome, stage, a4, game, final_);
    let plan = if stage as u8 as i8 == 2 && !(1..=3).contains(&(outcome as u8)) { -1 } else { 0 };
    SCOREBOARD.record(r == plan, || format!("world_club_champ_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8 as i8, r));
    r
}

/// Rule: the Club World Championship seeds its entrants by their continent's strength, the stronger first; then by
/// their nation's reputation, then by the nation id (lower first) and last by the club's reputation; a club without a
/// nation sorts before one with.
///
/// `compare_entrants_by_continent_strength(&a, &b)` (cdecl, the qsort comparison of WorldClubChamp::draw_first_group):
/// no nation (`club::NATION`) on either side: 0; only a's missing: −1; only b's missing: 1. Otherwise d = the strength
/// of b's continent − a's (the f64 at +0xbe of the `nation::CONTINENT` record, in x87): d > 0.0 (`rdata::F64_0`): 1; d <
/// 0.0: −1; equal: b's nation reputation − a's (`nation::REPUTATION`, words), else a's nation id − b's, else b's club
/// reputation − a's (`club::REPUTATION`, words). The x87 difference keeps the sign of the f64 one (a NaN strength,
/// which the database does not hold, would take the equal path here and −1 in the original).
/// Original: `compare_entrants_by_continent_strength` (0x92b400, world_club_champ.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_entrants_by_continent_strength(a: *const usize, b: *const usize) -> i32 {
    let (ca, cb) = (*a, *b);
    let (na, nb) = (read_usize(ca + club::NATION), read_usize(cb + club::NATION));
    if na == 0 {
        return if nb == 0 { 0 } else { -1 };
    }
    if nb == 0 {
        return 1;
    }
    let strength = |n: usize| read_f64(read_usize(n + nation::CONTINENT) + 0xbe); // the continent's strength [inference]
    let d = strength(nb) - strength(na);
    let zero = read_f64(rdata::F64_0);
    if d > zero {
        return 1;
    }
    if d < zero {
        return -1;
    }
    let r = read_i16(nb + nation::REPUTATION) as i32 - read_i16(na + nation::REPUTATION) as i32;
    if r != 0 {
        return r;
    }
    let id = (read_u32(na) as i32).wrapping_sub(read_u32(nb) as i32); // rx10_12: the lower nation id first
    if id != 0 {
        return id;
    }
    read_i16(cb + club::REPUTATION) as i32 - read_i16(ca + club::REPUTATION) as i32
}

/// Comparer for `compare_entrants_by_continent_strength`: side by side (it only reads): the same value.
pub(crate) unsafe extern "C" fn compare_compare_entrants_by_continent_strength(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(%%I1%%)(a, b);
    let c = compare_entrants_by_continent_strength(a, b);
    SCOREBOARD.record(r == c, || format!("compare_entrants_by_continent_strength({:#x}, {:#x}): original {} copy {}", *a, *b, r, c));
    r
}

/// Rule: the Club World Championship's first group is the top club of each of the four pots of three: the twelve
/// entrants seeded by continent strength (`compare_entrants_by_continent_strength`), the 1st, 4th, 7th and 10th go to
/// the championship's own four-club table, and the group draw news is sent.
///
/// `WorldClubChamp::draw_first_group()`: `league::CLUB_COUNT` = 4; the table (`exe::crt_malloc`(0xec); none: the
/// world_club_champ.cpp box at line 0x220 and 0); the entrant list (+0xee, its count the dword +0xf2) sorted
/// (`exe::crt_qsort`(list, count, 4, 0x92b400)); row r (byte) gets entrant 3r (`exe::league_init_table_row`(this, r,
/// club, 0, 0)); League::send_group_draw_news (`exe::league_send_group_draw_news`). Returns 1.
/// Original: `WorldClubChamp::draw_first_group` (0x929aa0, world_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_champ_draw_first_group(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    l.set_table(t);
    if t == 0 {
        show_error_at(0xad5764, 0x220);
        return 0;
    }
    covered!("rx10_12 club world championship first group");
    exe::crt_qsort(read_usize(this + 0xee), read_u32(this + 0xf2), 4, 0x92b400usize); // rx10_12: seeded by continent strength
    let mut r: i8 = 0;
    while (r as i16) < l.club_count() {
        exe::league_init_table_row(l, r as u8 as u32, read_usize(read_usize(this + 0xee) + 12 * r as usize), 0, 0);
        r += 1;
    }
    exe::league_send_group_draw_news(l);
    1
}

/// Comparer for `world_club_champ_draw_first_group`: only the original (it allocates and sends news), with the plan:
/// 1 and the four rows hold entrants 0, 3, 6 and 9 of the sorted list.
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_draw_first_group(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let l = League(this);
    let list = read_usize(this + 0xee);
    let ok = r == 0 || (l.club_count() == 4 && (0..4).all(|k| read_usize(l.table() + k * league_table_row::SIZE) == read_usize(list + 12 * k)));
    SCOREBOARD.record(ok, || format!("world_club_champ_draw_first_group({:#x}): returned {}", this, r));
    r
}
