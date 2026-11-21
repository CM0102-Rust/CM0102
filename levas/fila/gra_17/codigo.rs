// ---------- batch %%LEVA%%: the board's weekly confidence update (manager_manager.cpp) ----------

/// Offsets used by `weekly_board_confidence_update` that have no name elsewhere.
#[allow(dead_code)]
pub(crate) mod board_week_fields {
    /// Board row: the finance mood, moved with the confidence by the club's wealth band (i16) [inference].
    pub(crate) const FINANCE_MOOD: usize = 0x10;
    /// comp_stats_club: two bytes summed and compared with the games played × 1.6..2.1 by the "on fire" story [unknown].
    pub(crate) const STAT_69: usize = 0x69;
    pub(crate) const STAT_6A: usize = 0x6a;
    /// comp_stats_club: the last date of each run (8 bytes; run + 0xc).
    pub(crate) const WINNING_RUN_DATE: usize = 0x25;
    pub(crate) const LOSING_RUN_DATE: usize = 0x39;
    pub(crate) const UNBEATEN_RUN_DATE: usize = 0x4d;
    pub(crate) const WINLESS_RUN_DATE: usize = 0x61;
}

/// Exe addresses `weekly_board_confidence_update` uses besides its calls (cm0102.exe 3.9.68). The calls without an
/// `exe::` wrapper go by literal address: 0x72b810 Media::pick_former_international_pundit (thiscall(club, subject),
/// ret 8), 0x72b940 Media::pick_rival_manager_id (thiscall(club), ret 4), 0x48f340 Comp::get_first_schedule_date
/// (thiscall(out), ret 4), 0x8cc030 TRANSFER_MANAGER::get_past_transfers (thiscall, ret 0x18).
#[allow(dead_code)]
pub(crate) mod board_week_fns {
    pub(crate) const MEDIA_STORY_ENTRY: usize = 0x74b060; // Media::post_team_media_story (the comparer's recorder)
}

/// .rdata constants of `weekly_board_confidence_update` not in `rdata` (cm0102.exe 3.9.68).
#[allow(dead_code)]
pub(crate) mod board_week_rdata {
    pub(crate) const F64_9: usize = 0x96d230; // f64 = 9.0
    pub(crate) const F64_22: usize = 0x96af40; // f64 = 22.0
    pub(crate) const F64_2500: usize = 0x9690b0; // f64 = 2500.0
    pub(crate) const F64_3000: usize = 0x96d200; // f64 = 3000.0
    pub(crate) const F64_NEG_0_25: usize = 0x96d218; // f64 = -0.25
}

/// The league size N the board's pre-season stories read before it is written (see `weekly_board_confidence_update`):
/// the last one of the previous call, the copy's stand-in for the original's stack garbage.
static BOARD_WEEK_LEAGUE_SIZE: AtomicU8 = AtomicU8::new(0);

/// One club's pass of `weekly_board_confidence_update` (the original's locals).
struct BoardWeek {
    this: usize,
    index: usize,
    club: usize,
    manager: usize,
    sheet: usize,
    confidence_before: i16,
    league_size: i8,
    told_finals_hopes: bool,
    told_finals_doubts: bool,
    entry: usize,
    division_object: usize,
    proj_best: u8,
    proj_normal: u8,
    proj_worst: u8,
    x: f32,
    weight: f64,
}

impl BoardWeek {
    /// The club's board row: [this] + loop index × 0x49 (re-read every time, as the original).
    unsafe fn row(&self) -> usize {
        read_usize(self.this).wrapping_add(self.index.wrapping_mul(0x49))
    }
    unsafe fn add(&self, field: usize, v: i16) {
        let a = self.row() + field;
        write_i16(a, read_i16(a).wrapping_add(v));
    }
    unsafe fn confidence(&self) -> i16 {
        read_i16(self.row() + board_record::CONFIDENCE)
    }
    unsafe fn set_confidence(&self, v: i16) {
        write_i16(self.row() + board_record::CONFIDENCE, v);
    }
}

/// The competition object of a competition record: [[G_COMP_OBJECT_TABLE] + record id × 4].
unsafe fn board_week_comp_object(record: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(record) as usize).wrapping_mul(4)))
}

/// The non-player sheet of the club's chairman (0 without chairman or sheet).
unsafe fn board_week_chairman_sheet(club: usize) -> usize {
    let c = read_usize(club + club::CHAIRMAN);
    if c == 0 { 0 } else { read_usize(c + staff::NON_PLAYER) }
}

/// The chairman's patience factor: 1 → `one`, 20 → 0.25, otherwise (21 − patience) × 0.1 kept within 0.5..2.
unsafe fn board_week_patience_factor(patience: i8, one: usize) -> f64 {
    if patience == 1 {
        return read_f32(one) as f64;
    }
    if patience == 20 {
        return read_f32(rdata::F32_0_25) as f64;
    }
    let m = (21 - patience as i32) as f64 * read_f64(rdata::F64_0_1);
    if m > read_f64(rdata::F64_2) {
        read_f32(rdata::F32_2) as f64
    } else if m < read_f64(rdata::F64_0_5) {
        read_f32(rdata::F32_0_5) as f64
    } else {
        m
    }
}

/// Who comments a board story: on a draw(n) other than 0 a former international (`pick_former_international_pundit`),
/// otherwise a rival's manager (`pick_rival_manager_id`, none when below 0).
unsafe fn board_week_story_person(club: usize, n: i32) -> usize {
    if random(n) != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x72b810usize);
        f(read_usize(G_MEDIA), club, 0)
    } else {
        let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x72b940usize);
        let id = f(read_usize(G_MEDIA), club);
        if id > -1 { staff_at(id as isize).addr() } else { 0 }
    }
}

/// Posts a board story about the club (`Media::post_team_media_story(club, kind, person, 0, 0, extra)`); in the
/// comparer's replay it checks the call against the original's tape and applies the story's recorded effects instead.
unsafe fn board_week_post_story(club: usize, kind: u32, person: usize, extra: i32) {
    if board_week_replay_story(club, kind, person, extra) {
        return;
    }
    exe::media_post_team_media_story(read_usize(G_MEDIA), club, kind, person as u32, 0, 0, extra);
}

/// `MANAGER_MANAGER::weekly_board_confidence_update()`: every Sunday (daily_update 0x68f9c0), for each club with a
/// manager and his non-player sheet that can be managed (`exe::club_can_be_managed`; a national team only with a
/// nation on a continent), the board's, supporters' and other confidences in the manager, the board stories in the
/// media and the manager's reputations.
///
/// Per club (row = [this] + loop index × 0x49): `decay_board_week`; the national team's finals stories
/// (`national_finals_stories`); otherwise the league week (`league_board_week`, which may post one story and moves the
/// confidence by the form, the projected position against the target and the chairman); then
/// `board_week_floors_and_reputation` (the continental floor, the 0..10000 clamps, the chairman's and the club's likes
/// and dislikes, the manager's reputations and the wealth band). Defect of the original (`docs/DEFEITOS_DO_ORIGINAL.md`,
/// approved to fix only in the all-Rust exe): the pre-season stories read the league size N ([esp+0x2b]) before it is
/// written (0x6984ad, 0x6986ff, 0x6987d6): the first club reads stack garbage, the next ones the N of the previous
/// club's league. Ported faithfully: N survives from club to club; the first one gets the last N of the previous call
/// (`BOARD_WEEK_LEAGUE_SIZE`; the comparer passes the byte the original really reads).
/// Original: `MANAGER_MANAGER::weekly_board_confidence_update` (0x697c40, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn weekly_board_confidence_update(this: usize) {
    let n = board_week_run(this, BOARD_WEEK_LEAGUE_SIZE.load(Relaxed) as i8);
    BOARD_WEEK_LEAGUE_SIZE.store(n as u8, Relaxed);
}

/// The club loop of `weekly_board_confidence_update` from the given stale league size; returns the last N.
unsafe fn board_week_run(this: usize, league_size: i8) -> i8 {
    let mut s = BoardWeek {
        this, index: 0, club: 0, manager: 0, sheet: 0, confidence_before: 0, league_size,
        told_finals_hopes: false, told_finals_doubts: false, entry: 0, division_object: 0,
        proj_best: 0, proj_normal: 0, proj_worst: 0, x: 0.0, weight: 0.0,
    };
    let base = read_usize(G_CLUB_TABLE);
    while (s.index as i32) < read_i32(G_CLUB_COUNT) {
        s.club = base.wrapping_add(s.index.wrapping_mul(club::SIZE));
        board_week_club(&mut s);
        s.index += 1;
    }
    s.league_size
}

/// One club of the weekly update (see `weekly_board_confidence_update`).
unsafe fn board_week_club(s: &mut BoardWeek) {
    let club = s.club;
    if exe::database_is_national_team(club) != 0 {
        let nation = read_usize(club + club::NATION);
        if nation == 0 || read_u32(nation + nation::CONTINENT) == 0 {
            return;
        }
    }
    let manager = read_usize(club + club::MANAGER);
    if manager == 0 || read_usize(manager + staff::NON_PLAYER) == 0 || exe::club_can_be_managed(club) == 0 {
        return;
    }
    s.manager = manager;
    s.sheet = read_usize(manager + staff::NON_PLAYER);
    decay_board_week(s);
    if !national_finals_stories(s) {
        league_board_week(s);
    }
    board_week_floors_and_reputation(s);
}

