// ---------- sx05b_5: the team award, year award and year ratings constructors; the award builders the nations share ----------

/// The award base fields every award class shares (`AWARD`, award.cpp): +4 the award type byte (0 team, 1 year, 2
/// month), +0xd the nation id and +0x11 the award's index in its nation (the news slots 0 and 1); the rest is
/// `award_fields`.
pub(crate) mod award_base {
    pub(crate) const TYPE: usize = 0x4;
    pub(crate) const STAFF_COMP: usize = 0x5;
    pub(crate) const CLUB_COMP: usize = 0x9;
    pub(crate) const NATION_ID: usize = 0xd;
    pub(crate) const INDEX: usize = 0x11;
}

/// The year award (`YEAR_AWARD`, 0x2b bytes): the record kind byte Staff_Records keeps (+0x1d), a second option byte
/// (+0x1e) [inference], the owned shortlist (+0x1f, or 0) and its first and last record offsets in the award manager's
/// year history (+0x23, +0x27; −1 none).
pub(crate) mod year_award_obj {
    pub(crate) const SIZE: u32 = 0x2b;
    pub(crate) const VTABLE: usize = 0x9713b8;
    pub(crate) const KIND: usize = 0x1d;
    pub(crate) const OPTION: usize = 0x1e;
    pub(crate) const SHORTLIST: usize = 0x1f;
    pub(crate) const HISTORY_FIRST: usize = 0x23;
    pub(crate) const HISTORY_LAST: usize = 0x27;
}

/// The team award (`TEAM_AWARD`, 0x25d bytes; the selection itself is `TeamAward`): the update date (+0x1d), the
/// season span in years (+0x25), the period start and end dates (+0x242, +0x24a, inside the 0x205-byte record from
/// +0x4e the history keeps; the filled count +0x252 is `TeamAward::filled`), the substitutes count (+0x253), the
/// history offsets (+0x254, +0x258) and the formation mode (+0x25c).
pub(crate) mod team_award_obj {
    pub(crate) const SIZE: u32 = 0x25d;
    pub(crate) const VTABLE: usize = 0x9703f0;
    pub(crate) const UPDATE_DATE: usize = 0x1d;
    pub(crate) const SEASON_SPAN: usize = 0x25;
    pub(crate) const RECORD: usize = 0x4e;
    pub(crate) const RECORD_SIZE: u32 = 0x205;
    pub(crate) const PERIOD_START: usize = 0x242;
    pub(crate) const PERIOD_END: usize = 0x24a;
    pub(crate) const SUBSTITUTES: usize = 0x253;
    pub(crate) const HISTORY_FIRST: usize = 0x254;
    pub(crate) const HISTORY_LAST: usize = 0x258;
    pub(crate) const FORMATION_MODE: usize = 0x25c;
}

/// `AWARD::AWARD(nation, index, staff comp, national rep, world rep, club comp, file, for players)` (0x412e40,
/// thiscall, ret 0x20): the base fields (`award_base`, `award_fields`).
const FN_AWARD_AWARD: usize = 0x412e40;
/// `YEAR_AWARD::load_from_disk(scope, file)` (0x92fff0, thiscall, ret 8): 1 when read.
const FN_YEAR_AWARD_LOAD_FROM_DISK: usize = 0x92fff0;
/// `TEAM_AWARD::load_from_disk(file)` (0x8af660, thiscall, ret 4).
const FN_TEAM_AWARD_LOAD_FROM_DISK: usize = 0x8af660;
/// `YEAR_RATINGS::load_from_disk(file)` (0x931390, thiscall, ret 4).
const FN_YEAR_RATINGS_LOAD_FROM_DISK: usize = 0x931390;
/// The year award constructor (0x92fe80, thiscall, ret 0x2c) and the team award one (0x8af520, ret 0x28), as the
/// nations' `init_awards` call them (through the hooks).
const FN_YEAR_AWARD_CONSTRUCT: usize = 0x92fe80;
const FN_TEAM_AWARD_CONSTRUCT: usize = 0x8af520;
/// `MONTH_AWARD::MONTH_AWARD` (0x766e10, thiscall, ret 0x30; 12 arguments), 0xa2 bytes.
const FN_MONTH_AWARD_CONSTRUCT: usize = 0x766e10;
const MONTH_AWARD_SIZE: u32 = 0xa2;

