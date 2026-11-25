// ---------- rx10_7: the Portuguese Segunda B constructor, rules and table (por_second_b.cpp) ----------

/// The Portuguese Segunda Divisão B: vtable 0x96e858, the Portuguese discipline rules, two section stages (the Central
/// and South zones), its set_up_rules 0x7d2610, create_table 0x7d2400, create_zone_stage 0x7d2870 and
/// set_predicted_positions 0x7d2cd0, the boxes of por_second_b.cpp at lines 0x47, 0x4e, 0x55, 0x5d and 0x66.
pub(crate) const SEGUNDA_B: DividedLeague = DividedLeague { vtable: 0x96e858, discipline: discipline_rule::PORTUGAL, sections: 2, set_up_rules: 0x7d2610,
    create_table: 0x7d2400, section_stage: 0x7d2870, predicted: 0x7d2cd0, file: 0xa78798, lines: [0x47, 0x4e, 0x55, 0x5d, 0x66] };

/// Rule: the Portuguese Segunda B is a league of the game's year under the Portuguese discipline rules, in three
/// zones (North in its own table, Central and South as stages); with nothing saved, its rules, North table, order,
/// statistics, fixtures, the two zone stages and the predicted positions are made.
///
/// `construct_divided_league` with `SEGUNDA_B` (the original pushes the zone index with the statistics pointer's high
/// bytes; the zone routine reads the byte for its stage slot, but passes the whole dword to virtual
/// `comp_vtable::CREATE_SCHEDULE`, which reads the byte too).
/// Original: `PorSecondB::PorSecondB` (0x7d1920, por_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_por_second_b(this: usize, year: u32, record: usize) -> usize {
    covered!("rx10_7 new segunda b");
    construct_divided_league(this, year, record, &SEGUNDA_B)
}

/// Comparer for `construct_por_second_b` (`compare_divided_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_por_second_b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_divided_league_ctor("construct_por_second_b", this, year, record, &SEGUNDA_B, || o(this, year, record))
}

/// The rule writes of the Portuguese Segunda B (offset, size, value).
///
/// Two legs, 3 points a win and 1 a draw, category 2, national clubs, tie-breaks 4, 1 and 2, one promoted from each
/// zone to the Liga de Honra (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION` in `league::UPPER_DIVISION_ID`), no play-off,
/// 4 relegated with no league below (−1), `league::SQUAD_FLAGS` = 2, 7 substitutes named and 3 used.
pub(crate) unsafe fn segunda_b_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0)];
    v.push((0xc1, 1, 4)); // rx10_7: four go down from each zone
    v.extend_from_slice(&[(0x1c, 4, read_usize(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION) as u32), (0x52, 1, 2), (0x20, 4, 0xffff_ffff), (0x49, 1, 7), (0x4a, 1, 3)]);
    v
}

/// Rule: the Segunda B is played home and away, 3 points a win and 1 a draw; each zone's champion goes up to the Liga
/// de Honra and four go down, with no league below; 7 substitutes named and 3 used.
///
/// The fields of `segunda_b_rules`, then the round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1,
/// &`league::ROUND_COUNT`, &`league::STAGE_ID`, 0)) in `league::ROUND_DATES` (`eng_prm_apply_league_rules`). Returns 1.
/// Original: `PorSecondB::set_up_rules` (0x7d2610, por_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_b_set_up_rules(this: usize) -> u32 {
    covered!("rx10_7 segunda b rules");
    eng_prm_apply_league_rules(League(this), &segunda_b_rules())
}

/// Comparer for `por_second_b_set_up_rules`, as a dry run (`eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_por_second_b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("por_second_b_set_up_rules", League(this), segunda_b_rules(), || o(this), || por_second_b_set_up_rules(this))
}

/// The three Segunda B zones (North, Central, South) as division keys.
pub(crate) const SEGUNDA_B_ZONES: [usize; 3] = [KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL,
    KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH];

/// Rule: the Segunda B North has 20 clubs: the Segunda B clubs of the North zone, in database order. In 2001 the
/// clubs of the three old zone divisions are first gathered into the Segunda B, the zone kept as their section.
///
/// `league::CLUB_COUNT` = 20; in 2001 the clubs before the national teams whose division id is one of
/// `SEGUNDA_B_ZONES` get `club::RESERVE_DIVISION` = that division and `club::DIVISION` = this league's record; then
/// `fill_section_table` with the Segunda B (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`) and the North zone, no B team
/// marks, the boxes at lines 0x192 and 0x1a3. No prize fund. Returns 1 when full.
/// Original: `PorSecondB::create_table` (0x7d2400, por_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_b_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x14); // rx10_7: 20 in each zone
    if l.year() == 0x7d1 {
        covered!("rx10_7 segunda b gathered");
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 - 2 * read_usize(G_NATION_COUNT) as i32 {
            let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
            let d = club.division().addr();
            if d != 0 && SEGUNDA_B_ZONES.iter().any(|&g| read_usize(d) == read_usize(g)) {
                write_usize(club.addr() + club::RESERVE_DIVISION, d);
                write_usize(club.addr() + club::DIVISION, l.competition().addr());
            }
            i += 1;
        }
    }
    fill_section_table(l, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH, false, 0xa78798, [0x192, 0x1a3])
}

/// Comparer for `por_second_b_create_table`: only the original (in 2001 it moves clubs), with the plan
/// (`compare_section_table`).
pub(crate) unsafe extern "thiscall" fn compare_por_second_b_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_section_table("por_second_b_create_table", this, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH, || o(this))
}
