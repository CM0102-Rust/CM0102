// ---------- cx05_1: the destructors of the national-team tournaments, the friendly mini cup and mini league and the friendlies competition ----------

/// The vtables the destructors write back.
const AFRICAN_NATIONS_VTABLE: usize = 0x967270;
const ASIA_NATIONS_VTABLE: usize = 0x967614;
const EURO_CHAMP_VTABLE: usize = 0x96a020;
const MINI_CUP_VTABLE: usize = 0x96d8ac;
const MINI_LEAGUE_VTABLE: usize = 0x96d94c;
const FRIENDLY_VTABLE: usize = 0x96ae7c;

/// `Comp::~Comp` (thiscall, ret 0): the base destructor of the friendlies competition.
const FN_COMP_DESTRUCTOR_COMP: usize = 0x48c650;

/// Destroys the statistics object when there is one (`Comp_Stats::~Comp_Stats` and the operator delete); the pointer
/// is left as it is.
unsafe fn delete_stats_if_any(stats: usize) {
    if stats != 0 {
        cup_load_delete_comp_stats(stats);
    }
}

/// The body of the national-team tournaments' destructors (thiscall, ret 0; the SEH frame only covers the base on the
/// exception path): the class vtable back, the table freed (`exe::league_free_table`(this, 0)), the round table
/// `.round_dates()` freed and zeroed, each stage 0..=`.current_stage()` (signed) destroyed through its virtual 0 with 1,
/// the stage vector freed and zeroed, the statistics destroyed (pointer left as it is), then `League::~League`.
pub(crate) unsafe fn drop_nations_tournament(this: League, vtable: usize) {
    write_usize(this.addr(), vtable);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
    let mut i = 0i32;
    while i <= this.current_stage() as i32 {
        let stage = read_usize(this.stages() + i as usize * 4);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(stage, 1);
        }
        i += 1;
    }
    if this.stages() != 0 {
        exe::crt_free(this.stages());
        this.set_stages(0);
    }
    delete_stats_if_any(this.comp_stats());
    exe::drop_league(this);
}

/// Destroys the African Nations Cup (`drop_nations_tournament` with its vtable).
/// Original: `AfricanNations::~AfricanNations` (0x401220, african_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_african_nations(this: usize) {
    covered!("cx05_1 african nations destroyed");
    drop_nations_tournament(League(this), AFRICAN_NATIONS_VTABLE);
}

/// Destroys the Asian Cup (`drop_nations_tournament` with its vtable).
/// Original: `AsiaNations::~AsiaNations` (0x40ea40, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_asia_nations(this: usize) {
    covered!("cx05_1 asian cup destroyed");
    drop_nations_tournament(League(this), ASIA_NATIONS_VTABLE);
}

/// Destroys the European Championship (`drop_nations_tournament` with its vtable).
/// Original: `EuroChamp::~EuroChamp` (0x57aad0, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_euro_champ(this: usize) {
    covered!("cx05_1 european championship destroyed");
    drop_nations_tournament(League(this), EURO_CHAMP_VTABLE);
}

/// Destroys a friendly mini cup: the national cups' mould (`drop_national_cup` of cx01_14: the team table, the round
/// table, each round's tie array and the array freed, the statistics destroyed, then `Cup::~Cup`) with the mini cup's
/// vtable.
/// Original: `MiniCup::~MiniCup` (0x765ea0, mini_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_mini_cup(this: usize) {
    covered!("cx05_1 mini cup destroyed");
    drop_national_cup(this, MINI_CUP_VTABLE);
}

/// Destroys a friendly mini league: its vtable back, the table freed (`exe::league_free_table`(this, 0)), the round
/// table `.round_dates()` freed but not zeroed, the statistics destroyed, then `League::~League`. A mini league has
/// no stages.
/// Original: `MiniLeague::~MiniLeague` (0x7668f0, mini_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_mini_league(this: usize) {
    covered!("cx05_1 mini league destroyed");
    let league = League(this);
    write_usize(this, MINI_LEAGUE_VTABLE);
    exe::league_free_table(league, 0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
    }
    delete_stats_if_any(league.comp_stats());
    exe::drop_league(league);
}

/// Destroys a linked list owned by the friendlies (`LLIST::~LLIST` and the operator delete) when there is one.
unsafe fn delete_llist_if_any(list: usize) {
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
}

/// Destroys the friendlies competition: its vtable back; each of the 127 tournament slots (`friendly::TOURNAMENT_SLOTS`,
/// 0x69 bytes) with a host and a competition destroys the competition (virtual 0 with 1); the planned friendlies freed;
/// the four lists (friendly matches, dates requests, pending tournaments, requests awaiting a reply) destroyed; the
/// international date table and the sorted nations freed; each nation's list (`G_NATION_COUNT` of them) destroyed and
/// the array freed; the banned nation pairs freed; then `Comp::~Comp`. The pointers are left as they are.
/// Original: `Friendly::~Friendly` (0x5c22d0, friendly.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_friendly(this: usize) {
    covered!("cx05_1 friendlies destroyed");
    let f = Friendly(this);
    write_usize(this, FRIENDLY_VTABLE);
    for k in 0..0x7fusize {
        let slot = this + friendly::TOURNAMENT_SLOTS + k * 0x69;
        let competition = read_usize(slot + tournament_slot::COMPETITION);
        if read_usize(slot + tournament_slot::HOST) != 0 && competition != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(competition) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(competition, 1);
        }
    }
    if f.planned_friendlies() != 0 {
        exe::crt_free(f.planned_friendlies());
    }
    delete_llist_if_any(f.friendly_match_list().addr());
    delete_llist_if_any(f.scheduled_slot_list());
    delete_llist_if_any(f.pending_tournament_list().addr());
    delete_llist_if_any(f.request_reply_list());
    if f.intl_date_table() != 0 {
        exe::crt_free(f.intl_date_table());
    }
    if f.sorted_nations() != 0 {
        exe::crt_free(f.sorted_nations());
    }
    if f.nation_lists() != 0 {
        let mut k = 0i32;
        while k < read_i32(G_NATION_COUNT) {
            delete_llist_if_any(read_usize(f.nation_lists() + k as usize * 4));
            k += 1;
        }
        exe::crt_free(f.nation_lists());
    }
    if f.banned_nation_pairs() != 0 {
        exe::crt_free(f.banned_nation_pairs());
    }
    // ponytail: the original then runs the C runtime's vector destructor over the 127 slots with the slot destructor
    // 0x5a53c0, an empty body: nothing to undo, so the copy skips it.
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_COMP_DESTRUCTOR_COMP);
    base(this);
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_african_nations(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_asia_nations(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_euro_champ(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_mini_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_mini_league(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_friendly(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}