/// The weekly pull of the supporters' confidence towards the board's: the supporters (`board_record::SUPPORTERS_CONFIDENCE`)
/// kept within half and double the board's (`board_record::CONFIDENCE`), then the board's moves by 10% of the gap
/// (confidence − truncate((supporters − confidence) × −0.1)); the change also goes to `board_record::FAVOURITES_MOOD`.
unsafe fn decay_board_week(s: &mut BoardWeek) {
    let row = s.row();
    let confidence = read_i16(row + board_record::CONFIDENCE) as i32;
    let supporters = read_i16(row + board_record::SUPPORTERS_CONFIDENCE) as i32;
    covered!("gra_17 decay");
    let half = confidence / 2;
    let v = if half > supporters { half } else if confidence * 2 < supporters { confidence * 2 } else { supporters };
    write_i16(row + board_record::SUPPORTERS_CONFIDENCE, v as i16);
    let row = s.row();
    let supporters = read_i16(row + board_record::SUPPORTERS_CONFIDENCE) as i32;
    s.confidence_before = read_i16(row + board_record::CONFIDENCE);
    let d = ftol((supporters - s.confidence_before as i32) as f64 * read_f64(rdata::F64_NEG_0_1));
    write_i16(row + board_record::CONFIDENCE, s.confidence_before.wrapping_sub(d as i16));
    s.add(board_record::FAVOURITES_MOOD, s.confidence().wrapping_sub(s.confidence_before));
}

/// Compares two dates by year (unsigned) then day (signed), as the original's inline tests: −1, 0 or 1.
unsafe fn board_week_date_order(a: usize, b: usize) -> i32 {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    if ya != yb {
        return if ya < yb { -1 } else { 1 };
    }
    let (da, db) = (read_i16(a), read_i16(b));
    if da < db { -1 } else if da > db { 1 } else { 0 }
}

/// The national team's finals stories: only the nation's main national team (`exe::database_get_national_team`), on a
/// draw(10) of 0, with its current competition (`exe::nationalteams_get_current_competition`) a finals one
/// (`exe::national_teams_is_finals_comp`) with an object, starting (`Comp::get_first_schedule_date`) more than 10 and
/// less than 100 days from today. Returns false otherwise (the club goes to the league week). With it, nothing more
/// without the competition's statistics, the team's entry or with a match played; N = the competition's entrant count
/// (low byte). Once per call each: with the best position 1 and the previous final position below N / 5 − draw(3) + 2,
/// story 0x5a (the hopes; the person by `board_week_story_person` with a draw(4)); otherwise, with the final position
/// above draw(N² / 200 + 1) + 1 and below N / 5 + N / 10 + 2 and the team's reputation above 350 × the competition's,
/// story 0x44 (the doubts).
unsafe fn national_finals_stories(s: &mut BoardWeek) -> bool {
    let club = s.club;
    if exe::database_get_national_team(read_usize(club + club::NATION)) != club || random(10) != 0 {
        return false;
    }
    let comp = exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), read_usize(club + club::NATION));
    if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), comp) == 0 || board_week_comp_object(comp) == 0 {
        return false;
    }
    let first_schedule = |out: &mut [u32; 2]| -> usize {
        let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x48f340usize);
        f(board_week_comp_object(comp), out.as_mut_ptr() as usize)
    };
    let (mut soon, mut first) = ([0u32; 2], [0u32; 2]);
    let d = exe::date_add_days(G_CURRENT_DATE, soon.as_mut_ptr() as usize, 10);
    let soon_copy = [read_u32(d), read_u32(d + 4)];
    if board_week_date_order(soon_copy.as_ptr() as usize, first_schedule(&mut first)) >= 0 {
        return false;
    }
    let (mut later, mut first2) = ([0u32; 2], [0u32; 2]);
    let d = exe::date_add_days(G_CURRENT_DATE, later.as_mut_ptr() as usize, 100);
    let later_copy = [read_u32(d), read_u32(d + 4)];
    if board_week_date_order(later_copy.as_ptr() as usize, first_schedule(&mut first2)) <= 0 {
        return false;
    }
    let stats = read_usize(board_week_comp_object(comp) + league::COMP_STATS);
    if stats == 0 {
        return true;
    }
    let entry = exe::comp_stats_find_club_entry(stats, club);
    if entry == 0 || read_u8(entry + comp_stats_club::MATCHES_PLAYED) != 0 {
        return true;
    }
    let n = read_i8(board_week_comp_object(comp) + league::ENTRANT_COUNT);
    s.league_size = n;
    let n = n as i32;
    if !s.told_finals_hopes && read_u8(entry + comp_stats_club::BEST_POSITION) == 1 {
        let r = random(3);
        if (read_u8(entry + comp_stats_club::FINAL_POSITION) as i32) < n / 5 - r + 2 {
            covered!("gra_17 finals hopes");
            let person = board_week_story_person(club, 4);
            board_week_post_story(club, 0x5a, person, -1);
            s.told_finals_hopes = true;
            return true;
        }
    }
    if s.told_finals_doubts {
        return true;
    }
    let last = read_u8(entry + comp_stats_club::FINAL_POSITION) as i32;
    if last <= random(n * n / 200 + 1) + 1 || last >= n / 5 + n / 10 + 2
        || read_i16(club + club::REPUTATION) as i32 <= read_i16(comp + competition::REPUTATION) as i32 * 350
    {
        return true;
    }
    covered!("gra_17 finals doubts");
    let person = board_week_story_person(club, 4);
    board_week_post_story(club, 0x44, person, -1);
    s.told_finals_doubts = true;
    true
}

/// The league week: nothing without a division with an object, its statistics (`league::COMP_STATS`) and the club's
/// entry (`exe::comp_stats_find_club_entry`). The projected positions (`exe::comp_stats_get_club_projected_positions`:
/// best, normal, worst), the form factor (`board_week_form_factor`), with best ≠ worst in the entry the form scaled by
/// the position against the previous final position, otherwise (no game played) the pre-season stories
/// (`preseason_board_stories`); the chairman's patience and the new manager's grace (`board_week_chairman_and_grace`);
/// then the change goes in (`apply_board_week_form`) and, with best ≠ worst and outside the new manager's grace, the
/// position against the target moves the confidence (`board_week_position`).
unsafe fn league_board_week(s: &mut BoardWeek) {
    let club = s.club;
    let division = read_usize(club + club::DIVISION);
    if division == 0 || board_week_comp_object(division) == 0 {
        return;
    }
    let stats = read_usize(board_week_comp_object(division) + league::COMP_STATS);
    if stats == 0 {
        return;
    }
    s.entry = exe::comp_stats_find_club_entry(stats, club);
    if s.entry == 0 {
        return;
    }
    let stats = read_usize(board_week_comp_object(read_usize(club + club::DIVISION)) + league::COMP_STATS);
    exe::comp_stats_get_club_projected_positions(stats, club, &mut s.proj_best, &mut s.proj_normal, &mut s.proj_worst);
    s.x = board_week_form_factor(s);
    let division = read_usize(club + club::DIVISION);
    let division_rep = read_i16(division + competition::REPUTATION);
    let club_rep = read_i16(club + club::REPUTATION);
    if division_rep != 0 && club_rep != 0 {
        if division_rep > 7 {
            let q = ((division_rep as i32 + 1) * 500) as f64 / club_rep as i32 as f64;
            let q = if q > read_f64(rdata::F64_1_25) {
                read_f32(rdata::F32_1_25) as f64
            } else if q < read_f64(rdata::F64_0_25) {
                read_f32(rdata::F32_0_25) as f64
            } else {
                q
            };
            let x = s.x as f64;
            s.x = if x < 0.0 && (q < read_f64(rdata::F64_1) || q > read_f64(rdata::F64_1)) { (x / q) as f32 } else { (q * x) as f32 };
        }
        if s.x < 0.0 && read_u8(club + club::PROFESSIONAL_STATUS) != 1 {
            s.x = (s.x as f64 * read_f64(rdata::F64_0_75)) as f32;
        }
        let e = s.entry;
        if read_u8(e + comp_stats_club::BEST_POSITION) != read_u8(e + comp_stats_club::WORST_POSITION) {
            let (current, last) = (read_u8(e + comp_stats_club::CURRENT_POSITION), read_u8(e + comp_stats_club::FINAL_POSITION));
            if current < last && s.x < 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_05)) as f32;
            } else if current == last && s.x < 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_1)) as f32;
            } else if current > last && s.x > 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_25)) as f32;
            }
        } else if read_u8(e + comp_stats_club::MATCHES_PLAYED) == 0 {
            preseason_board_stories(s);
        }
        board_week_chairman_and_grace(s);
    }
    apply_board_week_form(s);
    board_week_position(s);
}

