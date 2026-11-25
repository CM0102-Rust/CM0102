// ---------- cx10_1: the destructors of the Finnish and French leagues, their placeholder comps and the finance table ----------

/// Destroys a league with no stages (thiscall, ret 0; the SEH frame only covers the base on the exception path): the
/// class vtable back, the table freed (`exe::league_free_table` with 0), the round dates freed and cleared, the
/// statistics destroyed (left as they are), then League::~League (`exe::drop_league`). The same body as the Belgian
/// First Division's (`drop_bel_first`).
pub(crate) unsafe fn drop_plain_league(this: League, vtable: usize) {
    write_usize(this.addr(), vtable);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
    delete_comp_stats(this.comp_stats());
    exe::drop_league(this);
}

/// Destroys an owned competition through its scalar deleting destructor (virtual slot 0) with 1, when there is one.
pub(crate) unsafe fn delete_owned_comp(comp: usize) {
    if comp != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(comp) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
        destroy(comp, 1);
    }
}

/// Destroys a placeholder comp: its vtable back, then Comp::~Comp (0x48c650, through its detour).
unsafe fn drop_placeholder_comp(this: usize, vtable: usize) {
    write_usize(this, vtable);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x48c650usize); // Comp::~Comp
    base(this);
}

/// Destroys the Finnish Premier Division (`drop_plain_league` with the vtable 0x96a4c0).
/// Original: `FinPrm::~FinPrm` (0x595420, fin_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fin_prm(this: usize) {
    covered!("cx10_1 finnish premier destroyed");
    drop_plain_league(League(this), 0x96a4c0);
}

/// The scalar deleting destructor of the Finnish Premier Division (`delete_through`).
/// Original: `FinPrm::scalar_deleting_destructor` (0x595400, fin_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fin_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x595420)
}

/// Destroys the finance table: with a record array, the array block (the count dword just before it) is freed with
/// the operator delete, its pointer left as it is. The original first runs the CRT vector destructor iterator over the
/// records with Finance::~Finance (0x5a8f60), an empty body: the copy does not call it.
/// Original: `FinanceTable::~FinanceTable` (0x59a5a0, fin_prm.cpp (finance.CPP), thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_finance_table(this: usize) {
    let records = read_usize(this);
    if records != 0 {
        exe::crt_operator_delete(records - 4);
    }
}

/// The scalar deleting destructor of the French amateur league (FraCfa::~FraCfa, already ported, and `delete_through`).
/// Original: `FraCfa::scalar_deleting_destructor` (0x5b9440, fra_cfa.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_cfa(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5b9460)
}

/// Destroys the CFA placeholder comp of the French First Division (`drop_placeholder_comp` with the vtable 0x96a994).
/// Original: `FraCfaStubComp::~FraCfaStubComp` (0x5bb580, fra_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fra_cfa_stub_comp(this: usize) {
    drop_placeholder_comp(this, 0x96a994);
}

/// The scalar deleting destructor of the CFA placeholder comp (`delete_through`).
/// Original: `FraCfaStubComp::scalar_deleting_destructor` (0x5bb560, fra_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_cfa_stub_comp(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5bb580)
}

/// Destroys the lower division placeholder comp of the French First Division (`drop_placeholder_comp` with the vtable
/// 0x96a904).
/// Original: `FraLowerStubComp::~FraLowerStubComp` (0x5bb5b0, fra_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fra_lower_stub_comp(this: usize) {
    drop_placeholder_comp(this, 0x96a904);
}

/// The scalar deleting destructor of the lower division placeholder comp (`delete_through`).
/// Original: `FraLowerStubComp::scalar_deleting_destructor` (0x5bb590, fra_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_lower_stub_comp(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5bb5b0)
}

/// Destroys the French First Division: the vtable 0x96aa24 back, the table freed (`exe::league_free_table` with 0),
/// the round dates freed and cleared, the statistics destroyed, then the CFA placeholder comp (`cfa_marker`) and the
/// lower division placeholder comp (`lower_marker`) destroyed through their virtual slot 0 with 1 (both pointers left
/// as they are), then League::~League (`exe::drop_league`).
/// Original: `FraFirst::~FraFirst` (0x5bb5e0, fra_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fra_first(this: usize) {
    let l = League(this);
    write_usize(this, 0x96aa24);
    exe::league_free_table(l, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    delete_comp_stats(l.comp_stats());
    let first = FraFirst(this);
    delete_owned_comp(first.cfa_marker());
    delete_owned_comp(first.lower_marker());
    exe::drop_league(l);
}

/// The scalar deleting destructor of the French First Division (`delete_through`).
/// Original: `FraFirst::scalar_deleting_destructor` (0x5bb5c0, fra_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5bb5e0)
}

/// The scalar deleting destructor of the French lower leagues (FraLower::~FraLower, already ported, and
/// `delete_through`).
/// Original: `FraLower::scalar_deleting_destructor` (0x5bd850, fra_lower.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_lower(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5bd870)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_fin_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fin_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_finance_table(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_cfa(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fra_cfa_stub_comp(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_cfa_stub_comp(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fra_lower_stub_comp(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_lower_stub_comp(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fra_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_lower(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I10%%)(this, flags)
}
