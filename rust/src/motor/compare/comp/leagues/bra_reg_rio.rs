use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_rio.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_rio.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_bra_reg_rio` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_rio(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_RIO);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_rio", this, year, record, &CARIOCA_CHAMPIONSHIP, || o(this, year, record))
}

/// Comparer for `construct_bra_reg_santa` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_santa(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_SANTA);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_santa", this, year, record, &CATARINENSE_STATE_CHAMPIONSHIP, || o(this, year, record))
}

/// Comparer for `construct_bra_reg_sp` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_sp(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_SP);
    bra_reg_rio_compare_state_league_ctor("construct_bra_reg_sp", this, year, record, &PAULISTA_STATE_LEAGUE, || o(this, year, record))
}

/// Comparer for `cqt4_atualizar_media_de_publico` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da média de público: lado a lado (a cópia sem contar os sorteios, com o estado do sorteador
/// restaurado entre as duas) sobre a tabela dos clubes inteira (onde ficam +0x73, +0x77 e +0x7b) e o estado do
/// sorteador; a tabela final (0x4a4b20) é alocada e liberada nas duas.
pub(crate) unsafe extern "C" fn comparar_cqt4_atualizar_media_de_publico(league: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_CQT4_ATUALIZAR_MEDIA_DE_PUBLICO);
    let mut regions = vec![(read_usize(G_CLUB_TABLE), read_usize(G_CLUB_COUNT) * club::SIZE)];
    regions.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regions, || o(league), || crate::aleatorio::sem_contar(|| cqt4_atualizar_media_de_publico(league)));
    PLACAR.registrar(igual, || format!("cqt4_atualizar_media_de_publico({:#x}): o público dos clubes não bate", league));
}

/// Comparer for `bra_reg_rio_create_half_season_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_BRA_REG_RIO_CREATE_HALF_SEASON_STAGE);
    compare_half_season_stage("bra_reg_rio_create_half_season_stage", this, k, || o(this, k))
}

/// Comparer for `bra_reg_rio_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_START_NEW_SEASON);
    compare_new_season("bra_reg_rio_start_new_season", this, || o(this))
}

/// Comparer for `bra_reg_rio_advance_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da fase seguinte: só a original (simplification: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_advance_stage(this: usize) {
    let before = read_usize(this + 0x30) as i32;
    let last = before >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_RIO_ADVANCE_STAGE)(this);
    let after = read_usize(this + 0x30) as i32;
    let same = if last { after == before } else if before + 1 == 2 { after == 2 || after == 1 } else { after == before + 1 };
    PLACAR.registrar(same, || format!("bra_reg_rio_advance_stage({:#x}) antes={} depois={}", this, before, after));
}

/// Comparer for `bra_reg_rio_create_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_final_stage(this: usize) {
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_RIO_CREATE_FINAL_STAGE)(this);
    let same = if a == b {
        let mark = |k: usize| *((read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let in_league = (0..read_i16(this + 0x3e).max(0) as usize).all(|i| {
            let row = read_usize(this + 0xb1) + i * 0x3b;
            read_usize(row) != a || *((row + 0x37) as *const u8) == 5
        });
        read_usize(this + 0x30) == 1 && read_usize(this + 0x2c) == 2 && mark(0) && mark(1) && in_league
    } else {
        let stage = read_usize(read_usize(this + 0xc) + 8);
        if stage == 0 {
            false
        } else {
            let l = read_usize(stage + 0xa7);
            let (x, y) = (read_usize(l), read_usize(l + 6));
            read_i16(stage + 0x3e) == 2 && read_usize(stage + 4) == read_usize(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && read_usize(this + 0x30) == 2
        }
    };
    PLACAR.registrar(same, || format!("bra_reg_rio_create_final_stage({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, read_usize(this + 0x30), read_usize(this + 0x2c)));
}

/// Comparer for `bra_reg_rio_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador do fim da temporada: só a original (simplification: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_9 historico da final" e "cqo_9 premios") e é provada pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_RECORD_SEASON_HONOURS)(this)
}

/// Comparer for `bra_reg_rio_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_CREATE_TABLE);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table", this, 0xc, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, || o(this))
}

/// Comparer for `bra_reg_rio_create_table_43bb70` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table_43bb70(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_CREATE_TABLE_43BB70);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table_43bb70", this, 0xa, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, || o(this))
}

/// Comparer for `bra_reg_rio_create_table_43cc40` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table_43cc40(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_CREATE_TABLE_43CC40);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table_43cc40", this, 0x10, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, || o(this))
}

/// Comparador: como o do Paranaense.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_rio_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_RIO_SET_UP_RULES)(this), || {
        carioca_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_rio_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