/// The form factor x (f32): the current run in the league, counted only when its last game was less than 7 days ago
/// (`exe::date_days_since`): wins above 1 (× 25), defeats above 1 (× −30), winless above 2 (× −5) or unbeaten above 2
/// (× 5), in that order; the run n weighs n² below 4, otherwise min(n, 5)² × 0.2 + 9. A negative x is scaled by the
/// division's reputation × 0.1 (within 0.25..2) and, outside national teams, with the squad investment
/// (`board_record::SQUAD_INVESTMENT`) above the spending (`board_record::SPENDING`), by their ratio (at most 2.5), the
/// transfer mood (`board_record::TRANSFER_MOOD`) losing 150.
unsafe fn board_week_form_factor(s: &mut BoardWeek) -> f32 {
    let e = s.entry;
    let recent = |date: usize| exe::date_days_since(G_CURRENT_DATE, read_u32(e + date), read_u32(e + date + 4)) < 7;
    let (n, weight): (i16, i32) = if read_i32(e + comp_stats_club::WINNING_RUN) > 1 && recent(board_week_fields::WINNING_RUN_DATE) {
        covered!("gra_17 winning run");
        (read_i16(e + comp_stats_club::WINNING_RUN), 25)
    } else if read_i32(e + comp_stats_club::LOSING_RUN) > 1 && recent(board_week_fields::LOSING_RUN_DATE) {
        (read_i16(e + comp_stats_club::LOSING_RUN), -30)
    } else if read_i32(e + comp_stats_club::WINLESS_RUN) > 2 && recent(board_week_fields::WINLESS_RUN_DATE) {
        (read_i16(e + comp_stats_club::WINLESS_RUN), -5)
    } else if read_i32(e + comp_stats_club::UNBEATEN_RUN) > 2 && recent(board_week_fields::UNBEATEN_RUN_DATE) {
        (read_i16(e + comp_stats_club::UNBEATEN_RUN), 5)
    } else {
        (0, 0)
    };
    let f = if n >= 4 {
        let m = n.min(5) as i32;
        (m * m) as f64 * read_f64(rdata::F64_0_2) + read_f64(board_week_rdata::F64_9)
    } else {
        (n as i32 * n as i32) as f64
    };
    let mut x = (weight as f64 * f) as f32;
    if x < 0.0 {
        let m = read_i16(read_usize(s.club + club::DIVISION) + competition::REPUTATION) as i32 as f64 * read_f64(rdata::F64_0_1);
        let m = if m > read_f64(rdata::F64_2) {
            read_f32(rdata::F32_2) as f64
        } else if m < read_f64(rdata::F64_0_25) {
            read_f32(rdata::F32_0_25) as f64
        } else {
            m
        };
        x = (m * x as f64) as f32;
        if exe::database_is_national_team(s.club) == 0 {
            let row = s.row();
            if read_f64(row + board_record::SQUAD_INVESTMENT) > read_f64(row + board_record::SPENDING) {
                let r = read_f64(row + board_record::SQUAD_INVESTMENT) / read_f64(row + board_record::SPENDING);
                let r = if r > read_f64(rdata::F64_2_5) { read_f32(rdata::F32_2_5) as f64 } else { r };
                s.add(board_record::TRANSFER_MOOD, -150);
                x = (r * x as f64) as f32;
            }
        }
    }
    x
}

/// The pre-season stories (no game played, best = worst in the entry), with the division in a selected nation's league
/// (`nation::LEAGUE_SELECTED`), a manager and a top division (`league::UPPER_DIVISION_ID` = −1). N is the stale league
/// size (the defect, see `weekly_board_confidence_update`); positions are the previous final position. With the
/// position below N / 4 or the reputation above 6500 + draw(2000): the spending story (`board_week_spending_story`).
/// Otherwise, below N / 5 − draw(3) + 1: story 0x5d (title favourites); otherwise, above 1 and below N / 5 + N / 10:
/// story 0x42 (European hopes); the person by `board_week_story_person` with a draw(2).
unsafe fn preseason_board_stories(s: &mut BoardWeek) {
    let club = s.club;
    let in_league = || {
        let nation = read_usize(club + club::NATION);
        read_usize(club + club::DIVISION) != 0 && nation != 0 && read_u8(nation + nation::LEAGUE_SELECTED) != 0
            && read_usize(club + club::MANAGER) != 0
            && read_i32(board_week_comp_object(read_usize(club + club::DIVISION)) + league::UPPER_DIVISION_ID) == -1
    };
    let last = read_u8(s.entry + comp_stats_club::FINAL_POSITION) as i32;
    let n = s.league_size as i32;
    if in_league() && (last < n / 4 || read_i16(club + club::REPUTATION) as i32 > random(2000) + 6500) {
        board_week_spending_story(s);
        return;
    }
    if in_league() && last < n / 5 - random(3) + 1 {
        covered!("gra_17 preseason favourites");
        let person = board_week_story_person(club, 2);
        board_week_post_story(club, 0x5d, person, -1);
        return;
    }
    if in_league() && last > 1 && last < n / 5 + n / 10 {
        covered!("gra_17 preseason european hopes");
        let person = board_week_story_person(club, 2);
        board_week_post_story(club, 0x42, person, -1);
    }
}

/// The spending story: the club's transfers (`TRANSFER_MANAGER::get_past_transfers`) from the later of today − 100
/// days and the manager's arrival (`staff::DATE_JOINED_CLUB`) to the current year: what it paid minus what it got;
/// each purchase above 2,500,000 + draw(5,000,000) is a big one, and above 7,500,000 + draw(5,000,000) its player is
/// the star (the last such). With more than draw(2) + 1 big ones and the net above 15,000,000 + draw(10,000,000):
/// story 0x5b with the net as the extra (the star only with at least draw(2) + 2 big ones).
unsafe fn board_week_spending_story(s: &mut BoardWeek) {
    let club = s.club;
    let mut since = [0u32; 2];
    let d = exe::date_sub_days(G_CURRENT_DATE, since.as_mut_ptr() as usize, 100);
    let mut start = [read_u32(d), read_u32(d + 4)];
    let joined = read_usize(club + club::MANAGER) + staff::DATE_JOINED_CLUB;
    let joined_copy = [read_u32(joined), read_u32(joined + 4)];
    if board_week_date_order(joined_copy.as_ptr() as usize, start.as_ptr() as usize) > 0 {
        let mut tmp = [0u32; 2];
        exe::date_operator_3d(start.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, joined);
    }
    let start_year = read_u32(start.as_ptr() as usize + 2);
    let year = read_u16(G_CURRENT_DATE + 2) as u32;
    let (mut list, mut count) = (0usize, 0i32);
    let today_month = exe::date_get_month(G_CURRENT_DATE) as u32;
    let start_month = exe::date_get_month(start.as_ptr() as usize) as u32;
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, *mut usize, *mut i32) = core::mem::transmute(0x8cc030usize);
    f(read_usize(G_TRANSFER_MANAGER), start_month, start_year, today_month, year, &mut list, &mut count);
    let (mut net, mut big, mut star) = (0i32, 0i32, 0usize);
    let id = read_u32(club);
    for k in 0..count.max(0) as usize {
        let t = list + k * 0x25;
        if read_u32(t + 0x10) == id {
            let fee = read_i32(t + 0x18);
            net = net.wrapping_add(fee);
            if fee > random(5_000_000) + 2_500_000 {
                big += 1;
            }
            if read_i32(t + 0x18) > random(5_000_000) + 7_500_000 {
                star = staff_at(read_i32(t) as isize).addr();
            }
        } else if read_u32(t + 0x14) == id {
            net = net.wrapping_sub(read_i32(t + 0x18));
        }
    }
    if list != 0 {
        exe::crt_free(list);
    }
    if big > random(2) + 1 && net > random(10_000_000) + 15_000_000 {
        covered!("gra_17 spending story");
        if big < random(2) + 2 {
            star = 0;
        }
        board_week_post_story(club, 0x5b, star, net);
    }
}

/// The chairman's patience on a negative x (the patience reset to 10 below 1; factor 4 at 1, 0.25 at 20, otherwise
/// (21 − patience) × 0.1 within 0.5..2); without a chairman's sheet, and after it, a negative x with the board's
/// new-manager flag (`board_record_flags::NEW_MANAGER`) is × 0.2 in the manager's first 180 days, otherwise × 0.75.
unsafe fn board_week_chairman_and_grace(s: &mut BoardWeek) {
    let club = s.club;
    let sheet = board_week_chairman_sheet(club);
    if sheet != 0 {
        if !(s.x < 0.0) {
            return;
        }
        if read_i8(sheet + non_player::PATIENCE) < 1 {
            write_u8(sheet + non_player::PATIENCE, 10);
        }
        let m = board_week_patience_factor(read_i8(board_week_chairman_sheet(club) + non_player::PATIENCE), rdata::F32_4);
        s.x = (m * s.x as f64) as f32;
    }
    let manager = read_usize(club + club::MANAGER);
    if s.x < 0.0 && manager != 0 && read_u8(s.row() + board_record::FLAGS) & board_record_flags::NEW_MANAGER != 0 {
        let days = exe::date_days_since(G_CURRENT_DATE, read_u32(manager + staff::DATE_JOINED_CLUB), read_u32(manager + staff::DATE_JOINED_CLUB + 4));
        s.x = (s.x as f64 * if days < 180 { read_f64(rdata::F64_0_2) } else { read_f64(rdata::F64_0_75) }) as f32;
    }
}

