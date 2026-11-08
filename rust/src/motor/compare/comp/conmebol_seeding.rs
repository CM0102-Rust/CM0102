use super::*;

// The comparers of the routines in ../comp/conmebol_seeding.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/conmebol_seeding.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Lado a lado: os clubes do jogo e a marca da caixa.
pub(crate) unsafe extern "thiscall" fn comparar_assign_libertadores_places(this: usize, clubs: usize, count: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ASSIGN_LIBERTADORES_PLACES);
    let n = (read_u32(G_CLUB_COUNT) as i32).max(0) as usize;
    // the competition history too: 0x4afd80 reorders it (tied rows: the previous order changes the club found)
    let reg = [(read_u32(G_CLUB_TABLE) as usize, n * club::SIZE), (0xb67a34, 4), (read_u32(G_CLUB_COMP_HISTORY_TABLE) as usize, (read_u32(G_CLUB_COMP_HISTORY_COUNT) as i32).max(0) as usize * 0x1a)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubs, count), || assign_libertadores_places(this, clubs, count));
    SCOREBOARD.record(igual, || {
        let base = read_u32(G_CLUB_TABLE) as usize;
        let l: Vec<String> = memory_diff_addresses(&mo, &mr).iter().map(|&a| {
            if a < base || a - base >= mo.0[0].1.len() {
                return format!("{:x}", a);
            }
            let (q, k) = ((a - base) / 0x245, (a - base) % 0x245);
            let (x, y) = (mo.0[0].1[a - base], mr.0[0].1[a - base]);
            format!("clube {} +{:x}: {:02x}/{:02x}", q, k, x, y)
        }).collect();
        format!("assign_libertadores_places (ano {}): {}", *(G_CURRENT_DATE_YEAR as *const u16), l.join(", "))
    });
}

/// Comparer for `find_nth_unqualified_club_of_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] A devolução das duas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_nth_unqualified_club_of_nation(this: usize, clubs: usize, count: u32, nation: u32, n: u32, without_slot: u32, without_human: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize>(IDX_FIND_NTH_UNQUALIFIED_CLUB_OF_NATION);
    let r = o(this, clubs, count, nation, n, without_slot, without_human);
    let c = find_nth_unqualified_club_of_nation(this, clubs, count, nation, n, without_slot, without_human);
    SCOREBOARD.record(r == c, || format!("find_nth_unqualified_club_of_nation(nação {:x}, n {}, {} {}): original {:x} rust {:x}", nation, n as u8, without_slot, without_human, r, c));
    r
}

/// Comparer for `is_mercosur_invited_club` (comparison mode: see the header of motor.rs).
///
/// [pt] A devolução das duas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn comparar_is_mercosur_invited_club(this: usize, club: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_MERCOSUR_INVITED_CLUB);
    let r = o(this, club);
    let c = is_mercosur_invited_club(this, club);
    SCOREBOARD.record(r == c, || format!("is_mercosur_invited_club({:x}): original {} rust {}", club, r, c));
    r
}

/// Comparer for `collect_conmebol_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: a original com a saída de quem chama, a cópia com uma saída própria (liberada depois); os clubes, a
/// devolução e os registros das duas tabelas.
pub(crate) unsafe extern "stdcall" fn comparar_collect_conmebol_clubs(out: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(*mut usize) -> u32>(IDX_COLLECT_CONMEBOL_CLUBS);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut t2 = 0usize;
    let reg = club_seeding_regions();
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(out), || collect_conmebol_clubs(&mut t2));
    let (n, m) = (a as u16 as i16, b as u16 as i16);
    let t1 = *out;
    let table = n == m && (n <= 0 || t1 == 0 || t2 == 0 || core::slice::from_raw_parts(t1 as *const u8, n as usize * 12) == core::slice::from_raw_parts(t2 as *const u8, n as usize * 12));
    if t2 != 0 {
        soltar(t2);
    }
    SCOREBOARD.record(igual && table, || format!("collect_conmebol_clubs: {} e {} clubes, tabela {}, diferenças em {:x?}", n, m, table, memory_diff_addresses(&mo, &mr)));
    a
}

/// Comparer for `update_for_new_season` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: o ano gravado (+0x6f), os clubes e a marca da caixa (cada uma monta e solta a própria tabela).
pub(crate) unsafe extern "thiscall" fn comparar_update_for_new_season(this: usize, pular: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_UPDATE_FOR_NEW_SEASON);
    let mut reg = club_seeding_regions();
    reg.push((this + 0x6f, 2));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, pular), || update_for_new_season(this, pular));
    SCOREBOARD.record(igual, || format!("update_for_new_season({}): diferenças em {:x?}", pular, memory_diff_addresses(&mo, &mr)));
}

/// Lado a lado: o +0x71.
pub(crate) unsafe extern "thiscall" fn comparar_set_bra_champ_cup_winner(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SET_BRA_CHAMP_CUP_WINNER);
    let reg = [(this + 0x71, 4)];
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, club), || set_bra_champ_cup_winner(this, club));
    SCOREBOARD.record(igual, || format!("set_bra_champ_cup_winner({:x})", club));
}

/// Comparer for `conmebol_seeding_save_to_temp_file` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4c6b70: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_seeding_save_to_temp_file(this: usize) {
    // simplification: writes a file to CM3_TEMP (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_SEEDING_SAVE_TO_TEMP_FILE)(this)
}
