use super::*;

// The comparers of the routines in ../comp/leagues/eng_second.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/eng_second.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `eng_second_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_SECOND_CREATE_SCHEDULE);
    eng_second_compare_round_table("eng_second_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_second_create_schedule(this, fase, r, e, l))
}

/// Comparer for `eng_second_create_schedule_5770e0` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_create_schedule_5770e0(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_SECOND_CREATE_SCHEDULE_5770E0);
    eng_second_compare_round_table("eng_second_create_schedule_5770e0", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| eng_second_create_schedule_5770e0(this, fase, r, e, l))
}

/// Comparer for `eng_second_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x576c50: os registros gravam o histórico no arquivo, sorteiam o moral e podem renovar contratos
/// (não rodam duas vezes): só a original. O plano, feito antes, no mata-mata: a primeira linha do clube decide o que a
/// rotina grava (+0x37 = 0 e saiu = 1 com 1; +0x37 = 0xfe com 0xff ou 2; nada com 0, que segue procurando outra linha
/// do mesmo clube); depois a linha e `saiu` têm de estar assim. Na liga a rotina não grava nada: a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, saiu: usize, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(IDX_ENG_SECOND_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let s = situacao as u8;
    let mut plano: Option<(usize, u8, bool)> = None;
    if fase as u8 != 0xff {
        let tab = read_usize(this + 0xb1);
        let n = *((this + 0x3e) as *const i16) as i32;
        let mut i: i32 = 0;
        while i < n {
            let linha = tab + (i as usize) * 0x3b;
            if read_usize(linha) == clube {
                match s {
                    1 => {
                        plano = Some((linha, 0, true));
                        break;
                    }
                    0xff | 2 => {
                        plano = Some((linha, 0xfe, false));
                        break;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }
    let r = o(this, clube, situacao, fase, saiu, a, b);
    let igual = r == 0
        && match plano {
            Some((linha, v, marca)) => read_u8(linha + 0x37) == v && (!marca || read_u8(saiu) == 1),
            None => true,
        };
    PLACAR.registrar(igual, || format!("eng_second_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, clube, s, fase as u8, r, plano));
    r
}

/// Comparer for `eng_second_set_club_progress` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_ENG_SECOND_SET_CLUB_PROGRESS);
    compare_club_stage_status("eng_second_set_club_progress", this, clube, || o(this, clube, fase, atual, melhor, pior), || eng_second_set_club_progress(this, clube, fase, atual, melhor, pior))
}

/// Comparer for `eng_second_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_SECOND_ADVANCE_STAGE);
    compare_stage_advance("eng_second_advance_stage", this, || o(this), || eng_second_advance_stage(this))
}

/// Comparer for `eng_second_create_play_off_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_SECOND_CREATE_PLAY_OFF_STAGE);
    compare_playoff_stage("eng_second_create_play_off_stage", this, [5, 2, 4, 3], || o(this))
}

/// Comparer for `construct_eng_second` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_eng_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_SECOND);
    compare_english_constructor("construct_eng_second", this, ano, registro, &ENG_SECOND_DIVISION, || o(this, ano, registro))
}

/// Comparer for `eng_second_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_second_set_up_rules", this, eng_prm_second_division_rules(), || o(this), || eng_second_set_up_rules(this))
}

/// Comparer for `eng_second_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_eng_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_SECOND_CREATE_TABLE);
    eng_prm_compare_league_table("eng_second_create_table", this, 0x18, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, 0x4e20, || o(this))
}