/// The form goes in: d = truncate(x) on the board (`board_record::CONFIDENCE`), the supporters and the season's league
/// confidence (`board_record::LEAGUE_CONFIDENCE`), truncate(x × 1.25) on `board_record::FAVOURITES_MOOD`. Then N = the
/// division's club count (`league::CLUB_COUNT`, low byte) and the position weight w = (22 / N, or 1 without clubs),
/// with a division of reputation 10+ and the club's reputation above 500 × it, × ((rep − 500 × div) + (rep − 5000)) ×
/// 0.0002 + 1; w = w^1.5 (`game_pow`).
unsafe fn apply_board_week_form(s: &mut BoardWeek) {
    let d = ftol(s.x as f64) as i16;
    s.add(board_record::CONFIDENCE, d);
    s.add(board_record::SUPPORTERS_CONFIDENCE, d);
    s.add(board_record::FAVOURITES_MOOD, ftol(s.x as f64 * read_f64(rdata::F64_1_25)) as i16);
    s.add(board_record::LEAGUE_CONFIDENCE, d);
    let division = read_usize(s.club + club::DIVISION);
    s.division_object = board_week_comp_object(division);
    s.league_size = read_i8(s.division_object + league::CLUB_COUNT);
    let mut w = if s.league_size > 0 { read_f64(board_week_rdata::F64_22) / s.league_size as f64 } else { 1.0 };
    let division_rep = read_i16(division + competition::REPUTATION) as i32;
    let club_rep = read_i16(s.club + club::REPUTATION) as i32;
    if division_rep >= 10 && club_rep > division_rep * 500 {
        w = (((club_rep - division_rep * 500) as f64 + (club_rep - 5000) as f64) * read_f64(rdata::F64_0_0002) + read_f64(rdata::F64_1)) * w;
    }
    s.weight = game_pow(w, read_f64(rdata::F64_1_5));
}

/// The position against the target (only with best ≠ worst in the entry and without the new-manager flag): with even
/// the worst projection better than the previous final position, the flag `board_record_flags::PROJECTED_ABOVE_TARGET`
/// is set with +750 (already set: +750 on the board below 4000); with even the best one worse, the flag
/// `board_record_flags::PROJECTED_BELOW_TARGET` with −750 (already set: −750 above 6500); otherwise up to 4 games
/// played +5 above the final position, +1 on it and −5 below; after that the league stories
/// (`league_board_stories`) and the expectation (`board_week_expectation`). The change goes in by
/// `apply_board_week_position`.
unsafe fn board_week_position(s: &mut BoardWeek) {
    let e = s.entry;
    if read_u8(e + comp_stats_club::BEST_POSITION) == read_u8(e + comp_stats_club::WORST_POSITION) {
        return;
    }
    let flags = read_u8(s.row() + board_record::FLAGS);
    if flags & board_record_flags::NEW_MANAGER != 0 {
        return;
    }
    let last = read_u8(e + comp_stats_club::FINAL_POSITION);
    let delta: i32 = if read_u8(e + comp_stats_club::WORST_POSITION) < last {
        if flags & board_record_flags::PROJECTED_ABOVE_TARGET == 0 {
            write_u8(s.row() + board_record::FLAGS, flags | board_record_flags::PROJECTED_ABOVE_TARGET);
            750
        } else {
            if s.confidence() < 4000 {
                s.set_confidence(s.confidence().wrapping_add(750));
            }
            0
        }
    } else if read_u8(e + comp_stats_club::BEST_POSITION) > last {
        if flags & board_record_flags::PROJECTED_BELOW_TARGET == 0 {
            write_u8(s.row() + board_record::FLAGS, flags | board_record_flags::PROJECTED_BELOW_TARGET);
            -750
        } else {
            if s.confidence() > 6500 {
                s.set_confidence(s.confidence().wrapping_sub(750));
            }
            0
        }
    } else if read_u8(e + comp_stats_club::MATCHES_PLAYED) <= 4 {
        covered!("gra_17 early weeks");
        let current = read_u8(e + comp_stats_club::CURRENT_POSITION);
        if current < last { 5 } else if current == last { 1 } else { -5 }
    } else {
        league_board_stories(s);
        board_week_expectation(s)
    };
    apply_board_week_position(s, delta as i16);
}

/// The league stories (from the 5th game; at most one, in this order; "in a league" = division, a selected nation's
/// league, a manager; top = `league::UPPER_DIVISION_ID` −1; N = the division's club count; positions: current and final
/// of the entry, best/normal/worst projected; "runs" = wins above draw(a) + 2 or unbeaten above draw(b) + 4, "bad runs"
/// = defeats above draw(a) + 2 or winless above draw(b) + 4; the person by `board_week_story_person` with a draw(2)
/// unless "none"):
/// 0x5f below the top, 10 + draw(10) to 25 + draw(5) games, current below 4, final below 6 − draw(3), best within the
/// promotion and play-off places, worst not 1, runs (5, 5), draw(3) of 0; 0x60 more than 20 + draw(10) games, wins
/// above 1 or unbeaten above draw(3) + 2, a league with relegation (`league_vtable::HAS_RELEGATION`), current − draw(3)
/// + draw(3) above N − relegated, best below N − N / 2 − relegated, worst at least N − relegated; 0x43 more than
/// 10 + draw(200) games, unbeaten and winless below 3, (draw(6) × 0.1 + 1.6) × games above the entry's two stat bytes,
/// current − draw(4) + draw(4) = N / 2, best above N / 5, worst − draw(2) + draw(2) below N − relegated; at the top:
/// 0x63 (current 2..3, final 4..5) and 0x61 (current 2..3, final above 5), 10 + draw(10) to 25 + draw(5) games, best 1,
/// worst not 1, runs (5, 5); 0x62 10 + draw(10) to 29 games, current, final and best 1, worst not 1, runs (3, 4);
/// below the top 0x40, 25 + draw(10) games, current 2..8, final below 7, best within and worst outside the promotion
/// places, bad runs (3, 4); at the top 0x41, 25 + draw(10) games, current below 6, final below 5, normal 2 or 3, best
/// 1, worst not 1, bad runs (3, 4); 0x5e (none) reputation below 7250 + draw(2000), more than 5 + draw(10) and fewer
/// than 25 − draw(5) games, current above the final and below 5, best 1, worst not 1, wins non-zero and above 4 or
/// unbeaten above 7; 0x3e (none) current below the final, 5 + draw(10) to 25 − draw(5) games, defeats non-zero and above
/// 4 or winless above 7, relegation, current at least N − relegated and best above it; 0x3f more than 15 + draw(15)
/// games, defeats above 1 or winless above draw(3) + 2, relegation, draw(2) + current above N − relegated, normal at
/// least that, best below it and above N + N / 2 − relegated (never: the original's test); 0x3c (none, −250 on
/// `board_record::FAVOURITES_MOOD`) more than 20 games, final − current below −5, best below the final, the club
/// disliking the manager (`exe::club_dislikes_staff`) or a draw(4) of 0, the supporters (row by club id) below 4275,
/// no continental competition, defeats above draw(4) + 2 or winless above draw(7) + 3.
unsafe fn league_board_stories(s: &mut BoardWeek) {
    let club = s.club;
    let e = s.entry;
    let played = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32;
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION) as i32;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION) as i32;
    let (best, worst) = (s.proj_best as i32, s.proj_worst as i32);
    let division = || read_usize(club + club::DIVISION);
    let object = || board_week_comp_object(division());
    let nation_ok = || {
        let nation = read_usize(club + club::NATION);
        nation != 0 && read_u8(nation + nation::LEAGUE_SELECTED) != 0 && read_usize(club + club::MANAGER) != 0
    };
    let run = |field: usize| read_i32(e + field);
    let n = || s.league_size as i32;
    let relegated = || read_i8(object() + league::RELEGATED_COUNT) as i32;
    let promotion_places = || read_i8(object() + league::PROMOTION_PLAYOFF_COUNT) as i32 + read_i8(object() + league::PROMOTED_COUNT) as i32;
    let story = |kind: u32| {
        let person = board_week_story_person(club, 2);
        board_week_post_story(club, kind, person, -1);
    };
    // 0x5f: the promotion push
    if division() != 0 && nation_ok() && read_i32(s.division_object + league::UPPER_DIVISION_ID) != -1
        && played >= random(10) + 10 && played < random(5) + 25 && current < 4 && last < 6 - random(3)
        && best <= promotion_places() && worst != 1
        && (run(comp_stats_club::WINNING_RUN) > random(5) + 2 || run(comp_stats_club::UNBEATEN_RUN) > random(5) + 4)
        && random(3) == 0
    {
        covered!("gra_17 story 0x5f");
        return story(0x5f);
    }
    if division() == 0 {
        return board_week_story_0x63(s);
    }
    // 0x60: relegation trouble
    if nation_ok() && played > random(10) + 20
        && (run(comp_stats_club::WINNING_RUN) > 1 || run(comp_stats_club::UNBEATEN_RUN) > random(3) + 2)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        let v = current - random(3);
        let v = v + random(3);
        if v > safe && best < n() - n() / 2 - relegated() && worst >= safe {
            covered!("gra_17 story 0x60");
            return story(0x60);
        }
    }
    // 0x43: the steady mid-table
    if division() != 0 && nation_ok() && played > random(200) + 10
        && run(comp_stats_club::UNBEATEN_RUN) < 3 && run(comp_stats_club::WINLESS_RUN) < 3
    {
        let stat = (read_u8(e + board_week_fields::STAT_6A) as i32 + read_u8(e + board_week_fields::STAT_69) as i32) as f64;
        let r = random(6);
        if (r as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_1_6)) * played as f64 > stat {
            let a = current - random(4);
            let a = a + random(4);
            if a == n() / 2 && best > n() / 5 {
                let b = worst - random(2);
                let b = b + random(2);
                if b < n() - relegated() {
                    covered!("gra_17 story 0x43");
                    return story(0x43);
                }
            }
        }
    }
    board_week_story_0x63(s)
}

