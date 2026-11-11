use super::*;

/// Writes the rule set common to the Brazilian state leagues (3 pts win, 1 draw, tiebreakers, no league above/below, 7
/// subs, 3 substitutions), with rounds, top places (+0xbe) and relegated (+0xc1) as parameters.
///
/// [pt] As regras comuns dos estaduais (as três rotinas gravam o mesmo conjunto; mudam os turnos +0x3c, as vagas de cima
/// +0xbe e os rebaixados +0xc1): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates confronto
/// direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb (copiadas em cada partida: +0x38 e +0x36) em 0.
pub(crate) unsafe fn bra_reg_central_set_state_league_rules(o: usize, rounds: u16, cima: u8, relegated: u8) {
    write_u16_at(o, 0xab, 0);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc5, 3);
    write_u8_at(o, 0xc1, relegated);
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xbe, cima);
    write_u16_at(o, 0xd9, 8);
    write_u16_at(o, 0x3c, rounds);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc6, 1);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 7);
}

/// State league rules for Centro-Oeste (`bra_reg_central_set_up_rules`): single round, 8 top places, 3 relegated.
///
/// [pt] O Centro-Oeste (0x430ff0): turno único, 8 vagas de cima (o mata-mata de 8), 3 rebaixados.
pub(crate) unsafe fn centro_oeste_rules(o: usize) {
    bra_reg_central_set_state_league_rules(o, 1, 8, 3); // cqi8: o Centro-Oeste
}

/// State league rules for Gaucho (`bra_reg_gaucho_set_up_rules`, also Nordeste and Paulista): single round, 4 top places, 2 relegated.
///
/// [pt] O Gaúcho (0x4322d0, também o Nordeste e o Paulista): turno único, 4 vagas de cima, 2 rebaixados.
pub(crate) unsafe fn gaucho_rules(o: usize) {
    bra_reg_central_set_state_league_rules(o, 1, 4, 2); // cqi8: o Gaúcho
}

