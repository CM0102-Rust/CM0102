// --- sx01d_1: Media::apply_team_story_response (0x73e870, media.cpp, 10,160 bytes), the whole routine in named pieces ---

/// The constants of `apply_team_story_response` that have no name in `rdata_consts.rs` yet (cm0102.exe .rdata); the
/// f32 30.0 is `STORY_F32_30` (sx01c_1).
const TEAM_STORY_F32_67: usize = 0x96d8a8; // f32 = 67.0
const TEAM_STORY_F32_NEG_30: usize = 0x96d8a4; // f32 = -30.0
const TEAM_STORY_F32_NEG_1500: usize = 0x96d8a0; // f32 = -1500.0
const TEAM_STORY_F64_NEG_0_25: usize = 0x96d218; // f64 = -0.25
const TEAM_STORY_F64_NEG_250: usize = 0x96d890; // f64 = -250.0
const TEAM_STORY_F64_NEG_500: usize = 0x96d898; // f64 = -500.0
const TEAM_STORY_F64_250: usize = 0x968898; // f64 = 250.0

/// What `apply_team_story_response` keeps in its frame.
struct TeamStory {
    this: usize,
    club: Club,
    /// the person the story is about (may be null), passed on to the confidence adjustments
    subject: usize,
    /// the story kind as passed (its word picks the branch; some branches pass the whole dword on)
    kind: u32,
    /// the manager's answer as passed (its low byte, 1..3, picks the branch; the whole dword is passed on)
    response: u32,
    /// the club's division stats object (0 for a national team)
    stats: usize,
    /// the chairman's weight: (ambition - 10) x 0.05 + 1, 0.25 without a chairman
    weight: f64,
    /// the club's stat 3 in the division stats (0 for a national team), the weight of the story effects
    stat: f32,
    /// trunc(stat x 67): a dword whose low word (`x`) is the story's base
    base: i32,
    /// the kind's multiplier from the board's expectation flags (1.0 by default)
    mult: f64,
}

/// `v`, or `cap` when `cap < v` (the x87 `fcomp`/`fstp` pairs of the original).
fn team_story_at_most(v: f64, cap: f64) -> f64 {
    if cap < v { cap } else { v }
}

impl TeamStory {
    /// The base as the original reads it, `movsx` of the low word.
    fn x(&self) -> i32 {
        self.base as i16 as i32
    }

    /// The low word of `high` replaced by `t` (the original writes a word into a dword slot and passes the dword on).
    fn word_into(high: u32, t: i32) -> u32 {
        (high & 0xffff_0000) | (t as u32 & 0xffff)
    }

    /// The club's `board_record::EXPECTATION_FLAGS` word.
    unsafe fn expectations(&self) -> u16 {
        read_u16(manager_manager_row(self.club) + board_record::EXPECTATION_FLAGS)
    }

    /// Clears `clear` and sets `set` in byte `k` (0 or 1) of the club's expectation flags.
    unsafe fn swap_expectations(&self, k: usize, clear: u8, set: u8) {
        let p = manager_manager_row(self.club) + board_record::EXPECTATION_FLAGS + k;
        write_u8(p, read_u8(p) & !clear | set);
    }