/// The league stories from 0x63 on (see `league_board_stories`).
unsafe fn board_week_story_0x63(s: &mut BoardWeek) {
    let club = s.club;
    let e = s.entry;
    let played = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32;
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION) as i32;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION) as i32;
    let (best, normal, worst) = (s.proj_best as i32, s.proj_normal as i32, s.proj_worst as i32);
    let division = || read_usize(club + club::DIVISION);
    let object = || board_week_comp_object(division());
    let in_league = || {
        let nation = read_usize(club + club::NATION);
        division() != 0 && nation != 0 && read_u8(nation + nation::LEAGUE_SELECTED) != 0 && read_usize(club + club::MANAGER) != 0
    };
    let top = || read_i32(object() + league::UPPER_DIVISION_ID) == -1;
    let run = |field: usize| read_i32(e + field);
    let n = || s.league_size as i32;
    let relegated = || read_i8(object() + league::RELEGATED_COUNT) as i32;
    let promotion_places = || read_i8(object() + league::PROMOTION_PLAYOFF_COUNT) as i32 + read_i8(object() + league::PROMOTED_COUNT) as i32;
    let story = |kind: u32| {
        let person = board_week_story_person(club, 2);
        board_week_post_story(club, kind, person, -1);
    };
    let good_runs = |a: i32, b: i32| run(comp_stats_club::WINNING_RUN) > random(a) + 2 || run(comp_stats_club::UNBEATEN_RUN) > random(b) + 4;
    let bad_runs = |a: i32, b: i32| run(comp_stats_club::LOSING_RUN) > random(a) + 2 || run(comp_stats_club::WINLESS_RUN) > random(b) + 4;
    // 0x63 and 0x61: chasing the title from the second or third place
    let title_chase: [(u32, fn(i32) -> bool); 2] = [(0x63, |l| l > 3 && l < 6), (0x61, |l| l > 5)];
    for (kind, last_ok) in title_chase {
        if in_league() && top() && played >= random(10) + 10 && played < random(5) + 25 && current < 4 && current > 1
            && last_ok(last) && best == 1 && worst != 1 && good_runs(5, 5)
        {
            covered!("gra_17 story 0x63 0x61");
            return story(kind);
        }
    }
    // 0x62: top of the table
    if in_league() && top() && played >= random(10) + 10 && played < 30 && current == 1 && last == 1 && best == 1 && worst != 1
        && good_runs(3, 4)
    {
        covered!("gra_17 story 0x62");
        return story(0x62);
    }
    // 0x40: slipping from the promotion places
    if in_league() && !top() && played >= random(10) + 25 && current < 9 && current > 1 && last < 7
        && best <= promotion_places() && worst > promotion_places() && bad_runs(3, 4)
    {
        covered!("gra_17 story 0x40");
        return story(0x40);
    }
    // 0x41: slipping from the title race
    if in_league() && top() && played >= random(10) + 25 && current < 6 && last < 5 && (normal == 2 || normal == 3) && best == 1
        && worst != 1 && bad_runs(3, 4)
    {
        covered!("gra_17 story 0x41");
        return story(0x41);
    }
    if division() == 0 {
        return board_week_story_0x3c(s);
    }
    // 0x5e: the surprise package
    if in_league() && (read_i16(club + club::REPUTATION) < (random(2000) + 7250) as i16) && played > random(10) + 5
        && played < 25 - random(5) && current < last && current < 5 && best == 1 && worst != 1
        && run(comp_stats_club::WINNING_RUN) != 0 && (run(comp_stats_club::WINNING_RUN) > 4 || run(comp_stats_club::UNBEATEN_RUN) > 7)
    {
        covered!("gra_17 story 0x5e");
        return board_week_post_story(club, 0x5e, 0, -1);
    }
    // 0x3e: the relegation zone
    if in_league() && current > last && played > random(10) + 5 && played < 25 - random(5)
        && run(comp_stats_club::LOSING_RUN) != 0 && (run(comp_stats_club::LOSING_RUN) > 4 || run(comp_stats_club::WINLESS_RUN) > 7)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        if current >= safe && best < safe {
            covered!("gra_17 story 0x3e");
            return board_week_post_story(club, 0x3e, 0, -1);
        }
    }
    // 0x3f: the relegation fight (the original's last test can never hold)
    if in_league() && played > random(15) + 15
        && (run(comp_stats_club::LOSING_RUN) > 1 || run(comp_stats_club::WINLESS_RUN) > random(3) + 2)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        if random(2) + current > safe && normal >= safe && best < safe && best > n() / 2 - relegated() + n() {
            covered!("gra_17 story 0x3f");
            return story(0x3f);
        }
    }
    board_week_story_0x3c(s)
}

/// The last league story, 0x3c (see `league_board_stories`).
unsafe fn board_week_story_0x3c(s: &mut BoardWeek) {
    let club = s.club;
    let e = s.entry;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION);
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION);
    if read_u8(e + comp_stats_club::MATCHES_PLAYED) > 20 && (last as i16 - current as i16) < -5 && s.proj_best > last
        && (exe::club_dislikes_staff(read_usize(club + club::MANAGER), club) != 0 || random(4) == 0)
        && (club == 0 || read_i16(read_usize(s.this).wrapping_add((read_i32(club) as usize).wrapping_mul(0x49)) + board_record::SUPPORTERS_CONFIDENCE) < 4275)
        && read_i32(club + club::CONTINENTAL_COMP_ID) == -1
        && (read_i32(e + comp_stats_club::LOSING_RUN) > random(4) + 2 || read_i32(e + comp_stats_club::WINLESS_RUN) > random(7) + 3)
    {
        covered!("gra_17 story 0x3c");
        board_week_post_story(club, 0x3c, 0, -1);
        s.add(board_record::FAVOURITES_MOOD, -250);
    }
}

/// The expectation against the previous final position (from the 5th game, after the stories): below it (better)
/// by more than 4, +15 a place + 40 (the board raised to 6500 instead, with no change, when below it); otherwise +10 a
/// place (raised to 4275 instead); on it, +5 (below 4275: raised to 4275 instead on a draw(patience) other than 0, or a
/// draw(10) without a chairman's sheet); above it (worse) by `board_week_overshoot`.
unsafe fn board_week_expectation(s: &mut BoardWeek) -> i32 {
    let e = s.entry;
    let (current, last) = (read_u8(e + comp_stats_club::CURRENT_POSITION), read_u8(e + comp_stats_club::FINAL_POSITION));
    if current < last {
        let d = (last as i16).wrapping_sub(current as i16);
        if d > 4 {
            if s.confidence() < 6500 {
                s.set_confidence(6500);
                return 0;
            }
            return d as i32 * 15 + 40;
        }
        if s.confidence() < 4275 {
            s.set_confidence(4275);
            return 0;
        }
        return d as i32 * 10;
    }
    if current == last {
        if s.confidence() < 4275 && board_week_patience_draw(s.club, 10) {
            s.set_confidence(4275);
            return 0;
        }
        return 5;
    }
    board_week_overshoot(s, current, last)
}

/// A draw(the chairman's patience) other than 0, or a draw(`n`) other than 0 without the chairman's sheet.
unsafe fn board_week_patience_draw(club: usize, n: i32) -> bool {
    let sheet = board_week_chairman_sheet(club);
    if sheet != 0 { random(read_i8(sheet + non_player::PATIENCE) as i32) != 0 } else { random(n) != 0 }
}

