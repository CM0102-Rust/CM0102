// ---------- batch %%LEVA%%: the constructors of the Belgian, Danish and German cups (bel_fa_cup.cpp, den_cup.cpp, ger_cup.cpp) ----------

/// What changes from one cup to another in the constructors of the Belgian, Danish, German and Dutch cups.
pub(crate) struct NationCup {
    /// The vtable of the derived class.
    pub(crate) vtable: usize,
    /// The bench, when the constructor writes it: substitutes named and substitutions allowed.
    pub(crate) bench: Option<(u8, u8)>,
    /// The discipline rule (`cup::DISCIPLINE_INDEX`), the line of the Discipline table.
    pub(crate) discipline: u8,
    /// The entrants routine (collect_entrants or select_teams; thiscall, ret 0).
    pub(crate) entrants: usize,
    /// How the entrants and the tie tree fail: `true` when the original tests the return for 0 (test; jne), `false`
    /// when for a negative value (test; jge).
    pub(crate) zero_fails: bool,
    /// Whether the constructor ends with the predicted positions (Cup::rank_teams).
    pub(crate) predicted: bool,
    /// The source file and the lines of the three error boxes (entrants, tie tree, statistics).
    pub(crate) source_file: usize,
    pub(crate) box_lines: [u32; 3],
}

/// The body of the cup constructors of Belgium, Denmark, Germany and Holland (thiscall(this, year, record), ret 8, SEH
/// frame only for the operator new); returns this.
///
/// The base (Cup::Cup `exe::new_cup`: mov ecx -> thiscall, ret 0), the year (`+0x40`), the competition record (`+4`),
/// the vtable, `cup::CURRENT_ROUND` = 0, stage 0xff (`+0x44`), current stage -1 (`+0x30`), no stages (`+0x2c` = 0),
/// played by clubs (`cup::CLUB_SIDES` = 1), the bench when the cup writes it, the discipline rule and
/// `cup::RUN_BY_STAGES` = 0; then it tries to load the saved state (Cup::load_from_disk `0x51fc00`: push 1; mov ecx ->
/// thiscall, ret 4). Loaded: done.
///
/// Otherwise the new cup: the entrants (direct call), the round table (virtual +0x3c with stage -1, &`+0x3c`, &`+0x3a`
/// and 0; into `cup::ROUND_TABLE`, unchecked), the tie tree (Cup::map_fixture_tree `0x518790`, thiscall, ret 0), the
/// statistics (operator new of 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats `exe::new_comp_stats`: push
/// cup; mov ecx, new -> thiscall, ret 4) into `+8` and, in the cups that have them, the predicted positions
/// (Cup::rank_teams `0x5223a0`, thiscall, ret 0). The entrants and the tree that fail (0 or negative, as `zero_fails`)
/// and the statistics out of memory show the error box of their line and end.
pub(crate) unsafe fn construct_nation_cup(this: Cup, year: u32, record: usize, p: &NationCup) -> usize {
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(p.entrants);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    let tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let rank: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5223a0usize);
    let failed = |r: i32| if p.zero_fails { r == 0 } else { r < 0 };
    exe::new_cup(this);
    this.set_year((year as u16) as i16);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), p.vtable);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(0);
    this.set_club_sides(1);
    if let Some((named, allowed)) = p.bench {
        this.set_substitutes_named(named);
        this.set_substitutions_allowed(allowed);
    }
    this.set_discipline_index(p.discipline);
    this.set_run_by_stages(0);
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("mwc_16 new cup");
    if failed(entrants(this.addr())) {
        show_error_at(p.source_file, p.box_lines[0]);
        return this.addr();
    }
    let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = rounds(this.addr(), 0xffff_ffff, this.addr() + 0x3c, this.addr() + 0x3a, 0);
    this.set_round_table(t);
    if failed(tree(this.addr())) {
        show_error_at(p.source_file, p.box_lines[1]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    write_usize(this.addr() + 8, s);
    if s == 0 {
        show_error_at(p.source_file, p.box_lines[2]);
        return this.addr();
    }
    if p.predicted {
        rank(this.addr());
    }
    this.addr()
}

/// The comparer of these cup constructors: the original builds the cup (it allocates, loads files and draws), so only
/// it runs. Checks the return value, vtable, year, record, discipline rule and, for a new cup, the bench, the club
/// sides and the statistics.
pub(crate) unsafe fn compare_nation_cup_ctor(name: &str, this: Cup, year: u32, record: usize, p: &NationCup, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtable && (this.year() as u16) == year as u16
        && read_usize(this.addr() + league::COMPETITION) == record && this.discipline_index() == p.discipline && this.stage_index() == stage_index::MAIN;
    if !loaded {
        ok = ok && this.club_sides() == 1 && read_usize(this.addr() + 8) != 0;
        if let Some((named, allowed)) = p.bench {
            ok = ok && this.substitutes_named() == named && this.substitutions_allowed() == allowed;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}, bench {}/{}]: returned {:#x}, vtable {:#x}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, read_usize(this.addr() + league::COMPETITION), record, this.discipline_index(), this.substitutes_named(), this.substitutions_allowed(), r, read_usize(this.addr()), read_usize(this.addr() + 8)));
    r
}

pub(crate) const BEL_FA_CUP_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x9678a0,
    bench: Some((5, 3)),
    discipline: discipline_rule::BELGIUM_CUP, // mwc16: the Belgian Cup discipline rule
    entrants: 0x41d8d0,
    zero_fails: false,
    predicted: true,
    source_file: 0x989ae0,
    box_lines: [0x50, 0x5e, 0x65],
};

/// The Belgian Cup is born with discipline rule 5, the bench (5 substitutes named, 3 substitutions); with no saved
/// state, the entrants (BelFaCup::collect_entrants), the round table, the tie tree (a failure is a negative return),
/// the statistics and the predicted positions (`construct_nation_cup`).
/// Original: `BelFaCup::BelFaCup` (0x41d050, bel_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_fa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &BEL_FA_CUP_CUP_PARAMS)
}

