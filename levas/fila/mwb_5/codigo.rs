// ---------- mwb_5: the Scottish First Division (sco_first.cpp): constructor, table and rules; the moulds of the Scottish and Welsh divisions ----------

/// Describes one of the Scottish and Welsh league divisions: the vtable, the stadium requirement (capacity and seats,
/// `None` when the constructor leaves them), the discipline rule, whether the 1999 season had one stage beyond the league
/// (the First Division's promotion play-off), the custom table compare the saved state gets back (the Premier Division's
/// split, `None` elsewhere), the rules and table routines (thiscall, ret 0), the source file and the error-box lines
/// (rules, table, fixtures, order, statistics).
pub(crate) struct BritishDivisionSpec {
    pub(crate) vtable: usize,
    pub(crate) stadium: Option<(u16, u16)>,
    pub(crate) discipline: u8,
    pub(crate) playoff_in_1999: bool,
    pub(crate) split_compare: Option<usize>,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The number of stages beyond the league: 1 in the 1999 season of a division with a play-off then, 0 otherwise.
pub(crate) unsafe fn british_division_stage_count(this: League, e: &BritishDivisionSpec) -> u32 {
    if e.playoff_in_1999 && this.year() == 0x7cf { 1 } else { 0 }
}

/// The constructor shared by the Scottish Premier, First, Second and Third Divisions and the League of Wales (the same
/// template, checked instruction by instruction: only the vtable, the stadium requirement, the 1999 play-off, the
/// Premier's split compare, the two routines, the source file and the lines change).
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); the record (`league::COMPETITION`), the vtable, the
/// year (word) = `year`, the stadium requirement (`league::REQUIRED_CAPACITY`, `league::REQUIRED_SEATS`) and the
/// discipline rule. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`, nonzero), the Premier gets
/// its split compare back (`league::CUSTOM_COMPARE`, when the season is played in four rounds and the year is not 1999)
/// and it returns this.
///
/// Otherwise (a new career): `league::STAGE_INDEX` = 0xff, `league::CURRENT_STAGE` = −1, the stages beyond the league
/// (`british_division_stage_count`) and their vector (`exe::crt_malloc` of 4 per stage, unchecked; 0 without stages);
/// then, each with its own error box (and returning this) when it returns 0: the rules, the table, the fixtures
/// (`exe::league_create_fixtures`) and the order (`exe::league_sort_table(this, 0)`); the statistics (operator new
/// `exe::crt_operator_new(0x5ce)` and `exe::new_comp_stats(new, this)`, under the original's SEH frame) into
/// `league::COMP_STATS` (null: the box); finally the predicted positions (`exe::league_set_predicted_positions`).
pub(crate) unsafe fn construct_british_division(this: League, year: u32, record: usize, e: &BritishDivisionSpec, ramo: &'static str) -> usize {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), e.vtable);
    this.set_year(year as u16 as i16);
    if let Some((capacity, seats)) = e.stadium {
        this.set_required_capacity(capacity);
        this.set_required_seats(seats);
    }
    this.set_discipline_rule(e.discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        if let Some(f) = e.split_compare {
            if this.legs() == 4 && this.year() != 0x7cf {
                this.set_custom_compare(f);
            }
        }
        return this.addr();
    }
    covered!(ramo);
    this.set_stage_index(0xff);
    this.set_current_stage(0xffff_ffff);
    let n = british_division_stage_count(this, e);
    this.set_stage_count(n as usize);
    if n as i32 > 0 {
        let v = exe::crt_malloc(n * 4);
        this.set_stages(v);
    } else {
        this.set_stages(0);
    }
    if rules(this.addr()) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this.addr();
    }
    if table(this.addr()) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let p = exe::crt_operator_new(0x5ce);
    let s = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Comparer for the division constructors: only the original runs (it allocates the table, the fixtures and the
/// statistics); then the plan: the returned this, the vtable, the year, the record, the discipline rule, the stadium
/// requirement and, in a new career with statistics, the stages and the table.
pub(crate) unsafe fn compare_british_division_ctor(name: &str, this: League, year: u32, record: usize, e: &BritishDivisionSpec, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtable && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == e.discipline;
    if let Some((c, s)) = e.stadium {
        ok = ok && this.required_capacity() == c && read_u16(this.addr() + league::REQUIRED_SEATS) == s;
    }
    if !loaded && this.comp_stats() != 0 {
        let n = british_division_stage_count(this, e) as usize;
        ok = ok && this.stage_index() == 0xff && this.stage_count() == n && (this.stages() != 0) == (n != 0) && this.table() != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year arg {}, record arg {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, year {}, record {:#x}, discipline {}, stages {} at {:#x}, table {:#x}, stats {:#x}",
        name, this.addr(), year as u16, record, loaded, r, read_usize(this.addr()), this.year(), this.competition().addr(), this.discipline_rule(), this.stage_count(), this.stages(), this.table(), this.comp_stats()));
    r
}

/// The table of a Scottish or Welsh division: `league::CLUB_COUNT` = `n`, the table (`exe::crt_malloc` of n × 0x3b;
/// out of memory, the box of `row` and 0), the clubs whose division (`club::DIVISION`) is the competition of global `g`,
/// in database order, each into the next row (`exe::league_init_table_row(this, row, club, 0, 0)`), until full
/// (`rotate` is 0 in the original: row k gets the k-th club; it exists only for the mutations).
///
/// Not full: 0. Full: with a prize fund, `league::PRIZE_FUND` = League::calc_prize_fund(`prize`) and `league::PRIZE_MODE`
/// = 1, and with TV money, League::pay_tv_rights(`tv`, 0) to every club (the Premier); returns 1 (the Second and Third
/// Divisions, the Welsh League and the Premier from 2000 on have no prize fund or TV money of their own here: only
/// those given).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn fill_british_division_table(this: League, n: i16, file: usize, row: u32, g: usize, prize: Option<i32>, tv: Option<i32>, rotate: u8, ramo: &'static str) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(g) {
            // simplification: the original pushes the row in eax with the high bits of the division id; 0x687430 only
            // reads the byte
            let l = ((k as u32 + rotate as u32) % n as u32) as u8;
            exe::league_init_table_row(this, l as u32, c, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    covered!(ramo);
    if let Some(q) = prize {
        let f = exe::league_calc_prize_fund(this, q);
        this.set_prize_fund(f);
        this.set_prize_mode(1);
    }
    if let Some(v) = tv {
        exe::league_pay_tv_rights(this, v, 0usize);
    }
    1
}

/// Comparer for the division tables: the division's clubs planned beforehand (database order, up to `n`); only the
/// original runs (it allocates and pays); then the count, the return (1 when the plan filled up), the rows and, when full
/// and with a prize fund, the fund and the mode.
pub(crate) unsafe fn compare_british_division_table(name: &str, this: League, n: i16, g: usize, prize: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let c = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(g) {
            plan.push(c);
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let expected = (plan.len() as i16 == n) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
        if let (1, Some(q)) = (r, prize) {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, q);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// The rules of a Scottish division (the Premier has its own): 4 rounds (`league::LEGS`), 3 points for a win and 1 for a
/// draw, category 2, club sides, tie-breaks 1, 2 and 3 (goal difference, goals scored, position [inference from the
/// English codes]), `promoted` promoted directly; in 1999 `playoff_1999` to the promotion play-off and nobody relegated,
/// from 2000 `relegated` relegated directly; the divisions above and below (`upper` and `lower`: the globals of the ids,
/// 0 = none, −1), 3 substitutions and 5 substitutes named.
pub(crate) unsafe fn scottish_division_rules(this: League, win: u8, promoted: u8, playoff_1999: u8, relegated: u8, upper: usize, lower: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 4), (league::WIN_POINTS, 1, win as u32), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
                     (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3), (league::PROMOTED_COUNT, 1, promoted as u32)];
    if this.year() == 0x7cf {
        v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, playoff_1999 as u32), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0)]);
    } else {
        v.extend_from_slice(&[(league::RELEGATED_COUNT, 1, relegated as u32), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0)]);
    }
    let id = |g: usize| if g == 0 { 0xffff_ffff } else { read_usize(g) as u32 };
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, id(upper)), (league::LOWER_DIVISION_ID, 4, id(lower)), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// The Scottish First Division: discipline rule 0x18, no stadium requirement, the promotion play-off in 1999.
pub(crate) const SCOTTISH_FIRST_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f4b4,
    stadium: None,
    discipline: 0x18, // mwb5: the Scottish discipline rule
    playoff_in_1999: true,
    split_compare: None,
    rules: 0x7eee50,
    table: 0x7eed30,
    file: 0xa7ade4,
    rows: [0x4c, 0x53, 0x5a, 0x63, 0x6a],
};

