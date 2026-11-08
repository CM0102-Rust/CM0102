use super::*;

// The comparers of the routines in ../comp/leagues/spa_first.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/spa_first.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_first_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spa_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("spa_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| spa_first_create_schedule(this, stage, r, e, l))
}

/// Comparer for `spa_first_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spa_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_FIRST_SET_UP_RULES);
    spa_first_compare_league_rules("spa_first_set_up_rules", this, primera_rules(), || o(this), || spa_first_set_up_rules(this))
}

/// Comparer for `spa_first_create_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x84fc70: a original aloca a tabela, muda a divisão dos clubes e paga a TV (com notícia): só ela. O
/// plano, feito antes: os clubes da divisão de baixo espanhola (que passam à genérica) e os da Primera na ordem do
/// banco (até 20). Depois: os primeiros com +0x57 = o registro de [0x9cf764]; +0x3e = 20, a devolução, as linhas com
/// os clubes da Primera na ordem e, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe extern "thiscall" fn comparar_spa_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_FIRST_CREATE_TABLE);
    let (mut baixo, mut plano): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    for i in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) as usize {
        let club = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            baixo.push(club);
        } else if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && plano.len() < 0x14 {
            plano.push(club);
        }
    }
    let r = o(this);
    let t = read_usize(this + 0xb1);
    let generica = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let mut same = r == (plano.len() == 0x14) as u32 && read_i16(this + 0x3e) == 0x14 && t != 0 && baixo.iter().all(|&c| read_usize(c + 0x57) == generica);
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plano;
        if r == 1 {
            let prize: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
            same = same && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == prize(this, 0x3d090);
        }
    }
    SCOREBOARD.record(same, || format!("spa_first_create_table({:#x}) original={} baixo={} plano={:x?} tabela={:x?}", this, r, baixo.len(), plano, found));
    r
}


/// Comparer for `spa_first_set_club_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x850880: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_spa_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_SPA_FIRST_SET_CLUB_PROGRESS);
    let find: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = find(read_usize(this + 8), club);
    if e == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(e + 0x73, 3)], || o(this, club, stage, current, best, worst), || spa_first_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(igual, || format!("spa_first_set_club_progress({:#x}, {:#x}) original={:x?} rust={:x?}", this, club, mo, mr));
}

/// Comparer for `construct_spa_first` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_spa_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_FIRST);
    compare_division_league_ctor("construct_spa_first", this, year, record, &SPAIN_PRIMERA_DIVISION, || o(this, year, record))
}