/// Runs the award base constructor with the eight arguments the class constructors pass it.
unsafe fn award_construct_base(this: usize, a: [u32; 8]) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_AWARD_AWARD);
    f(this, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]);
}

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

/// Builds a year award: the award base (with the file and the player flag as its last two arguments), type 1, the
/// kind (`kind`) and option (`option`) bytes, no shortlist and no history (−1, −1). With a file,
/// `YEAR_AWARD::load_from_disk(scope, file)`; when there is none or the load fails, a scope gets a new shortlist
/// (`construct_award_shortlist`(scope, no file) in 0x40 bytes of `exe::crt_operator_new`; 0 when out of memory).
/// Returns this.
/// Original: `YEAR_AWARD::YEAR_AWARD` (0x92fe80, year_award.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn year_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, kind: u32, scope: usize, option: u32, file: usize, for_players: u32) -> usize {
    use year_award_obj as ya;
    award_construct_base(this, [nation, index, staff_comp, national_rep, world_rep, club_comp, file as u32, for_players]);
    write_usize(this, ya::VTABLE);
    write_usize(this + ya::SHORTLIST, 0);
    write_u8(this + ya::KIND, kind as u8);
    write_u8(this + ya::OPTION, option as u8);
    write_i32(this + ya::HISTORY_FIRST, -1);
    write_i32(this + ya::HISTORY_LAST, -1);
    write_u8(this + award_base::TYPE, 1);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_YEAR_AWARD_LOAD_FROM_DISK);
        if load(this, scope, file) != 0 {
            return this;
        }
    }
    if scope != 0 {
        covered!("sx05b_5 year award shortlist");
        let p = exe::crt_operator_new(0x40);
        write_usize(this + ya::SHORTLIST, if p != 0 { construct_award_shortlist(p, scope, 0) } else { 0 });
    }
    this
}

/// Builds an empty year ratings table for `year` (no entries, count and capacity 0, the year word at +0xc) and, with
/// a file, loads it (`YEAR_RATINGS::load_from_disk`, its result not read). Returns this.
/// Original: `YEAR_RATINGS::YEAR_RATINGS` (0x931360, year_ratings.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn year_ratings_construct(this: usize, year: u32, file: usize) -> usize {
    write_u16(this + 0xc, year as u16);
    write_usize(this, 0);
    write_u32(this + 4, 0);
    write_u32(this + 8, 0);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_YEAR_RATINGS_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// One award a nation builds in its `init_awards`: the constructor's arguments in order (the nation id and the
/// award's index first, the file second to last).
pub(crate) enum AwardSpec {
    Year([u32; 11]),
    Team([u32; 10]),
    Month([u32; 12]),
}

/// A new award object from its spec (`exe::crt_operator_new` of the class size, then its constructor through the
/// game's address); 0 when out of memory, as the original leaves it.
pub(crate) unsafe fn award_new(spec: &AwardSpec) -> usize {
    let size = match spec {
        AwardSpec::Year(_) => year_award_obj::SIZE,
        AwardSpec::Team(_) => team_award_obj::SIZE,
        AwardSpec::Month(_) => MONTH_AWARD_SIZE,
    };
    let p = exe::crt_operator_new(size);
    if p == 0 {
        return 0;
    }
    match spec {
        AwardSpec::Year(a) => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_YEAR_AWARD_CONSTRUCT);
            f(p, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10])
        }
        AwardSpec::Team(a) => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_TEAM_AWARD_CONSTRUCT);
            f(p, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9])
        }
        AwardSpec::Month(a) => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_MONTH_AWARD_CONSTRUCT);
            f(p, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11])
        }
    }
}

/// A nation's awards object (`NationAwards`): the vtable, the award table (+4) and count (+8, i16), the month ratings
/// (+0xa), the type byte (+0x12), the nation record (+0xe) and the nation id the awards carry (+0x13).
pub(crate) mod nation_awards_fields {
    pub(crate) const TABLE: usize = 0x4;
    pub(crate) const COUNT: usize = 0x8;
    pub(crate) const MONTH_RATINGS: usize = 0xa;
    pub(crate) const NATION: usize = 0xe;
    pub(crate) const TYPE: usize = 0x12;
    pub(crate) const NATION_ID: usize = 0x13;
}

