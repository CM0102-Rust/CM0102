use super::*;

// The comparers of the routines in ../comp/intercomp/euro_champ_qual.rs.
// [pt] Os comparadores das rotinas de ../comp/intercomp/euro_champ_qual.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `euro_champ_qual_set_up_rules`.
///
/// [pt] Lado a lado a partir do mesmo estado (os campos +0x3a..+0xc9 da copa, fora o ponteiro +0xba); a tabela das rodadas que
/// cada lado aloca em +0xba (pela virtual +0x3c, original) só é conferida como montada, e a da cópia é liberada.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EURO_CHAMP_QUAL_SET_UP_RULES);
    let before = read_u32_as_usize(this + league::ROUND_DATES);
    let reg = [(this + league::STAGE_ID, 0x80usize), (this + league::PROMOTED_COUNT, 0xb)];
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        to.set(read_u32_as_usize(this + league::ROUND_DATES));
        core::ptr::write_unaligned((this + league::ROUND_DATES) as *mut u32, before as u32);
        r
    }, || {
        let r = set_up_rules(this);
        tr.set(read_u32_as_usize(this + league::ROUND_DATES));
        core::ptr::write_unaligned((this + league::ROUND_DATES) as *mut u32, to.get() as u32);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    // the table is built by virtual +0x3c (original on both sides) and has bytes it does not write (malloc leftover):
    // checks only that both sides built it
    let table = to != 0 && tr != 0;
    if tr != 0 && tr != to {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(tr);
    }
    SCOREBOARD.record(igual && a == b && table, || format!("euro_champ_qual_set_up_rules({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, table));
    a
}

/// Comparer for `get_group_team_count`.
pub(crate) unsafe extern "thiscall" fn compare_get_group_team_count(this: usize, g: u32) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(IDX_GET_GROUP_TEAM_COUNT);
    let r = o(this, g);
    if !matches!(*((this + euro_champ_qual::GROUP_FORMAT) as *const i8), 1 | 2) {
        return r; // simplification: the unknown format shows the message box: original only
    }
    let n = group_team_count(this, g);
    SCOREBOARD.record(r == n, || format!("get_group_team_count(formato {}, grupo {}) original={} rust={}", *((this + euro_champ_qual::GROUP_FORMAT) as *const i8), g as u8, r, n));
    r
}

/// Comparer for `get_qualification_status`.
pub(crate) unsafe extern "thiscall" fn compare_get_qualification_status(this: usize, club: usize) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(IDX_GET_QUALIFICATION_STATUS);
    let r = o(this, club);
    let n = qualification_status(this, club);
    SCOREBOARD.record(r == n, || format!("get_qualification_status({:#x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `init_group_one_table` (comparison mode: see the header of motor.rs): only the original routine runs
/// (`0x687430` allocates the form and the head-to-head of each row, and the news goes to the humans), checking the plan
/// made beforehand.
///
/// It returns 1, `+0x3e` = the group size −1, `+0xa7` = (`+0x3e` − 1) × `+0x3c` and the national teams of `+0xf7` in
/// the rows, in order.
///
/// [pt] Só a original (a 0x687430 aloca a forma e o confronto direto de cada linha e a notícia vai aos humanos), conferindo o
/// plano feito antes: devolve 1, +0x3e = o tamanho do grupo −1, +0xa7 = (+0x3e − 1) × +0x3c e as seleções de +0xf7 nas
/// linhas, na ordem.
pub(crate) unsafe extern "thiscall" fn compare_init_group_one_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INIT_GROUP_ONE_TABLE);
    let n = if matches!(*((this + 0xf6) as *const i8), 1 | 2) { 5i16 } else { return o(this) };
    let plan: Vec<usize> = (0..n as usize).map(|k| read_u32_as_usize(read_u32_as_usize(this + 0xf7) + k * 4)).collect();
    let r = o(this);
    let t = read_u32_as_usize(this + league::TABLE);
    let ok = r == 1 && read_i16(this + league::CLUB_COUNT) == n && read_i16(this + league::GAMES_PER_CLUB) == (n - 1).wrapping_mul(read_i16(this + league::LEGS)) && t != 0
        && plan.iter().enumerate().all(|(k, &c)| read_u32_as_usize(t + k * 0x3b) == c);
    SCOREBOARD.record(ok, || format!("init_group_one_table({:#x}) devolveu {}; plano {:x?}", this, r, plan));
    r
}