    /// The division's competition object in `G_COMP_OBJECT_TABLE` (read without a null check, as the original).
    unsafe fn competition_object(&self) -> usize {
        read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(self.club.division().addr())))
    }

    /// The club's entry in the division stats exists and its byte +4 is below 25 [inference: the league position].
    unsafe fn table_entry_ok(&self) -> bool {
        exe::comp_stats_find_club_entry(self.stats, self.club) != 0
            && read_u8(exe::comp_stats_find_club_entry(self.stats, self.club) + 4) < 0x19
    }

    /// `Media::adjust_supporter_confidence(club, subject, kind, target, response)`.
    unsafe fn supporters(&self, kind: u32, target: u32) {
        player_story_supporters(self.this, self.club, Staff(self.subject), kind, target as i32, self.response)
    }

    /// `Media::adjust_board_confidence(club, subject, kind, target, response, sixth)`.
    unsafe fn board(&self, kind: u32, target: u32, sixth: u32) {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(player_story_fns::ADJUST_BOARD_CONFIDENCE);
        f(self.this, self.club.addr(), self.subject, kind, target, self.response, sixth)
    }

    /// `Media::apply_story_effect_flags(club, kind, flags, stat, response, change, player)`.
    unsafe fn effects(&self, kind: u32, flags: u32, change: u32, player: usize) {
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, usize) = core::mem::transmute(player_story_fns::APPLY_STORY_EFFECT_FLAGS);
        f(self.this, self.club.addr(), kind, flags, self.stat.to_bits(), self.response, change, player)
    }

    /// Adds trunc(v) to the club's `board_record::STAFF_CONFIDENCE`, kept between 1 and 10000.
    unsafe fn staff_confidence_add(&self, v: f64) {
        add_clamped_confidence(manager_manager_row(self.club) + board_record::STAFF_CONFIDENCE, ftol(v) as i16);
    }
}

/// `Media::apply_team_story_response(club, kind, response, subject)`: the effects of the manager's answer (1, 2 or 3,
/// the low byte of `response`) to a media story about his club, through the supporters' confidence
/// (`Media::adjust_supporter_confidence`), the board's (`Media::adjust_board_confidence`), the staff's
/// (`board_record::STAFF_CONFIDENCE`) and the story effects on the squad (`Media::apply_story_effect_flags`, weighted by
/// the club's stat).
///
/// Nothing without the club, its manager (+0xcf) with a `.non_player()` sheet or, for a club that is not a national
/// team (`exe::database_is_national_team`), its division's object in `G_COMP_OBJECT_TABLE` with the division stats
/// (+8). The chairman's weight is (`.ambition()` - 10) x 0.05 + 1 (0.25 without a chairman); the stat is the club's
/// stat 3 in the division stats (`exe::comp_stats_get_club_stat`, as a float, 0 for a national team) and the base
/// x = trunc(stat x 67) (a word). The kind picks the piece: 0x3c/0x3d `team_story_crowd`, 0x63/0x5b/0x61, 0x62/0x5d,
/// 0x5f and 0x5c `team_story_standard`, 0x3f and 0x60 `team_story_relegation`, 0x40/0x41 `team_story_minus_thousand`
/// (with `team_story_promotion_reply`), 0x43 `team_story_injuries`, 0x42 `team_story_fixed`, 0x52
/// `team_story_subject`, 0x44 and 0x5a `team_story_supporters_only`; any other kind or an answer outside 1..3 does
/// nothing.
///
/// Defects of the original: answer 2 to kind 0x43 with the board expecting promotion or a title (flags & 6) passes the
/// story effects the change computed with the formula of flags & 9 (-500 - stat x 30 instead of -1500 - stat x 40);
/// answer 1 to kind 0x60 passes the story effects trunc(-x / multiplier) while the confidences get +x / multiplier;
/// several targets are a dword whose low word is the computed value and whose high word is the kind's or the base's
/// (the adjustments only use the low word).
/// Original: `Media::apply_team_story_response` (0x73e870, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn apply_team_story_response(this: usize, club: usize, kind: u32, response: u32, subject: usize) {
    let club = Club(club);
    if club.is_null() {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().is_null() {
        return;
    }
    let comp_object = || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(club.division().addr())));
    if exe::database_is_national_team(club) == 0 && (club.division().is_null() || comp_object() == 0) {
        return;
    }
    let mut stats = 0usize;
    if exe::database_is_national_team(club) == 0 {
        stats = read_usize(comp_object() + 8);
        if stats == 0 {
            return;
        }
    }
    let chairman = club.chairman();
    let weight = if chairman.is_null() {
        0.25
    } else {
        (chairman.ambition() as i32 - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1)
    };
    let stat = if stats != 0 { exe::comp_stats_get_club_stat(stats, club, 3) as f32 } else { 0.0 };
    let base = ftol(stat as f64 * read_f32(TEAM_STORY_F32_67) as f64);
    let mut s = TeamStory { this, club, subject, kind, response, stats, weight, stat, base, mult: 1.0 };
    let flags = s.expectations();
    match kind as u16 {
        0x3c | 0x3d => team_story_crowd(&s),
        0x63 | 0x5b | 0x61 => {
            s.mult = if flags & 2 != 0 { 1.5 } else if flags & 1 != 0 { 1.1 } else { 1.0 };
            covered!("sx01d_1 team story bad run");
            team_story_standard(&s, kind, 100, read_f64(rdata::F64_500), false, 0x10, |s| team_story_raise(s, 0x71, 2));
        }
        0x62 | 0x5d => {
            s.mult = if flags & 2 != 0 { 1.75 } else if flags & 1 != 0 { 1.3 } else { 1.0 };
            covered!("sx01d_1 team story good run");
            team_story_standard(&s, kind, 0x14, read_f64(rdata::F64_200), true, 0x50, |s| team_story_raise(s, 0x71, 2));
        }
        0x5f => {
            s.mult = if flags & 4 != 0 { 1.5 } else if flags & 8 != 0 { 1.1 } else { 1.0 };
            covered!("sx01d_1 team story 0x5f");
            team_story_standard(&s, 0x5f, 100, read_f64(rdata::F64_500), true, 0x50, |s| team_story_raise(s, 0x78, 4));
        }
        0x5c => {
            s.mult = if flags & 0x400 != 0 { 1.5 } else { 1.0 };
            covered!("sx01d_1 team story continental");
            team_story_standard(&s, 0x5c, 100, read_f64(rdata::F64_500), true, 0x50, |s| {
                s.swap_expectations(1, 0, 4); // board_record::EXPECTATION_FLAGS | 0x400, no table test
                true
            });
        }
        0x3f | 0x60 => {
            s.mult = team_story_relegation_multiplier(flags, 0.5, 0.75, 1.5, 3.0);
            team_story_relegation(&s, kind as u16 as u32);
        }
        0x43 => team_story_injuries(&s, flags),
        0x40 | 0x41 => {
            s.mult = team_story_relegation_multiplier(flags, 0.1, 0.2, 1.75, 3.0);
            team_story_minus_thousand(&s);
        }
        0x42 => {
            s.mult = if flags & 6 != 0 { 1.5 } else { 1.0 };
            team_story_fixed(&s);
        }
        0x52 => team_story_subject(&s),
        0x44 | 0x5a => team_story_supporters_only(&s, kind as u16 as u32),
        _ => {}
    }
}

