// ---------- rx13_13: the Oceania Club Championship at career creation (oceania_club_champ.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\oceania_club_champ.cpp" (the error boxes).
const OCEANIA_CLUB_CHAMP_CPP: usize = 0xa6a068;
/// The Oceania Club Championship's vtable and its `set_up_rules` (0x7979b0, thiscall, ret 0; through its hook).
const OCEANIA_CLUB_CHAMP_VTABLE: usize = 0x96e018;
const FN_OCEANIA_CLUB_CHAMP_SET_UP_RULES: usize = 0x7979b0;

/// The Oceania Club Championship's edition: an even year before 2001 moves on to the next odd one; an odd year, or an
/// even one from 2001 on, stays (the remainder by 2 signed, as the original's `and 0x80000001` with its adjustment).
pub(crate) fn oceania_club_champ_edition(year: i16) -> i16 {
    let mut y = year;
    if (y as i32) % 2 != 0 {
        return y;
    }
    while y < 0x7d1 { // rx13_13: only before 2001
        y = y.wrapping_add(1);
        if (y as i32) % 2 != 0 {
            break;
        }
    }
    y
}

/// rule: the Oceania Club Championship is a club competition on the league base, played every two years in odd years
/// (an even year before 2001 moves on to the next odd one); discipline rule 0x23 and importance 0xf; from a saved game it
/// loads its state, otherwise it starts with two stages beyond the league, its statistics and its rules
///
/// League::League (`exe::new_league`); the vtable 0x96e018; `league::YEAR` = `year` (word); `league::COMPETITION` =
/// the record; `club_sides` 2; the upper and lower division ids −1; the edition (`oceania_club_champ_edition`, the
/// year word rewritten); `discipline_rule` 0x23 and `importance` 0xf. If the saved state loads
/// (`exe::league_load_from_temp_file(this, 1)` nonzero), this. Otherwise `stage_index` 0xff, `current_stage` −1,
/// `stage_count` 2, the stage vector (`exe::crt_malloc(8)`, unchecked), `active` 0; the statistics (operator new
/// `exe::crt_operator_new` of 0x5ce and `exe::new_comp_stats(new, this)`) in `comp_stats` (null: the box at line 0x51
/// and this); the rules (OceaniaClubChamp::set_up_rules, its return not read). Returns this. The original's SEH frame
/// only matters for an exception in the statistics constructor.
/// Original: `OceaniaClubChamp::OceaniaClubChamp` (0x796fd0, oceania_club_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_oceania_club_champ(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    exe::new_league(l);
    write_usize(this, OCEANIA_CLUB_CHAMP_VTABLE);
    l.set_year(year as u16 as i16);
    write_usize(this + league::COMPETITION, record);
    l.set_club_sides(2);
    l.set_upper_division_id(-1);
    l.set_lower_division_id(usize::MAX);
    l.set_year(oceania_club_champ_edition(year as u16 as i16));
    l.set_discipline_rule(0x23); // rx13_13: the discipline rule
    l.set_importance(0xf);
    if exe::league_load_from_temp_file(l, 1) != 0 {
        return this;
    }
    covered!("rx13_13 oceania club championship new");
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(usize::MAX);
    l.set_stage_count(2); // rx13_13: two stages beyond the league
    l.set_stages(exe::crt_malloc(8));
    l.set_active(0);
    let p = exe::crt_operator_new(0x5ce);
    let s = if p != 0 { exe::new_comp_stats(p, l) } else { 0 };
    l.set_comp_stats(s);
    if s == 0 {
        show_error_at(OCEANIA_CLUB_CHAMP_CPP, 0x51);
        return this;
    }
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_OCEANIA_CLUB_CHAMP_SET_UP_RULES);
    rules(this);
    this
}

/// Comparer for `construct_oceania_club_champ`: only the original runs (it allocates and may load the saved state);
/// then the plan: the return, the vtable, the edition, the record, the discipline rule and, in a new career, the
/// current stage −1, the two stages and their vector, and the statistics.
pub(crate) unsafe extern "thiscall" fn compare_construct_oceania_club_champ(this: usize, year: u32, record: usize) -> usize {
    let loads = league_will_load_saved_state(record);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let l = League(this);
    let mut ok = r == this && read_usize(this) == OCEANIA_CLUB_CHAMP_VTABLE && l.competition().addr() == record && l.discipline_rule() == 0x23
        && built_year_ok(l.year() as u16, oceania_club_champ_edition(year as u16 as i16) as u16);
    if ok && !loads {
        ok = l.current_stage() as i32 == -1 && l.stage_count() as u32 == 2 && l.stages() != 0 && l.comp_stats() != 0;
    }
    SCOREBOARD.record(ok, || format!("construct_oceania_club_champ({:#x}, {}, {:#x}) returned {:#x}, vtable {:#x}, year {}, stages {} {:#x}, statistics {:#x}",
        this, year as u16, record, r, read_usize(this), l.year(), l.stage_count() as u32, l.stages(), l.comp_stats()));
    r
}
