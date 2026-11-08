use super::*;

// The comparers of the routines in ../comp/leagues/spa_first.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/spa_first.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `spa_first_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_spa_first_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("spa_first_create_schedule", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| spa_first_create_schedule(this, fase, r, e, l))
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
        let clube = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let comp = read_usize(clube + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) {
            baixo.push(clube);
        } else if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && plano.len() < 0x14 {
            plano.push(clube);
        }
    }
    let r = o(this);
    let t = read_usize(this + 0xb1);
    let generica = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let mut igual = r == (plano.len() == 0x14) as u32 && read_i16(this + 0x3e) == 0x14 && t != 0 && baixo.iter().all(|&c| read_usize(c + 0x57) == generica);
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, 0x3d090);
        }
    }
    PLACAR.registrar(igual, || format!("spa_first_create_table({:#x}) original={} baixo={} plano={:x?} tabela={:x?}", this, r, baixo.len(), plano, achou));
    r
}

/// Comparer for `spa_first_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x654180: os registros vão ao histórico das pessoas e mexem nas reputações e no moral (não rodam duas
/// vezes): só a original. O plano, feito antes, nas fases 0 e 1: a primeira linha do clube com situação 1, 2 ou 0xff
/// decide o +0x37 que a rotina grava (fase 0: 0 com 1, 0xfe com 2/0xff; fase 1: 0xfe com 1, 3 com 2/0xff); depois a
/// linha tem de estar assim. Na liga ela só registra: a devolução 0.
pub(crate) unsafe extern "thiscall" fn comparar_spa_first_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_SPA_FIRST_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let (s, f) = (situacao as u8, fase as u8);
    let mut plano: Option<(usize, u8)> = None;
    if f <= 1 {
        let n = read_i16(this + 0x3e) as i32;
        for i in 0..n.max(0) as usize {
            let l = read_usize(this + 0xb1) + i * 0x3b;
            if read_usize(l) == clube && (s == 1 || s == 2 || s == 0xff) {
                plano = Some((l, match (f, s) {
                    (0, 1) => 0,
                    (0, _) => 0xfe,
                    (_, 1) => 0xfe,
                    _ => 3,
                }));
                break;
            }
        }
    }
    let r = o(this, clube, situacao, fase, a4, jogo, a6);
    let igual = r == 0 && plano.map_or(true, |(l, v)| *((l + 0x37) as *const u8) == v);
    PLACAR.registrar(igual, || format!("spa_first_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, clube, s, f, r, plano));
    r
}

/// Comparer for `spa_first_set_club_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x850880: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_spa_first_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_SPA_FIRST_SET_CLUB_PROGRESS);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = achar(read_usize(this + 8), clube);
    if e == 0 {
        return o(this, clube, fase, atual, melhor, pior);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], || o(this, clube, fase, atual, melhor, pior), || spa_first_set_club_progress(this, clube, fase, atual, melhor, pior));
    PLACAR.registrar(igual, || format!("spa_first_set_club_progress({:#x}, {:#x}) original={:x?} rust={:x?}", this, clube, mo, mr));
}

/// Comparer for `construct_spa_first` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_spa_first(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_FIRST);
    compare_division_league_ctor("construct_spa_first", this, ano, registro, &SPAIN_PRIMERA_DIVISION, || o(this, ano, registro))
}