/// The Scottish First Division at career creation: discipline rule 0x18, the 1999 promotion play-off, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`).
/// Original: `ScoFirst::ScoFirst` (0x7edfc0, sco_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_first(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_FIRST_DIVISION, "mwb_5 new scottish first")
}

/// Comparer for `construct_sco_first` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_british_division_ctor("construct_sco_first", League(this), year, record, &SCOTTISH_FIRST_DIVISION, || o(this, year, record))
}

/// The First Division table: 10 clubs, those whose division is the Scottish First, in database order; the prize fund of
/// 25,000 per place (League::calc_prize_fund) and no TV money; out of memory, the box of line 0x183
/// Original: `ScoFirst::create_table` (0x7eed30, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_first_create_table(this: usize) -> u32 {
    fill_british_division_table(League(this), 0xa, 0xa7ade4, 0x183, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, Some(0x61a8), None, 0, "mwb_5 scottish first table") // mwb5: the First Division rows
}

/// Comparer for `sco_first_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_british_division_table("sco_first_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, Some(0x61a8), || o(this))
}

/// The First Division rule writes (`scottish_division_rules`): the champion goes up to the Premier; in 1999 2 to the
/// promotion play-off and nobody down, from 2000 2 down to the Second.
pub(crate) unsafe fn scottish_first_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 1, 2, 2, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION) // mwb5: the First Division rules
}

/// rule: the Scottish First Division (10 clubs) plays 4 rounds, 3 points for a win and 1 for a draw; the champion goes
/// up to the Premier and the bottom 2 go down to the Second (in 1999: 2 to the promotion play-off and nobody down);
/// tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; 5 substitutes named and 3
/// allowed; builds the round schedule (virtual +0x3c into `league::ROUND_DATES`); returns 1
/// Original: `ScoFirst::set_up_rules` (0x7eee50, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_first_set_up_rules(this: usize) -> u32 {
    covered!("mwb_5 scottish first rules");
    eng_prm_apply_league_rules(League(this), &scottish_first_rules(League(this)))
}

/// Comparer for `sco_first_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("sco_first_set_up_rules", League(this), scottish_first_rules(League(this)), || o(this), || sco_first_set_up_rules(this))
}
