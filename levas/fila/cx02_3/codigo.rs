// ---------- cx02_3: the cup destructors (3) ----------

/// Deletes each stage object from 0 to `CURRENT_STAGE` (read again each time; none when it is negative) through its
/// scalar deleting destructor (vtable slot 0 with 1), then frees the stage vector (`STAGES`) and zeroes it.
unsafe fn delete_stages_and_vector(this: League) {
    let mut k = 0i32;
    while k <= this.current_stage() as i32 {
        let stage = read_usize(this.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    if this.stages() != 0 {
        exe::crt_free(this.stages());
        this.set_stages(0);
    }
}

/// The body of the destructors of the cups built on a league with group stages (thiscall, ret 0; SEH frame only for
/// the base; identical apart from the vtable and, in the Irish Super Cup, the entrant list): the class vtable back,
/// the table freed (`exe::league_free_table` with 0), the round dates freed and zeroed (`ROUND_DATES`), with
/// `entrants` the entrant club vector freed (`ita_cup_fields::ENTRANT_CLUBS`, left in place), the stages deleted
/// (`delete_stages_and_vector`), the statistics destroyed (left in place), then League::~League (`exe::drop_league`).
unsafe fn drop_group_cup(this: usize, vtable: usize, entrants: bool) {
    let cup = League(this);
    covered!("cx02_3 group cup destroyed");
    write_usize(this, vtable);
    exe::league_free_table(this, 0);
    if cup.round_dates() != 0 {
        exe::crt_free(cup.round_dates());
        cup.set_round_dates(0);
    }
    if entrants {
        let list = ItaCupFields(this).entrant_clubs();
        if list != 0 {
            exe::crt_free(list);
        }
    }
    delete_stages_and_vector(cup);
    delete_comp_stats(cup.comp_stats());
    exe::drop_league(this);
}

/// Destroys the Northern Irish Charity Shield (`drop_national_cup`).
/// Original: `NirCharity::~NirCharity` (0x789fd0, nir_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_nir_charity(this: usize) {
    drop_national_cup(this, 0x96da24);
}

/// Destroys the Northern Irish Cup (`drop_national_cup`).
/// Original: `NirCup::~NirCup` (0x78a7a0, nir_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_nir_cup(this: usize) {
    drop_national_cup(this, 0x96dac4);
}

/// Destroys the Norwegian Cup (`drop_national_cup`).
/// Original: `NorCup::~NorCup` (0x78f210, nor_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_nor_cup(this: usize) {
    drop_national_cup(this, 0x96dd80);
}

/// Destroys the Greek Cup (`drop_group_cup` with its entrant list).
/// Original: `GreCup::~GreCup` (0x5e8b10, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_gre_cup(this: usize) {
    drop_group_cup(this, 0x96b4b4, true);
}

/// Destroys the Dutch Cup (`drop_group_cup` with its entrant list).
/// Original: `HolCup::~HolCup` (0x5f3600, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_hol_cup(this: usize) {
    drop_group_cup(this, 0x96b7b8, true);
}

/// Destroys the Irish League Cup (`drop_group_cup` with its entrant list).
/// Original: `IreLgeCup::~IreLgeCup` (0x637040, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ire_lge_cup(this: usize) {
    drop_group_cup(this, 0x96be20, true);
}

/// Destroys the Serie C cup (`drop_group_cup` with its entrant list).
/// Original: `ItaCCup::~ItaCCup` (0x63ceb0, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_c_cup(this: usize) {
    drop_group_cup(this, 0x96c264, true);
}

/// Destroys the Coppa Italia (`drop_group_cup` with its entrant list).
/// Original: `ItaCup::~ItaCup` (0x63eb70, ita_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_cup(this: usize) {
    drop_group_cup(this, 0x96c318, true);
}

/// Destroys the Korean League Cup (`drop_group_cup` with its entrant list).
/// Original: `KorLeagueCup::~KorLeagueCup` (0x66c160, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_kor_league_cup(this: usize) {
    drop_group_cup(this, 0x96ce84, true);
}

/// Destroys the Northern Irish League Cup (`drop_group_cup` with its entrant list).
/// Original: `NirLgeCup::~NirLgeCup` (0x78c9f0, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_nir_lge_cup(this: usize) {
    drop_group_cup(this, 0x96dc18, true);
}

/// Destroys the Auto Windscreens Shield: its vtable back, the team table and the round table freed, each round's
/// tie array (`TIE_TABLES`, `ROUND_COUNT` of them) and the array freed, the section and final stages deleted
/// (`delete_stages_and_vector`), the statistics destroyed, the chosen clubs list (`eng_auto_cup::SELECTED_CLUBS`)
/// freed, all left in place but the stage vector, then the Cup base (`exe::drop_cup`).
/// Original: `EngAutoCup::~EngAutoCup` (0x56acc0, eng_auto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_auto_cup(this: usize) {
    let cup = Cup(this);
    covered!("cx02_3 auto windscreens destroyed");
    write_usize(this, 0x969894);
    for table in [cup.team_table(), cup.round_table()] {
        if table != 0 {
            exe::crt_free(table);
        }
    }
    if cup.tie_tables() != 0 {
        let mut r = 0i32;
        while r < cup.round_count() as i32 {
            let round = read_usize(cup.tie_tables() + 4 * r as usize);
            if round != 0 {
                exe::crt_free(round);
            }
            r += 1;
        }
        exe::crt_free(cup.tie_tables());
    }
    delete_stages_and_vector(League(this));
    delete_comp_stats(League(this).comp_stats());
    let selected = EngAutoCup(this).selected_clubs();
    if selected != 0 {
        exe::crt_free(selected);
    }
    exe::drop_cup(this);
}

/// Destroys the Irish Super Cup (`drop_group_cup` without an entrant list).
/// Original: `IreSuperCup::~IreSuperCup` (0x63aea0, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ire_super_cup(this: usize) {
    drop_group_cup(this, 0x96c0c8, false);
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_nir_charity(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_nir_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_nor_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_gre_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_hol_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ire_lge_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_c_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_kor_league_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_nir_lge_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_auto_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ire_super_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I11%%)(this)
}
