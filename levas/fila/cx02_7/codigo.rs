// ---------- cx02_7: the Emperor's Cup rounds, the J. League points and the Coppa Italia seeding pots (jap_emp_cup.cpp, kor_league_cup.cpp, ita_cup.cpp) ----------

/// The Emperor's Cup rounds: a prize fund of 7,000,000 and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `JapEmpCup::set_up_rounds` (0x662b00, jap_emp_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_emp_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx02_7 emperor cup rounds");
    set_up_prize_rounds(Cup(this), 7_000_000)
}

/// The points a club takes from a fixture in the J1, the J2 and the Korean League Cup (the shared virtual): the win
/// points (`WIN_POINTS`) for a win in normal time, 0 for a defeat; level after normal time, with extra time
/// (`MATCH_RULES` bit 2) and a winner there, 2 for the extra-time winner and 0 for the loser; still level, with
/// penalties (bits 0x21) and a winner there, the draw points (`DRAW_POINTS`) for the shoot-out winner and 0 for the
/// loser; otherwise the draw points. The goals are compared as signed bytes. Only al is written: the 3 high bytes of
/// eax are the fixture's address.
/// Original: `KorLeagueCup::points_for_result` (0x66d770, kor_league_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_points_for_result(this: usize, club: usize, fixture: usize) -> u32 {
    let (league, p) = (League(this), Fixture(fixture));
    let (home, away) = (p.home_club(), p.away_club());
    let won = |a: i8, b: i8| (club == home.addr() && a > b) || (club == away.addr() && a < b);
    let rules = league.match_rules() as u8;
    let v = if p.home_goals() != p.away_goals() {
        if won(p.home_goals(), p.away_goals()) { league.win_points() } else { 0 }
    } else if rules & 2 != 0 && p.home_goals_extra_time() != p.away_goals_extra_time() {
        covered!("cx02_7 j league extra time");
        if won(p.home_goals_extra_time(), p.away_goals_extra_time()) { 2 } else { 0 }
    } else if rules & 0x21 != 0 && p.home_penalties() != p.away_penalties() {
        if won(p.home_penalties(), p.away_penalties()) { league.draw_points() } else { 0 }
    } else {
        league.draw_points()
    };
    (fixture as u32 & 0xffff_ff00) | v as u32
}

/// The Coppa Italia seeding pot of a club, from its last season (`club::LAST_DIVISION`, `club::LAST_POSITION` as a
/// signed byte) and its division: 4 for 3rd to 6th in Serie B, or relegated from Serie A into Serie B; 3 for 7th to
/// 14th in Serie B; 2 for 15th to 18th in Serie B, or a Serie B club that came from a division other than Serie B and
/// Serie A (promoted); 1 otherwise (and without a last division).
unsafe fn coppa_italia_seeding_pot(club: Club) -> i8 {
    let last = club.last_division();
    if last.is_null() {
        return 1;
    }
    let (serie_a, serie_b) = (read_u32(KEY_CLUB_COMP_ITALIAN_SERIE_A), read_u32(KEY_CLUB_COMP_ITALIAN_SERIE_B));
    let (id, position) = (read_u32(last.addr()), club.last_position() as i8);
    let division = club.division();
    if (id == serie_b && (3..=6).contains(&position)) || (id == serie_a && read_u32(division.addr()) == serie_b) {
        return 4;
    }
    if id == serie_b && (7..=14).contains(&position) {
        return 3;
    }
    if (id == serie_b && (15..=18).contains(&position)) || (read_u32(division.addr()) == serie_b && last.addr() != division.addr() && id != serie_a) {
        return 2;
    }
    1
}

/// qsort callback ordering the Coppa Italia entrants by seeding pot (`coppa_italia_seeding_pot`): pot of a − pot of b.
/// Original: `compare_clubs_by_seeding_pot` (0x640590, ita_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_clubs_by_seeding_pot(a: *const usize, b: *const usize) -> i32 {
    let (pa, pb) = (coppa_italia_seeding_pot(Club(*a)), coppa_italia_seeding_pot(Club(*b)));
    if pa == 4 {
        covered!("cx02_7 coppa italia pot 4");
    }
    pa as i32 - pb as i32
}

pub(crate) unsafe extern "thiscall" fn compare_jap_emp_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_set_up_prize_rounds("jap_emp_cup_set_up_rounds", Cup(this), || o(this), || jap_emp_cup_set_up_rounds(this))
}

/// Comparer for `kor_league_cup_points_for_result`: the query only reads; the returned byte (the caller reads al) must
/// match.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_points_for_result(this: usize, club: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(this, club, fixture);
    let c = kor_league_cup_points_for_result(this, club, fixture);
    SCOREBOARD.record(r as u8 == c as u8, || format!("kor_league_cup_points_for_result({:#x}, {:#x}) original {} copy {}", club, fixture, r as u8, c as u8));
    r
}

compare_order!(compare_compare_clubs_by_seeding_pot, compare_clubs_by_seeding_pot, %%I2%%);