/// Worse than the final position: c = max((2 − games) × 2, truncate(final × w − current × w)) (16 bits). With the
/// title or promotion expected (`expectation_flags::TITLE_CHALLENGE`, `expectation_flags::PROMOTION`) and c below 0,
/// the current position within the division's play-off places (`league::PROMOTION_PLAYOFF_COUNT`, non-zero) counts as
/// c = 0; else with the play-offs expected (`expectation_flags::PLAYOFFS`) the same, the board raised to 6500. Then by
/// c: above −3, −5 (above 9500 the board pulled to 9500 instead when games ≥ patience / 2 + 2c + 10; below 4275 raised
/// to 4275 on the patience draw); −3 and −4, 10c − 25 (above 6500 to 6500 when games ≥ patience / 2 + c + 12; below
/// 3500 to 3500 on the patience draw); −5 to −9, 25c + 60 (above 6500 to 6500 when games ≥ patience / 2 + 10; above
/// 4275 to 4275 when games ≥ patience / 2 + 15; below 3000 to 3000 on the patience draw); −10 and below, 30c + 105 (the
/// same, but below 3500 to 3500 when draw(patience) > draw(30) + 1, or on a draw(5) of 0 without the chairman's sheet).
/// Each pull replaces the change (0).
unsafe fn board_week_overshoot(s: &mut BoardWeek, current: u8, last: u8) -> i32 {
    let club = s.club;
    let played = read_u8(s.entry + comp_stats_club::MATCHES_PLAYED) as i32;
    let v = ftol(last as i32 as f64 * s.weight - current as i32 as f64 * s.weight) as i16 as i32;
    let mut c = (2 - played) * 2;
    if !(c > v) {
        c = v;
    }
    let flags = read_u16(s.row() + board_record::EXPECTATION_FLAGS) as u8;
    let division = read_usize(club + club::DIVISION);
    let in_playoffs = || {
        let o = board_week_comp_object(read_usize(club + club::DIVISION));
        o != 0 && read_i8(o + league::PROMOTION_PLAYOFF_COUNT) != 0 && current as i32 <= read_i8(o + league::PROMOTION_PLAYOFF_COUNT) as i32
    };
    let mut pulled = false;
    if flags & (expectation_flags::TITLE_CHALLENGE | expectation_flags::PROMOTION) as u8 != 0 && (c as i16) < 0 && division != 0 {
        if in_playoffs() {
            c = 0;
            pulled = true;
        }
    } else if flags & expectation_flags::PLAYOFFS as u8 != 0 && (c as i16) < 0 && division != 0 && in_playoffs() {
        if s.confidence() < 6500 {
            s.set_confidence(6500);
        }
        c = 0;
        pulled = true;
    }
    let c16 = c as i16;
    let patience = || read_i8(board_week_chairman_sheet(club) + non_player::PATIENCE) as i32 / 2;
    let has_chairman = || board_week_chairman_sheet(club) != 0;
    if pulled || c16 > -3 {
        let conf = s.confidence();
        if conf > 9500 {
            if has_chairman() && played >= patience() + c16 as i32 * 2 + 10 {
                s.set_confidence(9500);
                return 0;
            }
        } else if conf < 4275 && board_week_patience_draw(club, 10) {
            s.set_confidence(4275);
            return 0;
        }
        return -5;
    }
    if c16 > -5 {
        let conf = s.confidence();
        if conf > 6500 {
            if has_chairman() && played >= patience() + c16 as i32 + 12 {
                s.set_confidence(6500);
                return 0;
            }
        } else if conf < 3500 && board_week_patience_draw(club, 10) {
            s.set_confidence(3500);
            return 0;
        }
        return c * 10 - 25;
    }
    let (delta, low) = if c16 > -10 { (c * 25 + 60, 3000) } else { (c * 30 + 105, 3500) };
    let conf = s.confidence();
    if conf > 6500 && has_chairman() && played >= patience() + 10 {
        s.set_confidence(6500);
        return 0;
    }
    if conf > 4275 {
        if has_chairman() && played >= patience() + 15 {
            s.set_confidence(4275);
            return 0;
        }
    } else if conf < low {
        let pull = if c16 > -10 {
            board_week_patience_draw(club, 10)
        } else {
            let sheet = board_week_chairman_sheet(club);
            if sheet != 0 {
                let a = random(read_i8(sheet + non_player::PATIENCE) as i32);
                a > random(30) + 1
            } else {
                random(5) == 0
            }
        };
        if pull {
            s.set_confidence(low as i16);
            return 0;
        }
    }
    delta
}

/// The position change goes in: d = truncate(change × the division's reputation × 0.1, within 0.5..2); for a club of
/// reputation above 5000 and above 400 × the division's, with final × w − current × w below −4, the final position
/// below 5 / w and d negative: d = truncate(min(2d, games × gap × 1.5)). A negative d × the chairman's patience factor
/// (patience outside 1..20 reset to 10; 3 at 1, 0.25 at 20). Outside national teams: with the squad investment above
/// the spending, a negative d × their ratio (at most 3) and the transfer mood −75; otherwise a positive d × 1.05 and the
/// transfer mood +5. A negative d × 0.75 for a club not fully professional, and again for a Welsh or Irish club (the
/// nations `KEY_NATION_WALES`, `KEY_NATION_REPUBLIC_OF_IRELAND`, `KEY_NATION_NORTHERN_IRELAND`). With a continental
/// competition and d negative, board and supporters lose truncate(d × −0.25); otherwise the board takes d and the
/// supporters truncate(d × 1.05).
unsafe fn apply_board_week_position(s: &mut BoardWeek, delta: i16) {
    let club = s.club;
    let e = s.entry;
    let division_rep = read_i16(read_usize(club + club::DIVISION) + competition::REPUTATION) as i32;
    let m = division_rep as f64 * read_f64(rdata::F64_0_1);
    let m = if m > read_f64(rdata::F64_2) {
        read_f32(rdata::F32_2) as f64
    } else if m < read_f64(rdata::F64_0_5) {
        read_f32(rdata::F32_0_5) as f64
    } else {
        m
    };
    let mut d = ftol(delta as i32 as f64 * m) as i16;
    let club_rep = read_i16(club + club::REPUTATION) as i32;
    if club_rep > 5000 && club_rep > division_rep * 400 {
        let (last, current) = (read_u8(e + comp_stats_club::FINAL_POSITION) as i32, read_u8(e + comp_stats_club::CURRENT_POSITION) as i32);
        let gap = last as f64 * s.weight - current as f64 * s.weight;
        if gap < read_f64(rdata::F64_NEG_4) && (last as f64) < read_f64(rdata::F64_5) / s.weight && d < 0 {
            let a = (d as i32 * 2) as f64;
            let u = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32 as f64 * gap * read_f64(rdata::F64_1_5);
            d = ftol(if a < u { a } else { u }) as i16;
        }
    }
    let sheet = board_week_chairman_sheet(club);
    if sheet != 0 && d < 0 {
        let p = read_i8(sheet + non_player::PATIENCE);
        if p < 1 || p > 20 {
            write_u8(sheet + non_player::PATIENCE, 10);
        }
        let f = board_week_patience_factor(read_i8(board_week_chairman_sheet(club) + non_player::PATIENCE), rdata::F32_3);
        d = ftol(d as i32 as f64 * f) as i16;
    }
    if exe::database_is_national_team(club) == 0 {
        let row = s.row();
        if read_f64(row + board_record::SQUAD_INVESTMENT) > read_f64(row + board_record::SPENDING) {
            if d < 0 {
                let r = read_f64(row + board_record::SQUAD_INVESTMENT) / read_f64(row + board_record::SPENDING);
                let r = if r > read_f64(rdata::F64_3) { read_f32(rdata::F32_3) as f64 } else { r };
                d = ftol(d as i32 as f64 * r) as i16;
            }
            s.add(board_record::TRANSFER_MOOD, -75);
        } else if d > 0 {
            d = ftol(d as i32 as f64 * read_f64(rdata::F64_1_05)) as i16;
            s.add(board_record::TRANSFER_MOOD, 5);
        }
    }
    if d < 0 {
        if read_u8(club + club::PROFESSIONAL_STATUS) != 1 {
            d = ftol(d as i32 as f64 * read_f64(rdata::F64_0_75)) as i16;
        }
        let nation = read_usize(club + club::NATION);
        if d < 0 && nation != 0 {
            let id = read_u32(nation);
            if id == read_u32(KEY_NATION_WALES) || id == read_u32(KEY_NATION_REPUBLIC_OF_IRELAND) || id == read_u32(KEY_NATION_NORTHERN_IRELAND) {
                d = ftol(d as i32 as f64 * read_f64(rdata::F64_0_75)) as i16;
            }
        }
    }
    if read_i32(club + club::CONTINENTAL_COMP_ID) != -1 && d < 0 {
        let a = ftol(d as i32 as f64 * read_f64(board_week_rdata::F64_NEG_0_25)) as i16;
        s.add(board_record::CONFIDENCE, a.wrapping_neg());
        s.add(board_record::SUPPORTERS_CONFIDENCE, a.wrapping_neg());
    } else {
        s.add(board_record::CONFIDENCE, d);
        s.add(board_record::SUPPORTERS_CONFIDENCE, ftol(d as i32 as f64 * read_f64(rdata::F64_1_05)) as i16);
    }
}

/// The end of every club's pass: with a continental competition other than the Intertoto Cup
/// (`KEY_CLUB_COMP_INTER_TOTO_CUP`; the club read again by index from `G_CLUB_TABLE`) the board at least 6500, or
/// 4275 when European success is expected (`expectation_flags_byte1::EUROPEAN_SUCCESS`) and the projection is below
/// the target (`board_record_flags::PROJECTED_BELOW_TARGET`); the board within 0..10000; at most 5250 when the manager
/// is disliked by the chairman (`exe::staff_is_disliked_by`), else at least 4250 when favoured
/// (`exe::staff_is_favoured_by`); the supporters within 0..10000, at most 3000 when the club dislikes the manager
/// (`exe::club_dislikes_staff`), else at least 7000 when it favours him (`exe::club_favours_staff`). Without the flags
/// 1, 4 and 8 of `board_record::FLAGS`: the manager's reputations (`update_board_week_reputation`) and, outside
/// national teams, the wealth band (`board_week_wealth_band`).
unsafe fn board_week_floors_and_reputation(s: &mut BoardWeek) {
    let club = s.club;
    let row = s.row();
    let european = read_u16(row + board_record::EXPECTATION_FLAGS) & ((expectation_flags_byte1::EUROPEAN_SUCCESS as u16) << 8);
    let continental = read_i32(read_usize(G_CLUB_TABLE).wrapping_add(s.index.wrapping_mul(club::SIZE)) + club::CONTINENTAL_COMP_ID) != -1
        && read_i32(club + club::CONTINENTAL_COMP_ID) != read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP);
    if continental {
        let floor = if european != 0 && read_u8(row + board_record::FLAGS) & board_record_flags::PROJECTED_BELOW_TARGET != 0 { 4275 } else { 6500 };
        if read_i16(row + board_record::CONFIDENCE) < floor {
            write_i16(row + board_record::CONFIDENCE, floor);
        }
    }
    let conf = s.confidence();
    let change = conf.wrapping_sub(s.confidence_before);
    if conf > 10000 {
        s.set_confidence(10000);
    } else if conf < 0 {
        s.set_confidence(0);
    }
    let chairman = read_usize(club + club::CHAIRMAN);
    if exe::staff_is_disliked_by(s.manager, chairman) != 0 {
        s.set_confidence(s.confidence().min(5250));
    } else if exe::staff_is_favoured_by(s.manager, chairman) != 0 {
        s.set_confidence(s.confidence().max(4250));
    }
    let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
    let v = read_i16(a);
    if v > 10000 {
        write_i16(a, 10000);
    } else if v < 0 {
        write_i16(a, 0);
    }
    if exe::club_dislikes_staff(s.manager, club) != 0 {
        let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
        write_i16(a, read_i16(a).min(3000));
    } else if exe::club_favours_staff(s.manager, club) != 0 {
        let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
        write_i16(a, read_i16(a).max(7000));
    }
    let flags = read_u8(s.row() + board_record::FLAGS);
    if flags & (board_record_flags::NEW_MANAGER | board_record_flags::PROJECTED_ABOVE_TARGET | board_record_flags::PROJECTED_BELOW_TARGET) != 0 {
        return;
    }
    let mut d = change;
    if d < 0 {
        let rep = read_i16(club + club::REPUTATION);
        d = if rep > 5000 {
            ftol(read_f64(board_week_rdata::F64_2500) / rep as i32 as f64 * d as i32 as f64)
        } else {
            ftol(d as i32 as f64 * read_f64(rdata::F64_0_5))
        } as i16;
    }
    update_board_week_reputation(s, d);
    if exe::database_is_national_team(club) == 0 {
        board_week_wealth_band(s);
    }
}