/// The multiplier of kinds 0x3f, 0x60, 0x40 and 0x41 by the expectation flags: & 0x20, & 0x10, & 9, & 6 (1.0 without).
fn team_story_relegation_multiplier(flags: u16, a: f64, b: f64, c: f64, d: f64) -> f64 {
    if flags & 0x20 != 0 {
        a
    } else if flags & 0x10 != 0 {
        b
    } else if flags & 9 != 0 {
        c
    } else if flags & 6 != 0 {
        d
    } else {
        1.0
    }
}

/// Kinds 0x63/0x5b/0x61, 0x62/0x5d, 0x5f and 0x5c, by answer:
/// - 3: when the stat - 5 is below draw(-100), the supporters get trunc(x × multiplier) and the board that × the
///   chairman's weight; story effects 4;
/// - 2: the supporters get trunc(-x × multiplier) and the board trunc(-x × multiplier × weight), both at most 0; story
///   effects 8 with trunc(x × -0.25), at most 0;
/// - 1: the supporters get trunc(x × multiplier); when draw(`draw`) is below the stat and `raise` raises the
///   expectations (`team_story_raise`; 0x5c sets 0x400 without the table test), the board gets trunc(x × multiplier × weight + `bonus`) with the sixth argument
///   1, otherwise without the bonus and 0; then with `staff` the staff's confidence moves by trunc(x × 0.25) and the
///   story effects get `flags` with 0, otherwise `flags` with trunc(x × 0.25).
unsafe fn team_story_standard(s: &TeamStory, kind: u32, draw: i32, bonus: f64, staff: bool, flags: u32, raise: impl FnOnce(&TeamStory) -> bool) {
    let x = s.x() as f64;
    match s.response as i8 {
        3 => {
            let f = (s.stat as f64 - read_f32(rdata::F32_5) as f64) as f32;
            if exe::utils_random(-100) as f64 > f as f64 {
                covered!("sx01d_1 team story standard 3");
                let p = x * s.mult;
                s.supporters(kind, ftol(p) as u32);
                s.board(kind, ftol(p * s.weight) as u32, 0);
            }
            s.effects(kind, 4, 0, 0);
        }
        2 => {
            covered!("sx01d_1 team story standard 2");
            let p = x * s.mult;
            s.supporters(kind, ftol(team_story_at_most(-p, read_f64(rdata::F64_0))) as u32);
            s.board(kind, ftol(team_story_at_most(-(p * s.weight), read_f64(rdata::F64_0))) as u32, 0);
            s.effects(kind, 8, ftol(team_story_at_most(x * read_f64(TEAM_STORY_F64_NEG_0_25), read_f64(rdata::F64_0))) as u32, 0);
        }
        1 => {
            let p = x * s.mult;
            s.supporters(kind, ftol(p) as u32);
            if (exe::utils_random(draw) as f64) < s.stat as f64 && raise(s) {
                covered!("sx01d_1 team story standard 1 raised");
                s.board(kind, ftol(p * s.weight + bonus) as u32, 1);
            } else {
                s.board(kind, ftol(p * s.weight) as u32, 0);
            }
            if staff {
                s.staff_confidence_add(x * read_f64(rdata::F64_0_25));
                s.effects(kind, flags, 0, 0);
            } else {
                s.effects(kind, flags, ftol(x * read_f64(rdata::F64_0_25)) as u32, 0);
            }
        }
        _ => {}
    }
}

