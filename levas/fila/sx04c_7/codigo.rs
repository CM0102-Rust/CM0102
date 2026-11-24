// ---------- sx04c_7: the Norwegian awards: the object, its eleven awards and the season's end (award/norway_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\norway_awards.cpp" (the error boxes).
const NORWAY_AWARDS_CPP: usize = 0xa69838;
/// `NorwayAwards::init_awards(file)` (0x793a80, thiscall, ret 4), through its hook.
const FN_NORWAY_AWARDS_INIT: usize = 0x793a80;
/// The data version from which the Norwegian month awards are in the save.
const NORWAY_MONTH_AWARDS_SAVED: i32 = 0x3e007;
/// The data version from which the save no longer holds the two placeholder year awards before each division's month
/// awards.
const NORWAY_PLACEHOLDERS_DROPPED: i32 = 0x3e009;
/// An award's virtual deleting destructor (slot 0: thiscall(flags)).
const AWARD_DELETE_SLOT: usize = 0x0;

/// The Norwegian awards' class: the vtable 0x96dfd0, Norway (`KEY_NATION_NORWAY`), nation id 0xf, type 1, with month
/// ratings (the error box at line 0x50 when they cannot be built).
const NORWAY_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96dfd0, nation_key: Some(KEY_NATION_NORWAY), nation_id: 0xf, kind: 1,
    month_ratings: Some((NORWAY_AWARDS_CPP, 0x50)) };

/// Builds the Norwegian awards (`nation_awards_construct` with `NORWAY_AWARDS`): the saved awards with a file, else the
/// month ratings and `NorwayAwards::init_awards(file)`. Returns this.
/// Original: `NorwayAwards::NorwayAwards` (0x793940, norway_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn norway_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &NORWAY_AWARDS, file, core::mem::transmute(FN_NORWAY_AWARDS_INIT))
}

/// Whether a save before data version 0x3e009 still holds the two placeholder year awards before each division's
/// month awards: only with the month awards' file (`awards_file_from_version` with 0x3e007).
unsafe fn norway_save_has_placeholders(file: usize) -> bool {
    read_i32(G_GAME_DATA_VERSION).wrapping_abs() < NORWAY_PLACEHOLDERS_DROPPED && awards_file_from_version(file, NORWAY_MONTH_AWARDS_SAVED) != 0
}

/// Reads one placeholder year award (no competition, reputations 0) from the save and deletes it at once (its virtual
/// deleting destructor with flags 1).
unsafe fn norway_skip_placeholder(n: u32, index: u32, file: u32) {
    let award = award_new(&AwardSpec::Year([n, index, 0, 0, 0, u32::MAX, 0, 0, 0, file, 1]));
    if award != 0 {
        let delete: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(award) + AWARD_DELETE_SLOT));
        delete(award, 1);
    }
}

