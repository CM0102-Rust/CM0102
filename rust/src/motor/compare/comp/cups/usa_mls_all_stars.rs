use super::*;

// The comparers of the routines in ../comp/cups/usa_mls_all_stars.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/usa_mls_all_stars.rs (modo comparação: ver o cabeçalho do motor.rs).



/// Comparer for `usa_mls_all_stars_start_new_season_90a760` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_all_stars_start_new_season_90a760(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_ALL_STARS_START_NEW_SEASON_90A760);
    compare_cup_new_season("usa_mls_all_stars_start_new_season_90a760", this, || o(this))
}

/// Comparer for `set_up_teams` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado a partir do mesmo estado (o sorteador, +0x3e, os dois clubes das estrelas e [0xb67a34]); a original deixa
/// a lista dela em +0xa7, a cópia faz a sua (sem contar o sorteio), comparada nos 12 bytes e liberada. A lista antiga
/// (+0xa7 na entrada) não é liberada por nenhuma das duas (a virada já a liberou).
pub(crate) unsafe extern "thiscall" fn comparar_set_up_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SET_UP_TEAMS);
    let before = read_u32_as_usize(this + 0xa7);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(this + 0x3e, 2usize), (0xb67a34, 4), (usa_mls_all_stars_club_by_index(read_u32_as_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST)), 0x245), (usa_mls_all_stars_club_by_index(read_u32_as_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST)), 0x245)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xa7));
        write_u32_from_usize(this + 0xa7, before);
        r
    }, || {
        let r = crate::rng::without_counting(|| set_up_all_star_teams(this));
        lr.set(read_u32_as_usize(this + 0xa7));
        write_u32_from_usize(this + 0xa7, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0xc) == core::slice::from_raw_parts(lr as *const u8, 0xc)
    } else {
        lo == lr
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("set_up_teams({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, list));
    a
}

/// Comparer for `usa_mls_all_stars_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_all_stars_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_USA_MLS_ALL_STARS_UPDATE_FOR_DATE)(this, today, stage)
}

/// Comparer for `usa_mls_all_stars_create_round_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). Fora da
/// fase −1, só a original (nenhuma grava nada).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_all_stars_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_USA_MLS_ALL_STARS_CREATE_ROUND_TABLE);
    if stage as u8 != 0xff {
        let a = o(this, stage, rounds, extra, out);
        SCOREBOARD.record(a == 0, || format!("usa_mls_all_stars_create_round_table(fase {}) devolveu {:#x}", stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut s2 = if out.is_null() { 0 } else { *out };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, stage, rounds, extra, out),
        || usa_mls_all_stars_create_round_table(this, stage, &mut r2, &mut e2, if out.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let outs = *rounds == r2 && *extra == e2 && (out.is_null() || *out == s2);
    let table = if a != 0 && b != 0 { spa_super_written_round_bytes(a) == spa_super_written_round_bytes(b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("usa_mls_all_stars_create_round_table: saídas {} tabela {}", outs, table));
    a
}
