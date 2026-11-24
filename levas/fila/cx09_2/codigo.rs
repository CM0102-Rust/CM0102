// ---------- cx09_2: the destructors of three leagues without stages and nine Brazilian state championships ----------

/// The body of the destructors of the Brazilian state championships and the Danish Second Division (thiscall, ret 0;
/// SEH frame only for the base): like `drop_group_cup` without an entrant list, but the stage vector (`STAGES`) is
/// freed without a null check and left in place (not zeroed): the class vtable back, the table freed
/// (`exe::league_free_table` with 0), the round dates freed and zeroed, each stage from 0 to `CURRENT_STAGE` (read
/// again each time) deleted through its scalar deleting destructor (vtable slot 0 with 1), the vector freed, the
/// statistics destroyed (left in place), then League::~League (`exe::drop_league`).
unsafe fn drop_regional_league(this: usize, vtable: usize) {
    let league = League(this);
    covered!("cx09_2 regional league destroyed");
    write_usize(this, vtable);
    exe::league_free_table(this, 0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
        league.set_round_dates(0);
    }
    let mut k = 0i32;
    while k <= league.current_stage() as i32 {
        let stage = read_usize(league.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    exe::crt_free(league.stages()); // no null check and not cleared, as in the original
    delete_comp_stats(league.comp_stats());
    exe::drop_league(this);
}

/// Destroys the Danish First Division (`drop_plain_league`).
/// Original: `DenFirst::~DenFirst` (0x551920, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_den_first(this: usize) {
    drop_plain_league(this, 0x969630);
}

/// Destroys the Danish Superliga (`drop_plain_league`).
/// Original: `DenPrm::~DenPrm` (0x552870, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_den_prm(this: usize) {
    drop_plain_league(this, 0x9696e4);
}

/// Destroys the English Premier League (`drop_plain_league`).
/// Original: `EngPrm::~EngPrm` (0x5735d0, eng_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eng_prm(this: usize) {
    drop_plain_league(this, 0x969d1c);
}

/// Destroys the Bahia state championship (`drop_regional_league`).
/// Original: `BraRegBahia::~BraRegBahia` (0x42f510, bra_reg_bahia.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_bahia(this: usize) {
    drop_regional_league(this, 0x967fb4);
}

/// Destroys the Central regional championship (`drop_regional_league`).
/// Original: `BraRegCentral::~BraRegCentral` (0x4306f0, bra_reg_central.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_central(this: usize) {
    drop_regional_league(this, 0x968068);
}

/// Destroys the Rio Grande do Sul state championship (`drop_regional_league`).
/// Original: `BraRegGaucho::~BraRegGaucho` (0x431a00, bra_reg_gaucho.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_gaucho(this: usize) {
    drop_regional_league(this, 0x96811c);
}

/// Destroys the Goias state championship (`drop_regional_league`).
/// Original: `BraRegGoias::~BraRegGoias` (0x432d90, bra_reg_goias.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_goias(this: usize) {
    drop_regional_league(this, 0x9681d0);
}

/// Destroys the Minas Gerais state championship (`drop_regional_league`).
/// Original: `BraRegMinasGerais::~BraRegMinasGerais` (0x433f90, bra_reg_minas_gerais.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_minas_gerais(this: usize) {
    drop_regional_league(this, 0x968284);
}

/// Destroys the North regional championship (`drop_regional_league`).
/// Original: `BraRegNorth::~BraRegNorth` (0x435580, bra_reg_north.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_north(this: usize) {
    drop_regional_league(this, 0x968338);
}

/// Destroys the Northeast regional championship (`drop_regional_league`).
/// Original: `BraRegNortheast::~BraRegNortheast` (0x436630, bra_reg_northeast.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_northeast(this: usize) {
    drop_regional_league(this, 0x9683ec);
}

/// Destroys the Parana state championship (`drop_regional_league`).
/// Original: `BraRegParana::~BraRegParana` (0x437640, bra_reg_parana.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_parana(this: usize) {
    drop_regional_league(this, 0x9684a0);
}

/// Destroys the Pernambuco state championship (`drop_regional_league`).
/// Original: `BraRegPern::~BraRegPern` (0x438b30, bra_reg_pern.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_pern(this: usize) {
    drop_regional_league(this, 0x968554);
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_den_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_den_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eng_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_bahia(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_central(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_gaucho(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_goias(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_minas_gerais(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_north(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_northeast(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_parana(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_pern(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I11%%)(this)
}
