// ---------- mwb_18: the Welsh League Cup (wel_lge_cup.cpp): the group table, the rules and the next stage; the moulds of the two Welsh cups played in groups ----------

/// The entrant pool of the Welsh League Cup and the Welsh Premier Cup (pointer to an array of club pointers, taken
/// group by group: each club is zeroed when it goes into a group; the count, 18 or 12, is at +0xf2).
pub(crate) const WELSH_CUP_POOL: usize = 0xee;

/// The own table of a Welsh group cup (the league object's table holds one group): `league::CLUB_COUNT` = `n`, the
/// table (`exe::crt_malloc` of n × 0x3b; out of memory, the box of `row` and 0), row k gets pool entry k × `stride`
/// (`exe::league_init_table_row(this, k, club, 0, 0)`) and that pool entry is zeroed; then the group draw news
/// (League::send_group_draw_news 0x684230, thiscall, ret 0); returns 1 (`rotate` is 0 in the original: row k gets the
/// k-th club; it exists only for the mutations).
pub(crate) unsafe fn welsh_cup_group_table(this: League, n: i16, stride: usize, file: usize, row: u32, rotate: u8, ramo: &'static str) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(ramo);
    let mut k: u8 = 0;
    while (k as i8 as i16) < this.club_count() {
        let e = read_usize(this.addr() + WELSH_CUP_POOL) + k as i8 as usize * stride * 4;
        // simplification: the original pushes the row as a stack dword with stale high bytes; 0x687430 reads the byte
        exe::league_init_table_row(this, ((k as u32 + rotate as u32) % n as u32) as u8 as u32, read_usize(e), 0, 0);
        write_usize(e, 0);
        k = k.wrapping_add(1);
    }
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    news(this.addr());
    1
}

/// Comparer for the Welsh group tables: the pool entries planned beforehand; only the original runs (it allocates and
/// sends news); then the count, the return 1, the rows and the pool entries zeroed.
pub(crate) unsafe fn compare_welsh_cup_group_table(name: &str, this: League, n: i16, stride: usize, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + WELSH_CUP_POOL);
    let plan: Vec<usize> = (0..n.max(0) as usize).map(|k| read_usize(pool + k * stride * 4)).collect();
    let r = original();
    let t = this.table();
    let mut ok = r == 1 && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if ok {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        ok = found == plan && (0..plan.len()).all(|k| read_usize(pool + k * stride * 4) == 0);
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// The rules of the two Welsh group cups: category 4, 2 rounds, no promotion play-off or relegation, no division above
/// or below (−1), 3 points for a win and 1 for a draw, club sides, a prize fund of 100,000; `promoted` go through from
/// each group (`league::PROMOTED_COUNT`) and the tie-breaks (`tiebreaks`, the fourth 0).
pub(crate) fn welsh_cup_rules(promoted: u8, tiebreaks: [u8; 3]) -> Vec<(usize, u8, u32)> {
    vec![(league::MATCH_CATEGORY, 1, 4), (league::TIEBREAKS, 1, tiebreaks[0] as u32), (league::TIEBREAKS + 1, 1, tiebreaks[1] as u32), (league::TIEBREAKS + 2, 1, tiebreaks[2] as u32),
         (league::TIEBREAKS + 3, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0),
         (league::LEGS, 2, 2), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::WIN_POINTS, 1, 3),
         (league::DRAW_POINTS, 1, 1), (league::PROMOTED_COUNT, 1, promoted as u32), (league::CLUB_SIDES, 1, 1), (league::PRIZE_FUND, 4, 0x186a0)]
}

/// The next stage of a Welsh group cup: at the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1,
/// signed) the error box and nothing else; otherwise the current stage goes up by one and, on reaching `at`, the
/// knockout stage is built (`knockout`, thiscall, ret 0).
pub(crate) unsafe fn welsh_cup_advance_stage(this: League, file: usize, row: u32, at: i32, knockout: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let stage = stage + 1;
    this.set_current_stage(stage as usize);
    if stage == at {
        covered!("mwb_18 welsh cup knockout stage");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(knockout);
        f(this.addr());
    }
}

/// rule: the Welsh League Cup's own table holds one group of three: the first entrant of each pool row (the pool is 6
/// columns of 3; the column of group 0 is the first), taken out of the pool; then the group draw news; out of memory,
/// the box of line 0x182 and 0
/// Original: `WelLgeCup::create_table` (0x926700, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_table(this: usize) -> u32 {
    welsh_cup_group_table(League(this), 3, 6, 0xad56f4, 0x182, 0, "mwb_18 welsh league cup table") // mwb18: the League Cup first group
}

/// Comparer for `wel_lge_cup_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_welsh_cup_group_table("wel_lge_cup_create_table", League(this), 3, 6, || o(this))
}

/// The Welsh League Cup rule writes: the group winner goes through, tie-breaks 4, 1, 2 [inference: head-to-head, goal
/// difference, goals for].
pub(crate) fn welsh_league_cup_rules() -> Vec<(usize, u8, u32)> {
    welsh_cup_rules(1, [4, 1, 2]) // mwb18: the League Cup group rules
}

/// rule: the Welsh League Cup groups play 2 rounds, 3 points for a win and 1 for a draw, category 4; the winner of each
/// group goes through; tie-breaks codes 4, 1 and 2 [inference: head-to-head, goal difference, goals for]; a prize fund
/// of 100,000; builds the round schedule of the main stage (virtual +0x3c); returns 1
/// Original: `WelLgeCup::set_up_rules` (0x926800, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_set_up_rules(this: usize) -> u32 {
    covered!("mwb_18 welsh league cup rules");
    eng_prm_apply_league_rules(League(this), &welsh_league_cup_rules())
}

/// Comparer for `wel_lge_cup_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("wel_lge_cup_set_up_rules", League(this), welsh_league_cup_rules(), || o(this), || wel_lge_cup_set_up_rules(this))
}

/// rule: the Welsh League Cup moves from group to group (stages 0 to 4 are the other five groups) and, on reaching stage
/// 5, builds the knockout stage of the eight best (WelLgeCup::create_knockout_stage 0x926d00); past the last stage, the
/// box of line 0x222
/// Original: `WelLgeCup::advance_stage` (0x926a70, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_advance_stage(this: usize) {
    welsh_cup_advance_stage(League(this), 0xad56f4, 0x222, 5, 0x926d00)
}

/// Comparer for `wel_lge_cup_advance_stage`: only the original runs (the knockout builds a stage); the current stage
/// stays at the last stage, otherwise goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_advance_stage("wel_lge_cup_advance_stage", League(this), || o(this))
}
