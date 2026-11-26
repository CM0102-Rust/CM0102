use super::*;

/// Offers a player to the team award. Without a person, a player sheet or a club, nothing. Already chosen, his rating
/// rises to `rating` when higher. Otherwise his positions (`position_mask`, or his sheet's `exe::formation_get_position_mask`
/// when 0) are widened (`widen_award_positions`) and he takes the first empty slot that fits; with none, the first
/// filled slot that fits him whose occupant fits an empty slot: the occupant moves there and he takes the slot. His
/// squad number (absolute) is kept when his club uses squad numbers. Returns 1 when the selection is complete.
/// Original: `TEAM_AWARD::add_candidate` (0x8afa40, team_award.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn team_award_add_candidate(this: usize, person: usize, club: usize, rating: f32, position_mask: u16) -> u32 {
    let t = TeamAward(this);
    let n = t.slot_count();
    let person = Staff(person);
    if person.is_null() || person.player().is_null() || club == 0 {
        return t.complete() as u32;
    }
    covered!("sx05_6 candidate");
    if let Some(k) = (0..n.max(0) as usize).find(|&k| t.slot_id(k) == read_i32(person.addr())) {
        covered!("sx05_6 chosen again");
        if rating > read_f32(t.slot(k) + 0x15) {
            write_f32(t.slot(k) + 0x15, rating);
        }
        return t.complete() as u32;
    }
    let mask = widen_award_positions(if position_mask != 0 { position_mask } else { exe::formation_get_position_mask(person.player()) });
    let mut number = 0u8;
    if exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 0) != 0 {
        number = (exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), person, club) as i32).unsigned_abs() as u8;
    }
    let slots = 0..n.max(0) as usize;
    let place = |k: usize| {
        team_award_place(t, k, person, club, rating, number);
        write_u8(this + 0x252, read_u8(this + 0x252).wrapping_add(1));
        t.complete() as u32
    };
    if let Some(k) = slots.clone().find(|&k| t.slot_id(k) == -1 && t.fits(k, mask)) {
        return place(k);
    }
    for k in slots.clone() {
        if t.slot_id(k) == -1 || !t.fits(k, mask) {
            continue;
        }
        let occupant = widen_award_positions(exe::formation_get_position_mask(staff_at(t.slot_id(k) as isize).player()));
        if let Some(j) = slots.clone().find(|&j| t.slot_id(j) == -1 && t.fits(j, occupant)) {
            covered!("sx05_6 occupant moved");
            core::ptr::copy(t.slot(k) as *const u8, t.slot(j) as *mut u8, TEAM_AWARD_SLOT_SIZE);
            return place(k);
        }
    }
    t.complete() as u32
}

/// Empties the twenty slots (id, name ids and club -1, rating 0; the squad number is left) and the filled count.
/// Original: `TEAM_AWARD::clear_selection` (0x8afd90, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_clear_selection(this: usize) {
    let t = TeamAward(this);
    for k in 0..20 {
        for f in [0, 4, 8, 0xc, 0x10] {
            write_i32(t.slot(k) + f, -1);
        }
        write_u32(t.slot(k) + 0x15, 0);
    }
    write_u8(this + 0x252, 0);
}

