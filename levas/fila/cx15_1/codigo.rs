// ---------- cx15_1: the destructors of the MLS, the League of Wales, the African and Asian qualifiers, the stadiums and the UEFA seeding ----------

/// Destroys the MLS (`drop_stats_league` with the vtable 0x970844).
/// Original: `UsaMls::~UsaMls` (0x907210, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_usa_mls(this: usize) {
    covered!("cx15_1 mls destroyed");
    drop_stats_league(this, 0x970844);
}

/// The scalar deleting destructor of the MLS (`delete_through`).
/// Original: `UsaMls::scalar_deleting_destructor` (0x9071f0, usa_mls.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_usa_mls(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x907210)
}

/// Destroys the League of Wales (`drop_plain_league` with the vtable 0x970f80).
/// Original: `WelFirst::~WelFirst` (0x925060, wel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wel_first(this: usize) {
    drop_plain_league(League(this), 0x970f80);
}

/// The scalar deleting destructor of the League of Wales (`delete_through`).
/// Original: `WelFirst::scalar_deleting_destructor` (0x925040, wel_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wel_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x925060)
}

/// Destroys the African World Cup qualifiers: the vtable 0x970ad0 back, the team table and the round table freed, each
/// round's tie array (`cup::TIE_TABLES`, `cup::ROUND_COUNT` of them) and the array freed (all left as they are); every
/// stage up to the current one destroyed (virtual slot 0 with 1) and the stage vector freed and cleared; the statistics
/// destroyed; the entrant list (`wc_cup_turnover_consts::CUP_ENTRANT_LIST`) freed without a test; then the Cup base
/// (`exe::drop_cup`). The same body as `drop_european_cup` up to the statistics.
/// Original: `WcAfricanCup::~WcAfricanCup` (0x90f4f0, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wc_african_cup(this: usize) {
    let cup = Cup(this);
    write_usize(this, 0x970ad0); // the African qualifiers vtable
    for table in [cup.team_table(), cup.round_table()] {
        if table != 0 {
            exe::crt_free(table);
        }
    }
    let ties = cup.tie_tables();
    if ties != 0 {
        let mut r = 0i32;
        while r < cup.round_count() as i32 {
            let round = read_usize(ties + 4 * r as usize);
            if round != 0 {
                exe::crt_free(round);
            }
            r += 1;
        }
        exe::crt_free(cup.tie_tables());
    }
    destroy_owned_stages(League(this));
    delete_comp_stats(League(this).comp_stats());
    exe::crt_free(read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST));
    exe::drop_cup(this);
}

/// Every stage up to the current one destroyed (`destroy_comp_stages`), then the stage vector freed and cleared when
/// there is one: the stage part of `drop_stats_league`.
unsafe fn destroy_owned_stages(l: League) {
    destroy_comp_stages(l);
    if l.stages() != 0 {
        exe::crt_free(l.stages());
        l.set_stages(0);
    }
}

/// The scalar deleting destructor of the African World Cup qualifiers (`delete_through`).
/// Original: `WcAfricanCup::scalar_deleting_destructor` (0x90f4d0, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wc_african_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x90f4f0)
}

/// Destroys the Asian World Cup qualifiers: the vtable 0x970b70 back, the table freed (`exe::league_free_table` with 0),
/// the round dates freed and cleared, the national team list (`wc_turnover_consts::ENTRANT_LIST`) freed when there is one
/// (left as it is), every stage up to the current one destroyed and the stage vector freed and cleared, the statistics
/// destroyed, the first round group draw (+0xf7) freed when there is one (left as it is), then League::~League
/// (`exe::drop_league`).
/// Original: `WcAsiaLeague::~WcAsiaLeague` (0x911e10, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wc_asia_league(this: usize) {
    covered!("cx15_1 asian qualifiers destroyed");
    let l = League(this);
    write_usize(this, 0x970b70); // the Asian qualifiers vtable
    exe::league_free_table(l, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    let entrants = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    if entrants != 0 {
        exe::crt_free(entrants);
    }
    destroy_owned_stages(l);
    delete_comp_stats(l.comp_stats());
    let draw = read_usize(this + 0xf7); // the first round group draw (draw_first_round_groups)
    if draw != 0 {
        exe::crt_free(draw);
    }
    exe::drop_league(l);
}

/// The scalar deleting destructor of the Asian World Cup qualifiers (`delete_through`).
/// Original: `WcAsiaLeague::scalar_deleting_destructor` (0x911df0, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wc_asia_league(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x911e10)
}

/// Destroys the stadium object: the booking list (`stadium_obj::BOOKINGS`) and the neutral venue list
/// (`stadium_obj::VENUES`) freed, each only when there is one, both left as they are.
/// Original: `Stadium::~Stadium` (0x85b7f0, stadium.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_stadium(this: usize) {
    for list in [read_usize(this + stadium_obj::BOOKINGS), read_usize(this + stadium_obj::VENUES)] {
        if list != 0 {
            exe::crt_free(list);
        }
    }
}

/// Destroys the UEFA seeding: the Intertoto volunteer list (`uefa_seeding::INTERTOTO_VOLUNTEERS`) freed when there is
/// one, left as it is.
/// Original: `UefaSeeding::~UefaSeeding` (0x903570, uefa_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_uefa_seeding(this: usize) {
    let volunteers = read_usize(this + uefa_seeding::INTERTOTO_VOLUNTEERS);
    if volunteers != 0 {
        exe::crt_free(volunteers);
    }
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_usa_mls(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_usa_mls(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_wel_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wel_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_wc_african_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wc_african_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_wc_asia_league(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wc_asia_league(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_stadium(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_uefa_seeding(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}
