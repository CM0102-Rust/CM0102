use super::*;

// The comparers of the routines in ../comp/leagues/spa_lower.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/spa_lower.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_lower_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spa_lower_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_LOWER_CREATE_SCHEDULE);
    compare_round_table_any_stage("spa_lower_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| spa_lower_create_schedule(this, stage, r, e, l))
}

/// Comparer for `construct_spa_lower` (comparison mode: see the header of motor.rs).
///
/// [pt] Aloca e sorteia (pela tabela): só a original, conferindo a devolução, a vtable, o ano, a disciplina e o registro.
pub(crate) unsafe extern "thiscall" fn comparar_construct_spa_lower(this: usize, year: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_CONSTRUCT_SPA_LOWER);
    let r = o(this, year);
    let ok = r == this && read_u32_as_usize(this) == 0x96fb1c && core::ptr::read_unaligned((this + 0x40) as *const u16) == year as u16
        && *((this + 0x50) as *const u8) == 0xff && read_u32_as_usize(this + 4) == comp_record_of_key(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION);
    PLACAR.registrar(ok, || format!("construct_spa_lower({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}", this, r, read_u32_as_usize(this), read_u32_as_usize(this + 4)));
    r
}

/// Comparer for `spa_lower_create_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Aloca e sorteia: só a original. O plano, feito antes: os times B (da Primera e da Segunda) das primeiras linhas.
/// Depois: +0x3e e o clube das primeiras linhas. O +0x60 não entra: a segunda passada (a dos times B da Segunda) zera o
/// +0x60 de todo time espanhol que não entra nela, inclusive os da Primera que a primeira acabou de pôr na tabela
/// (comportamento da original, que a cópia repete; achado pela comparação de 8 ligas, 05/11/2026).
pub(crate) unsafe extern "thiscall" fn comparar_spa_lower_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_LOWER_CREATE_TABLE);
    let mut plan: Vec<usize> = Vec::new();
    let mut k: u8 = 0;
    for g in [KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION] {
        for_each_spanish_b_team(g, &mut k, 0x16, |c, entra| {
            if entra {
                plan.push(c);
            }
        });
    }
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut ok = read_i16(this + 0x3e) == 0x16 && t != 0;
    if ok {
        for (i, &c) in plan.iter().enumerate() {
            ok = ok && read_u32_as_usize(t + i * 0x3b) == c;
        }
    }
    PLACAR.registrar(ok, || format!("spa_lower_create_table({:#x}): devolveu {}, plano {:x?}, tabela {:x?}", this, r, plan,
        (0..plan.len()).map(|i| if t != 0 { read_u32_as_usize(t + i * 0x3b) } else { 0 }).collect::<Vec<_>>()));
    r
}

/// Comparer for `spa_lower_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spa_lower_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_LOWER_SET_UP_RULES);
    spa_first_compare_league_rules("spa_lower_set_up_rules", this, lower_division_rules(), || o(this), || spa_lower_set_up_rules(this))
}