/// The raised expectations of answer 1 to kinds 0x63/0x5b/0x61, 0x62/0x5d and 0x5f: with `table_entry_ok`, the
/// expectation flags lose `clear` and take `set`.
unsafe fn team_story_raise(s: &TeamStory, clear: u8, set: u8) -> bool {
    if !s.table_entry_ok() {
        return false;
    }
    s.swap_expectations(0, clear, set);
    true
}

/// Answers 3 and 2 of kinds 0x3f, 0x60, 0x40 and 0x41, with e = -1000 - x:
/// - 3 (`won`): the supporters get trunc(e × multiplier) and the board that × the weight, both at most 0; story effects
///   8 with e (at most 0); without `won` story effects 4;
/// - 2: the same confidences; story effects 8 with trunc(e × 0.5), at most 0.
unsafe fn team_story_minus_thousand_low(s: &TeamStory, kind: u32, won: bool) {
    let e = -1000 - s.x();
    if s.response as i8 == 3 && !won {
        s.effects(kind, 4, 0, 0);
        return;
    }
    let p = e as f64 * s.mult;
    s.supporters(kind, ftol(team_story_at_most(p, read_f64(rdata::F64_0))) as u32);
    s.board(kind, ftol(team_story_at_most(p * s.weight, read_f64(rdata::F64_0))) as u32, 0);
    if s.response as i8 == 3 {
        covered!("sx01d_1 team story minus thousand 3");
        s.effects(kind, 8, e.min(0) as u32, 0);
    } else {
        covered!("sx01d_1 team story minus thousand 2");
        s.effects(kind, 8, ftol(team_story_at_most(e as f64 * read_f64(rdata::F64_0_5), read_f64(rdata::F64_0))) as u32, 0);
    }
}

/// The board's part of answer 1 to kinds 0x3f, 0x60, 0x40 and 0x41 with the capped value c (at most 500): with
/// `raised` trunc(c × weight + 250) and the sixth argument 1, otherwise trunc(c × weight) and 0.
unsafe fn team_story_promotion_board(s: &TeamStory, kind: u32, c: f64, raised: bool) {
    if raised {
        covered!("sx01d_1 team story expectations raised");
        s.board(kind, ftol(c * s.weight + read_f64(TEAM_STORY_F64_250)) as u32, 1);
    } else {
        s.board(kind, ftol(c * s.weight) as u32, 0);
    }
}