/// Sets the position mask of each slot: the eleven by the formation mode (0, 1 and 2 their own, any other the
/// fourth), the nine substitutes always goalkeeper then three times (0x40, 4, 0x10).
/// Original: `TEAM_AWARD::init_slot_positions` (0x8afdc0, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_init_slot_positions(this: usize) {
    let eleven = &TEAM_AWARD_ELEVENS[(read_u8(this + 0x25c) as usize).min(3)];
    for (k, &m) in eleven.iter().chain(TEAM_AWARD_SUBSTITUTES.iter()).enumerate() {
        write_u16(this + 0x26 + 2 * k, m);
    }
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_add_candidate(this: usize, person: usize, club: usize, rating: f32, position_mask: u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, f32, u16) -> u32>(IDX_TEAM_AWARD_ADD_CANDIDATE);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(this + TEAM_AWARD_SELECTION.0, TEAM_AWARD_SELECTION.1)],
        || o(this, person, club, rating, position_mask) & 0xff, || team_award_add_candidate(this, person, club, rating, position_mask));
    SCOREBOARD.record(equal && r == v, || format!("team_award_add_candidate(person {:#x}) original={} rust={} {:?}", person, r, v, first_differences(&a, &b)));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_clear_selection(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEAM_AWARD_CLEAR_SELECTION);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + TEAM_AWARD_SELECTION.0, TEAM_AWARD_SELECTION.1)],
        || o(this), || team_award_clear_selection(this));
    SCOREBOARD.record(equal, || format!("team_award_clear_selection({:#x}) {:?}", this, first_differences(&a, &b)));
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_init_slot_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEAM_AWARD_INIT_SLOT_POSITIONS);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + 0x26, 2 * 20)], || o(this), || team_award_init_slot_positions(this));
    SCOREBOARD.record(equal, || format!("team_award_init_slot_positions(mode {}) {:?}", read_u8(this + 0x25c), first_differences(&a, &b)));
}

/// A player's position bits widened for the team award slots: 2 also gives 4, 8 or 0x20 give 0x10, and 0x40 with
/// 0x80 or with 0x800 gives 0x200.
pub(crate) fn widen_award_positions(mut m: u16) -> u16 {
    if m & 2 != 0 {
        m |= 4;
    }
    if m & 8 != 0 { // sx05_6: 8 brings 0x10
        m |= 0x10;
    }
    if m & 0x20 != 0 {
        m |= 0x10;
    }
    if m & 0x40 != 0 && (m & 0x80 != 0 || m & 0x800 != 0) {
        m |= 0x200;
    }
    m
}

/// Writes `person` into team award slot `k`: his id, rating, club id, squad number and three name ids.
pub(crate) unsafe fn team_award_place(t: TeamAward, k: usize, person: Staff, club: usize, rating: f32, number: u8) {
    let s = t.slot(k);
    write_i32(s, read_i32(person.addr()));
    write_f32(s + 0x15, rating);
    write_i32(s + 0x10, read_i32(club));
    write_u8(s + 0x14, number);
    write_i32(s + 4, award_name_id(person.first_name()));
    write_i32(s + 8, award_name_id(person.second_name()));
    write_i32(s + 0xc, award_name_id(person.common_name()));
}

/// The name id of a staff name record (+0x33), -1 without one.
pub(crate) unsafe fn award_name_id(record: usize) -> i32 {
    if record == 0 { -1 } else { read_i32(record + 0x33) }
}

/// The team award's selection (`TEAM_AWARD`): the position mask word of each slot at +0x26 (2 bytes a slot), the
/// 20 slots of 0x19 bytes at +0x4e (+0 staff id, +4/+8/+0xc the three name ids, +0x10 the club id, +0x14 the squad
/// number, +0x15 the rating, f32), the filled count at +0x252, the substitutes count at +0x253 (the slots are 11 +
/// it) and the formation mode at +0x25c.
#[derive(Clone, Copy)]
pub(crate) struct TeamAward(pub(crate) usize);

impl TeamAward {
    pub(crate) unsafe fn slot_count(self) -> i8 {
        (read_u8(self.0 + 0x253) as i8).wrapping_add(11)
    }
    pub(crate) unsafe fn filled(self) -> i8 {
        read_i8(self.0 + 0x252)
    }
    pub(crate) fn slot(self, k: usize) -> usize {
        self.0 + 0x4e + k * TEAM_AWARD_SLOT_SIZE
    }
    pub(crate) unsafe fn slot_mask(self, k: usize) -> u16 {
        read_u16(self.0 + 0x26 + 2 * k)
    }
    pub(crate) unsafe fn slot_id(self, k: usize) -> i32 {
        read_i32(self.slot(k))
    }
    /// The slot accepts a player whose (widened) positions are `mask`: all its own bits are in it.
    pub(crate) unsafe fn fits(self, k: usize, mask: u16) -> bool {
        self.slot_mask(k) & mask == self.slot_mask(k)
    }
    /// The selection is complete: the filled count equals the slot count.
    pub(crate) unsafe fn complete(self) -> bool {
        self.filled() == self.slot_count()
    }
}