/// The reputation band of a reputation: v / 1000 + 16, at least 22 (signed byte).
fn board_week_band(v: i16) -> i8 {
    let b = ((v as i32 / 1000) as i8).wrapping_add(16);
    if b < 22 { 22 } else { b }
}

/// Keeps a value pulled outside [lo, hi] at the nearer bound, the bounds limited to 1..10000 (as the original's tests:
/// only a value outside is touched).
fn board_week_pull(v: i32, lo: i32, hi: i32) -> i32 {
    if v > hi {
        if hi > 10000 { 10000 } else { hi }
    } else if v < lo {
        if lo < 1 { 1 } else { lo }
    } else {
        v
    }
}

/// The manager's reputations (`non_player::CURRENT_REPUTATION`, `non_player::HOME_REPUTATION`,
/// `non_player::WORLD_REPUTATION`) move with the board's change d (16 bits; a fall scaled by 2500 / reputation above
/// 5000, otherwise halved): targets = reputation + d, pulled within the club's reputation ± 3000 (world: −5000..+1000).
/// Each one's speed n = its band (`board_week_band`) − 19; for the home one 23 − club reputation / 500 when the manager
/// is foreign to the club's nation (`staff::NATION`), and for the world one 23 − club reputation / 500 outside national
/// teams (n 0 counts as 1). New = ((n − 1) × old + target) / n, kept within old − f × 250 and old + f × 300 (home:
/// 375; world: ± f × 500 rounded to f32) with f = 2 − old × 0.0001; capped at the club's reputation (or 50 × its
/// division's, the larger) within 5000..10000 but not below the old value (world: 2/3 of it within 3000..10000), and at
/// least 1. Each reputation becomes new / 2 + old / 2.
unsafe fn update_board_week_reputation(s: &mut BoardWeek, d: i16) {
    let club = s.club;
    let sheet = s.sheet;
    let home = read_i16(sheet + non_player::HOME_REPUTATION);
    let current = read_i16(sheet + non_player::CURRENT_REPUTATION);
    let world = read_i16(sheet + non_player::WORLD_REPUTATION);
    let club_rep = read_i16(club + club::REPUTATION) as i32;
    let current_target = board_week_pull(current.wrapping_add(d) as i32, club_rep - 3000, club_rep + 3000);
    let home_target = board_week_pull(home.wrapping_add(d) as i32, club_rep - 3000, club_rep + 3000);
    let world_target = board_week_pull(world.wrapping_add(d) as i32, club_rep - 5000, club_rep + 1000);
    covered!("gra_17 manager reputation");
    let (current_band, home_band, world_band) = (board_week_band(current), board_week_band(home), board_week_band(world));
    let nonzero = |n: i16| if n == 0 { 1 } else { n };
    let n_current = nonzero((current_band as i16).wrapping_sub(19));
    let foreign = exe::database_is_national_team(club) == 0
        && read_u32(club + club::NATION) != read_u32(s.manager + staff::NATION);
    let n_home = nonzero(if foreign { (23 - read_i16(club + club::REPUTATION) as i32 / 500) as i16 } else { (home_band as i16).wrapping_sub(19) });
    let n_world = nonzero(if exe::database_is_national_team(club) != 0 {
        (world_band as i16).wrapping_sub(19)
    } else {
        (23 - read_i16(club + club::REPUTATION) as i32 / 500) as i16
    });
    let step = |n: i16, old: i16, target: i32| ((n as i32 - 1).wrapping_mul(old as i32).wrapping_add(target as i16 as i32)) / n as i32;
    let new_current = step(n_current, current, current_target) as i16;
    let new_home = step(n_home, home, home_target) as i16;
    let new_world = step(n_world, world, world_target) as i16;
    let limit = |old: i16, new: i16, f: f32, down: f64, up: f64| {
        let o = old as i32 as f32 as f64;
        let nf = new as i32 as f32 as f64;
        let lo = o - f as f64 * down;
        if lo > nf {
            return ftol(lo);
        }
        let hi = f as f64 * up + o;
        if hi < nf { ftol(hi) } else { ftol(nf) }
    };
    let factor = |v: i16| read_f64(rdata::F64_2) - v as i32 as f64 * read_f64(rdata::F64_0_0001);
    let final_current = limit(current, new_current, factor(current) as f32, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_300) as f64) as i16;
    let final_home = limit(home, new_home, factor(home) as f32, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_375) as f64) as i16;
    let k = (factor(world) * read_f32(rdata::F32_500) as f64) as f32 as f64;
    let wf = world as i32 as f32 as f64;
    let nwf = new_world as i32 as f32 as f64;
    let final_world = if wf - k > nwf { ftol(wf - k) } else if k + wf < nwf { ftol(k + wf) } else { ftol(nwf) } as i16;
    let mut cap_rep = read_i16(club + club::REPUTATION);
    let division = read_usize(club + club::DIVISION);
    if division != 0 {
        let e = read_i16(division + competition::REPUTATION) as i32 * 50;
        if e > cap_rep as i32 {
            cap_rep = e as i16;
        }
    }
    let band = |v: i16| if v < 5000 { 5000 } else if v > 10000 { 10000 } else { v };
    let cap = |c: i16, old: i16, v: i16| {
        let c = if c > old { c } else { old };
        let v = if v > c { c } else { v };
        if v < 1 { 1 } else { v }
    };
    let final_home = cap(band(cap_rep), home, final_home);
    let final_current = cap(band(cap_rep), current, final_current);
    let w = cap_rep as i32 as f64 * read_f64(rdata::F64_0_6666666667);
    let w = if !(read_f64(board_week_rdata::F64_3000) <= w) {
        read_f64(board_week_rdata::F64_3000)
    } else if read_f64(rdata::F64_10000) < w {
        read_f64(rdata::F64_10000)
    } else {
        w
    };
    let final_world = cap(ftol(w) as i16, world, final_world);
    write_i16(sheet + non_player::CURRENT_REPUTATION, (final_current as i32 / 2 + current as i32 / 2) as i16);
    write_i16(sheet + non_player::HOME_REPUTATION, (final_home as i32 / 2 + home as i32 / 2) as i16);
    write_i16(sheet + non_player::WORLD_REPUTATION, (final_world as i32 / 2 + world as i32 / 2) as i16);
}

/// The wealth band (`exe::finance_get_wealth_band` of `exe::financetable_get_club_finance`) against the one the board
/// took at the season's start (`board_record::WEALTH_BAND`, row by club id), on the board and on
/// `board_week_fields::FINANCE_MOOD`: band 2 below 2 → +35 (reputation below 3500) or +15; band 1 below 1 → +30 or
/// +10; band 0: from below 0 → +25 or +5, from above → −5; band −1: from below 0 → +5, otherwise −50 / −125 for a plc
/// (`club::PLC`) or −25 / −50; band −2: from −2 → +5, otherwise −75 / −175 (plc) or −50 / −75; another band shows the
/// manager_manager.cpp box (line 0x28aa). Then the board within 0..10000.
unsafe fn board_week_wealth_band(s: &mut BoardWeek) {
    let club = s.club;
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if finance != 0 {
        let band = exe::finance_get_wealth_band(finance) as i8;
        let taken = read_i8(read_usize(s.this).wrapping_add((read_i32(club) as usize).wrapping_mul(0x49)) + board_record::WEALTH_BAND);
        let low = read_i16(club + club::REPUTATION) < 3500;
        let plc = read_u8(club + club::PLC) != 0;
        let (board, mood): (i16, i16) = match band {
            2 => if taken >= 2 { (0, 0) } else if low { (35, 35) } else { (15, 15) },
            1 => if taken >= 1 { (0, 0) } else if low { (30, 30) } else { (10, 10) },
            0 => if taken < 0 { if low { (25, 25) } else { (5, 5) } } else if taken > 0 { (-5, -5) } else { (0, 0) },
            -1 => if taken < 0 { (5, 5) } else if plc { (-50, -125) } else { (-25, -50) },
            -2 => if taken == -2 { (5, 5) } else if plc { (-75, -175) } else { (-50, -75) },
            _ => {
                show_error_at(0x9f18a4, 0x28aa);
                (0, 0)
            }
        };
        if board != 0 || mood != 0 {
            covered!("gra_17 wealth band");
            s.add(board_record::CONFIDENCE, board);
            s.add(board_week_fields::FINANCE_MOOD, mood);
        }
    }
    let conf = s.confidence();
    if conf > 10000 {
        s.set_confidence(10000);
    } else if conf < 0 {
        s.set_confidence(0);
    }
}