/// Kinds 0x3f and 0x60: answers 3 and 2 by `team_story_minus_thousand_low` (answer 3 when draw(100) is below the
/// stat + 5). Answer 1: q = -x / multiplier for 0x3f, +x / multiplier for 0x60, at most 500 (c); the supporters get
/// trunc(c); when draw(100) is below the stat + 5, `table_entry_ok` and the expectation flags have 0x20, the flags
/// trade 0x20 for 0x10 and the board is raised (`team_story_promotion_board`); story effects 0x50 with trunc(c) for
/// 0x3f and trunc(-x / multiplier), at most 500, for 0x60.
unsafe fn team_story_relegation(s: &TeamStory, kind: u32) {
    let up = |s: &TeamStory| (exe::utils_random(100) as f64) < ((s.stat as f64 + read_f32(rdata::F32_5) as f64) as f32) as f64;
    match s.response as i8 {
        3 => {
            let won = up(s);
            team_story_minus_thousand_low(s, kind, won);
        }
        2 => team_story_minus_thousand_low(s, kind, false),
        1 => {
            covered!("sx01d_1 team story relegation 1");
            let x = s.x() as f64;
            let q = if kind == 0x3f { -x / s.mult } else { x / s.mult };
            let c = team_story_at_most(q, read_f64(rdata::F64_500));
            s.supporters(kind, ftol(c) as u32);
            let raised = up(s) && s.table_entry_ok() && s.expectations() & 0x20 != 0;
            if raised {
                s.swap_expectations(0, 0x20, 0x10);
            }
            team_story_promotion_board(s, kind, c, raised);
            let change = if kind == 0x3f { c } else { team_story_at_most(-x / s.mult, read_f64(rdata::F64_500)) };
            s.effects(kind, 0x50, ftol(change) as u32, 0);
        }
        _ => {}
    }
}

/// Kinds 0x40 and 0x41 (the whole kind dword is passed on): answers 3 and 2 by `team_story_minus_thousand_low`
/// (answer 3 when the stat - 5 is below draw(-100)). Answer 1: c = -x / multiplier, at most 500; the supporters get
/// trunc(c); with `table_entry_ok` the expectation flags may be raised (`team_story_promotion_reply`) and the board
/// follows (`team_story_promotion_board`); story effects 0x50 with trunc(c).
unsafe fn team_story_minus_thousand(s: &TeamStory) {
    match s.response as i8 {
        3 => {
            let f = (s.stat as f64 - read_f32(rdata::F32_5) as f64) as f32;
            let won = exe::utils_random(-100) as f64 > f as f64;
            team_story_minus_thousand_low(s, s.kind, won);
        }
        2 => team_story_minus_thousand_low(s, s.kind, false),
        1 => {
            covered!("sx01d_1 team story minus thousand 1");
            let c = team_story_at_most(-(s.x() as f64) / s.mult, read_f64(rdata::F64_500));
            s.supporters(s.kind, ftol(c) as u32);
            let raised = s.table_entry_ok() && team_story_promotion_reply(s);
            team_story_promotion_board(s, s.kind, c, raised);
            s.effects(s.kind, 0x50, ftol(c) as u32, 0);
        }
        _ => {}
    }
}