/// Builds the eleven Norwegian awards (`nation_awards_fields::COUNT` 11, the table by `exe::crt_malloc`; out of memory
/// the error box at line 0x7a and nothing more), each built in its slot (`award_new`): the player (national and world
/// reputations 6000/2250), the goalkeeper (4000/1000), the defender (4000/1000), the midfielder (4000/1500) and the
/// attacker of the year (4000/1750), no competition, kind 0; the Premier Division team of the week (team award,
/// 3000/500, span 0, mode 1); its manager and player of the month (month awards from April to October of the current
/// year, 3000/1500 each); the First Division team of the week (1000/100); its manager (1500/750) and player of the month
/// (1000/500). The month awards are loaded from the file only when the save's data version reaches 0x3e007
/// (`awards_file_from_version`). A save of that version but before 0x3e009 holds two placeholder year awards before
/// each division's month awards: they are read and deleted (`norway_skip_placeholder`), the month awards are then
/// created without the file, and the awards' indices run on past the placeholders (8, 9, 10, 13, 14 instead of 6 to
/// 10).
/// Original: `NorwayAwards::init_awards` (0x793a80, norway_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn norway_awards_init(this: usize, file: usize) {
    use nation_awards_fields as na;
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + na::NATION_ID);
    write_i16(this + na::COUNT, 11);
    let table = exe::crt_malloc(0x2c);
    write_usize(this + na::TABLE, table);
    if table == 0 {
        show_error_at(NORWAY_AWARDS_CPP, 0x7a);
        return;
    }
    let f = file as u32;
    let fg = awards_file_from_version(file, NORWAY_MONTH_AWARDS_SAVED) as u32;
    let k = |key: usize| read_u32(key);
    let (prem, first) = (k(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION), k(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION));
    let put = |slot: usize, spec: AwardSpec| write_usize(table + 4 * slot, award_new(&spec));
    let year = |i: u32, key: usize, national: u32, world: u32| AwardSpec::Year([n, i, k(key), national, world, u32::MAX, 0, 0, 0, f, 1]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32, file: u32, players: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 3, y, 9, y, file, players]);
    put(0, year(0, KEY_STAFF_COMP_NORWEGIAN_PLAYER_OF_THE_YEAR, 6000, 2250));
    put(1, year(1, KEY_STAFF_COMP_NORWEGIAN_GOALKEEPER_OF_THE_YEAR, 4000, 1000));
    put(2, year(2, KEY_STAFF_COMP_NORWEGIAN_DEFENDER_OF_THE_YEAR, 4000, 1000));
    put(3, year(3, KEY_STAFF_COMP_NORWEGIAN_MIDFIELDER_OF_THE_YEAR, 4000, 1500));
    put(4, year(4, KEY_STAFF_COMP_NORWEGIAN_ATTACKER_OF_THE_YEAR, 4000, 1750));
    put(5, AwardSpec::Team([n, 5, k(KEY_STAFF_COMP_NORWEGIAN_PREMIER_DIVISION_TEAM_OF_THE_W), 3000, 500, prem, 0, 1, f, 1]));
    let placeholders = norway_save_has_placeholders(file);
    // a division's two month awards, after its placeholders in an old save; returns the next index
    let division = |slot: usize, mut index: u32, manager: (usize, u32, u32), player: (usize, u32, u32), comp: u32| -> u32 {
        let month_file = if placeholders {
            covered!("sx04c_7 placeholders skipped");
            norway_skip_placeholder(n, index, fg);
            norway_skip_placeholder(n, index + 1, fg);
            index += 2;
            0
        } else {
            fg
        };
        put(slot, month(index, manager.0, manager.1, manager.2, comp, month_file, 0));
        put(slot + 1, month(index + 1, player.0, player.1, player.2, comp, month_file, 1));
        index + 2
    };
    let index = division(6, 6, (KEY_STAFF_COMP_NORWEGIAN_PREMIER_DIVISION_MANAGER_OF_TH, 3000, 1500), (KEY_STAFF_COMP_NORWEGIAN_PREMIER_DIVISION_PLAYER_OF_THE, 3000, 1500), prem);
    put(8, AwardSpec::Team([n, index, k(KEY_STAFF_COMP_NORWEGIAN_FIRST_DIVISION_TEAM_OF_THE_WEE), 1000, 100, first, 0, 1, f, 1]));
    division(9, index + 1, (KEY_STAFF_COMP_NORWEGIAN_FIRST_DIVISION_MANAGER_OF_THE, 1500, 750), (KEY_STAFF_COMP_NORWEGIAN_FIRST_DIVISION_PLAYER_OF_THE_M, 1000, 500), first);
    covered!("sx04c_7 norwegian awards built");
}

/// The Norwegian awards' day: on 20 November (`awards_day_is`), the player, goalkeeper, defender, midfielder and attacker
/// of the year from the year ratings' top three (`award_top_three_from_ratings`: Norwegian players, position masks 0,
/// 1, 6, 0x18 and 0x40, mode 0x11). Every day, on the nation's season end day (`is_season_end_day`), the four month
/// awards move on to the next year (`month_award_start_new_season` with today's year + 1); then the base daily update.
/// Original: `NorwayAwards::daily_update` (0x7942c0, norway_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn norway_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 0xa) {
        covered!("sx04c_7 norwegian awards day");
        let norway = read_i32(KEY_NATION_NORWAY);
        for (k, mask) in [(0, 0), (1, 1), (2, 6), (3, 0x18), (4, 0x40)] {
            award_top_three_from_ratings(this, today, nation_award(this, k), [-1, -1, -1, norway, -1, mask, 0x11]);
        }
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_7 month awards move on");
        for k in [6, 7, 9, 10] {
            month_award_start_new_season(nation_award(this, k), read_u16(today + cm_date::YEAR).wrapping_add(1));
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_norway_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &NORWAY_AWARDS, 11, "norway_awards_construct")
}

/// Comparer for `norway_awards_init`: only the original runs (it allocates); the awards' nation and index are checked
/// unless an old save's placeholders shifted the indices.
pub(crate) unsafe extern "thiscall" fn compare_norway_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    if norway_save_has_placeholders(file) {
        return o(this, file);
    }
    compare_nation_awards_init(this, || o(this, file), "norway_awards_init");
}

/// Comparer for `norway_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_norway_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