/// The position masks of the starting eleven by the formation mode (+0x25c), then of the nine substitutes.
pub(crate) const TEAM_AWARD_ELEVENS: [[u16; 11]; 4] = [
    [1, 0x804, 0x84, 0x204, 0x204, 0x210, 0x810, 0x210, 0x240, 0x240, 0x90],
    [1, 0x804, 0x84, 0x204, 0x204, 0x90, 0x210, 0x810, 0xc0, 0x240, 0x840],
    [1, 0x808, 0x88, 0x204, 0x204, 2, 0x210, 0x210, 0x240, 0x240, 0x210],
    [1, 0x804, 0x84, 2, 0x204, 0x208, 0x90, 0x810, 0x220, 0x240, 0x840],
];

/// The team award's slot masks, slots and filled count (+0x26..+0x253).
pub(crate) const TEAM_AWARD_SELECTION: (usize, usize) = (0x26, 0x253 - 0x26);

pub(crate) const TEAM_AWARD_SLOT_SIZE: usize = 0x19;

/// Builds a team award: the award base (nation, index, staff comp, national and world reputations, club comp, the
/// file and the player flag), three empty dates (the update date and the period's), the season span (`span`), the
/// formation mode (`mode`), type 0, three substitutes, no history (−1, −1), the selection cleared
/// (`team_award_clear_selection`) and the slot masks (`team_award_init_slot_positions`); with a file, then
/// `TEAM_AWARD::load_from_disk(file)` (its result not read). Returns this (the original builds the dates under an SEH
/// frame; nothing here throws).
/// Original: `TEAM_AWARD::TEAM_AWARD` (0x8af520, team_award.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn team_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, span: u32, mode: u32, file: usize, for_players: u32) -> usize {
    use team_award_obj as ta;
    award_construct_base(this, [nation, index, staff_comp, national_rep, world_rep, club_comp, file as u32, for_players]);
    exe::date_date_54a110(this + ta::UPDATE_DATE);
    exe::date_date_54a110(this + ta::PERIOD_START);
    exe::date_date_54a110(this + ta::PERIOD_END);
    write_u8(this + ta::SEASON_SPAN, span as u8);
    write_u8(this + ta::SUBSTITUTES, 3);
    write_u8(this + ta::FORMATION_MODE, mode as u8);
    write_usize(this, ta::VTABLE);
    write_u8(this + award_base::TYPE, 0);
    write_i32(this + ta::HISTORY_FIRST, -1);
    write_i32(this + ta::HISTORY_LAST, -1);
    team_award_clear_selection(this);
    team_award_init_slot_positions(this);
    if file != 0 {
        covered!("sx05b_5 team award loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TEAM_AWARD_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, span: u32, mode: u32, file: usize, for_players: u32) -> usize {
    use team_award_obj as ta;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> usize>(IDX_TEAM_AWARD_CONSTRUCT);
    let r = o(this, nation, index, staff_comp, national_rep, world_rep, club_comp, span, mode, file, for_players);
    let t = TeamAward(this);
    compare_award_built(r, this, file, || {
        read_usize(this) == ta::VTABLE && read_u8(this + ta::SEASON_SPAN) == span as u8 && read_u8(this + ta::FORMATION_MODE) == mode as u8
            && read_u8(this + ta::SUBSTITUTES) == 3 && read_i32(this + ta::HISTORY_FIRST) == -1 && read_i32(this + ta::HISTORY_LAST) == -1
            && t.filled() == 0 && (0..20).all(|k| t.slot_id(k) == -1) && read_u32(this + award_base::NATION_ID) == nation
            && read_u32(this + award_base::INDEX) == index
    }, || format!("team_award_construct(nation {}, index {}, span {}, mode {}): vtable {:#x} filled {}", nation, index, span, mode, read_usize(this), t.filled()))
}

/// Runs the award base constructor with the eight arguments the class constructors pass it.
pub(crate) unsafe fn award_construct_base(this: usize, a: [u32; 8]) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_AWARD_AWARD);
    f(this, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]);
}

