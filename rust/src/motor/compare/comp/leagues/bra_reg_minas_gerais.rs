use super::*;

// The comparers of the routines in ../comp/leagues/bra_reg_minas_gerais.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/bra_reg_minas_gerais.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_reg_minas_gerais_advance_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da fase seguinte: só a original (simplification: ao chegar a 2 monta a final, que aloca e sorteia; não pode
/// rodar duas vezes), conferindo o plano: na última fase, +0x30 igual; senão +0x30 + 1, ou, ao chegar a 2, 2 (com
/// final) ou 1 (o mesmo campeão nos dois turnos, que a final grava).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_advance_stage(this: usize) {
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_MINAS_GERAIS_ADVANCE_STAGE)(this);
    let depois = read_usize(this + 0x30) as i32;
    let igual = if ultima { depois == antes } else if antes + 1 == 2 { depois == 2 || depois == 1 } else { depois == antes + 1 };
    PLACAR.registrar(igual, || format!("bra_reg_minas_gerais_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// Comparer for `bra_reg_minas_gerais_create_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), conferindo o plano feito
/// antes: os campeões dos turnos. O mesmo nos dois: +0x30 = 1, +0x2c = 2 e a situação 5 na primeira linha de cada turno
/// e na linha dele da tabela da liga. Diferentes: a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os
/// dois campeões na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio da rodada pode reordenar), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_create_final_stage(this: usize) {
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_REG_MINAS_GERAIS_CREATE_FINAL_STAGE)(this);
    let igual = if a == b {
        let marca = |k: usize| *((read_usize(read_usize(read_usize(this + 0xc) + 4 * k) + 0xb1) + 0x37) as *const u8) == 5;
        let na_liga = (0..read_i16(this + 0x3e).max(0) as usize).all(|i| {
            let linha = read_usize(this + 0xb1) + i * 0x3b;
            read_usize(linha) != a || *((linha + 0x37) as *const u8) == 5
        });
        read_usize(this + 0x30) == 1 && read_usize(this + 0x2c) == 2 && marca(0) && marca(1) && na_liga
    } else {
        let fase = read_usize(read_usize(this + 0xc) + 8);
        if fase == 0 {
            false
        } else {
            let l = read_usize(fase + 0xa7);
            let (x, y) = (read_usize(l), read_usize(l + 6));
            read_i16(fase + 0x3e) == 2 && read_usize(fase + 4) == read_usize(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && read_usize(this + 0x30) == 2
        }
    };
    PLACAR.registrar(igual, || format!("bra_reg_minas_gerais_create_final_stage({:#x}) campeoes={:#x},{:#x} +0x30={} +0x2c={}", this, a, b, read_usize(this + 0x30), read_usize(this + 0x2c)));
}

/// Comparer for `bra_reg_minas_gerais_record_season_honours` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador do fim da temporada: só a original (simplification: grava o histórico, que aloca o registro, paga os prêmios e
/// manda a notícia, e pode montar a Copa dos Campeões; não pode rodar duas vezes). A cópia roda no modo Rust (contadores
/// "cqo_7 historico da final" e "cqo_7 premios") e é provada pelas mutações e pelas duas carreiras Rust.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_MINAS_GERAIS_RECORD_SEASON_HONOURS)(this)
}

/// Comparer for `bra_reg_minas_gerais_set_club_progress` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela entrada do clube (+0x73..+0x75), achada antes (a 0x4a4850 ordena na primeira vez e só
/// busca depois; a original e a cópia a chamam de novo); a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_set_club_progress(this: usize, clube: usize, fase: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(e + 0x73, 3)], || original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_BRA_REG_MINAS_GERAIS_SET_CLUB_PROGRESS)(this, clube, fase, a, b, c), || bra_reg_minas_gerais_set_club_progress(this, clube, fase, a, b, c));
    PLACAR.registrar(igual && r == n, || format!("bra_reg_minas_gerais_set_club_progress({:x}, {:x}, {}, {}, {}, {}) original={:x} rust={:x} {:?} {:?}", this, clube, fase as u8 as i8, a as u8, b as u8, c as u8, r, n, mo, mr));
    r
}

/// Comparador: como o do Centro-Oeste.
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = mineiro_end_of_stage(this, clube, situacao, fase, jogo, false);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_BRA_REG_MINAS_GERAIS_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("bra_reg_minas_gerais_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// Comparer for `construct_bra_reg_minas_gerais` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_bra_reg_minas_gerais(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_REG_MINAS_GERAIS);
    bra_reg_goias_compare_state_league_ctor("construct_bra_reg_minas_gerais", this, ano, registro, &MINEIRO_STATE_LEAGUE, || o(this, ano, registro))
}

/// Comparer for `bra_reg_minas_gerais_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_MINAS_GERAIS_CREATE_TABLE);
    bra_reg_goias_compare_state_league_table("bra_reg_minas_gerais_create_table", this, 0xc, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, || o(this))
}

/// Comparer for `bra_reg_minas_gerais_create_half_season_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_BRA_REG_MINAS_GERAIS_CREATE_HALF_SEASON_STAGE);
    compare_half_season_stage("bra_reg_minas_gerais_create_half_season_stage", this, k, || o(this, k))
}

/// Comparer for `bra_reg_minas_gerais_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_REG_MINAS_GERAIS_START_NEW_SEASON);
    compare_new_season("bra_reg_minas_gerais_start_new_season", this, || o(this))
}