/// The raised expectations of answer 1 to kinds 0x40 and 0x41: up to four draws(100), each below the stat + 5, tried
/// in turn: flags 8 become 4; 1 becomes 2; 0x40 becomes 8 when the division's object has +0x1c not -1 and the byte
/// +0xbf set; 0x40 becomes 1 when +0x1c is -1. The first that applies raises the board.
unsafe fn team_story_promotion_reply(s: &TeamStory) -> bool {
    let f = ((s.stat as f64 + read_f32(rdata::F32_5) as f64) as f32) as f64;
    if (exe::utils_random(100) as f64) < f && s.expectations() & 8 != 0 {
        s.swap_expectations(0, 8, 4);
        return true;
    }
    if (exe::utils_random(100) as f64) < f && s.expectations() & 1 != 0 {
        s.swap_expectations(0, 1, 2);
        return true;
    }
    if (exe::utils_random(100) as f64) < f && s.expectations() & 0x40 != 0 {
        let o = s.competition_object();
        if read_i32(o + 0x1c) != -1 && read_u8(o + 0xbf) != 0 {
            // +0x1c and +0xbf of the division's competition object, unnamed
            s.swap_expectations(0, 0x40, 8);
            return true;
        }
    }
    if (exe::utils_random(100) as f64) < f && s.expectations() & 0x40 != 0 && read_i32(s.competition_object() + 0x1c) == -1 {
        s.swap_expectations(0, 0x40, 1);
        return true;
    }
    false
}

/// Kind 0x43 (the multiplier and the weight are not used). Answer 3: story effects 4. Answer 2, by the expectation
/// flags: & 0x20 the supporters and the board get 0 and story effects 4; & 0x10 v = stat × -30 (a float, at most 0) to
/// both and story effects 4 with v; & 9 v = -500 - stat × 30 and story effects 8 with v; & 6 v = -1500 - stat × 40 and
/// story effects 8 with -500 - stat × 30 (the original's slip); otherwise story effects 8. Answer 1: & 0x20 or & 0x10
/// the supporters get trunc(v + 200) with v = stat × 20 or × 30, and when draw(100) is below the stat + 5 and
/// `table_entry_ok`, the flags trade 0x20 for 0x10 (0x10 for 0x40) and the board gets trunc(v + 400), otherwise the
/// supporters' value (in a dword with the kind's or the base's high word); story effects 0x50 with that dword, or with
/// trunc(stat × -30) (at most 0) for 0x10; & 9 and & 6 t = trunc(stat × 20) (- 150 for 6) to both and to the story
/// effects 0x50; otherwise story effects 0x50.
unsafe fn team_story_injuries(s: &TeamStory, flags: u16) {
    let stat = s.stat as f64;
    let zero = read_f32(rdata::F32_0) as f64;
    match s.response as i8 {
        3 => s.effects(0x43, 4, 0, 0),
        2 => {
            covered!("sx01d_1 team story injuries 2");
            let both = |v: f32| {
                let c = ftol(team_story_at_most(v as f64, zero)) as u32;
                s.supporters(0x43, c);
                s.board(0x43, c, 0);
                c
            };
            let alt = || ftol(team_story_at_most(read_f32(rdata::F32_NEG_500) as f64 - stat * read_f32(STORY_F32_30) as f64, zero)) as u32;
            if flags & 0x20 != 0 {
                s.supporters(0x43, 0);
                s.board(0x43, 0, 0);
                s.effects(0x43, 4, 0, 0);
            } else if flags & 0x10 != 0 {
                let c = both((stat * read_f32(TEAM_STORY_F32_NEG_30) as f64) as f32);
                s.effects(0x43, 4, c, 0);
            } else if flags & 9 != 0 {
                let c = both((read_f32(rdata::F32_NEG_500) as f64 - stat * read_f32(STORY_F32_30) as f64) as f32);
                s.effects(0x43, 8, c, 0);
            } else if flags & 6 != 0 {
                both((read_f32(TEAM_STORY_F32_NEG_1500) as f64 - stat * read_f32(rdata::F32_40) as f64) as f32);
                s.effects(0x43, 8, alt(), 0);
            } else {
                s.effects(0x43, 8, 0, 0);
            }
        }
        1 => {
            covered!("sx01d_1 team story injuries 1");
            if flags & 0x30 != 0 {
                let (scale, clear, set) = if flags & 0x20 != 0 { (read_f32(rdata::F32_20), 0x20, 0x10) } else { (read_f32(STORY_F32_30), 0x10, 0x40) };
                let v = (stat * scale as f64) as f32;
                let t = ftol(stat * scale as f64 + read_f32(rdata::F32_200) as f64);
                s.supporters(0x43, t as u32);
                let high = if flags & 0x20 != 0 { s.kind } else { s.base as u32 };
                let kept = TeamStory::word_into(high, t);
                let up = (exe::utils_random(100) as f64) < ((stat + read_f32(rdata::F32_5) as f64) as f32) as f64;
                if up && s.table_entry_ok() {
                    s.swap_expectations(0, clear, set);
                    s.board(0x43, ftol(v as f64 + read_f32(rdata::F32_400) as f64) as u32, 0);
                } else {
                    s.board(0x43, kept, 0);
                }
                let change = if flags & 0x20 != 0 { kept } else { ftol(team_story_at_most(stat * read_f32(TEAM_STORY_F32_NEG_30) as f64, zero)) as u32 };
                s.effects(0x43, 0x50, change, 0);
            } else if flags & 0xf != 0 {
                let mut v = stat * read_f32(rdata::F32_20) as f64;
                if flags & 9 == 0 {
                    v -= read_f32(rdata::F32_150) as f64;
                }
                let t = TeamStory::word_into(s.kind, ftol(v));
                s.supporters(0x43, t);
                s.board(0x43, t, 0);
                s.effects(0x43, 0x50, t, 0);
            } else {
                s.effects(0x43, 0x50, 0, 0);
            }
        }
        _ => {}
    }
}