/// The comparer of a constructor that allocates (and may load): only the original runs; `check` then reads what it
/// built (without a file the plan is known) and `what` names the call.
pub(crate) unsafe fn compare_award_built(r: usize, this: usize, file: usize, check: impl FnOnce() -> bool, what: impl Fn() -> String) -> usize {
    if file != 0 {
        return r;
    }
    let ok = r == this && check();
    SCOREBOARD.record(ok, what);
    r
}

/// `AWARD::AWARD(nation, index, staff comp, national rep, world rep, club comp, file, for players)` (0x412e40,
/// thiscall, ret 0x20): the base fields (`award_base`, `award_fields`).
pub(crate) const FN_AWARD_AWARD: usize = 0x412e40;

/// `TEAM_AWARD::load_from_disk(file)` (0x8af660, thiscall, ret 4).
pub(crate) const FN_TEAM_AWARD_LOAD_FROM_DISK: usize = 0x8af660;

/// Records the team: the 0x205-byte selection record (from +0x4e) goes to the award manager's team history (between
/// the award's first and last offsets, `award_manager_add_team_award_record`); each of the eleven starters (the
/// substitutes are not credited) with a staff id gets the award: his reputations rise (`award_raise_winner_reputation`
/// with the award's national and world reputations and player flag) and his history gets the team of the year entry
/// (`Staff_Records::add_team_of_year_record` with his club, the award's staff competition, the period end year and the
/// season span). Then the human clubs get their news (`TEAM_AWARD::send_club_news`) and the selection is cleared
/// (`team_award_clear_selection`).
/// Defect of the original: a starter's club counts only when its id is above 0, so the players of club 0 get the award
/// without a club (no club in the reputation rise and −1 in the history entry).
/// Original: `TEAM_AWARD::record_winners` (0x8affb0, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_record_winners(this: usize) {
    use team_award_obj as ta;
    let t = TeamAward(this);
    award_manager_add_team_award_record(read_usize(G_AWARD_MANAGER), this + ta::RECORD, this + ta::HISTORY_FIRST, this + ta::HISTORY_LAST);
    for k in 0..11 {
        let id = t.slot_id(k);
        if id < 0 {
            continue;
        }
        covered!("sx05b_1 starter credited");
        let person = staff_at(id as isize);
        let club_id = read_i32(t.slot(k) + 0x10);
        let club = if club_id > 0 { club_at(club_id as isize).addr() } else { 0 };
        if !person.is_null() {
            exe::award_raise_winner_reputation(person, club, read_u16(this + award_fields::NATIONAL_REPUTATION) as u32,
                read_u16(this + award_fields::WORLD_REPUTATION) as u32, read_u32(this + award_fields::FOR_PLAYERS));
        }
        add_team_of_year_record(read_usize(G_STAFF_RECORDS), person.addr(), club, read_usize(this + award_base::STAFF_COMP),
            read_u16(this + ta::PERIOD_END + 2) as u32, read_u8(this + ta::SEASON_SPAN) as u32);
    }
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEAM_AWARD_SEND_CLUB_NEWS);
    news(this);
    team_award_clear_selection(this);
}

