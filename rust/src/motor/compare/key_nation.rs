use super::*;

// The comparers of the routines in ../key_nation.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../key_nation.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `key_nation_index_of_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_index_of_nation(nacao: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_NATION)(nacao);
    let n = key_nation_index_of_nation(nacao);
    PLACAR.registrar(r == n, || format!("key_nation_index_of_nation(nação {:x}) original={} rust={}", nacao, r, n));
    r
}

/// Comparer for `key_nation_index_of_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_index_of_club(clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_CLUB);
    if hits_key_nation_error(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = key_nation_index_of_club(clube);
    PLACAR.registrar(r == n, || format!("key_nation_index_of_club(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `key_nation_get_club_season_year` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_get_club_season_year(clube: usize) -> u16 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_KEY_NATION_GET_CLUB_SEASON_YEAR);
    if clube != 0 && hits_key_nation_error(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = key_nation_get_club_season_year(clube);
    PLACAR.registrar(r == n, || format!("key_nation_get_club_season_year(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `key_nation_club_in_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_club_in_season(clube: usize, inicio: usize, fim: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_KEY_NATION_CLUB_IN_SEASON)(clube, inicio, fim);
    let (mut i, mut f) = ([0u8; 8], [0u8; 8]);
    let n = key_nation_club_in_season(
        clube,
        if inicio != 0 { i.as_mut_ptr() as usize } else { 0 },
        if fim != 0 { f.as_mut_ptr() as usize } else { 0 },
    );
    let ler = |p: usize| if p != 0 { *(p as *const [u8; 8]) } else { [0u8; 8] };
    let (oi, of) = (ler(inicio), ler(fim));
    let ok = r == n && (inicio == 0 || oi == i) && (fim == 0 || of == f);
    PLACAR.registrar(ok, || {
        format!("key_nation_club_in_season(clube {:x}) original={} {:02x?} {:02x?} rust={} {:02x?} {:02x?}", clube, r, oi, of, n, i, f)
    });
    r
}

/// Comparer for `key_nation_get_main_continent` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_get_main_continent() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(IDX_KEY_NATION_GET_MAIN_CONTINENT)();
    let n = key_nation_get_main_continent();
    PLACAR.registrar(r == n, || format!("key_nation_get_main_continent() original={:x} rust={:x}", r, n));
    r
}

/// Comparer for `key_nation_get_main_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_get_main_nation() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(IDX_KEY_NATION_GET_MAIN_NATION)();
    let n = key_nation_get_main_nation();
    PLACAR.registrar(r == n, || format!("key_nation_get_main_nation() original={:x} rust={:x}", r, n));
    r
}

/// Comparer for `key_nation_is_world_cup_comp` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_is_world_cup_comp(comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_KEY_NATION_IS_WORLD_CUP_COMP);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = key_nation_is_world_cup_comp(comp);
    PLACAR.registrar(r == n, || format!("key_nation_is_world_cup_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

/// Comparer for `key_nation_is_international_comp` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_is_international_comp(comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_KEY_NATION_IS_INTERNATIONAL_COMP);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = key_nation_is_international_comp(comp);
    PLACAR.registrar(r == n, || format!("key_nation_is_international_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

/// Comparer for `key_nation_index_of_comp` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_key_nation_index_of_comp(comp: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_COMP)(comp);
    let n = key_nation_index_of_comp(comp);
    PLACAR.registrar(r == n, || format!("key_nation_index_of_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}
