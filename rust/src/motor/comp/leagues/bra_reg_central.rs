use super::*;

// ---------- 274a leva: as regras dos estaduais do Centro-Oeste, Gaúcho (Nordeste, Paulista), Baiano e Catarinense (bra_reg_central.cpp, bra_reg_gaucho.cpp, bra_reg_santa.cpp) ----------




/// As regras comuns dos estaduais (as três rotinas gravam o mesmo conjunto; mudam os turnos +0x3c, as vagas de cima
/// +0xbe e os rebaixados +0xc1): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates confronto
/// direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb (copiadas em cada partida: +0x38 e +0x36) em 0.
pub(crate) unsafe fn cqi8_regras(o: usize, turnos: u16, cima: u8, rebaixados: u8) {
    write_u16_at(o, 0xab, 0);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc5, 3);
    write_u8_at(o, 0xc1, rebaixados);
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xbe, cima);
    write_u16_at(o, 0xd9, 8);
    write_u16_at(o, 0x3c, turnos);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc6, 1);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 7);
}

/// O Centro-Oeste (0x430ff0): turno único, 8 vagas de cima (o mata-mata de 8), 3 rebaixados.
pub(crate) unsafe fn cqi8_regras_centro(o: usize) {
    cqi8_regras(o, 1, 8, 3); // cqi8: o Centro-Oeste
}

/// O Gaúcho (0x4322d0, também o Nordeste e o Paulista): turno único, 4 vagas de cima, 2 rebaixados.
pub(crate) unsafe fn cqi8_regras_gaucho(o: usize) {
    cqi8_regras(o, 1, 4, 2); // cqi8: o Gaúcho
}

/// O Catarinense (0x43bc70, também o Baiano): dois turnos, 4 vagas de cima, 1 rebaixado.
pub(crate) unsafe fn cqi8_regras_catarinense(o: usize) {
    cqi8_regras(o, 2, 4, 1); // cqi8: o Catarinense
}



