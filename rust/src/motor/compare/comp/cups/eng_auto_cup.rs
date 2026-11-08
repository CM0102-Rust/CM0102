use super::*;

// The comparers of the routines in ../comp/cups/eng_auto_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/eng_auto_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `select_section_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado a partir do mesmo estado (o sorteador, +0xb6, [0xb67a34]); a original deixa a lista dela em +0xb2, a cópia
/// faz a sua (sem contar os sorteios), comparada nas +0xb6 vagas e liberada. Quando a original não acha o rebaixado (o
/// caso em que ela lê a dword sem valor da pilha), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_select_section_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SELECT_SECTION_CLUBS);
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let tem_rebaixado = (0..read_u32_as_usize(G_CLUB_COUNT) as i32).any(|i| {
        let c = base + i as usize * club::SIZE;
        let antes = read_u32_as_usize(c + 0x5b);
        eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) && antes != 0 && read_u32_as_usize(antes) == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION)
    });
    if !tem_rebaixado {
        return o(this);
    }
    let antes = read_u32_as_usize(this + 0xb2);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0xb6, 4usize), (0xb67a34, 4)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xb2));
        write_u32_from_usize(this + 0xb2, antes);
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| eng_auto_cup_select_section_clubs(this));
        lr.set(read_u32_as_usize(this + 0xb2));
        write_u32_from_usize(this + 0xb2, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = 4 * read_u32_as_usize(this + 0xb6).min(0x38);
    let lista = if lo != 0 && lr != 0 && a == 1 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        true
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("select_section_clubs({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// Comparer for `create_section_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (aloca a fase), conferindo o plano: a fase em [+0xc][0] e +0x30 = 0.
pub(crate) unsafe extern "thiscall" fn comparar_create_section_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_SECTION_STAGE);
    o(this);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc));
    PLACAR.registrar(f != 0 && read_u32_as_usize(this + 0x30) == 0, || format!("create_section_stage({:#x}) fase {:#x} +0x30 {}", this, f, read_u32_as_usize(this + 0x30) as i32));
}

/// Comparer for `eng_auto_cup_advance_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (a final aloca e sorteia), conferindo o plano: +0x30 sobe um fora da última fase e fica na última.
pub(crate) unsafe extern "thiscall" fn comparar_eng_auto_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_AUTO_CUP_ADVANCE_STAGE);
    let antes = read_u32_as_usize(this + 0x30) as i32;
    let ultima = antes >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("eng_auto_cup_advance_stage({:#x}) fase {} -> {}", this, antes, depois));
}

/// Comparer for `eng_auto_cup_create_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (aloca a fase e sorteia), conferindo o plano: a fase em [+0xc][1] (com o vetor dos dois finalistas
/// achados antes, quando há fase).
pub(crate) unsafe extern "thiscall" fn comparar_eng_auto_cup_create_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_AUTO_CUP_CREATE_FINAL_STAGE);
    o(this);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4);
    PLACAR.registrar(f != 0, || format!("eng_auto_cup_create_final_stage({:#x}) sem a fase da final", this));
}

/// Comparer for `eng_auto_cup_rank_teams` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_auto_cup_rank_teams(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_AUTO_CUP_RANK_TEAMS)(this)
}

/// Comparer for `eng_auto_cup_collect_entrants` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_auto_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_AUTO_CUP_COLLECT_ENTRANTS);
    eng_cc_cup_compare_entrant_list("eng_auto_cup_collect_entrants", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || collect_auto_cup_entrants(this), |_, n| n.max(0) as usize * 6)
}

/// Comparer for `eng_auto_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_auto_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_AUTO_CUP_CREATE_ROUND_TABLE);
    asia_club_champ_compare_round_table("eng_auto_cup_create_round_table", fase, ldv_trophy_stage(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| eng_auto_cup_create_round_table(this, fase, r, e, l))
}
