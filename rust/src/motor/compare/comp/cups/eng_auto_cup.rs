use super::*;

// The comparers of the routines in ../comp/cups/eng_auto_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/eng_auto_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `select_section_clubs`.
///
/// Side by side from the same state (the random generator, +0xb6, [0xb67a34]); the original routine leaves its list in
/// +0xb2, the copy builds its own (without counting the random draws), compared over the +0xb6 slots and freed.
///
/// When the original does not find the relegated club (the case where it reads the valueless dword from the stack),
/// only the original.
///
/// [pt] Lado a lado a partir do mesmo estado (o sorteador, +0xb6, [0xb67a34]); a original deixa a lista dela em +0xb2, a cópia
/// faz a sua (sem contar os sorteios), comparada nas +0xb6 vagas e liberada. Quando a original não acha o rebaixado (o
/// caso em que ela lê a dword sem valor da pilha), só a original.
pub(crate) unsafe extern "thiscall" fn compare_select_section_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SELECT_SECTION_CLUBS);
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let tem_rebaixado = (0..read_u32_as_usize(G_CLUB_COUNT) as i32).any(|i| {
        let c = base + i as usize * club::SIZE;
        let before = read_u32_as_usize(c + club::LAST_DIVISION);
        eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) && before != 0 && read_u32_as_usize(before) == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION)
    });
    if !tem_rebaixado {
        return o(this);
    }
    let before = read_u32_as_usize(this + eng_auto_cup::SELECTED_CLUBS);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(this + eng_auto_cup::SELECTED_COUNT, 4usize), (0xb67a34, 4)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + eng_auto_cup::SELECTED_CLUBS));
        write_u32_from_usize(this + eng_auto_cup::SELECTED_CLUBS, before);
        r
    }, || {
        let r = crate::rng::without_counting(|| eng_auto_cup_select_section_clubs(this));
        lr.set(read_u32_as_usize(this + eng_auto_cup::SELECTED_CLUBS));
        write_u32_from_usize(this + eng_auto_cup::SELECTED_CLUBS, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = 4 * read_u32_as_usize(this + eng_auto_cup::SELECTED_COUNT).min(0x38);
    let list = if lo != 0 && lr != 0 && a == 1 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        true
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("select_section_clubs({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, list));
    a
}

/// Comparer for `create_section_stage`.
///
/// [pt] Só a original (aloca a fase), conferindo o plano: a fase em [+0xc][0] e +0x30 = 0.
pub(crate) unsafe extern "thiscall" fn compare_create_section_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_SECTION_STAGE);
    o(this);
    let f = read_u32_as_usize(read_u32_as_usize(this + cup::STAGES));
    SCOREBOARD.record(f != 0 && read_u32_as_usize(this + cup::CURRENT_STAGE) == 0, || format!("create_section_stage({:#x}) fase {:#x} +0x30 {}", this, f, read_u32_as_usize(this + cup::CURRENT_STAGE) as i32));
}

/// Comparer for `eng_auto_cup_advance_stage`.
///
/// [pt] Só a original (a final aloca e sorteia), conferindo o plano: +0x30 sobe um fora da última fase e fica na última.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_AUTO_CUP_ADVANCE_STAGE);
    let before = read_u32_as_usize(this + cup::CURRENT_STAGE) as i32;
    let last = before >= (read_u32_as_usize(this + cup::STAGE_COUNT) as i32).wrapping_sub(1);
    o(this);
    let after = read_u32_as_usize(this + cup::CURRENT_STAGE) as i32;
    SCOREBOARD.record(after == if last { before } else { before + 1 }, || format!("eng_auto_cup_advance_stage({:#x}) fase {} -> {}", this, before, after));
}

/// Comparer for `eng_auto_cup_create_final_stage`.
///
/// [pt] Só a original (aloca a fase e sorteia), conferindo o plano: a fase em [+0xc][1] (com o vetor dos dois finalistas
/// achados antes, quando há fase).
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_create_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_AUTO_CUP_CREATE_FINAL_STAGE);
    o(this);
    let f = read_u32_as_usize(read_u32_as_usize(this + cup::STAGES) + 4);
    SCOREBOARD.record(f != 0, || format!("eng_auto_cup_create_final_stage({:#x}) sem a fase da final", this));
}

/// Comparer for `eng_auto_cup_rank_teams`.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_rank_teams(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_AUTO_CUP_RANK_TEAMS)(this)
}

/// Comparer for `eng_auto_cup_collect_entrants`.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_AUTO_CUP_COLLECT_ENTRANTS);
    eng_cc_cup_compare_entrant_list("eng_auto_cup_collect_entrants", this, &[(this + cup::TEAM_COUNT, 2), (0xb67a34, 4)], || o(this), || collect_auto_cup_entrants(this), |_, n| n.max(0) as usize * 6)
}

/// Comparer for `eng_auto_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_eng_auto_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_AUTO_CUP_CREATE_ROUND_TABLE);
    asia_club_champ_compare_round_table("eng_auto_cup_create_round_table", stage, ldv_trophy_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| eng_auto_cup_create_round_table(this, stage, r, e, l))
}
