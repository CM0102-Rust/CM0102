use super::*;

// The comparers of the routines in ../award/month_ratings.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../award/month_ratings.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `record_match_ratings` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador das notas da partida: lado a lado (cabeçalho e as vagas da tabela que podem mudar) quando cabem as 40
/// vagas sem crescer a tabela; senão só a original (a 0x767fa0 realoca e a original não pode rodar duas vezes).
pub(crate) unsafe extern "thiscall" fn comparar_record_match_ratings(this: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RECORD_MATCH_RATINGS);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    if tab == 0 || n < 0 || cap.wrapping_sub(n) < 40 {
        // simplification: a tabela pode crescer (realloc): só a original; a 0x767fa0 compara cada nota
        return o(this, jogo, partida);
    }
    let regioes = [(this, 12), (tab, (n as usize + 40) * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, partida), || record_match_ratings(this, jogo, partida));
    PLACAR.registrar(igual, || format!("record_match_ratings(this {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `add_player_rating` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da nota do mês: lado a lado (cabeçalho e os registros até o novo) quando a tabela não precisa crescer;
/// senão só a original (realloc).
pub(crate) unsafe extern "thiscall" fn comparar_add_player_rating(this: usize, e: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_PLAYER_RATING);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    let achou = tab != 0 && (0..n.max(0) as usize).any(|k| read_i32(tab + k * 12) == read_i32(e));
    if tab == 0 || n < 0 || (n >= cap && !achou) {
        // simplification: a tabela cresce (malloc/realloc): só a original
        return o(this, e);
    }
    let vagas = if n < cap { n as usize + 1 } else { n as usize };
    let regioes = [(this, 12), (tab, vagas * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, e), || add_player_rating(this, e));
    PLACAR.registrar(igual, || format!("add_player_rating(pessoa {}) original={:x?} rust={:x?}", read_i32(e), mo, mr));
}

/// Comparer for `find_best_player` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador do melhor do mês: lado a lado pela tabela ordenada e pelo registro devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_find_best_player(this: usize, liga: i32, idade: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32) -> usize>(IDX_FIND_BEST_PLAYER);
    let (tab, n) = (read_usize(this), read_i32(this + 4));
    let regioes: Vec<(usize, usize)> = if tab != 0 && n > 0 { vec![(tab, n as usize * 12)] } else { Vec::new() };
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, liga, idade), || find_best_player(this, liga, idade));
    PLACAR.registrar(igual && r == m, || format!("find_best_player(liga {}, idade {}) original={:#x} {:x?} rust={:#x} {:x?}", liga, idade as u8 as i8, r, mo, m, mr));
    r
}