/// State league rules for Catarinense (`bra_reg_santa_set_up_rules`, also Baiano): two rounds, 4 top places, 1 relegated.
///
/// [pt] O Catarinense (0x43bc70, também o Baiano): dois turnos, 4 vagas de cima, 1 rebaixado.
pub(crate) unsafe fn catarinense_rules(o: usize) {
    bra_reg_central_set_state_league_rules(o, 2, 4, 1); // cqi8: o Catarinense
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
pub(crate) unsafe extern "thiscall" fn bra_reg_central_set_up_rules(this: usize) -> u32 {
    centro_oeste_rules(this);
    covered!("cqi_8 centro-oeste rules");
    create_main_schedule(League(this));
    1
}

/// League stage (-1) of the state leagues: qualified clubs (status 6) get the qualification record, relegated (3) the
/// relegation record; `fazer` false is a dry run.
///
/// [pt] A fase de pontos corridos (fase -1) dos estaduais: os classificados (6) ganham o registro de classificação (rodada
/// 0x82, peso 0x32), os rebaixados (3) o de rebaixamento. Igual nas duas rotinas dos estaduais.
pub(crate) unsafe fn record_state_league_stage(this: usize, club: usize, st: u8, fazer: bool) {
    let comp = read_usize(this + 4);
    if st == 6 {
        if fazer {
            covered!("cqk_7 state league qualified");
            bra_nat_first_add_qualified_record(club, comp, 0, 0x82, 0x32); // cqk7: the state championship qualifier
        }
    } else if st == 3 && fazer {
        covered!("cqk_7 state league relegated");
        bra_nat_first_add_relegation_record(club, comp); // cqk7: the club relegated from the state championship
    }
}

/// State league knockout: finds the club's table row; champion gets the champion record and destination 5, eliminated
/// get the knocked-out record (weight scaled by round) and destination -2; returns (row, destination).
///
/// [pt] O mata-mata dos estaduais: a linha do clube na tabela (+0xb1, +0x3e linhas); o campeão (1) ganha o registro de
/// campeão e o destino 5; os eliminados (2 ou -1), o registro de eliminação (0x86c000) com o peso da escala 0x4bf850
/// (30 a 50 pela rodada do jogo em `rodadas`) e o destino -2. Devolve (linha gravada ou 0, destino).
pub(crate) unsafe fn record_state_knockout(this: League, club: usize, st: u8, game: usize, rounds: u32, fazer: bool) -> (usize, u8) {
    let comp = this.competition().addr();
    let n = this.club_count() as i32;
    let mut i = 0i32;
    while i < n {
        if read_usize(this.table() + league_table_row::SIZE * i as usize) == club {
            if st == 1 {
                if fazer {
                    covered!("cqk_7 state league champion");
                    add_champions_record(club, comp); // cqk7: the state champion record
                }
                let l = this.table() + league_table_row::SIZE * i as usize;
                if fazer {
                    write_u8(l + league_table_row::STATUS, 5);
                }
                return (l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // simplification: the original pushes eax (caller garbage above the word) and whole ecx/edx; the
                    // 0x4bf850 and 0x86c000 only read the words: the words are passed.
                    covered!("cqk_7 state league eliminated");
                    let weight = exe::comp_util_scale_by_round(0x1e, 0x32, read_i16(game + fixture::ROUND_NUMBER) as u16 as u32, rounds) as u32;
                    exe::staff_records_add_knocked_out_record(read_usize(G_STAFF_RECORDS), club, comp, read_i16(game + fixture::ROUND_PREFIX) as u16 as u32, read_i16(game + fixture::ROUND_NAME) as u16 as u32, weight); // cqk7: the eliminated club record
                }
                let l = this.table() + league_table_row::SIZE * i as usize;
                if fazer {
                    write_u8(l + league_table_row::STATUS, 0xfe);
                }
                return (l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0)
}

/// Centro-Oeste and Goiano (`bra_reg_central_update_staff_records_for_status`) stage handler: league stage -1, knockout at stage 0 (3-round scale); returns
/// (result, row, destination).
///
/// [pt] O Centro-Oeste e o Goiano (0x4314e0): a fase -1 de pontos corridos e o mata-mata na fase 0 (escala de 3 rodadas).
/// `fazer` = falso: o ensaio do comparador. Devolve (devolução, linha gravada ou 0, destino).
pub(crate) unsafe fn centro_oeste_end_of_stage(this: usize, club: usize, status: u32, stage: u32, game: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (status as u8, stage as u8);
    if fa == 0xff {
        record_state_league_stage(this, club, st, fazer);
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let (l, d) = record_state_knockout(League(this), club, st, game, 3, fazer);
    (0, l, d)
}

/// Mineiro, Paranaense and Carioca (`bra_reg_minas_gerais_update_staff_records_for_status`) stage handler: league stage -1, stages 0-1 return -1, knockout at stage
/// 2; returns (result, row, destination).
///
/// [pt] O Mineiro, o Paranaense e o Carioca (0x434d90): a fase -1 de pontos corridos; as fases 0 e 1 (os turnos) devolvem -1;
/// o mata-mata na fase 2 (escala de 1 rodada: a média, 40); as outras, 0.
pub(crate) unsafe fn mineiro_end_of_stage(this: usize, club: usize, status: u32, stage: u32, game: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (status as u8, stage as u8 as i8);
    if fa == -1 {
        record_state_league_stage(this, club, st, fazer);
        return (0, 0, 0);
    }
    if fa < 2 {
        return (u32::MAX, 0, 0);
    }
    if fa != 2 {
        return (0, 0, 0);
    }
    let (l, d) = record_state_knockout(League(this), club, st, game, 1, fazer);
    (0, l, d)
}

/// NSL (`aus_nsl_update_staff_records_for_status`) stage handler: qualification records in the regular season and final stage, failed-to-qualify record,
/// champion record; returns (result, 0, 0).
///
/// [pt] A NSL (0x411980). Devolve (devolução, 0, 0): não grava na tabela.
pub(crate) unsafe fn nsl_end_of_stage(this: usize, club: usize, status: u32, stage: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (status as u8, stage as u8);
    let comp = read_usize(this + 4);
    if fa == 0xff {
        // a temporada regular
        if st == 6 {
            if fazer {
                covered!("cqk_7 nsl qualified");
                bra_nat_first_add_qualified_record(club, comp, 0x435, 0, 0x28); // cqk7: the NSL qualifier
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
                bra_nat_first_add_qualified_record(club, comp, 0, 0x96, 0x28);
            }
            return (0, 0, 0);
        }
        if st == 0xfe {
            if fazer {
                exe::staff_records_add_failed_to_qualify_record(read_usize(G_STAFF_RECORDS), club, comp, 0x435, 0x28);
            }
            return (u32::MAX, 0, 0);
        }
        return (0, 0, 0);
    }
    if st == 1 && fazer {
        add_champions_record(club, comp);
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
pub(crate) unsafe extern "thiscall" fn bra_reg_central_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    centro_oeste_end_of_stage(this, club, status, stage, game, true).0
}

/// The Centre-West state league at career creation: the year and the database competition, disciplinary rule 7 and,
/// with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the
/// table with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegCentral::BraRegCentral` (0x4304b0, bra_reg_central.cpp, thiscall, ret 8).
///
/// [pt] { 0x4304b0 } (thiscall, ret 8, bra_reg_central.cpp): `BraRegCentral::BraRegCentral(ano, registro)`: o Centro-Oeste (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_central(this: usize, year: u32, record: usize) -> usize {
    bra_reg_bahia_construct_state_league(League(this), year, record, &CENTRO_OESTE_STATE_CHAMPIONSHIP, 7, true, "cqt_1 centro_oeste novo") // cqt1: the discipline and expected positions of the Centro-Oeste
}

/// The Centre-West turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegCentral::start_new_season` (0x431090, bra_reg_central.cpp, thiscall, ret 0).
///
/// [pt] { 0x431090 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::start_new_season()`, a virada da temporada do Centro-Oeste (a tabela:
/// 0x430ef0; caixas nas linhas 0x1cb, 0x1d2, 0x1d8 e 0x1df).
pub(crate) unsafe extern "thiscall" fn bra_reg_central_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x430ef0, 0x98ae10, [0x1cb, 0x1d2, 0x1d8, 0x1df], 1, "cqp_7 virada do centro_oeste") // cqp7: the next year of the Centro-Oeste
}

/// The Centre-West next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 8 go to the
/// quarter-finals (1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th); past the last stage it is an error (the box)
/// Original: `BraRegCentral::advance_stage` (0x431240, bra_reg_central.cpp, thiscall, ret 0).
///
/// [pt] { 0x431240 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::advance_stage()`, a fase seguinte do Centro-Oeste: no fim dos pontos corridos,
/// o mata-mata de 8 (0x4312d0); a caixa da linha 0x1f9.
pub(crate) unsafe extern "thiscall" fn bra_reg_central_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98ae10, 0x1f9, 0x4312d0);
}

/// the Centre-West league has 15 clubs: those with it as their reserve division in the database (the field that holds
/// the state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegCentral::load_region_clubs` (0x430ef0, bra_reg_central.cpp, thiscall, ret 0).
///
/// [pt] { 0x430ef0 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::load_region_clubs()`: o Centro-Oeste tem 15 clubes (divisão reserva +0x60 = [0x9cf998]).
/// Fonte 0x98ae10, linha 0x168.
pub(crate) unsafe extern "thiscall" fn bra_reg_central_load_region_clubs(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xf, 0x98ae10, 0x168, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 0, "cql_8 centro_oeste cheio")
}

/// Centre-West: after the round robin, the top 8 go to the quarter-finals: 1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th
/// (the seeds are the positions [inference: two legs])
/// Original: `BraRegCentral::create_knockout_stage` (0x4312d0, bra_reg_central.cpp, thiscall, ret 0).
///
/// [pt] { 0x4312d0 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (linhas 0, 7, 2, 5, 6, 1, 4, 3). Fonte 0x98ae10, linhas 0x21c (sem lista) e 0x234 (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_central_create_knockout_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae10, 0x21c, 0x234, "cqn_9 centro_oeste"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Centro-Oeste
}

