use super::*;

// The comparers of the routines in ../award/month_award.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../award/month_award.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `award_best_rated_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_award_best_rated_player(this: usize, notas: usize, idade: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_AWARD_BEST_RATED_PLAYER)(this, notas, idade)
}

/// Comparer for `set_winner` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_winner(this: usize, id: i32, clube_id: i32, mes: u32, ano: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, u32)>(IDX_SET_WINNER);
    let mut regioes = vec![(this + 0x1d, 12 * 11)];
    if id > -1 {
        winner_reputation_regions(&mut regioes, month_ratings_staff_by_id(id), if clube_id > -1 { month_award_club_by_id(clube_id) } else { 0 });
    }
    compare_month_award("set_winner", &regioes, || o(this, id, clube_id, mes, ano), || set_winner(this, id, clube_id, mes, ano));
}

/// Comparer for `award_raise_winner_reputation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_award_raise_winner_reputation(pessoa: usize, clube: usize, nac: u32, mundo: u32, jogador: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(IDX_AWARD_RAISE_WINNER_REPUTATION);
    let mut regioes = Vec::new();
    winner_reputation_regions(&mut regioes, pessoa, clube);
    compare_month_award("award_raise_winner_reputation", &regioes, || o(pessoa, clube, nac, mundo, jogador), || award_raise_winner_reputation(pessoa, clube, nac, mundo, jogador));
}
