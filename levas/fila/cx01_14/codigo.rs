// ---------- cx01_14: the destructors of the competition, the cup and five cups, and the cup's base virtuals (comp.cpp, Cup.cpp, ...) ----------

/// The cup vtables written back by their destructors.
const BEL_FA_CUP_VTABLE: usize = 0x9678a0;
const BEL_SUPER_VTABLE: usize = 0x967aa8;
const BRA_CUP_VTABLE: usize = 0x967cf8;
const CRO_CUP_VTABLE: usize = 0x9692e0;

/// The Comp base part of the destructors: the Comp vtable back, the entrant book (+0x14) freed (left as it is) and the
/// second owned buffer (+0x18) freed and cleared.
unsafe fn drop_comp_buffers(this: usize) {
    let comp = League(this);
    write_u32(this, 0x9689e8); // the Comp vtable
    if comp.entrant_book() != 0 {
        exe::crt_free(comp.entrant_book());
    }
    if read_usize(this + 0x18) != 0 {
        exe::crt_free(read_usize(this + 0x18));
        write_usize(this + 0x18, 0);
    }
}

/// Destroys the Comp base object (`drop_comp_buffers`).
/// Original: `Comp::~Comp` (0x48c650, comp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_comp(this: usize) {
    covered!("cx01_14 comp destroyed");
    drop_comp_buffers(this);
}

/// The scalar deleting destructor of a plain Comp: its buffers (`drop_comp_buffers`) and, with bit 0 of `flags`, the
/// object itself (`exe::crt_operator_delete`). Returns the object.
/// Original: `Comp::scalar_deleting_destructor` (0x48c600, comp.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_comp(this: usize, flags: u32) -> usize {
    drop_comp_buffers(this);
    if flags as u8 & 1 != 0 {
        exe::crt_operator_delete(this);
    }
    this
}

/// Destroys the Cup base object: the Cup vtable back, then the Comp base (`drop_comp`, through its detour).
/// Original: `Cup::~Cup` (0x518690, Cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_cup(this: usize) {
    write_u32(this, 0x9693c8); // the Cup vtable
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x48c650usize); // Comp::~Comp
    base(this);
}

/// Destroys a statistics object (`Comp_Stats::~Comp_Stats` and the operator delete).
unsafe fn delete_comp_stats(stats: usize) {
    if stats != 0 {
        cup_load_delete_comp_stats(stats);
    }
}

/// The body of the national cups' destructors (thiscall, ret 0; SEH frame only for the base): the cup's vtable back,
/// the team table (+0xa7) and the round table (+0xa3) freed, each round's tie array (`TIE_TABLES`, `ROUND_COUNT` of
/// them) and the array freed, the statistics destroyed, all left as they are, then the Cup base (`exe::drop_cup`).
unsafe fn drop_national_cup(this: usize, vtable: usize) {
    let cup = Cup(this);
    write_usize(this, vtable);
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
    delete_comp_stats(League(this).comp_stats());
    exe::drop_cup(this);
}

/// Destroys the Belgian Cup (`drop_national_cup`).
/// Original: `BelFaCup::~BelFaCup` (0x41d240, bel_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bel_fa_cup(this: usize) {
    drop_national_cup(this, BEL_FA_CUP_VTABLE);
}

/// Destroys the Belgian Super Cup (`drop_national_cup`).
/// Original: `BelSuper::~BelSuper` (0x4210e0, bel_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bel_super(this: usize) {
    drop_national_cup(this, BEL_SUPER_VTABLE);
}

/// Destroys the Brazilian Cup (`drop_national_cup`).
/// Original: `BraCup::~BraCup` (0x427630, bra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_cup(this: usize) {
    drop_national_cup(this, BRA_CUP_VTABLE);
}

/// Destroys the Croatian Cup (`drop_national_cup`).
/// Original: `CroCup::~CroCup` (0x517670, cro_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_cro_cup(this: usize) {
    drop_national_cup(this, CRO_CUP_VTABLE);
}

/// Destroys the Brazilian Champions Cup: its vtable back, the table freed (`exe::league_free_table` with 0), the round
/// dates freed and cleared, every stage up to the current one destroyed (virtual slot 0 with 1), the stage vector
/// freed and cleared, the statistics destroyed (left as they are), then League::~League (`exe::drop_league`).
/// Original: `BraChampCup::~BraChampCup` (0x425b50, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_champ_cup(this: usize) {
    let cup = League(this);
    write_usize(this, 0x967c44); // the Champions Cup vtable
    exe::league_free_table(this, 0);
    if cup.round_dates() != 0 {
        exe::crt_free(cup.round_dates());
        cup.set_round_dates(0);
    }
    let mut k = 0i32;
    while k <= cup.current_stage() as i32 {
        let stage = read_usize(cup.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    if cup.stages() != 0 {
        exe::crt_free(cup.stages());
        cup.set_stages(0);
    }
    delete_comp_stats(cup.comp_stats());
    exe::drop_league(this);
}

/// The base version of the virtual that builds a cup's round table, which every cup overrides: the Cup.cpp error box
/// (line 0x54) and no table (0).
/// Original: `Cup::create_round_table` (0x5186a0, Cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cup_create_round_table(_this: usize, _stage: u32, _rounds: usize, _extra: usize, _league: usize) -> usize {
    show_error_at(CUP_SOURCE, 0x54);
    0
}

/// The base version of the virtual that sets up a cup's rounds, which every cup overrides: the Cup.cpp error box (line
/// 0x60) and −1.
/// Original: `Cup::set_up_rounds` (0x518710, Cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cup_set_up_rounds(_this: usize) -> i32 {
    show_error_at(CUP_SOURCE, 0x60);
    -1
}

/// The comparers of the destructors and of the two base virtuals: they free memory or show the error box (cannot run
/// twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_comp(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_comp(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bel_fa_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bel_super(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_cro_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_champ_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_cup_create_round_table(this: usize, stage: u32, rounds: usize, extra: usize, league: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize>(%%I8%%)(this, stage, rounds, extra, league)
}

pub(crate) unsafe extern "thiscall" fn compare_cup_set_up_rounds(this: usize) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I9%%)(this)
}
