// ---------- sbA_4: the FIFA ranking's constructor and an international result (fifa_rankings.cpp) ----------

/// `FIFA_RANKINGS::load_dat(archive)` (0x58d7a0, thiscall, ret 4): reads the table from the saved game.
const FN_FIFA_RANKINGS_LOAD_DAT: usize = 0x58d7a0;

/// Builds the FIFA ranking: its date (+5) defaulted and then set to the game start date (`G_GAME_START_DATE`), whose
/// month goes to +4; from a saved game's archive the table is loaded (`FIFA_RANKINGS::load_dat`), and when that fails,
/// or for a new game, the default table is made (`FIFA_RANKINGS::create_default_table`). Returns `this`.
/// Original: `FIFA_RANKINGS::FIFA_RANKINGS` (0x58cea0, fifa_rankings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_fifa_rankings(this: usize, archive: usize) -> usize {
    exe::date_date_54a110(this + 5); // +5: the date of the last update
    write_u8(this + 4, exe::date_get_month(G_GAME_START_DATE)); // +4: the rollover month
    let mut copy = [0u8; 8];
    exe::date_operator_3d(this + 5, copy.as_mut_ptr() as usize, G_GAME_START_DATE);
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FIFA_RANKINGS_LOAD_DAT);
    if archive == 0 || load(this, archive) == 0 {
        create_default_table(this);
    }
    this
}

/// The FIFA points of an international result for both national sides, before they are appended: `Ok(None)` when the
/// result does not count, `Err(())` for a competition the ranking does not know.
///
/// It counts only with both clubs' nations in a continent and a competition. The weight F is 1, or the continent's
/// (+0xbe, double) when both nations are in the same one. The competition's weight W: 1.5 for the World Cup
/// (`KEY_CLUB_COMP_FIFA_WORLD_CUP`); 1.25 when `comp_active_key_nation_of_fixture` gives 0; for a finals competition
/// (`national_teams_is_finals_comp`, with a continent) 1.25 for the World Cup or the European Championship
/// (`KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`) and else 1.1; 1.1 when the key nation is 1 or for a finals
/// competition without a continent; 1 for a friendly (`Friendly::is_friendly_competition`); else unknown. The base points
/// (home, away) by the strength tiers (`FIFA_RANKINGS::get_nation_strength_tier`; the home tier minus 1 above the
/// away one is 2, the away minus 1 above the home -2, then 1, 0 or -1) and the result (after extra time when played):
/// see the table in the code. Each base b becomes b + goals (at most 5) × 0.2 × b × 0.3 (f32); the away side gets
/// 0.3 more when the stadium's city is in the home club's nation. Home = W × F × home (not negative), away = f32(W × F)
/// × away (f32, not negative); a home nation of reputation under 3500 caps home at 3 and lifts away to 2.5; an away
/// nation under 3500 caps away at 3 and lifts home to 2.5. The points are × 1000, truncated.
unsafe fn fifa_result_points(this: usize, fixture: Fixture) -> Result<Option<[(usize, u32); 2]>, ()> {
    let (home, away) = (fixture.home_club(), fixture.away_club());
    let (hn, an) = (home.nation(), away.nation());
    if hn.is_null() || hn.continent() == 0 || an.is_null() || an.continent() == 0 || fixture.competition().is_null() {
        return Ok(None);
    }
    let comp = fixture.competition();
    let f = if hn.continent() == an.continent() { read_f64(hn.continent() + 0xbe) } else { 1.0 }; // the continent's weight
    let th = get_nation_strength_tier(this, hn.addr()) as i8 as i32;
    let ta = get_nation_strength_tier(this, an.addr()) as i8 as i32;
    let code = if th - 1 > ta {
        2
    } else if ta - 1 > th {
        -2
    } else if ta < th {
        1
    } else if ta == th {
        0
    } else {
        -1
    };
    let (hg, ag) = if fixture.home_goals_extra_time() == -1 {
        (fixture.home_goals(), fixture.away_goals())
    } else {
        (fixture.home_goals_extra_time(), fixture.away_goals_extra_time())
    };
    let id = read_i32(comp.addr());
    let w = if id == read_i32(KEY_CLUB_COMP_FIFA_WORLD_CUP) {
        read_f64(rdata::F64_1_5)
    } else if exe::comp_active_key_nation_of_fixture(comp, 0usize) == 0 {
        read_f64(rdata::F64_1_25)
    } else if comp.continent() != 0 && exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), comp) != 0 {
        if id == read_i32(KEY_CLUB_COMP_FIFA_WORLD_CUP) || id == read_i32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) {
            read_f64(rdata::F64_1_25)
        } else {
            read_f64(rdata::F64_1_1)
        }
    } else if exe::comp_active_key_nation_of_fixture(comp, 0usize) == 1
        || (comp.continent() == 0 && exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), comp) != 0) {
        read_f64(rdata::F64_1_1)
    } else if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 {
        read_f64(rdata::F64_1)
    } else {
        return Err(());
    };
    // (away, home) base points by the tier code and the result
    let (away_base, home_base): (f32, f32) = match (code, hg.cmp(&ag)) {
        (2, core::cmp::Ordering::Greater) => (1.0, 1.0),
        (2, core::cmp::Ordering::Equal) => (2.0, 0.0),
        (2, core::cmp::Ordering::Less) => (3.0, -1.0),
        (1, core::cmp::Ordering::Greater) => (0.1, 1.9),
        (1, core::cmp::Ordering::Equal) => (1.1, 0.9),
        (1, core::cmp::Ordering::Less) => (2.1, -0.1),
        (0, core::cmp::Ordering::Greater) => (0.0, 2.0),
        (0, core::cmp::Ordering::Equal) => (1.0, 1.0),
        (0, core::cmp::Ordering::Less) => (2.0, 0.0),
        (-1, core::cmp::Ordering::Less) => (1.9, 0.1),
        (-1, core::cmp::Ordering::Equal) => (0.9, 1.1),
        (-1, core::cmp::Ordering::Greater) => (-0.1, 2.1),
        (_, core::cmp::Ordering::Less) => (1.0, 1.0),
        (_, core::cmp::Ordering::Equal) => (0.0, 2.0),
        (_, core::cmp::Ordering::Greater) => (-1.0, 3.0),
    };
    let goals = |g: i8, base: f32| (g.min(5) as f64 * read_f64(rdata::F64_0_2) * base as f64 * read_f64(rdata::F64_0_3) + base as f64) as f32;
    let home_pts = goals(hg, home_base);
    let mut away_pts = goals(ag, away_base);
    let stadium = read_usize(fixture.addr() + fixture::STADIUM);
    if stadium != 0 {
        let city = Stadium(stadium).city();
        if city != 0 && read_usize(city + city::NATION) == hn.addr() {
            covered!("sbA_4 away side in the home nation");
            away_pts = (away_pts as f64 + read_f32(0x96a324) as f64) as f32; // 0.3
        }
    }
    let wf = w * f;
    let mut h = wf * home_pts as f64;
    let mut a = ((wf as f32) as f64 * away_pts as f64) as f32;
    if h < 0.0 {
        h = 0.0;
    }
    if a < 0.0 {
        a = 0.0;
    }
    let (h_weak, a_weak) = (hn.reputation() < 3500, an.reputation() < 3500);
    if h_weak && h > 3.0 {
        h = 3.0;
    }
    if a_weak {
        if a > 3.0 {
            a = 3.0;
        }
        if h <= 2.5 {
            h = 2.5;
        }
    }
    if h_weak && a <= 2.5 {
        a = 2.5;
    }
    let ph = ftol(h * read_f64(rdata::F64_1000)) as u32;
    let pa = ftol(a as f64 * read_f64(rdata::F64_1000)) as u32;
    Ok(Some([(hn.addr(), ph), (an.addr(), pa)]))
}

