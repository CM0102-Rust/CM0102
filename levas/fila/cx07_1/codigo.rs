// ---------- cx07_1: the destructors of the Copa América, the Oceania Nations Cup, the Olympic football and the League base, the League base virtuals and the display copy of a league ----------

/// The vtables the national-team tournament destructors write back.
const GOLDCUP_VTABLE: usize = 0x96b400;
const OCEANIA_NATIONS_VTABLE: usize = 0x96e0cc;
const OLYMPICS_VTABLE: usize = 0x96e1d0;

/// Destroys the Copa América (`drop_nations_tournament` of cx05_1 with its vtable).
/// Original: `GoldCup::~GoldCup` (0x5e0840, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_goldcup(this: usize) {
    covered!("cx07_1 copa america destroyed");
    drop_nations_tournament(League(this), GOLDCUP_VTABLE);
}

/// The scalar deleting destructor of the Copa América: `GoldCup::~GoldCup` and, with bit 0 of `flags`, the object
/// (`delete_through` of cx03_3).
/// Original: `GoldCup::scalar_deleting_destructor` (0x5e0820, goldcup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_goldcup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5e0840)
}

/// Destroys the Oceania Nations Cup (`drop_nations_tournament` of cx05_1 with its vtable).
/// Original: `OceaniaNations::~OceaniaNations` (0x798aa0, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_oceania_nations(this: usize) {
    covered!("cx07_1 oceania nations cup destroyed");
    drop_nations_tournament(League(this), OCEANIA_NATIONS_VTABLE);
}

/// The scalar deleting destructor of the Oceania Nations Cup (`delete_through`).
/// Original: `OceaniaNations::scalar_deleting_destructor` (0x798a80, oceania_nations.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_oceania_nations(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x798aa0)
}

/// Destroys the Olympic football (`drop_nations_tournament` of cx05_1 with its vtable).
/// Original: `Olympics::~Olympics` (0x79f710, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_olympics(this: usize) {
    covered!("cx07_1 olympics destroyed");
    drop_nations_tournament(League(this), OLYMPICS_VTABLE);
}

/// The scalar deleting destructor of the Olympic football (`delete_through`).
/// Original: `Olympics::scalar_deleting_destructor` (0x79f6f0, olympics.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_olympics(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x79f710)
}

/// Destroys the League base: the League vtable back; the fixed fixture list `league::FIXED_FIXTURES` (an array of
/// 0x10-byte entries allocated with its count in the dword before it) freed with the operator delete from that dword;
/// then `Comp::~Comp` (0x48c650, thiscall, ret 0, through its detour). The pointer is left as it is.
/// Original: `League::~League` (0x682300, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_league(this: usize) {
    covered!("cx07_1 league destroyed");
    write_usize(this, LEAGUE_VTABLE);
    let fixtures = League(this).fixed_fixtures();
    if fixtures != 0 {
        // ponytail: the original first runs the C runtime's vector destructor over the entries with 0x5a53c0, an empty
        // body (shared_noop_ret0): nothing to undo, so the copy only frees the block.
        exe::crt_operator_delete(fixtures - 4);
    }
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x48c650usize); // Comp::~Comp
    base(this);
}

/// The scalar deleting destructor of the League base (`delete_through`).
/// Original: `League::scalar_deleting_destructor` (0x6822e0, league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_league(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x682300)
}

/// The base version of the virtual that builds a stage's round table, which every league overrides: the league.cpp
/// error box (line 0xb8) and null.
/// Original: `League::create_schedule` (0x682650, league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn league_create_schedule(_this: usize, _stage: u32, _rounds: usize, _extra: usize, _league: usize) -> usize {
    show_error_at(LEAGUE_SOURCE, 0xb8);
    0
}

/// The base version of the virtual that sets up a league's rules, which every league overrides: the league.cpp error
/// box (line 0xcf) and −1.
/// Original: `League::set_up_rules` (0x6826f0, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn league_set_up_rules(_this: usize) -> i32 {
    show_error_at(LEAGUE_SOURCE, 0xcf);
    -1
}

