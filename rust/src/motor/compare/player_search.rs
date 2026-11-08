use super::*;

// The comparers of the routines in ../player_search.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../player_search.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `player_search_record_scouted_club_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7bd310: lado a lado nas 50 vagas (o registro inteiro). Com as máscaras ligadas e o técnico do clube
/// humano, a 0x5b09d0 grava o que se sabe do jogador e sorteia: aí só a original roda (simplification: o caso de um humano
/// observando o próprio clube; a carreira automática não chega a esta rotina).
pub(crate) unsafe extern "thiscall" fn comparar_player_search_record_scouted_club_match(this: usize, game: usize, result: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_PLAYER_SEARCH_RECORD_SCOUTED_CLUB_MATCH);
    let club_obs = friendly_club_by_id(read_usize(this));
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 && read_usize(club_obs + club::MANAGER) != 0 {
        o(this, game, result);
        return;
    }
    let regions = [(this, 4 + 0x32 * 0x1a2), (0xae24d0, 0x100), (0xb67a34, 4)];
    let ((), (), igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, game, result),
        || player_search_record_scouted_club_match(this, game, result));
    SCOREBOARD.record(igual, || format!("player_search_record_scouted_club_match(clube {:#x}, jogo {:#x}) {:x?} x {:x?}", read_usize(this), game, mo, mr));
}

/// Comparer for `player_search_scout_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7b5be0: o relatório sorteia (0x7b62a0, 0x5b09d0) e põe registros na saída (aloca): só a original
/// roda. Antes, a cópia em ensaio decide até o relatório sem os efeitos (sem o histórico e sem o relatório, com o estado
/// do sorteador guardado e devolvido) e diz a devolução e se o começo da lista vai a amanhã; a original tem de bater.
pub(crate) unsafe extern "thiscall" fn comparar_player_search_scout_match(this: usize, game: usize, result: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_PLAYER_SEARCH_SCOUT_MATCH);
    if game == 0 || result == 0 {
        return o(this, game, result, out);
    }
    let state = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let before = (read_usize(this + 0x1a5), read_usize(this + 0x1a9));
    SCOUT_MATCH_DRY_RUN = true;
    SCOUT_MATCH_WOULD_REPORT = false;
    let n = crate::rng::without_counting(|| player_search_scout_match(this, game, result, out));
    SCOUT_MATCH_DRY_RUN = false;
    let relata = SCOUT_MATCH_WOULD_REPORT;
    state.write_back();
    let previsto = if relata {
        let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let mut t = [0u32; 2];
        let a = more(G_CURRENT_DATE, t.as_mut_ptr() as usize, 1);
        (read_i16(a + 2) as u16, read_i16(a))
    } else {
        (read_i16(this + 0x1a7) as u16, read_i16(this + 0x1a5))
    };
    let r = o(this, game, result, out);
    let after = (read_i16(this + 0x1a7) as u16, read_i16(this + 0x1a5));
    SCOREBOARD.record(r == n && after == previsto, || format!("player_search_scout_match(lista {:#x}, jogo {:#x}) original={} ensaio={} começo antes {:x?} previsto {:x?} depois {:x?}",
        this, game, r, n, before, previsto, after));
    r
}

/// Comparer for `contains_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7af600: pela devolução (só lê; as datas e a chave são locais). Pessoa nula: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_contains_player(this: usize, person: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CONTAINS_PLAYER);
    if person == 0 {
        return o(this, person);
    }
    let r = o(this, person);
    let n = contains_player(this, person);
    SCOREBOARD.record(r == n, || format!("contains_player(busca {:x}, pessoa {:x}) original={} rust={}", this, person, r, n));
    r
}

/// Comparer for `player_search_find_or_claim_player_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7bd700: sem `reservar`, pela devolução (só lê); com ele, lado a lado nas 50 vagas e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_player_search_find_or_claim_player_slot(this: usize, person: usize, reservar: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_PLAYER_SEARCH_FIND_OR_CLAIM_PLAYER_SLOT);
    if person == 0 {
        return o(this, person, reservar);
    }
    if reservar == 0 {
        let r = o(this, person, reservar);
        let n = player_search_find_or_claim_player_slot(this, person, reservar);
        SCOREBOARD.record(r == n, || format!("player_search_find_or_claim_player_slot(registro {:x}, pessoa {:x}) original={:x} rust={:x}", this, person, r, n));
        return r;
    }
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&[(this + 4, 0x32 * 0x1a2)], || o(this, person, reservar), || player_search_find_or_claim_player_slot(this, person, reservar));
    SCOREBOARD.record(igual && r == n, || format!("player_search_find_or_claim_player_slot(registro {:x}, pessoa {:x}, reservar) original={:x} rust={:x} iguais={}", this, person, r, n, igual));
    r
}