/// Kind 0x42. Answer 3: story effects 4. Answer 2: the supporters get trunc(multiplier × -500), the board
/// trunc(multiplier × weight × -500), story effects 8 with trunc(multiplier × -250). Answer 1: q = 500 / multiplier;
/// the supporters get trunc(q) (in a dword with the kind's high word), the board trunc(q × weight), story effects 0x50
/// with the supporters' dword.
unsafe fn team_story_fixed(s: &TeamStory) {
    match s.response as i8 {
        3 => s.effects(0x42, 4, 0, 0),
        2 => {
            covered!("sx01d_1 team story 0x42 2");
            s.supporters(0x42, ftol(s.mult * read_f64(TEAM_STORY_F64_NEG_500)) as u32);
            s.board(0x42, ftol(s.mult * s.weight * read_f64(TEAM_STORY_F64_NEG_500)) as u32, 0);
            s.effects(0x42, 8, ftol(s.mult * read_f64(TEAM_STORY_F64_NEG_250)) as u32, 0);
        }
        1 => {
            covered!("sx01d_1 team story 0x42 1");
            let q = read_f64(rdata::F64_500) / s.mult;
            let t = TeamStory::word_into(s.kind, ftol(q));
            s.supporters(0x42, t);
            s.board(0x42, ftol(q * s.weight) as u32, 0);
            s.effects(0x42, 0x50, t, 0);
        }
        _ => {}
    }
}

/// Kind 0x52, about a person (nothing without the subject). The supporters move by 200 or 1000 when the club favours
/// (`exe::club_favours_staff`) or dislikes (`exe::club_dislikes_staff`) the subject, the board by the weight × 200 or
/// × 1000 when the chairman favours (`exe::staff_is_favoured_by`) or dislikes him; the sign follows the answer: 3
/// (favoured -200 / disliked +200, story effects 4), 2 (-1000 / +1000, story effects 0x80), 1 (+1000 / -1000, story
/// effects 0x300); the story effects get the subject.
unsafe fn team_story_subject(s: &TeamStory) {
    if s.subject == 0 {
        return;
    }
    let (fans, board, flags) = match s.response as i8 {
        3 => (-200, (rdata::F64_NEG_200, rdata::F64_200), 4),
        2 => (-1000, (rdata::F64_NEG_1000, rdata::F64_1000), 0x80),
        1 => (1000, (rdata::F64_1000, rdata::F64_NEG_1000), 0x300),
        _ => return,
    };
    covered!("sx01d_1 team story subject");
    let subject = Staff(s.subject);
    if exe::club_favours_staff(subject, s.club) != 0 {
        s.supporters(0x52, fans as u32);
    } else if exe::club_dislikes_staff(subject, s.club) != 0 {
        s.supporters(0x52, (-fans) as u32);
    }
    let chairman = s.club.chairman();
    if exe::staff_is_favoured_by(subject, chairman) != 0 {
        s.board(0x52, ftol(s.weight * read_f64(board.0)) as u32, 0);
    } else if exe::staff_is_disliked_by(subject, chairman) != 0 {
        s.board(0x52, ftol(s.weight * read_f64(board.1)) as u32, 0);
    }
    s.effects(0x52, flags, 0, s.subject);
}

