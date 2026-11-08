use super::*;

// The comparers of the routines in ../comp/uefa_seeding.rs.
// [pt] Os comparadores das rotinas de ../comp/uefa_seeding.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `roll_coefficients`.
///
/// [pt] Lado a lado: a tabela das nações, as temporadas de cada nação e a posição gravada nas seleções.
pub(crate) unsafe extern "thiscall" fn compare_roll_coefficients(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ROLL_COEFFICIENTS);
    let reg = seeding_regions(this);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || roll_coefficients(this));
    SCOREBOARD.record(igual, || format!("roll_coefficients: diferenças em {:x?}", memory_diff_addresses(&mo, &mr)));
}

/// Comparer for `set_place_counts`.
///
/// [pt] Lado a lado: a tabela das nações, o sorteio (a cópia sem contar) e a marca da caixa.
pub(crate) unsafe extern "thiscall" fn compare_set_place_counts(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SET_PLACE_COUNTS);
    let mut reg = vec![(this, 0x43b), (0xb67a34, 4)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || crate::rng::without_counting(|| set_place_counts(this)));
    SCOREBOARD.record(igual, || format!("set_place_counts: diferenças em {:x?}", memory_diff_addresses(&mo, &mr)));
}

/// Comparer for `uefa_find_nth_unplaced_club_of_nation`.
///
/// [pt] A devolução das duas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn compare_uefa_find_nth_unplaced_club_of_nation(this: usize, clubs: usize, count: u32, nation: u32, n: u32, without_slot: u32, without_human: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize>(IDX_UEFA_FIND_NTH_UNPLACED_CLUB_OF_NATION);
    let r = o(this, clubs, count, nation, n, without_slot, without_human);
    let c = uefa_find_nth_unplaced_club_of_nation(this, clubs, count, nation, n, without_slot, without_human);
    SCOREBOARD.record(r == c, || format!("uefa_find_nth_unplaced_club_of_nation(nação {:x}, n {}, {} {}): original {:x} rust {:x}", nation, n as u8, without_slot, without_human, r, c));
    r
}

/// Comparer for `collect_european_clubs`.
///
/// [pt] Lado a lado: a original com a saída de quem chama, a cópia com uma saída própria (liberada depois); os clubes, a
/// linha de cada um em [0xae2d14], a devolução e os registros das duas tabelas.
pub(crate) unsafe extern "thiscall" fn compare_collect_european_clubs(this: usize, out: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize) -> u32>(IDX_COLLECT_EUROPEAN_CLUBS);
    let mut t2 = 0usize;
    let reg = club_regions();
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, out), || collect_european_clubs(this, &mut t2));
    let (n, m) = (a as u16 as i16, b as u16 as i16);
    let t1 = *out;
    let table = n == m && (n <= 0 || t1 == 0 || t2 == 0 || core::slice::from_raw_parts(t1 as *const u8, n as usize * 12) == core::slice::from_raw_parts(t2 as *const u8, n as usize * 12));
    if t2 != 0 {
        exe::crt_free(t2);
    }
    SCOREBOARD.record(igual && table, || format!("collect_european_clubs: {} e {} clubes, tabela {}, diferenças em {:x?}", n, m, table, memory_diff_addresses(&mo, &mr)));
    a
}

/// Comparer for `assign_champions_league_places`.
///
/// [pt] Lado a lado: a tabela das nações, os clubes e a marca da caixa.
pub(crate) unsafe extern "thiscall" fn compare_assign_champions_league_places(this: usize, clubs: usize, count: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ASSIGN_CHAMPIONS_LEAGUE_PLACES);
    let mut reg = club_regions();
    reg.push((this, 0x440));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubs, count), || assign_champions_league_places(this, clubs, count));
    SCOREBOARD.record(igual, || format!("assign_champions_league_places: diferenças em {:x?}", memory_diff_addresses(&mo, &mr)));
}

