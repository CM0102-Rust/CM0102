use super::*;

// The comparers of the routines in ../comp/eurocomp/intertoto_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/intertoto_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `intertoto_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_intertoto_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_INTERTOTO_CUP_CREATE_ROUND_TABLE);
    eng_cc_cup_compare_round_table("intertoto_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| intertoto_cup_create_round_table(this, fase, r, e, s))
}

/// Comparer for `intertoto_cup_create_round_table_92b770` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_intertoto_cup_create_round_table_92b770(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_INTERTOTO_CUP_CREATE_ROUND_TABLE_92B770);
    eng_cc_cup_compare_round_table("intertoto_cup_create_round_table_92b770", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| intertoto_cup_create_round_table_92b770(this, fase, r, e, s))
}

/// Comparer for `intertoto_cup_create_round_table_632340` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_intertoto_cup_create_round_table_632340(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_INTERTOTO_CUP_CREATE_ROUND_TABLE_632340);
    eng_cc_cup_compare_round_table("intertoto_cup_create_round_table_632340", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| intertoto_cup_create_round_table_632340(this, fase, r, e, s))
}

/// Comparer for `intertoto_cup_collect_entrants` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a lista dela em +0xa7, a cópia faz a
/// sua, comparada nos inscritos (com a devolução 1) e liberada.
pub(crate) unsafe extern "thiscall" fn comparar_intertoto_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_INTERTOTO_CUP_COLLECT_ENTRANTS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let antes = read_u32_as_usize(this + 0xa7);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, antes as u32);
        r
    }, || {
        let (r, n) = collect_intertoto_entrants(this);
        lr.set(read_u32_as_usize(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { nr.get().max(0) as usize * 6 } else { 0 };
    let lista = if lo != antes && lr != antes && lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == lr || k == 0
    };
    if lr != 0 && lr != lo && lr != antes {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("intertoto_cup_collect_entrants({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// Comparer for `intertoto_cup_collect_club_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: a original grava a lista dela em `saida` (de quem chama, que a libera); a cópia faz a sua numa saída
/// própria, comparada na conta e nos ponteiros, e a libera.
pub(crate) unsafe extern "thiscall" fn comparar_intertoto_cup_collect_club_list(this: usize, saida: *mut usize, ativos: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(IDX_INTERTOTO_CUP_COLLECT_CLUB_LIST);
    let mut s2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, saida, ativos), || collect_club_list(this, &mut s2, ativos));
    let lo = *saida;
    let lista = if lo != 0 && s2 != 0 {
        core::slice::from_raw_parts(lo as *const u8, a as usize * 4) == core::slice::from_raw_parts(s2 as *const u8, b as usize * 4)
    } else {
        lo == 0 && s2 == 0
    };
    if s2 != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(s2);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("intertoto_cup_collect_club_list({:#x}, {}) devolveu {} x {} lista {}", this, ativos as u8, a, b, lista));
    a
}

/// Comparer for `intertoto_cup_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_intertoto_cup_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_INTERTOTO_CUP_UPDATE_FOR_DATE);
    let uefa = uefa_cup_object();
    if uefa == 0 || read_u32_as_usize(this + 0xb2) != 0 {
        return o(this, hoje, fase);
    }
    let (v, k) = intertoto_winners_for_uefa(this, uefa);
    let n36 = read_i16(uefa + 0x36);
    let r = o(this, hoje, fase);
    let passou = read_u32_as_usize(this + 0xb2) == 1;
    let previsto = if passou { n36.wrapping_add(k as i16) } else { n36 };
    let mut igual = read_i16(uefa + 0x36) == previsto;
    if passou && igual {
        for (j, &c) in v.iter().enumerate().take(k) {
            let l = read_u32_as_usize(uefa + 0x14).wrapping_add(((n36 as isize) + j as isize) as usize * 6);
            igual &= read_u32_as_usize(l) == c && read_u8(l + 4) == 4;
        }
    }
    PLACAR.registrar(igual, || format!("intertoto_cup_update_for_date({:#x}) passou={} livro {} -> {} previsto {} vencedores {:x?}", this, passou, n36, read_i16(uefa + 0x36), previsto, &v[..k]));
    r
}

/// Comparer for `intertoto_cup_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_intertoto_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_INTERTOTO_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    if clube == 0 {
        return o(this, clube, desfecho, a3, a4, jogo, a6);
    }
    let antes = read_u32_as_usize(clube + club::EURO_FLAG);
    let previsto = match desfecho as u8 {
        1 => read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP),
        0xff | 2 => 0xffff_ffff,
        _ => antes,
    };
    let r = o(this, clube, desfecho, a3, a4, jogo, a6);
    let depois = read_u32_as_usize(clube + club::EURO_FLAG);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("intertoto_cup_update_staff_records_for_status({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", clube, desfecho as u8, antes, depois, previsto, r));
    r
}