/// Scores a finished international match for the FIFA ranking: each national side gets its points appended to its
/// history (`FIFA_RANKINGS::append_result`; see `fifa_result_points`). A competition the ranking does not know shows
/// the error box (fifa_rankings.cpp 0xde) and nothing is appended.
/// Original: `FIFA_RANKINGS::record_match_result` (0x58cf70, fifa_rankings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fifa_record_match_result(this: usize, fixture: usize) {
    match fifa_result_points(this, Fixture(fixture)) {
        Ok(Some(points)) => {
            for (nation, p) in points {
                append_result(this, nation, p, fixture);
            }
        }
        Ok(None) => {}
        Err(()) => show_error_at(0x9c74f4, 0xde), // fifa_rankings.cpp
    }
}

/// Comparer for `new_fifa_rankings`: only the original runs (it allocates or loads the table); it checks the month and
/// the date it leaves (the game start date).
pub(crate) unsafe extern "thiscall" fn compare_new_fifa_rankings(this: usize, archive: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, archive);
    let date = core::slice::from_raw_parts((this + 5) as *const u8, 8) == core::slice::from_raw_parts(G_GAME_START_DATE as *const u8, 8);
    let month = read_u8(this + 4) == exe::date_get_month(G_GAME_START_DATE);
    SCOREBOARD.record(r == this && date && month && read_usize(this) != 0,
        || format!("new_fifa_rankings(archive {:#x}): date {} month {} table {:#x}", archive, date, month, read_usize(this)));
    r
}

/// Comparer for `fifa_record_match_result`: the history may be reallocated (it does not run twice), so the copy plans
/// the points beforehand, the original runs and each nation's new last entry (`fifa_ranking_entry::HISTORY_COUNT`, 5
/// bytes an entry at +4) must hold the planned points; with no points planned the counts must not change.
pub(crate) unsafe extern "thiscall" fn compare_fifa_record_match_result(this: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let plan = fifa_result_points(this, Fixture(fixture));
    let Ok(Some(points)) = plan else {
        return o(this, fixture);
    };
    let before = points.map(|(n, _)| read_i16(ranking_entry(this, n) + fifa_ranking_entry::HISTORY_COUNT));
    o(this, fixture);
    for (k, &(nation, p)) in points.iter().enumerate() {
        let e = ranking_entry(this, nation);
        let count = read_i16(e + fifa_ranking_entry::HISTORY_COUNT);
        let last = read_u16(read_usize(e + 4).wrapping_add((count as isize as usize).wrapping_sub(1).wrapping_mul(5)));
        SCOREBOARD.record(count == before[k].wrapping_add(1) && last == p as u16,
            || format!("fifa_record_match_result(fixture {:#x}) nation {}: original {} points (count {}), rust {}", fixture, read_usize(nation), last, count, p as u16));
    }
}
