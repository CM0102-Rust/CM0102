use super::*;

// The comparers of the routines in ../comp/leagues/ita_ser_c2a.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/ita_ser_c2a.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_ser_c2a_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x654180: os registros vão ao histórico das pessoas e mexem nas reputações e no moral (não rodam duas
/// vezes): só a original. O plano, feito antes, nas fases 0 e 1: a primeira linha do clube com situação 1, 2 ou 0xff
/// decide o +0x37 que a rotina grava (fase 0: 0 com 1, 0xfe com 2/0xff; fase 1: 0xfe com 1, 3 com 2/0xff); depois a
/// linha tem de estar assim. Na liga ela só registra: a devolução 0.
pub(crate) unsafe extern "thiscall" fn comparar_ita_ser_c2a_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ITA_SER_C2A_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let (s, f) = (status as u8, stage as u8);
    let mut plan: Option<(usize, u8)> = None;
    if f <= 1 {
        let n = read_i16(this + 0x3e) as i32;
        for i in 0..n.max(0) as usize {
            let l = read_usize(this + 0xb1) + i * 0x3b;
            if read_usize(l) == club && (s == 1 || s == 2 || s == 0xff) {
                plan = Some((l, match (f, s) {
                    (0, 1) => 0,
                    (0, _) => 0xfe,
                    (_, 1) => 0xfe,
                    _ => 3,
                }));
                break;
            }
        }
    }
    let r = o(this, club, status, stage, a4, game, a6);
    let same = r == 0 && plan.map_or(true, |(l, v)| *((l + 0x37) as *const u8) == v);
    SCOREBOARD.record(same, || format!("ita_ser_c2a_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, club, s, f, r, plan));
    r
}
