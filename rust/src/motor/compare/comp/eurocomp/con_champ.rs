use super::*;

// The comparers of the routines in ../comp/eurocomp/con_champ.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/con_champ.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer of `con_champ_select_qualified_clubs`: side by side on the eight teams (+0xa7, 48 bytes), on +0x3e and on the random generator's
/// state (`comp_util_sort_and_shuffle_top_clubs` shuffles; the copy without counting).
///
/// When a list is short of clubs (fewer than 2 Mexican, 3 Central American, 1 Caribbean, or 2 MLS without the object
/// [0x9cf728]) the original reads stack leftovers: only the original.
///
/// [pt] Comparador da 0x4c11a0: lado a lado nos oito times (+0xa7, 48 bytes), em +0x3e e no estado do sorteador (a 0x4afbd0
/// embaralha; a cópia sem contar). Quando falta clube numa lista (menos de 2 mexicanos, 3 centro-americanos, 1
/// caribenho, ou 2 da MLS sem o objeto [0x9cf728]) a original lê sobra de pilha: só a original.
pub(crate) unsafe extern "thiscall" fn compare_con_champ_select_qualified_clubs(this: usize) -> u32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_CHAMP_SELECT_QUALIFIED_CLUBS);
    let l = concacaf_club_lists();
    let other = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_US_OPEN_CUP).wrapping_mul(4)));
    if l.n[0] < 2 || l.n[1] < 3 || l.n[2] < 1 || (other == 0 && l.n[3] < 2) || this.team_table() == 0 {
        return o(this.addr()); // simplification: short list (the original reads stack leftover) or no table: original only
    }
    let mut regions = vec![(this.addr() + cup::TEAM_COUNT, 2), (this.team_table(), 48)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    // 0x4afd80 reorders the competition history ([0xae23d4], [0xae237c] rows of 0x1a bytes) on every call; with
    // tie, the order the original left changes the champion the copy would find: the history goes into the regions
    let history = read_usize(G_CLUB_COMP_HISTORY_TABLE);
    if history != 0 {
        regions.push((history, read_usize(G_CLUB_COMP_HISTORY_COUNT).wrapping_mul(0x1a)));
    }
    let (r, rr, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this.addr()), || crate::rng::without_counting(|| con_champ_select_qualified_clubs(this.addr())));
    SCOREBOARD.record(igual && r == rr, || format!("con_champ_select_qualified_clubs({:#x}) listas {:?} original={:x?} rust={:x?}", this.addr(), l.n, mo, mr));
    r
}

/// Comparer for `con_champ_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_con_champ_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CON_CHAMP_UPDATE_FOR_DATE);
    if this.team_table() == 0 {
        return o(this.addr(), today, stage);
    }
    let seleciona = read_usize(this.team_table()) == 0 && stage != 0;
    let n = this.team_count();
    let r = o(this.addr(), today, stage);
    let same = if seleciona { this.team_count() == 8 } else { this.team_count() == n };
    SCOREBOARD.record(same, || format!("con_champ_update_for_date({:#x}, fase {}) seleciona={} times {} -> {}", this.addr(), stage, seleciona, n, this.team_count()));
    r
}

/// Comparer for `con_champ_set_up_rounds`.
pub(crate) unsafe extern "thiscall" fn compare_con_champ_set_up_rounds(this: usize) -> u32 {
    let this = Cup(this);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_CHAMP_SET_UP_ROUNDS)(this.addr());
    SCOREBOARD.record(r == 1 && (this.prize_fund() as usize) == 0x1e8480, || format!("con_champ_set_up_rounds({:#x}) devolveu {} prêmios {}", this.addr(), r, (this.prize_fund() as usize)));
    r
}

/// Comparer for `con_champ_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_con_champ_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CON_CHAMP_CREATE_ROUND_TABLE);
    let a = compare_cup_round_table("con_champ_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| con_champ_create_round_table(this, stage, r, e, s));
    // simplification: compare_cup_round_table does not look at the fixture days (+0x23..); day 0 of the first two rounds is
    // checked here against the fixed value the copy writes
    if a != 0 {
        let ok = (0..2usize).all(|r| {
            let e = a + r * 0x68 + 0x23;
            core::ptr::read_unaligned(e as *const u32) == 0 && core::slice::from_raw_parts((e + 4) as *const u8, 3) == [0xff, 0xff, 0xff]
        });
        SCOREBOARD.record(ok, || format!("con_champ_create_round_table: o dia 0 dos confrontos da original difere"));
    }
    a
}