/// Tells each human club which of its players made the team: one type 0x1b news item, built once and reused. For each
/// slot whose club id is not −1 and whose club a human runs (`exe::human_manager_is_human_club`(club, 0, 1, 0, 0)):
/// every slot from this one on with the same club is counted (the first three remembered) and loses its club id (−1,
/// so the club is told once); the item gets the award's nation and index (slots 0, 1), the period start and end years
/// (2, 3), the count (4) and the club id (5) and, with one to three players, each player's staff id and three name ids
/// (6 onwards, four slots a player); then it goes to the club (`NewsPost::Club`). With more than three, the slots from 6
/// keep what an earlier club left (the text counts them instead).
/// Original: `TEAM_AWARD::send_club_news` (0x8b0140, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_send_club_news(this: usize) {
    team_award_send_club_news_with(this, &mut |item, to| news_post(item, to));
}

/// Sets the award period to whole years: the start to day −1 of the year minus the season span (+0x25, signed), the
/// end to day −1 of `year` (each built with `Date::Date(day, year)` and copied with `Date::operator=`).
/// Original: `TEAM_AWARD::set_season_dates` (0x8aff50, team_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn team_award_set_season_dates(this: usize, year: u32) {
    use team_award_obj as ta;
    let (mut date, mut out) = ([0u8; 8], [0u8; 8]);
    let start = (year as i32).wrapping_sub(read_i8(this + ta::SEASON_SPAN) as i32) as u32;
    exe::new_date(date.as_mut_ptr() as usize, u32::MAX, start);
    exe::date_operator_3d(this + ta::PERIOD_START, out.as_mut_ptr() as usize, date.as_ptr() as usize);
    exe::new_date(date.as_mut_ptr() as usize, u32::MAX, year);
    exe::date_operator_3d(this + ta::PERIOD_END, out.as_mut_ptr() as usize, date.as_ptr() as usize);
}

/// Comparer for `team_award_record_winners`: the history, the reputations, the staff records and the news cannot be
/// written twice, so only the original runs; then the selection must be cleared and the history last offset moved.
pub(crate) unsafe extern "thiscall" fn compare_team_award_record_winners(this: usize) {
    use team_award_obj as ta;
    let t = TeamAward(this);
    let before = read_u32(this + ta::HISTORY_LAST);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEAM_AWARD_RECORD_WINNERS)(this);
    let ok = t.filled() == 0 && (0..20).all(|k| t.slot_id(k) == -1) && read_u32(this + ta::HISTORY_LAST) != before;
    SCOREBOARD.record(ok, || format!("team_award_record_winners({:#x}): filled {} history last {:#x} -> {:#x}", this, t.filled(), before, read_u32(this + ta::HISTORY_LAST)));
}

/// Comparer for `team_award_send_club_news`: the original posts the news; the copy rehearses it (the same item built,
/// nothing posted) from the same state, side by side over the selection's club ids (the told clubs lose theirs) and
/// the random generator.
pub(crate) unsafe extern "thiscall" fn compare_team_award_send_club_news(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TEAM_AWARD_SEND_CLUB_NEWS);
    let mut regions = vec![(this + team_award_obj::RECORD, 20 * TEAM_AWARD_SLOT_SIZE)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let mut posts = Vec::new();
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&regions, || o(this),
        || crate::rng::without_counting(|| team_award_send_club_news_with(this, &mut |_, to| posts.push(to))));
    SCOREBOARD.record(equal, || format!("team_award_send_club_news({:#x}) rust posts {:x?} {:?}", this, posts, first_differences(&a, &b)));
}

/// Comparer for `team_award_set_season_dates`: side by side over the two period dates.
pub(crate) unsafe extern "thiscall" fn compare_team_award_set_season_dates(this: usize, year: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_TEAM_AWARD_SET_SEASON_DATES);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + team_award_obj::PERIOD_START, 16)], || o(this, year), || team_award_set_season_dates(this, year));
    SCOREBOARD.record(equal, || format!("team_award_set_season_dates(year {}) {:?}", year as u16, first_differences(&a, &b)));
}

