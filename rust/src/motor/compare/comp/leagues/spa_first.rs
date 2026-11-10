use super::*;

// The comparers of the routines in ../comp/leagues/spa_first.rs.
// [pt] Os comparadores das rotinas de ../comp/leagues/spa_first.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_first_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("spa_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| spa_first_create_schedule(this, stage, r, e, l))
}

/// Comparer for `spa_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_FIRST_SET_UP_RULES);
    spa_first_compare_league_rules("spa_first_set_up_rules", League(this), primera_rules(), || o(this), || spa_first_set_up_rules(this))
}

/// Comparer for `spa_first_create_table`.
///
/// Comparer of `spa_first_create_table`: the original routine allocates the table, changes the clubs' division and pays the TV money
/// (with a news item), so only it runs. The plan, made beforehand: the clubs of the lower Spanish division (which move
/// to the generic one) and those of the Primera in database order (up to 20).
///
/// Afterwards: the former with +0x57 = the record of [0x9cf764]; +0x3e = 20, the return value, the rows with the
/// Primera clubs in order and, when full, +0xe1 = 1 and +0x28 = the quota fund.
///
/// [pt] Comparador da 0x84fc70: a original aloca a tabela, muda a divisão dos clubes e paga a TV (com notícia): só ela. O
/// plano, feito antes: os clubes da divisão de baixo espanhola (que passam à genérica) e os da Primera na ordem do
/// banco (até 20). Depois: os primeiros com +0x57 = o registro de [0x9cf764]; +0x3e = 20, a devolução, as linhas com
/// os clubes da Primera na ordem e, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_create_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_FIRST_CREATE_TABLE);
    let (mut baixo, mut plano): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    for i in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) as usize {
        let club = Club(read_usize(G_CLUB_TABLE) + i * club::SIZE);
        let comp = club.division().addr();
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            baixo.push(club.addr());
        } else if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && plano.len() < 0x14 {
            plano.push(club.addr());
        }
    }
    let r = o(this.addr());
    let t = this.table();
    let generica = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let mut same = r == (plano.len() == 0x14) as u32 && this.club_count() == 0x14 && t != 0 && baixo.iter().all(|&c| read_usize(c + club::DIVISION) == generica);
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plano;
        if r == 1 {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, 0x3d090);
        }
    }
    SCOREBOARD.record(same, || format!("spa_first_create_table({:#x}) original={} low={} plan={:x?} table={:x?}", this.addr(), r, baixo.len(), plano, found));
    r
}

/// Comparer for `spa_first_set_club_progress`.
///
/// [pt] Comparador da 0x850880: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_SPA_FIRST_SET_CLUB_PROGRESS);
    let e = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(this + 8), club));
    if e.is_null() {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(e.addr() + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst), || spa_first_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(igual, || format!("spa_first_set_club_progress({:#x}, {:#x}) original={:x?} rust={:x?}", this, club, mo, mr));
}

/// Comparer for `construct_spa_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_FIRST);
    compare_division_league_ctor("construct_spa_first", League(this), year, record, &SPAIN_PRIMERA_DIVISION, || o(this, year, record))
}