// ---------- the comparer: the original runs, the stories are taped, the copy replays them ----------

/// One story the original posted during the weekly update: the arguments, the compared state just before it and the
/// bytes it changed (address, new value).
struct BoardStoryCall {
    args: [u32; 4],
    before: Vec<u8>,
    changes: Vec<(usize, u8)>,
}

/// The tape of the stories: the compared regions, the calls, the next one to replay and the first mismatch.
struct BoardStoryTape {
    regions: Vec<(usize, usize)>,
    calls: Vec<BoardStoryCall>,
    next: usize,
    broken: Option<String>,
    recording: bool,
    depth: u32,
    target: usize,
}

static mut BOARD_STORY_TAPE: Option<BoardStoryTape> = None;

unsafe fn board_story_flat(regions: &[(usize, usize)]) -> Vec<u8> {
    let mut v = Vec::with_capacity(regions.iter().map(|r| r.1).sum());
    for &(a, n) in regions {
        v.extend_from_slice(core::slice::from_raw_parts(a as *const u8, n));
    }
    v
}

unsafe fn board_story_write(regions: &[(usize, usize)], flat: &[u8]) {
    let mut k = 0;
    for &(a, n) in regions {
        core::ptr::copy_nonoverlapping(flat.as_ptr().add(k), a as *mut u8, n);
        k += n;
    }
}

/// The first difference between two flat copies, by region (index, offset).
fn board_story_first_difference(regions: &[(usize, usize)], a: &[u8], b: &[u8]) -> Option<(usize, usize, u8, u8)> {
    let mut k = 0;
    for (i, &(_, n)) in regions.iter().enumerate() {
        if let Some(o) = (0..n).find(|&o| a[k + o] != b[k + o]) {
            return Some((i, o, a[k + o], b[k + o]));
        }
        k += n;
    }
    None
}

/// The recorder the comparer puts at the story entry (0x74b060) while the original runs: tapes the state before, calls
/// the hooked story and tapes what changed.
unsafe extern "thiscall" fn board_story_recorder(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    let f: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32, u32) = core::mem::transmute(tape.target);
    if tape.depth > 0 || !tape.recording {
        return f(this, club, kind, person, game, variant, extra);
    }
    tape.depth += 1;
    let before = board_story_flat(&tape.regions);
    f(this, club, kind, person, game, variant, extra);
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    tape.depth -= 1;
    let after = board_story_flat(&tape.regions);
    let mut changes = Vec::new();
    let mut k = 0;
    for &(a, n) in &tape.regions {
        for o in 0..n {
            if before[k + o] != after[k + o] {
                changes.push((a + o, after[k + o]));
            }
        }
        k += n;
    }
    tape.calls.push(BoardStoryCall { args: [club as u32, kind, person as u32, extra], before, changes });
}

/// In the comparer's replay, the copy's story: the next taped call must have the same arguments and the same state
/// before it; its recorded changes are applied. Returns false outside the replay.
unsafe fn board_week_replay_story(club: usize, kind: u32, person: usize, extra: i32) -> bool {
    let tape = match (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut() {
        Some(t) if !t.recording => t,
        _ => return false,
    };
    if tape.broken.is_some() {
        return true;
    }
    let args = [club as u32, kind, person as u32, extra as u32];
    let Some(call) = tape.calls.get(tape.next) else {
        tape.broken = Some(format!("story {} {:x?} the original did not post", tape.next, args));
        return true;
    };
    if call.args != args {
        tape.broken = Some(format!("story {}: copy {:x?} original {:x?}", tape.next, args, call.args));
        return true;
    }
    let now = board_story_flat(&tape.regions);
    if let Some((i, o, x, y)) = board_story_first_difference(&tape.regions, &now, &call.before) {
        tape.broken = Some(format!("state before story {} {:x?}: region {} (+{:#x}) copy {:02x} original {:02x}", tape.next, args, i, o, x, y));
        return true;
    }
    for &(a, b) in &call.changes {
        write_u8(a, b);
    }
    tape.next += 1;
    true
}

/// Calls the original as the game would and returns the byte it finds at its league size local ([esp+0x2b] of its
/// frame: the aligned frame base − 0x2b1), read just before the call.
unsafe fn board_week_call_original(f: usize, this: usize) -> u8 {
    let mut residue: u32 = 0;
    core::arch::asm!(
        "lea eax, [esp - 8]",
        "and eax, -8",
        "movzx eax, byte ptr [eax - 0x2b1]",
        "mov dword ptr [edi], eax",
        "call edx",
        in("edi") &mut residue as *mut u32,
        in("edx") f,
        in("ecx") this,
        clobber_abi("C"),
    );
    residue as u8
}

/// Comparer for `weekly_board_confidence_update`: the routine posts stories (news, the log, the manager's answer) and
/// cannot run twice. The original runs as the game's, with a recorder on the story entry (the `jmp` of the hook at
/// 0x74b060 pointed at `board_story_recorder` meanwhile) that tapes, for every story, its arguments, the compared state
/// before it and what it changed; the compared state is the board table ([this], clubs × 0x49), the random generator and
/// every club's chairman patience and manager reputations. Then the state before is put back and the copy runs from
/// it with the league size the original found on its stack: each story of the copy must be the next one taped, with
/// the same arguments and the same state before it, and gets the taped changes; at the end every story must have been
/// replayed and the state must equal the original's (which stays).
pub(crate) unsafe extern "thiscall" fn compare_weekly_board_confidence_update(this: usize) {
    let o = original_fn::<usize>(%%I0%%);
    let entry = board_week_fns::MEDIA_STORY_ENTRY;
    if read_u8(entry) != 0xe9 || read_usize(this) == 0 {
        board_week_call_original(o, this); // ponytail: no hook on the story entry (or no table): only the original
        return;
    }
    let target = entry.wrapping_add(5).wrapping_add(read_i32(entry + 1) as usize);
    let count = read_i32(G_CLUB_COUNT).max(0) as usize;
    let mut regions: Vec<(usize, usize)> = vec![(read_usize(this), count * 0x49)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let base = read_usize(G_CLUB_TABLE);
    for i in 0..count {
        let club = base + i * club::SIZE;
        let sheet = board_week_chairman_sheet(club);
        if sheet != 0 {
            regions.push((sheet + non_player::PATIENCE, 1));
        }
        let manager = read_usize(club + club::MANAGER);
        if manager != 0 && read_usize(manager + staff::NON_PLAYER) != 0 {
            regions.push((read_usize(manager + staff::NON_PLAYER) + non_player::HOME_REPUTATION, 6));
        }
    }
    let before = board_story_flat(&regions);
    *core::ptr::addr_of_mut!(BOARD_STORY_TAPE) = Some(BoardStoryTape { regions, calls: Vec::new(), next: 0, broken: None, recording: true, depth: 0, target });
    let patch = |to: usize| {
        let mut old = 0u32;
        crate::win::VirtualProtect(entry as *mut core::ffi::c_void, 5, crate::win::PAGE_EXECUTE_READWRITE, &mut old);
        write_i32(entry + 1, to.wrapping_sub(entry + 5) as i32);
        crate::win::VirtualProtect(entry as *mut core::ffi::c_void, 5, old, &mut old);
        crate::win::FlushInstructionCache(crate::win::GetCurrentProcess(), entry as *const core::ffi::c_void, 5);
    };
    patch(board_story_recorder as *const () as usize);
    let residue = board_week_call_original(o, this);
    patch(target);
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    tape.recording = false;
    let after = board_story_flat(&tape.regions);
    board_story_write(&tape.regions, &before);
    let n = crate::rng::without_counting(|| board_week_run(this, residue as i8));
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).take().unwrap();
    let copy = board_story_flat(&tape.regions);
    let difference = board_story_first_difference(&tape.regions, &copy, &after);
    board_story_write(&tape.regions, &after);
    BOARD_WEEK_LEAGUE_SIZE.store(n as u8, Relaxed);
    let ok = tape.broken.is_none() && tape.next == tape.calls.len() && difference.is_none();
    SCOREBOARD.record(ok, || {
        let place = difference.map(|(i, o, x, y)| {
            let what = if i == 0 { format!("board row of club {} +{:#x}", o / 0x49, o % 0x49) } else if i < 3 { format!("random generator +{:#x}", o) } else { format!("region {} +{:#x}", i, o) };
            format!("{} copy {:02x} original {:02x}", what, x, y)
        });
        format!("weekly_board_confidence_update(residue N {}): stories {} replayed {} {:?}; state {:?}", residue as i8, tape.calls.len(), tape.next, tape.broken, place)
    });
}
