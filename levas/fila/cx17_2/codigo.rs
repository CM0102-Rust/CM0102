// ---------- cx17_2: the qualification status and the staff records of a nation in the South American World Cup qualifying (wc_south_american_league.cpp) ----------

/// The qualifying places of the South American section: six less the exempt nations
/// (`wc_south_american_league_fields::EXEMPT_COUNT`, a signed byte); a progress code (the table position) below it is
/// inside them.
pub(crate) unsafe fn south_american_qualifying_places(this: usize) -> i32 {
    6 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i32 // cx17_2: six places less the exempt
}

/// rule: in the South American qualifying a nation's status is qualified (−2) while its table position (stat 0x12) is
/// inside the qualifying places (six less the exempt nations), and not qualified (−3) otherwise
///
/// `qualifying_stat_byte`(club, 0x12) below `south_american_qualifying_places`: −2, otherwise −3 (a word).
/// Original: `WcSouthAmericanLeague::get_qualification_status` (0x921630, wc_south_american_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_get_qualification_status(this: usize, club: usize) -> i16 {
    covered!("cx17_2 south american qualification status");
    let now = qualifying_stat_byte(this, club, 0x12) as i32;
    (now < south_american_qualifying_places(this)) as i16 - 3
}

/// rule: in the South American qualifying a nation's status changes down (−1) when its table position (stat 0x12) leaves
/// the qualifying places (stat 0x13, the previous position, was inside), up (1) when it enters them, otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now), signed, against `south_american_qualifying_places`: now
/// inside, (before inside) − 1; otherwise (before inside). The byte in al.
/// Original: `WcSouthAmericanLeague::get_qualification_status_change` (0x921670, wc_south_american_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx17_2 south american status change");
    let before = qualifying_stat_byte(this, club, 0x13) as i32;
    let now = qualifying_stat_byte(this, club, 0x12) as i32;
    let places = south_american_qualifying_places(this);
    if now < places {
        (before < places) as i8 - 1
    } else {
        (before < places) as i8
    }
}

/// NationalTeams::add_qualified_nation (thiscall, ret 8).
const FN_NATIONALTEAMS_ADD_QUALIFIED_NATION: usize = 0x7779b0;

/// rule: when the South American qualifying ends, a nation that qualifies (status 7) gets the World Cup qualification
/// record (weight 100), enters the World Cup and the national teams' list of qualified nations and its status becomes 4;
/// one that goes to the play-off against Oceania (1) gets the record of qualifying for the Oceania section (stage 0xa0,
/// weight 70); one that fails (−2) gets the failed-to-qualify record (weight 100) and ends its campaign
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44; only `club`, the outcome
/// byte and `out` are read). The records go to Staff_Records ([`G_STAFF_RECORDS`]). Outcome 7: the World Cup record
/// (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each) with
/// `european_cup_add_qualified_record`(club, record, 0, 0, 0x64); WorldCup::add_qualified_team on the World Cup object
/// (`exe::worldcup_add_qualified_team`); `*out` = 4; NationalTeams::add_qualified_nation(club, record) on
/// [`G_NATIONAL_TEAMS`] (`FN_NATIONALTEAMS_ADD_QUALIFIED_NATION`: 2 pushes; mov ecx -> thiscall, ret 8); returns −1.
/// Outcome 1: `european_cup_add_qualified_record`(club, the Oceania section's record, 0xa0, 0, 0x46); 0. Outcome 0xfe:
/// `european_cup_add_failed_to_qualify_record`(club, `league::COMPETITION`, 0, 0x64) and the national team's campaign
/// ended (`clear_national_team_competition`); 0. Any other outcome: 0.
/// Original: `WcSouthAmericanLeague::update_staff_records_for_status` (0x920a50, wc_south_american_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, _stage: u32, out: usize, _game: usize, _a6: u32) -> u32 {
    let comp_record = |key: usize| read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(key).wrapping_mul(0x6b));
    match outcome as u8 {
        7 => {
            covered!("cx17_2 south american to the world cup");
            european_cup_add_qualified_record(club, comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP), 0, 0, 0x64);
            let world_cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4)));
            exe::worldcup_add_qualified_team(world_cup, club); // cx17_2: the qualified nation enters the World Cup
            write_u8(out, 4);
            let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_QUALIFIED_NATION);
            add(read_usize(G_NATIONAL_TEAMS), club, comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP));
            u32::MAX
        }
        1 => {
            covered!("cx17_2 south american to the oceania play-off");
            european_cup_add_qualified_record(club, comp_record(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION), 0xa0, 0, 0x46);
            0
        }
        0xfe => {
            covered!("cx17_2 south american out");
            european_cup_add_failed_to_qualify_record(club, League(this).competition().addr(), 0, 0x64);
            clear_national_team_competition(Club(club)); // cx17_2: the eliminated nation ends its campaign
            0
        }
        _ => 0,
    }
}

/// Comparers of the two status readers: they only read the statistics; side by side on the result (the word or byte).
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_get_qualification_status(this: usize, club: usize) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(%%I0%%)(this, club);
    let b = wc_south_american_league_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("wc_south_american_league_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(%%I1%%)(this, club);
    let b = wc_south_american_league_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_south_american_league_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

/// Comparer for `wc_south_american_league_update_staff_records_for_status`: the records, the end of the national team's
/// campaign and the World Cup place do not run twice, so only the original runs; the plan: −1 for a qualified nation
/// (outcome 7) with `*out` = 4 and the World Cup's entrant count one more when it had room, 0 otherwise.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I2%%);
    let qualified = outcome as u8 == 7 && out != 0;
    let world_cup = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4))));
    if qualified && world_cup.is_null() {
        return o(this, club, outcome, stage, out, game, a6);
    }
    let n = if qualified { world_cup.entrant_count() } else { 0 };
    let room = qualified && n < world_cup.entrant_capacity();
    let r = o(this, club, outcome, stage, out, game, a6);
    let same = if qualified { r == u32::MAX && read_u8(out) == 4 && world_cup.entrant_count() == if room { n + 1 } else { n } } else { r == 0 };
    SCOREBOARD.record(same, || format!("wc_south_american_league_update_staff_records_for_status({:#x}, {}) returned {:#x}, list {}", club, outcome as u8, r, n));
    r
}