/// Comparer for `construct_bel_fa_cup`: only the original runs (`compare_nation_cup_ctor`).
unsafe extern "thiscall" fn compare_construct_bel_fa_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_cup_ctor("construct_bel_fa_cup", Cup(this), year, record, &BEL_FA_CUP_CUP_PARAMS, || o(this, year, record))
}

pub(crate) const DEN_CUP_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x969590,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc16: the Danish Cup discipline rule
    entrants: 0x551240,
    zero_fails: true,
    predicted: true,
    source_file: 0x9b8dfc,
    box_lines: [0x4b, 0x5a, 0x61],
};

/// The Danish Cup is born with discipline rule 8; with no saved state, the entrants (DenCup::collect_entrants), the
/// round table, the tie tree (a failure is a 0 return), the statistics and the predicted positions
/// (`construct_nation_cup`).
/// Original: `DenCup::DenCup` (0x5509b0, den_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &DEN_CUP_CUP_PARAMS)
}

/// Comparer for `construct_den_cup`: only the original runs (`compare_nation_cup_ctor`).
unsafe extern "thiscall" fn compare_construct_den_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_nation_cup_ctor("construct_den_cup", Cup(this), year, record, &DEN_CUP_CUP_PARAMS, || o(this, year, record))
}

pub(crate) const GER_CUP_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96b044,
    bench: Some((6, 3)),
    discipline: discipline_rule::GERMANY_CUP, // mwc16: the German Cup discipline rule
    entrants: 0x5d9370,
    zero_fails: true,
    predicted: true,
    source_file: 0x9cd3e0,
    box_lines: [0x4d, 0x5b, 0x62],
};

/// The German Cup is born with discipline rule 14, the bench (6 substitutes named, 3 substitutions); with no saved
/// state, the entrants (GerCup::collect_entrants), the round table, the tie tree (a failure is a 0 return), the
/// statistics and the predicted positions (`construct_nation_cup`).
/// Original: `GerCup::GerCup` (0x5d8c20, ger_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &GER_CUP_CUP_PARAMS)
}

/// Comparer for `construct_ger_cup`: only the original runs (`compare_nation_cup_ctor`).
unsafe extern "thiscall" fn compare_construct_ger_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_nation_cup_ctor("construct_ger_cup", Cup(this), year, record, &GER_CUP_CUP_PARAMS, || o(this, year, record))
}