/// Comparer for `assign_intertoto_places`.
///
/// [pt] Lado a lado em 2001 (a 0x903890 devolve 0 sem efeito) e sem a lista dos voluntários (soltá-la duas vezes não dá):
/// a tabela das nações, os clubes e o sorteio (a cópia sem contar). Fora disso, só a original (a 0x903890 sorteia e
/// entrega notícia).
pub(crate) unsafe extern "thiscall" fn compare_assign_intertoto_places(this: usize, clubs: usize, count: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ASSIGN_INTERTOTO_PLACES);
    if *(G_CURRENT_DATE_YEAR as *const i16) != 0x7d1 || read_u32(this + uefa_seeding::INTERTOTO_VOLUNTEERS) != 0 {
        return o(this, clubs, count);
    }
    let mut reg = club_regions();
    reg.push((this, 0x440));
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubs, count), || crate::rng::without_counting(|| assign_intertoto_places(this, clubs, count)));
    SCOREBOARD.record(igual, || format!("assign_intertoto_places: diferenças em {:x?}", memory_diff_addresses(&mo, &mr)));
}

/// Comparer for `assign_uefa_cup_places`.
///
/// [pt] Lado a lado: a classificação (com a tabela das finais em +0x440), os clubes, o sorteio (a cópia sem contar) e a marca
/// da caixa.
pub(crate) unsafe extern "thiscall" fn compare_assign_uefa_cup_places(this: usize, clubs: usize, count: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ASSIGN_UEFA_CUP_PLACES);
    let n = (read_u32(G_CLUB_COUNT) as i32).max(0) as usize;
    // the competition history too: 0x4afd80 reorders it (tied rows: the previous order changes the club found)
    let mut reg = vec![(this, 0x698), (read_u32(G_CLUB_TABLE) as usize, n * club::SIZE), (0xb67a34, 4), (read_u32(G_CLUB_COMP_HISTORY_TABLE) as usize, (read_u32(G_CLUB_COMP_HISTORY_COUNT) as i32).max(0) as usize * 0x1a)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubs, count), || crate::rng::without_counting(|| assign_uefa_cup_places(this, clubs, count)));
    SCOREBOARD.record(igual, || format!("assign_uefa_cup_places: diferenças em {:x?}", memory_diff_addresses(&mo, &mr)));
}

/// Comparer for `pick_english_intertoto_club`.
///
/// [pt] Em 2001 (devolve 0 sem efeito): a devolução das duas. Fora de 2001 só a original (sorteia, aloca a lista e entrega a
/// notícia).
pub(crate) unsafe extern "thiscall" fn compare_pick_english_intertoto_club(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_PICK_ENGLISH_INTERTOTO_CLUB);
    if read_i16(G_CURRENT_DATE_YEAR) != 0x7d1 {
        return o(this);
    }
    let r = o(this);
    let c = pick_english_intertoto_club(this);
    SCOREBOARD.record(r == c, || format!("pick_english_intertoto_club: original {:x} rust {:x}", r, c));
    r
}

/// Comparer for `update_season`.
///
/// [pt] Só a original (refaz as vagas pelas rotinas de dentro, solta a tabela, entrega notícias); a cópia leva o contador.
pub(crate) unsafe extern "thiscall" fn compare_update_season(this: usize, diario: u32) {
    // simplification: redoes the whole table and delivers news items: cannot run twice
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_UPDATE_SEASON)(this, diario)
}

/// Comparer for `uefa_seeding_save_to_temp_file`.
///
/// [pt] Comparador da 0x905510: só a original.
pub(crate) unsafe extern "thiscall" fn compare_uefa_seeding_save_to_temp_file(this: usize) {
    // simplification: writes a file to CM3_TEMP (cannot run twice); the copy is proven by the save proof
    // (the .tmp goes into the .qst via zadd_matching_files)
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UEFA_SEEDING_SAVE_TO_TEMP_FILE)(this)
}