/// Kinds 0x44 and 0x5a: answer 1 moves the supporters by draw(200), answer 2 by draw(-200); nothing else.
unsafe fn team_story_supporters_only(s: &TeamStory, kind: u32) {
    match s.response as i8 {
        1 => {
            covered!("sx01d_1 team story national 1");
            s.supporters(kind, exe::utils_random(200) as u32);
        }
        2 => s.supporters(kind, exe::utils_random(-200) as u32),
        _ => {}
    }
}

/// Kinds 0x3c and 0x3d (the whole kind dword is passed on). Answer 3: with a negative stat and expectation flags in
/// 0x48f, the supporters get trunc(x × 0.65) and the board that × the weight; otherwise the supporters get
/// trunc((draw(20) - draw(20)) × stat); story effects 0x44. Answer 2: p = x × (0.5 with flags in 0x48f, otherwise the
/// multiplier); the supporters get trunc(-p), the board draw(trunc(-p × weight)); story effects 2 with the base + 100.
/// Answer 1: p = x × (1.5 with flags in 0x48f, 0.65 with 0x30, otherwise the multiplier); the supporters get trunc(p),
/// the board trunc(p × weight); story effects 0x41 with trunc(x × -1.5).
unsafe fn team_story_crowd(s: &TeamStory) {
    let flags = s.expectations();
    let x = s.x() as f64;
    match s.response as i8 {
        3 => {
            covered!("sx01d_1 team story crowd 3");
            if (s.stat as f64) < read_f32(rdata::F32_0) as f64 && flags & 0x48f != 0 {
                let p = x * read_f64(rdata::F64_0_65);
                s.supporters(s.kind, ftol(p) as u32);
                s.board(s.kind, ftol(p * s.weight) as u32, 0);
            } else {
                let d = exe::utils_random(0x14).wrapping_sub(exe::utils_random(0x14));
                s.supporters(s.kind, ftol(d as f64 * s.stat as f64) as u32);
            }
            s.effects(s.kind, 0x44, 0, 0);
        }
        2 => {
            covered!("sx01d_1 team story crowd 2");
            let m = if flags & 0x48f != 0 { read_f64(rdata::F64_0_5) } else { s.mult };
            let p = x * m;
            s.supporters(s.kind, ftol(-p) as u32);
            s.board(s.kind, exe::utils_random(ftol(-(p * s.weight))) as u32, 0);
            s.effects(s.kind, 2, s.base.wrapping_add(100) as u32, 0);
        }
        1 => {
            covered!("sx01d_1 team story crowd 1");
            let m = if flags & 0x48f != 0 {
                read_f64(rdata::F64_1_5)
            } else if flags & 0x30 != 0 {
                read_f64(rdata::F64_0_65)
            } else {
                s.mult
            };
            let p = x * m;
            s.supporters(s.kind, ftol(p) as u32);
            s.board(s.kind, ftol(p * s.weight) as u32, 0);
            s.effects(s.kind, 0x41, ftol(x * read_f64(rdata::F64_NEG_1_5)) as u32, 0);
        }
        _ => {}
    }
}

/// Comparer for `apply_team_story_response`: the routine draws, posts news (through the confidence adjustments) and
/// passes the reaction to the squad, so it cannot run twice; only the original runs. The proof is the Rust careers and
/// the mutations (the routine runs on every answer to a team story).
pub(crate) unsafe extern "thiscall" fn compare_apply_team_story_response(this: usize, club: usize, kind: u32, response: u32, subject: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize)>(%%I0%%)(this, club, kind, response, subject)
}