/// Writes the headline or the body of the team award news (`team_award_send_club_news`'s item: slots 2 and 3 the
/// period years, 4 the count, 5 the club, 6.. the players' ids and name ids).
///
/// The headline: "<award name>" in the award's template, with the competition's link. The body: the period as "%d"
/// (one year) or "%d/%02d" (two, the second modulo 100); one player "<name> ... <award> ... <period>", two and three
/// players their own templates, any other count the count in words with a capital first letter, the club's short
/// name, the award and the period. Each named player (one to three) whose names still match the item gets his link
/// (0x7d7); with more than three, the club's (short name, 0x7d5); then the competition's (0x7d4). The players' names
/// are built (style 1, 0xc8 bytes each) before the headline test, as the original does.
/// Original: `TEAM_AWARD::write_news` (0x8b0370, team_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn team_award_write_news(this: usize, item: usize, text: usize, body: u32) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    let start = exe::newsitem_get_slot(item, 2);
    let end = exe::newsitem_get_slot(item, 3);
    let count = exe::newsitem_get_slot(item, 4) as u8 as i8;
    let club_id = exe::newsitem_get_slot(item, 5);
    let mut players = [([0u32; 4], [0u8; 0xc8]); 3];
    if count <= 3 {
        let mut s = 6i8;
        for (ids, name) in players.iter_mut().take(count.max(0) as usize) {
            for v in ids.iter_mut() {
                *v = exe::newsitem_get_slot(item, s as u8 as u32);
                s += 1;
            }
            exe::database_format_name_from_ids(ids[1], ids[2], ids[3], name.as_mut_ptr(), 0xc8, 1, ids[0]);
        }
    }
    let (comp_name, g) = award_comp_name(this);
    if body == 0 {
        fmt(buf, 0xa8c8b0, g, g, comp_name); // "{}<%s - Award name>{}"
        exe::text_set_string(text, buf as *const u8);
    } else {
        covered!("sx05b_2 team award body");
        let mut period = [0u8; 0x40];
        let pp = period.as_mut_ptr() as usize;
        if start as u16 != end as u16 {
            comp_crt_format()(pp, 0x988070, start & 0xffff, (end & 0xffff) % 100); // "%d/%02d"
        } else {
            comp_crt_format()(pp, 0x98806c, start & 0xffff); // "%d"
        }
        let name = |k: usize| players[k].1.as_ptr() as usize;
        match count {
            1 => fmt(buf, 0xa8cbcc, g, g, name(0), comp_name, pp),
            2 => fmt(buf, 0xa8cae0, g, g, name(0), name(1), comp_name, pp),
            3 => fmt(buf, 0xa8c9c8, g, g, name(0), name(1), name(2), comp_name, pp),
            _ => {
                covered!("sx05b_2 team award count in words");
                let mut words = [0u8; 0x34];
                exe::utils_number_to_words(count as i32, words.as_mut_ptr(), 0x32, 1);
                words[0] = exe::utils_char_to_upper(read_u32(words.as_ptr() as usize) as i32);
                let club = club_at(club_id as i32 as isize);
                let cg = (club.short_gender_name() as i8) as i32 as u32;
                fmt(buf, 0xa8c8ec, cg, cg, g, g, words.as_ptr() as usize, club.addr() + club::SHORT_NAME, comp_name, pp)
            }
        };
        exe::text_set_string(text, buf as *const u8);
        if count > 3 {
            let club = club_at(club_id as i32 as isize);
            exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        } else {
            for (ids, name) in players.iter().take(count.max(0) as usize) {
                let person = staff_at(ids[0] as i32 as isize);
                if exe::database_staff_matches_names(person, ids[1], ids[2], ids[3], 0usize) != 0 {
                    exe::text_add_link(text, name.as_ptr(), 0x7d7, person);
                }
            }
        }
    }
    let comp = read_usize(this + award_base::STAFF_COMP);
    exe::text_add_link(text, (comp + 4) as *const u8, 0x7d4, comp);
}