/// Centre-West: single round robin; 8 top places (the 8-club knockout [inference: the slot 11 stage]); 3 relegated; win
/// 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league above or below (the
/// relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next stage; no champion
/// band: the champion comes from the final stage); 7 bench substitutes and 3 substitutions; category 2 (league)
/// matches. Then the main stage schedule
/// Original: `BraRegCentral::set_up_rules` (0x430ff0, bra_reg_central.cpp, thiscall).
///
/// [pt] { 0x430ff0 } (thiscall, bra_reg_central.cpp): `BraRegCentral::set_up_rules` (slot 36), as regras e o calendário;
/// devolve 1.
pub(crate) unsafe extern "thiscall" fn cqi8_regras_do_centro_oeste(this: usize) -> u32 {
    cqi8_regras_centro(this);
    crate::comparacao::ramo("cqi_8 regras do centro-oeste");
    cqi7_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_cqi8_regras_do_centro_oeste(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CQI8_REGRAS_DO_CENTRO_OESTE)(this), || {
        cqi8_regras_centro(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_centro_oeste({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}



/// Bahia and Santa Catarina (the same routine in the two vtables): double round robin; 4 top places (the 4-club
/// semi-final); 1 relegated; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table;
/// no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead
/// to the next stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3
/// substitutions; category 2 (league) matches. Then the main stage schedule
/// Original: `BraRegSanta::set_up_rules` (0x43bc70, bra_reg_santa.cpp, thiscall).
///
/// [pt] { 0x43bc70 } (thiscall, bra_reg_santa.cpp): `BraRegSanta::set_up_rules`, nas vtables do Baiano (0x967fb4) e do
/// Catarinense (0x9686bc); devolve 1.
pub(crate) unsafe extern "thiscall" fn cqi8_regras_do_catarinense(this: usize) -> u32 {
    cqi8_regras_catarinense(this);
    crate::comparacao::ramo("cqi_8 regras do catarinense");
    cqi7_calendario(this);
    1
}

/// Comparador: como o do Centro-Oeste.
pub(crate) unsafe extern "thiscall" fn comparar_cqi8_regras_do_catarinense(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CQI8_REGRAS_DO_CATARINENSE)(this), || {
        cqi8_regras_catarinense(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_catarinense({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

// ---------- 274a leva: o destino de cada clube no fim das fases do Centro-Oeste e do Goiano, do Mineiro, Paranaense e Carioca, e da NSL (bra_reg_central.cpp, bra_reg_minas_gerais.cpp, aus_nsl.cpp) ----------






/// A fase de pontos corridos (fase -1) dos estaduais: os classificados (6) ganham o registro de classificação (rodada
/// 0x82, peso 0x32), os rebaixados (3) o de rebaixamento. Igual nas duas rotinas dos estaduais.
pub(crate) unsafe fn cqk7_pontos_corridos(this: usize, clube: usize, st: u8, fazer: bool) {
    let comp = read_u32_as_usize(this + 4);
    if st == 6 {
        if fazer {
            crate::comparacao::ramo("cqk_7 classificado do estadual");
            cqj9_classificou(clube, comp, 0, 0x82, 0x32); // cqk7: o classificado do estadual
        }
    } else if st == 3 && fazer {
        crate::comparacao::ramo("cqk_7 rebaixado do estadual");
        cqj9_rebaixado(clube, comp); // cqk7: o rebaixado do estadual
    }
}

/// O mata-mata dos estaduais: a linha do clube na tabela (+0xb1, +0x3e linhas); o campeão (1) ganha o registro de
/// campeão e o destino 5; os eliminados (2 ou -1), o registro de eliminação (0x86c000) com o peso da escala 0x4bf850
/// (30 a 50 pela rodada do jogo em `rodadas`) e o destino -2. Devolve (linha gravada ou 0, destino).
pub(crate) unsafe fn cqk7_mata_mata(this: usize, clube: usize, st: u8, jogo: usize, rodadas: u32, fazer: bool) -> (usize, u8) {
    let comp = read_u32_as_usize(this + 4);
    let n = read_i16(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < n {
        if read_u32_as_usize(read_u32_as_usize(this + 0xb1) + 0x3b * i as usize) == clube {
            if st == 1 {
                if fazer {
                    crate::comparacao::ramo("cqk_7 campeao do estadual");
                    cqj9_campeao(clube, comp); // cqk7: o registro do campeão do estadual
                }
                let l = read_u32_as_usize(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 5;
                }
                return (l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // ponytail: a original empurra eax (lixo do chamador acima da palavra) e ecx/edx inteiros; a
                    // 0x4bf850 e a 0x86c000 só leem as palavras: vão as palavras.
                    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_SCALE_BY_ROUND);
                    crate::comparacao::ramo("cqk_7 eliminado do estadual");
                    let peso = escala(0x1e, 0x32, read_i16(jogo + 0x34) as u16 as u32, rodadas);
                    let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_KNOCKED_OUT_RECORD);
                    elim(read_u32_as_usize(G_STAFF_RECORDS), clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso); // cqk7: o registro do eliminado
                }
                let l = read_u32_as_usize(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 0xfe;
                }
                return (l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0)
}

/// O Centro-Oeste e o Goiano (0x4314e0): a fase -1 de pontos corridos e o mata-mata na fase 0 (escala de 3 rodadas).
/// `fazer` = falso: o ensaio do comparador. Devolve (devolução, linha gravada ou 0, destino).
pub(crate) unsafe fn cqk7_centro(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    if fa == 0xff {
        cqk7_pontos_corridos(this, clube, st, fazer);
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let (l, d) = cqk7_mata_mata(this, clube, st, jogo, 3, fazer);
    (0, l, d)
}

/// O Mineiro, o Paranaense e o Carioca (0x434d90): a fase -1 de pontos corridos; as fases 0 e 1 (os turnos) devolvem -1;
/// o mata-mata na fase 2 (escala de 1 rodada: a média, 40); as outras, 0.
pub(crate) unsafe fn cqk7_mineiro(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    if fa == -1 {
        cqk7_pontos_corridos(this, clube, st, fazer);
        return (0, 0, 0);
    }
    if fa < 2 {
        return (u32::MAX, 0, 0);
    }
    if fa != 2 {
        return (0, 0, 0);
    }
    let (l, d) = cqk7_mata_mata(this, clube, st, jogo, 1, fazer);
    (0, l, d)
}

/// A NSL (0x411980). Devolve (devolução, 0, 0): não grava na tabela.
pub(crate) unsafe fn cqk7_nsl(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = read_u32_as_usize(this + 4);
    if fa == 0xff {
        // a temporada regular
        if st == 6 {
            if fazer {
                crate::comparacao::ramo("cqk_7 nsl classificado");
                cqj9_classificou(clube, comp, 0x435, 0, 0x28); // cqk7: o classificado da NSL
            }
            return (0, 0, 0);
        }
        if st == 0xfe {
            return (u32::MAX, 0, 0);
        }
        return (0, 0, 0);
    }
    if fa == 0 {
        // a fase final
        if st == 6 {
            if fazer {
                cqj9_classificou(clube, comp, 0, 0x96, 0x28);
            }
            return (0, 0, 0);
        }
        if st == 0xfe {
            if fazer {
                let nao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FAILED_TO_QUALIFY_RECORD);
                nao(read_u32_as_usize(G_STAFF_RECORDS), clube, comp, 0x435, 0x28);
            }
            return (u32::MAX, 0, 0);
        }
        return (0, 0, 0);
    }
    if st == 1 && fazer {
        cqj9_campeao(clube, comp);
    }
    (0, 0, 0)
}

/// Centre-West and Goias: at the end of the round robin, the clubs through to the knockout get the qualification record
/// (round 0x82, weight 0x32) and the relegated the relegation record; in the knockout (stage 0), the champion gets the
/// champion record and fate 5; the eliminated, the knocked-out record weighted 30 to 50 by round (over 3 rounds:
/// quarter-finals, semi-finals, final) and fate -2
/// Original: `BraRegCentral::update_staff_records_for_status` (0x4314e0, bra_reg_central.cpp, thiscall).
///
/// [pt] { 0x4314e0 } (thiscall, ret 0x18, bra_reg_central.cpp): `BraRegCentral::update_staff_records_for_status` (slot 18
/// do Centro-Oeste e do Goiano): os classificados e rebaixados dos pontos corridos; no mata-mata (fase 0), o campeão
/// (destino 5) e os eliminados (destino -2, registro de eliminação com peso de 30 a 50 em 3 rodadas). Devolve 0.
pub(crate) unsafe extern "thiscall" fn cqk7_destino_no_centro_oeste(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    cqk7_centro(this, clube, situacao, fase, jogo, true).0
}

/// Comparador: só a original (ponytail: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação, rodada antes): a devolução e o destino na linha.
pub(crate) unsafe extern "thiscall" fn comparar_cqk7_destino_no_centro_oeste(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let (n, l, dest) = cqk7_centro(this, clube, situacao, fase, jogo, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CQK7_DESTINO_NO_CENTRO_OESTE)(this, clube, situacao, fase, a4, jogo, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("cqk7_destino_no_centro_oeste({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}

/// The Centre-West state league at career creation: the year and the database competition, disciplinary rule 7 and,
/// with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the
/// table with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegCentral::BraRegCentral` (0x4304b0, bra_reg_central.cpp, thiscall, ret 8).
///
/// [pt] { 0x4304b0 } (thiscall, ret 8, bra_reg_central.cpp): `BraRegCentral::BraRegCentral(ano, registro)`: o Centro-Oeste (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn cqt1_construir_centro_oeste(this: usize, ano: u32, registro: usize) -> usize {
    cqt1_construir(this, ano, registro, &CQT1_CENTRO_OESTE, 7, true, "cqt_1 centro_oeste novo") // cqt1: a disciplina e as posições previstas do Centro-Oeste
}

pub(crate) unsafe extern "thiscall" fn comparar_cqt1_construir_centro_oeste(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CQT1_CONSTRUIR_CENTRO_OESTE);
    cqt1_comparar("cqt1_construir_centro_oeste", this, ano, registro, &CQT1_CENTRO_OESTE, || o(this, ano, registro))
}

/// The Centre-West turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegCentral::start_new_season` (0x431090, bra_reg_central.cpp, thiscall, ret 0).
///
/// [pt] { 0x431090 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::start_new_season()`, a virada da temporada do Centro-Oeste (a tabela:
/// 0x430ef0; caixas nas linhas 0x1cb, 0x1d2, 0x1d8 e 0x1df).
pub(crate) unsafe extern "thiscall" fn cqp7_virada_do_centro_oeste(this: usize) -> u32 {
    cqp7_virada(this, 0x430ef0, 0x98ae10, [0x1cb, 0x1d2, 0x1d8, 0x1df], 1, "cqp_7 virada do centro_oeste") // cqp7: o ano seguinte do Centro-Oeste
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp7_virada_do_centro_oeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CQP7_VIRADA_DO_CENTRO_OESTE);
    cqp7_comparar_virada("cqp7_virada_do_centro_oeste", this, || o(this))
}

/// The Centre-West next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 8 go to the
/// quarter-finals (1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th); past the last stage it is an error (the box)
/// Original: `BraRegCentral::advance_stage` (0x431240, bra_reg_central.cpp, thiscall, ret 0).
///
/// [pt] { 0x431240 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::advance_stage()`, a fase seguinte do Centro-Oeste: no fim dos pontos corridos,
/// o mata-mata de 8 (0x4312d0); a caixa da linha 0x1f9.
pub(crate) unsafe extern "thiscall" fn cqp7_fase_seguinte_do_centro_oeste(this: usize) {
    cqp7_fase_seguinte(this, 0x98ae10, 0x1f9, 0x4312d0);
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp7_fase_seguinte_do_centro_oeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_CQP7_FASE_SEGUINTE_DO_CENTRO_OESTE);
    cqp7_comparar_fase_seguinte("cqp7_fase_seguinte_do_centro_oeste", this, || o(this))
}

/// the Centre-West league has 15 clubs: those with it as their reserve division in the database (the field that holds
/// the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegCentral::load_region_clubs` (0x430ef0, bra_reg_central.cpp, thiscall, ret 0).
///
/// [pt] { 0x430ef0 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::load_region_clubs()`: o Centro-Oeste tem 15 clubes (divisão reserva +0x60 = [0x9cf998]).
/// Fonte 0x98ae10, linha 0x168.
pub(crate) unsafe extern "thiscall" fn cql8_tabela_do_centro_oeste(this: usize) -> u32 {
    cql8_tabela(this, 0xf, 0x98ae10, 0x168, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 0, "cql_8 centro_oeste cheio")
}

pub(crate) unsafe extern "thiscall" fn comparar_cql8_tabela_do_centro_oeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CQL8_TABELA_DO_CENTRO_OESTE);
    cql8_comparar("cql8_tabela_do_centro_oeste", this, 0xf, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, || o(this))
}

/// Centre-West: after the round robin, the top 8 go to the quarter-finals: 1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th
/// (the seeds are the positions [inference: two legs])
/// Original: `BraRegCentral::create_knockout_stage` (0x4312d0, bra_reg_central.cpp, thiscall, ret 0).
///
/// [pt] { 0x4312d0 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (linhas 0, 7, 2, 5, 6, 1, 4, 3). Fonte 0x98ae10, linhas 0x21c (sem lista) e 0x234 (sem fase).
pub(crate) unsafe extern "thiscall" fn cqn9_mata_mata_do_centro_oeste(this: usize) {
    cqn9_mata_mata(this, &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae10, 0x21c, 0x234, "cqn_9 centro_oeste"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Centro-Oeste
}

pub(crate) unsafe extern "thiscall" fn comparar_cqn9_mata_mata_do_centro_oeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_CQN9_MATA_MATA_DO_CENTRO_OESTE);
    cqn9_comparar("cqn9_mata_mata_do_centro_oeste", this, &[0, 7, 2, 5, 6, 1, 4, 3], || o(this))
}