/// `NationAwards::NationAwards` (0x768290, thiscall), `NationAwards::load_from_disk(file)` (0x76a280, thiscall, ret
/// 4: nonzero when read) and `MONTH_RATINGS::MONTH_RATINGS(file)` (0x767af0, thiscall, ret 4; 0xf bytes).
const FN_NATION_AWARDS_CONSTRUCT: usize = 0x768290;
const FN_NATION_AWARDS_LOAD_FROM_DISK: usize = 0x76a280;
const FN_MONTH_RATINGS_CONSTRUCT: usize = 0x767af0;

/// Fills a nation's award table: the count, the table (4 bytes an award, `exe::crt_malloc`; out of memory the error box
/// of `cpp` at `line` and nothing more) and each award built from its spec in order.
pub(crate) unsafe fn nation_awards_build(this: usize, specs: &[AwardSpec], cpp: usize, line: u32) {
    write_i16(this + nation_awards_fields::COUNT, specs.len() as i16);
    let table = exe::crt_malloc(4 * specs.len() as u32);
    write_usize(this + nation_awards_fields::TABLE, table);
    if table == 0 {
        show_error_at(cpp, line);
        return;
    }
    for (k, spec) in specs.iter().enumerate() {
        write_usize(table + 4 * k, award_new(spec));
    }
}

/// How a nation's awards object starts: its vtable, the key of its nation (none for the World Cup), the nation id, the
/// type byte, and the month ratings it needs (with the error line when they cannot be built).
pub(crate) struct NationAwardsClass {
    pub(crate) vtable: usize,
    pub(crate) nation_key: Option<usize>,
    pub(crate) nation_id: u32,
    pub(crate) kind: u8,
    pub(crate) month_ratings: Option<(usize, u32)>,
}

/// The shared constructor of the nations' awards: the `NationAwards` base, the class's vtable, the nation record
/// (`G_NATION_TABLE` at the nation key's index, or 0) and id, the type byte; with a file, the saved awards
/// (`NationAwards::load_from_disk`) and nothing more when they load. Otherwise, a class with month ratings builds them
/// (0xf bytes, `MONTH_RATINGS::MONTH_RATINGS(0)`; out of memory, the box and no awards) and then `init(this, file)`.
pub(crate) unsafe fn nation_awards_construct(this: usize, class: &NationAwardsClass, file: usize, init: unsafe extern "thiscall" fn(usize, usize)) -> usize {
    use nation_awards_fields as na;
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_NATION_AWARDS_CONSTRUCT);
    base(this);
    write_usize(this, class.vtable);
    let nation = match class.nation_key {
        Some(key) => read_usize(G_NATION_TABLE) + read_u32(key) as usize * nation::SIZE,
        None => 0,
    };
    write_usize(this + na::NATION, nation);
    write_u32(this + na::NATION_ID, class.nation_id);
    write_u8(this + na::TYPE, class.kind);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATION_AWARDS_LOAD_FROM_DISK);
        if load(this, file) != 0 {
            return this;
        }
    }
    if let Some((cpp, line)) = class.month_ratings {
        let p = exe::crt_operator_new(0xf);
        let ratings = if p != 0 {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_MONTH_RATINGS_CONSTRUCT);
            f(p, 0)
        } else {
            0
        };
        write_usize(this + na::MONTH_RATINGS, ratings);
        if ratings == 0 {
            show_error_at(cpp, line);
            return this;
        }
    }
    covered!("sx05b_5 nation awards created");
    init(this, file);
    this
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

pub(crate) unsafe extern "thiscall" fn compare_team_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, span: u32, mode: u32, file: usize, for_players: u32) -> usize {
    use team_award_obj as ta;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> usize>(%%I0%%);
    let r = o(this, nation, index, staff_comp, national_rep, world_rep, club_comp, span, mode, file, for_players);
    let t = TeamAward(this);
    compare_award_built(r, this, file, || {
        read_usize(this) == ta::VTABLE && read_u8(this + ta::SEASON_SPAN) == span as u8 && read_u8(this + ta::FORMATION_MODE) == mode as u8
            && read_u8(this + ta::SUBSTITUTES) == 3 && read_i32(this + ta::HISTORY_FIRST) == -1 && read_i32(this + ta::HISTORY_LAST) == -1
            && t.filled() == 0 && (0..20).all(|k| t.slot_id(k) == -1) && read_u32(this + award_base::NATION_ID) == nation
            && read_u32(this + award_base::INDEX) == index
    }, || format!("team_award_construct(nation {}, index {}, span {}, mode {}): vtable {:#x} filled {}", nation, index, span, mode, read_usize(this), t.filled()))
}