/// Comparer for `player_search_get_club_target_rating` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura (lê o clube, o elenco e as finanças; grava só `*base`): as duas sobre bases próprias.
pub(crate) unsafe extern "C" fn comparar_player_search_get_club_target_rating(club: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut i32, *const i16) -> i32>(IDX_PLAYER_SEARCH_GET_CLUB_TARGET_RATING);
    let (mut bo, mut br) = (0x7fff_0001_i32, 0x7fff_0002_i32);
    let r = o(club, &mut bo, forcada);
    let n = player_search_get_club_target_rating(club, &mut br, forcada);
    SCOREBOARD.record(r == n && bo == br, || format!("player_search_get_club_target_rating({:x}, forcada {}) {}/{} base {}/{}", club, if forcada.is_null() { -1 } else { *forcada as i32 }, r, n, bo, br));
    if !base.is_null() {
        *base = bo;
    }
    r
}

/// Comparer for `player_search_slot_get_match_entry` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura: compara a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_player_search_slot_get_match_entry(this: usize, i: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_PLAYER_SEARCH_SLOT_GET_MATCH_ENTRY)(this, i);
    let n = player_search_slot_get_match_entry(this, i);
    SCOREBOARD.record(r == n, || format!("player_search_slot_get_match_entry({:x}, {}) {:x}/{:x}", this, i as u8 as i8, r, n));
    r
}

/// Comparer for `player_search_slot_sum_goals` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura: compara a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_player_search_slot_sum_goals(this: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u16>(IDX_PLAYER_SEARCH_SLOT_SUM_GOALS)(this);
    let n = player_search_slot_sum_goals(this);
    SCOREBOARD.record(r == n, || format!("player_search_slot_sum_goals({:x}) {}/{}", this, r, n));
    r
}

/// Comparer for `set_slot_index` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_slot_index(search: usize, slot: i8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_SET_SLOT_INDEX);
    if !(0..=9).contains(&slot) {
        return o(search, slot);
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&[(search + 4, 1)], || o(search, slot), || set_slot_index(search, slot));
    SCOREBOARD.record(igual && ro == rr, || format!("set_slot_index({}) original={} rust={} {:x?} x {:x?}", slot, ro, rr, mo, mr));
    ro
}

/// Comparer for `set_list_type` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_list_type(search: usize, kind: i8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_SET_LIST_TYPE);
    if !(1..=4).contains(&kind) {
        return o(search, kind);
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&[(search + 0xa, 1)], || o(search, kind), || set_list_type(search, kind));
    SCOREBOARD.record(igual && ro == rr, || format!("set_list_type({}) original={} rust={} {:x?} x {:x?}", kind, ro, rr, mo, mr));
    ro
}

/// Comparer for `set_scout` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo olheiro e as missões (+0x174 a +0x199; a 0x53e990 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_set_scout(this: usize, scout: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_SCOUT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this + 0x174, 0x25)], || o(this, scout), || set_scout(this, scout));
    SCOREBOARD.record(igual && r == n, || format!("set_scout({:x}) {:x?} x {:x?}", scout, mo, mr));
    r
}

/// Comparer for `set_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_manager(search: usize, person: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_MANAGER);
    if person == 0 || *(person as *const i32) < *(G_STAFF_COUNT as *const i32) - 0x10 {
        return o(search, person);
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&[(search, 4)], || o(search, person), || set_manager(search, person));
    SCOREBOARD.record(igual && ro == rr, || format!("set_manager original={} rust={} {:x?} x {:x?}", ro, rr, mo, mr));
    ro
}

/// Comparer for `get_scout_assignment` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas seis saídas (a 0x53e990 só lê); com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_scout_assignment(this: usize, club: *mut u32, nation: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, opponent: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(IDX_GET_SCOUT_ASSIGNMENT);
    if check_search_scout(this).is_none() {
        return o(this, club, nation, regiao, comp, base, opponent);
    }
    let reg = [(club as usize, 4), (nation as usize, 4), (regiao as usize, 1), (comp as usize, 4), (base as usize, 4), (opponent as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, club, nation, regiao, comp, base, opponent),
        || get_scout_assignment(this, club, nation, regiao, comp, base, opponent));
    SCOREBOARD.record(igual && r == n, || format!("get_scout_assignment original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `scout_covers_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo valor e pelas missões +0x18c..+0x1a0 (a 0x7aefd0 as copia sobre elas mesmas).
pub(crate) unsafe extern "thiscall" fn comparar_scout_covers_nation(this: usize, nation: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SCOUT_COVERS_NATION);
    let reg = [(this + 0x18c, 0x15)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, nation), || scout_covers_nation(this, nation));
    SCOREBOARD.record(igual && r == n, || format!("scout_covers_nation(nacao {:#x}) original={} rust={} {:x?} x {:x?}", nation, r, n, mo, mr));
    r
}
