use super::*;

// The comparers of the routines in ../comp/leagues/spa_lower.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/spa_lower.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_lower_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_spa_lower_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_LOWER_CREATE_SCHEDULE);
    compare_league_round_table("spa_lower_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| spa_lower_create_schedule(this, stage, r, e, l))
}

/// Comparer for `construct_spa_lower`.
///
/// [pt] Aloca e sorteia (pela tabela): só a original, conferindo a devolução, a vtable, o ano, a disciplina e o registro.
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_lower(this: usize, year: u32) -> usize {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_CONSTRUCT_SPA_LOWER);
    let r = o(this.addr(), year);
    let ok = r == this.addr() && read_usize(this.addr()) == 0x96fb1c && (this.year() as u16) == year as u16
        && this.discipline_rule() == 0xff && this.competition().addr() == comp_record_of_key(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION);
    SCOREBOARD.record(ok, || format!("construct_spa_lower({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}", this.addr(), r, read_usize(this.addr()), this.competition().addr()));
    r
}

/// Comparer for `spa_lower_create_table`.
///
/// It allocates and draws: only the original runs. The plan, made beforehand: the B teams (of the Primera and the
/// Segunda) in the first rows. Afterwards: +0x3e and the club of the first rows. +0x60 is not checked: the second pass
/// (the one for the Segunda's B teams) zeroes +0x60 of every Spanish team that does not enter it, including the Primera
/// ones the first pass has just put in the table (behaviour of the original, which the copy repeats; found by the
/// 8-league comparison, 05/11/2026).
///
/// [pt] Aloca e sorteia: só a original. O plano, feito antes: os times B (da Primera e da Segunda) das primeiras linhas.
/// Depois: +0x3e e o clube das primeiras linhas. O +0x60 não entra: a segunda passada (a dos times B da Segunda) zera o
/// +0x60 de todo time espanhol que não entra nela, inclusive os da Primera que a primeira acabou de pôr na tabela
/// (comportamento da original, que a cópia repete; achado pela comparação de 8 ligas, 05/11/2026).
pub(crate) unsafe extern "thiscall" fn compare_spa_lower_create_table(this: usize) -> u32 {
    let this = League(this);
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
    let r = o(this.addr());
    let t = this.table();
    let mut ok = this.club_count() == 0x16 && t != 0;
    if ok {
        for (i, &c) in plan.iter().enumerate() {
            ok = ok && read_usize(t + i * 0x3b) == c;
        }
    }
    SCOREBOARD.record(ok, || format!("spa_lower_create_table({:#x}): devolveu {}, plano {:x?}, tabela {:x?}", this.addr(), r, plan,
        (0..plan.len()).map(|i| if t != 0 { read_usize(t + i * 0x3b) } else { 0 }).collect::<Vec<_>>()));
    r
}

/// Comparer for `spa_lower_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_spa_lower_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_LOWER_SET_UP_RULES);
    spa_first_compare_league_rules("spa_lower_set_up_rules", League(this), lower_division_rules(), || o(this), || spa_lower_set_up_rules(this))
}