pub(crate) unsafe extern "thiscall" fn compare_year_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, kind: u32, scope: usize, option: u32, file: usize, for_players: u32) -> usize {
    use year_award_obj as ya;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, u32, usize, u32) -> usize>(%%I1%%);
    let r = o(this, nation, index, staff_comp, national_rep, world_rep, club_comp, kind, scope, option, file, for_players);
    compare_award_built(r, this, file, || {
        read_usize(this) == ya::VTABLE && read_u8(this + ya::KIND) == kind as u8 && read_u8(this + ya::OPTION) == option as u8
            && read_i32(this + ya::HISTORY_FIRST) == -1 && read_i32(this + ya::HISTORY_LAST) == -1 && read_u8(this + award_base::TYPE) == 1
            && (read_usize(this + ya::SHORTLIST) != 0) == (scope != 0)
    }, || format!("year_award_construct(nation {}, index {}, kind {}, scope {:#x}): vtable {:#x} shortlist {:#x}", nation, index, kind, scope,
        read_usize(this), read_usize(this + ya::SHORTLIST)))
}

/// Comparer for `year_ratings_construct`: side by side over the 0xe bytes of the table without a file; with one, only
/// the original (it loads).
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_construct(this: usize, year: u32, file: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    if file != 0 {
        return o(this, year, file);
    }
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(this, 0xe)], || o(this, year, file), || year_ratings_construct(this, year, file));
    SCOREBOARD.record(equal && r == v, || format!("year_ratings_construct(year {}) {:?}", year as u16, first_differences(&a, &b)));
    r
}

/// Where an award or AWOL news item goes: one club's managers, a division's, a nation's, a continent's, or every
/// manager (`News::send_to_*` on `G_NEWS`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum NewsPost {
    Club(usize),
    Division(usize),
    Nation(usize),
    Continent(usize),
    All,
}

/// What a news routine does with its items: post them in the game, or (a comparer's rehearsal) only note where.
pub(crate) type NewsSink<'a> = &'a mut dyn FnMut(usize, NewsPost);

/// The game's sink: posts `item` (`exe::news_send_to_*`, thiscall on `G_NEWS`).
pub(crate) unsafe fn news_post(item: usize, to: NewsPost) {
    let news = read_usize(G_NEWS);
    match to {
        NewsPost::Club(c) => exe::news_send_to_club_manager(news, item, c),
        NewsPost::Division(d) => exe::news_send_to_division_managers(news, item, d),
        NewsPost::Nation(n) => exe::news_send_to_nation_managers(news, item, n),
        NewsPost::Continent(c) => exe::news_send_to_continent_managers(news, item, c),
        NewsPost::All => exe::news_send_to_all_managers(news, item),
    };
}

/// The comparer of a routine that posts news (it cannot run twice: the inbox allocates): the original runs with its
/// random draws traced, then `rehearsal` (the copy, building the same items without posting them, see `NewsSink`)
/// from the same generator state, without counting; the rehearsal's draws must be the start of the original's (the
/// posting may draw more). Returns whether they are and the two draw lists, for the message.
pub(crate) unsafe fn rehearse_news(original: impl FnOnce(), rehearsal: impl FnOnce()) -> (bool, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let (mut from_original, mut from_copy) = (Vec::new(), Vec::new());
    crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || from_original = crate::rng::trace(original).1,
        || from_copy = crate::rng::trace(|| crate::rng::without_counting(rehearsal)).1);
    (from_original.starts_with(&from_copy), from_original, from_copy)
}

/// The `NationAwards` routines the nations' `daily_update` call by address that the England item (sx03_2) has no
/// helper for: the player of the year from the year ratings (today, award, ratings, then seven filters; ret 0x28), the
/// top three from the ratings (today, award, then seven filters ending in the position mask and the mode; ret 0x24)
/// and the base daily update (today; ret 4).
pub(crate) const FN_NATION_AWARDS_PLAYER_OF_YEAR_FROM_RATINGS: usize = 0x7683c0;
pub(crate) const FN_NATION_AWARDS_TOP_THREE_FROM_RATINGS: usize = 0x769380;
pub(crate) const FN_NATION_AWARDS_BASE_DAILY_UPDATE: usize = 0x768310;
/// The award manager's year ratings table (`YEAR_RATINGS`, +0xc of `G_AWARD_MANAGER`) [inference].
pub(crate) const AWARD_MANAGER_YEAR_RATINGS: usize = 0xc;

