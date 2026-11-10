use super::*;

// The comparers of the routines in ../comp/leagues/ita_ser_c2a.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/ita_ser_c2a.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `ita_ser_c2a_update_staff_records_for_status`.
///
/// Comparer of `ita_ser_c2a_update_staff_records_for_status`: the records go to the people's history and change reputations and morale (they cannot run
/// twice), so only the original routine runs.
///
/// The plan, made beforehand, in stages 0 and 1: the club's first row with status 1, 2 or 0xff decides the +0x37 that
/// the routine writes (stage 0: 0 with 1, 0xfe with 2/0xff; stage 1: 0xfe with 1, 3 with 2/0xff); afterwards the row
/// must be like that. In the league it only records: return value 0.
///
/// [pt] Comparador da 0x654180: os registros vão ao histórico das pessoas e mexem nas reputações e no moral (não rodam duas
/// vezes): só a original. O plano, feito antes, nas fases 0 e 1: a primeira linha do clube com situação 1, 2 ou 0xff
/// decide o +0x37 que a rotina grava (fase 0: 0 com 1, 0xfe com 2/0xff; fase 1: 0xfe com 1, 3 com 2/0xff); depois a
/// linha tem de estar assim. Na liga ela só registra: a devolução 0.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ITA_SER_C2A_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let (s, f) = (status as u8, stage as u8);
    let mut plan: Option<(usize, u8)> = None;
    if f <= 1 {
        let n = this.club_count() as i32;
        for i in 0..n.max(0) as usize {
            let l = this.table() + i * 0x3b;
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
    let r = o(this.addr(), club, status, stage, a4, game, a6);
    let same = r == 0 && plan.map_or(true, |(l, v)| read_u8(l + league_table_row::STATUS) == v);
    SCOREBOARD.record(same, || format!("ita_ser_c2a_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} plan={:x?}", this.addr(), club, s, f, r, plan));
    r
}