/// Copies the three dates of a team award: the period start (+0x242) and end (+0x24a) and the update date (+0x1d),
/// each with `Date::operator=`.
/// Original: `TEAM_AWARD::set_dates` (0x8aff00, team_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn team_award_set_dates(this: usize, start: usize, end: usize, update: usize) {
    use team_award_obj as ta;
    let mut out = [0u8; 8];
    let o = out.as_mut_ptr() as usize;
    exe::date_operator_3d(this + ta::PERIOD_START, o, start);
    exe::date_operator_3d(this + ta::PERIOD_END, o, end);
    exe::date_operator_3d(this + ta::UPDATE_DATE, o, update);
}

/// Comparer for `team_award_write_news`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_team_award_write_news(this: usize, item: usize, text: usize, body: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_TEAM_AWARD_WRITE_NEWS);
    compare_text_writer(text, || o(this, item, text, body), |t| team_award_write_news(this, item, t, body),
        || format!("team_award_write_news(count {}, body {})", exe::newsitem_get_slot(item, 4) as u8 as i8, body));
}

/// Comparer for `team_award_set_dates`: side by side over the three dates.
pub(crate) unsafe extern "thiscall" fn compare_team_award_set_dates(this: usize, start: usize, end: usize, update: usize) {
    use team_award_obj as ta;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_TEAM_AWARD_SET_DATES);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + ta::PERIOD_START, 16), (this + ta::UPDATE_DATE, 8)],
        || o(this, start, end, update), || team_award_set_dates(this, start, end, update));
    SCOREBOARD.record(equal, || format!("team_award_set_dates({:#x}) {:?}", this, first_differences(&a, &b)));
}

/// `team_award_send_club_news` with the posts going to `sink`.
pub(crate) unsafe fn team_award_send_club_news_with(this: usize, sink: NewsSink) {
    use team_award_obj as ta;
    let t = TeamAward(this);
    let mut item = [0u8; NEWS_ITEM_BYTES];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x1b, 0);
    let mut i = 0i8;
    while (i as i32) < t.slot_count() as i32 {
        let club_id = read_i32(t.slot(i as usize) + 0x10);
        let club = club_at(club_id as isize);
        if club_id > -1 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
            covered!("sx05b_1 human club told");
            let (mut count, mut picked) = (0i8, [0usize; 3]);
            for j in i as usize..t.slot_count().max(0) as usize {
                if read_i32(t.slot(j) + 0x10) == club_id {
                    if count < 3 {
                        picked[count as usize] = j;
                    }
                    count = count.wrapping_add(1);
                    write_i32(t.slot(j) + 0x10, -1);
                }
            }
            exe::newsitem_set_slot(p, 0, read_u32(this + award_base::NATION_ID));
            exe::newsitem_set_slot(p, 1, read_u32(this + award_base::INDEX));
            exe::newsitem_set_slot(p, 2, read_u16(this + ta::PERIOD_START + 2) as u32);
            exe::newsitem_set_slot(p, 3, read_u16(this + ta::PERIOD_END + 2) as u32);
            exe::newsitem_set_slot(p, 4, count as i32 as u32);
            exe::newsitem_set_slot(p, 5, club_id as u32);
            if count <= 3 {
                let mut s = 6i8;
                for &k in &picked[..count.max(0) as usize] {
                    for f in [0, 4, 8, 0xc] {
                        exe::newsitem_set_slot(p, s, read_u32(t.slot(k) + f));
                        s += 1;
                    }
                }
            }
            sink(p, NewsPost::Club(club.addr()));
        }
        i = i.wrapping_add(1);
    }
}

/// The award's staff competition record: its name (+4) and the gender byte of the name (+0x37, sign-extended, as the
/// texts pass it).
pub(crate) unsafe fn award_comp_name(award: usize) -> (usize, u32) {
    let comp = read_usize(award + award_base::STAFF_COMP);
    (comp + 4, read_i8(comp + 0x37) as i32 as u32)
}

/// `TEAM_AWARD::send_club_news()` (0x8b0140, thiscall), called by `team_award_record_winners` through the hook.
pub(crate) const FN_TEAM_AWARD_SEND_CLUB_NEWS: usize = 0x8b0140;
