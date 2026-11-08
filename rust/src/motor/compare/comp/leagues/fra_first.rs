use super::*;

// The comparers of the routines in ../comp/leagues/fra_first.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/fra_first.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_fra_first`.
///
/// [pt] Sorteia (as ligas de fundo), aloca e mexe nos clubes: só a original, conferindo a devolução, a vtable, o ano, o
/// registro, a disciplina e os dois marcadores (vtable e registro) e, na liga nova, os campos do começo e as
/// estatísticas.
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_FIRST);
    let r = o(this, year, record);
    let marker = |m: usize, vt: usize, g: usize| m == 0 || (read_usize(m) == vt && read_usize(m + 4) == comp_record_of_key(g) && *((m + league::DISCIPLINE_RULE) as *const u8) == 0xff);
    let mut ok = r == this && read_usize(this) == 0x96aa24 && core::ptr::read_unaligned((this + league::YEAR) as *const u16) == year as u16
        && read_usize(this + 4) == record && *((this + league::DISCIPLINE_RULE) as *const u8) == 0xc
        && marker(read_usize(this + 0xee), 0x96a994, KEY_CLUB_COMP_FRENCH_CFA) && marker(read_usize(this + 0xf2), 0x96a904, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION);
    if *((this + league::STAGE_INDEX) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + league::STAGE_COUNT) == 0 && read_usize(this + league::STAGES) == 0;
    }
    SCOREBOARD.record(ok, || format!("construct_fra_first({:#x}): devolveu {:#x}, vtable {:#x}, +0xee = {:#x}, +0xf2 = {:#x}, +8 = {:#x}", this, r, read_usize(this), read_usize(this + 0xee), read_usize(this + 0xf2), read_usize(this + 8)));
    r
}

/// Comparer for `stub_return_one_ret8`.
///
/// [pt] Os dois tocos: compara a devolução.
pub(crate) unsafe extern "thiscall" fn compare_stub_return_one_ret8(this: usize, a: u32, b: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_STUB_RETURN_ONE_RET8)(this, a, b);
    let n = stub_return_one_ret8(this, a, b);
    SCOREBOARD.record(r == n, || format!("stub_return_one_ret8: original={} rust={}", r, n));
    r
}

/// Comparer for `stub_return_one_ret4`.
pub(crate) unsafe extern "thiscall" fn compare_stub_return_one_ret4(this: usize, a: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_STUB_RETURN_ONE_RET4)(this, a);
    let n = stub_return_one_ret4(this, a);
    SCOREBOARD.record(r == n, || format!("stub_return_one_ret4: original={} rust={}", r, n));
    r
}

/// Comparer for `fra_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_fra_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_FIRST_SET_UP_RULES);
    fra_first_compare_league_rules("fra_first_set_up_rules", this, fra_first_first_division_rules(this), || o(this), || fra_first_set_up_rules(this))
}

/// Comparer for `fra_first_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_fra_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_FIRST_CREATE_TABLE);
    let n: i16 = if core::ptr::read_unaligned((this + league::YEAR) as *const u16) == 0x7d1 { 0x12 } else { 0x14 };
    fra_first_compare_league_table("fra_first_create_table", this, n, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, Some(0x30d40), || o(this))
}

/// Comparer for `fra_first_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_fra_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_FIRST_CREATE_SCHEDULE);
    compare_cfa_schedule("fra_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| fra_first_create_schedule(this, stage, r, e, l))
}