/// The base version of the virtual that starts a league's new season, which every league overrides: the league.cpp
/// error box (line 0xe1).
/// Original: `League::start_new_season` (0x682760, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn league_start_new_season(_this: usize) {
    show_error_at(LEAGUE_SOURCE, 0xe1);
}

/// Makes `this` a private copy of the league `source` for a screen, then copies it into `dest`.
///
/// `this` takes the 0xee bytes of `source`; its round dates become a copy of the source's (`exe::crt_malloc` of
/// `.round_count()` × 0x41 bytes; the allocation is not checked) and its table a copy too (`exe::league_copy_table` on
/// the source, League::copy_table, thiscall); the points-average table +0x18, the fixed fixtures
/// `league::FIXED_FIXTURES` and the entrant book `league::ENTRANT_BOOK` are cleared, with `league::ENTRANT_CAPACITY`
/// and `league::ENTRANT_COUNT`. Then `dest` gets bytes 4 to 0xee of `this` (the original copies them field by field,
/// writing the Comp vtable 0x9689e8 and then the League vtable over +0) and the League vtable. Returns `dest`.
/// Original: `League::copy_for_display` (0x682380, league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn league_copy_for_display(this: usize, dest: usize, source: usize) -> usize {
    covered!("cx07_1 league display copy");
    core::ptr::copy(source as *const u8, this as *mut u8, LEAGUE_OBJECT_SIZE);
    let copy = League(this);
    let bytes = (copy.round_count() as i32 as u32).wrapping_mul(0x41);
    let dates = exe::crt_malloc(bytes);
    copy.set_round_dates(dates);
    core::ptr::copy(League(source).round_dates() as *const u8, dates as *mut u8, bytes as usize);
    copy.set_table(exe::league_copy_table(source));
    write_usize(this + 0x18, 0); // the points-average table
    copy.set_fixed_fixtures(0);
    copy.set_entrant_book(0);
    write_u16(this + league::ENTRANT_CAPACITY, 0);
    write_u16(this + league::ENTRANT_COUNT, 0);
    core::ptr::copy((this + 4) as *const u8, (dest + 4) as *mut u8, LEAGUE_OBJECT_SIZE - 4);
    write_usize(dest, LEAGUE_VTABLE);
    dest
}

/// Comparers of the destructors and of the base virtuals: they free memory or show the error box (cannot run twice);
/// only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_goldcup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_goldcup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_oceania_nations(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_oceania_nations(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_olympics(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_olympics(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_league(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_league(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_league_create_schedule(this: usize, stage: u32, rounds: usize, extra: usize, league: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize>(%%I8%%)(this, stage, rounds, extra, league)
}

pub(crate) unsafe extern "thiscall" fn compare_league_set_up_rules(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I9%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_league_start_new_season(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

/// Comparer for `league_copy_for_display`: only the original (it allocates the copy's round dates and table); then
/// the plan: `dest` holds bytes 4 to 0xee of `this` and the League vtable, `this`'s round dates hold the source's and
/// its entrant book, fixed fixtures and points-average table are cleared.
pub(crate) unsafe extern "thiscall" fn compare_league_copy_for_display(this: usize, dest: usize, source: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I11%%)(this, dest, source);
    let copy = League(this);
    let n = (copy.round_count() as i32).max(0) as usize * 0x41;
    let same = |a: usize, b: usize, n: usize| a != 0 && b != 0 && core::slice::from_raw_parts(a as *const u8, n) == core::slice::from_raw_parts(b as *const u8, n);
    let ok = r == dest
        && read_usize(dest) == LEAGUE_VTABLE
        && same(this + 4, dest + 4, LEAGUE_OBJECT_SIZE - 4)
        && (n == 0 || same(copy.round_dates(), League(source).round_dates(), n))
        && copy.entrant_book() == 0
        && copy.fixed_fixtures() == 0
        && read_usize(this + 0x18) == 0;
    SCOREBOARD.record(ok, || format!("league_copy_for_display({:#x}, dest {:#x}, source {:#x}): the copy does not match the plan", this, dest, source));
    r
}
