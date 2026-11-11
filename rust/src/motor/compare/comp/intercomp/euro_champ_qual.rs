use super::*;

// The comparers of the routines in ../comp/intercomp/euro_champ_qual.rs.
// [pt] Os comparadores das rotinas de ../comp/intercomp/euro_champ_qual.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `euro_champ_qual_set_up_rules`.
///
/// [pt] Lado a lado a partir do mesmo estado (os campos +0x3a..+0xc9 da copa, fora o ponteiro +0xba); a tabela das rodadas que
/// cada lado aloca em +0xba (pela virtual +0x3c, original) só é conferida como montada, e a da cópia é liberada.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EURO_CHAMP_QUAL_SET_UP_RULES);
    let before = this.round_dates();
    let reg = [(this.addr() + league::STAGE_ID, 0x80usize), (this.addr() + league::PROMOTED_COUNT, 0xb)];
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this.addr());
        to.set(this.round_dates());
        this.set_round_dates((before as u32) as usize);
        r
    }, || {
        let r = set_up_rules(this);
        tr.set(this.round_dates());
        this.set_round_dates((to.get() as u32) as usize);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    // the table is built by virtual +0x3c (original on both sides) and has bytes it does not write (malloc leftover):
    // checks only that both sides built it
    let table = to != 0 && tr != 0;
    if tr != 0 && tr != to {
        exe::crt_free(tr);
    }
    SCOREBOARD.record(igual && a == b && table, || format!("euro_champ_qual_set_up_rules({:#x}) returned {} x {} memory {} table {}", this.addr(), a, b, igual, table));
    a
}

/// Comparer for `get_group_team_count`.
pub(crate) unsafe extern "thiscall" fn compare_get_group_team_count(this: usize, g: u32) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(IDX_GET_GROUP_TEAM_COUNT);
    let r = o(this, g);
    if !matches!(read_i8(this + euro_champ_qual::GROUP_FORMAT), 1 | 2) {
        return r; // simplification: the unknown format shows the message box: original only
    }
    let n = group_team_count(this, g);
    SCOREBOARD.record(r == n, || format!("get_group_team_count(formato {}, group {}) original={} rust={}", read_i8(this + euro_champ_qual::GROUP_FORMAT), g as u8, r, n));
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
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INIT_GROUP_ONE_TABLE);
    let n = if matches!(read_i8(this.addr() + euro_champ_qual::GROUP_FORMAT), 1 | 2) { 5i16 } else { return o(this.addr()) };
    let plan: Vec<usize> = (0..n as usize).map(|k| read_usize(read_usize(this.addr() + euro_champ_qual::DRAWN_TEAMS) + k * 4)).collect();
    let r = o(this.addr());
    let t = this.table();
    let ok = r == 1 && this.club_count() == n && this.games_per_club() == (n - 1).wrapping_mul(this.legs()) && t != 0
        && plan.iter().enumerate().all(|(k, &c)| read_usize(t + k * 0x3b) == c);
    SCOREBOARD.record(ok, || format!("init_group_one_table({:#x}) returned {}; plan {:x?}", this.addr(), r, plan));
    r
}
