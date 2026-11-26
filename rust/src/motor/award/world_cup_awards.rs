use super::*;

/// Builds the World Cup awards (`nation_awards_construct` with `WORLD_CUP_AWARDS`): the saved awards with a file, else
/// `WorldCupAwards::init_awards(file)`. Returns this.
/// Original: `WorldCupAwards::WorldCupAwards` (0x92fc10, world_cup_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &WORLD_CUP_AWARDS, file, core::mem::transmute(FN_WORLD_CUP_AWARDS_INIT))
}

/// Builds the two World Cup awards (`nation_awards_build`, the error box at line 0x5d), both of the FIFA World Cup: the
/// golden boot (a year award, national and world reputations 9000 and 7000, option 1) and the dream team (a team award,
/// 8500 and 6750); kind 0, no scope.
/// Original: `WorldCupAwards::init_awards` (0x92fc80, world_cup_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let cup = read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    let specs = [
        AwardSpec::Year([n, 0, read_u32(KEY_STAFF_COMP_WORLD_CUP_GOLDEN_BOOT), 9000, 7000, cup, 0, 0, 1, f, 1]),
        AwardSpec::Team([n, 1, read_u32(KEY_STAFF_COMP_WORLD_CUP_DREAM_TEAM), 8500, 6750, cup, 0, 0, f, 1]),
    ];
    covered!("sx05b_20 world cup awards built");
    nation_awards_build(this, &specs, WORLD_CUP_AWARDS_CPP, 0x5d);
}

/// The World Cup awards' day: in a World Cup year ((year − 1998) divisible by 4), on 13 July (`Date::Date(13, 6, year,
/// 0)`, compared by day and year), the golden boot (the competition's best three by statistic 2) and the dream team.
/// There is no base daily update (the World Cup awards have no month awards).
/// Original: `WorldCupAwards::daily_update` (0x92fdf0, world_cup_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_awards_daily_update(this: usize, today: usize) {
    let year = read_u16(today + cm_date::YEAR);
    if (year as i32 - 0x7ce) % 4 != 0 {
        return;
    }
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 0xd, 6, year as u32, 0);
    if d[1] != year || d[0] != read_u16(today) {
        return;
    }
    covered!("sx05b_20 world cup awards day");
    let cup = read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    let player: unsafe extern "thiscall" fn(usize, u32, usize, usize, i32, u32, u32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TOURNAMENT_PLAYER);
    player(this, cup, today, nation_award(this, 0), -1, 0, 2);
    let team: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TOURNAMENT_TEAM);
    team(this, read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP), nation_award(this, 1));
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_WORLD_CUP_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &WORLD_CUP_AWARDS, 2, "world_cup_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_WORLD_CUP_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "world_cup_awards_init");
}

/// Comparer for `world_cup_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_world_cup_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_WORLD_CUP_AWARDS_DAILY_UPDATE)(this, today)
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

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\world_cup_awards.cpp" (the error box).
pub(crate) const WORLD_CUP_AWARDS_CPP: usize = 0xad58c4;

/// `WorldCupAwards::init_awards(file)` (0x92fc80, thiscall, ret 4), through its hook.
pub(crate) const FN_WORLD_CUP_AWARDS_INIT: usize = 0x92fc80;

/// `nation_awards_award_tournament_player(comp, today, award, max age, positions, stat)` (0x7690c0, ret 0x18) and
/// `NationAwards::award_tournament_team(comp, award)` (0x769af0, thiscall, ret 8): the best three players and the team
/// of a competition's statistics, crediting the national teams.
pub(crate) const FN_NATION_AWARDS_AWARD_TOURNAMENT_PLAYER: usize = 0x7690c0;

pub(crate) const FN_NATION_AWARDS_AWARD_TOURNAMENT_TEAM: usize = 0x769af0;

/// The World Cup awards' class: the vtable 0x9713a4, no nation record, nation id 0, type 4 (international).
pub(crate) const WORLD_CUP_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x9713a4, nation_key: None, nation_id: 0, kind: 4, month_ratings: None };