/// The player of the year of `award` from the year ratings (`NationAwards::award_player_of_year_from_ratings`) with
/// the award manager's ratings table and the seven filters as the nation passes them.
pub(crate) unsafe fn award_player_of_year_from_ratings(this: usize, today: usize, award: usize, filters: [i32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_PLAYER_OF_YEAR_FROM_RATINGS);
    let [a, b, c, d, e, g, h] = filters;
    f(this, today, award, read_usize(read_usize(G_AWARD_MANAGER) + AWARD_MANAGER_YEAR_RATINGS), a, b, c, d, e, g, h)
}

/// The top three of the year ratings for `award` (`nation_awards_award_top_three_from_ratings`) with the seven
/// filters as the nation passes them.
pub(crate) unsafe fn award_top_three_from_ratings(this: usize, today: usize, award: usize, filters: [i32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_TOP_THREE_FROM_RATINGS);
    let [a, b, c, d, e, g, h] = filters;
    f(this, today, award, a, b, c, d, e, g, h)
}

/// The base daily update of a nation's awards (`NationAwards::daily_update`: the month change).
pub(crate) unsafe fn nation_awards_base_daily_update(this: usize, today: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATION_AWARDS_BASE_DAILY_UPDATE);
    f(this, today)
}

/// The day a nation's month award moves on to the next season: today's day word (signed) is the season end day of the
/// nation's key nation record (`key_nation_record::SEASON_END_DAY` of `G_LEAGUE_TABLE` at
/// `exe::key_nation_index_of_nation`).
pub(crate) unsafe fn is_season_end_day(nation: usize, today: usize) -> bool {
    let k = exe::key_nation_index_of_nation(nation) as isize;
    read_i16(today) as i32 == read_u16((G_LEAGUE_TABLE as isize + k * 0x48) as usize + key_nation_record::SEASON_END_DAY) as i32
}

/// The comparer of a nation awards constructor: it allocates (and may load); only the original runs, then the object
/// must carry the class's vtable, nation id and type, and (without a file) `count` awards.
pub(crate) unsafe fn compare_nation_awards_built(r: usize, this: usize, file: usize, class: &NationAwardsClass, count: i16, what: &str) -> usize {
    use nation_awards_fields as na;
    let ok = r == this && read_usize(this) == class.vtable && read_u32(this + na::NATION_ID) == class.nation_id && read_u8(this + na::TYPE) == class.kind
        && (file != 0 || read_usize(this + na::TABLE) == 0 || read_i16(this + na::COUNT) == count);
    SCOREBOARD.record(ok, || format!("{}: vtable {:#x} nation id {:#x} type {} count {}", what, read_usize(this), read_u32(this + na::NATION_ID), read_u8(this + na::TYPE), read_i16(this + na::COUNT)));
    r
}

/// The comparer of a nation's `init_awards`: it allocates the awards; only the original runs, then each award must be
/// built for this nation in its slot (`award_base::NATION_ID`, `award_base::INDEX`) with its staff competition.
pub(crate) unsafe fn compare_nation_awards_init(this: usize, original: impl FnOnce(), what: &str) {
    use nation_awards_fields as na;
    original();
    let table = read_usize(this + na::TABLE);
    if table == 0 {
        return;
    }
    let n = read_i16(this + na::COUNT).max(0) as usize;
    let bad: Vec<usize> = (0..n).filter(|&k| {
        let a = read_usize(table + 4 * k);
        a != 0 && (read_u32(a + award_base::NATION_ID) != read_u32(this + na::NATION_ID) || read_u32(a + award_base::INDEX) != k as u32)
    }).collect();
    SCOREBOARD.record(bad.is_empty(), || format!("{}: awards with the wrong nation or index {:?}", what, bad));
}

/// The shared body of the nations' `update_team_of_the_week`: when `award` is one of the table's `slots`, the team of
/// the week (`build_team_of_the_week`).
pub(crate) unsafe fn nation_awards_team_of_the_week_of(this: usize, award: usize, slots: &[usize]) {
    if slots.iter().any(|&k| nation_award(this, k) == award) {
        covered!("sx05b_5 team of the week");
        build_team_of_the_week(this, award);
    }
}
