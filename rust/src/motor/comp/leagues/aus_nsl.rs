use super::*;




/// Club in row i of the table (+0xb1, 0x3b-byte rows) of league or stage t.
///
/// [pt] O clube da linha `i` da tabela (+0xb1, linhas de 0x3b bytes) da liga ou fase `t`.
pub(crate) unsafe fn aus_nsl_table_row_club(t: usize, i: usize) -> usize {
    read_usize(read_usize(t + 0xb1) + i * 0x3b)
}

/// The NSL has the regular season (round robin), the finals series (stage 0) and the grand final (stage 1): the next
/// stage moves the current stage on and builds the finals series at the end of the regular season and the grand final
/// at the end of the finals series; past the last stage it is an error (the box)
/// Original: `AusNsl::advance_stage` (0x411f70, aus_nsl.cpp, thiscall, ret 0).
///
/// [pt] { 0x411f70 } (thiscall, ret 0, aus_nsl.cpp): `AusNsl::advance_stage()`, a fase seguinte da NSL (a temporada regular,
/// a fase −1; a série final dos seis primeiros, a fase 0; a grande final, a fase 1). Na última fase (+0x30 ≥ +0x2c − 1,
/// com sinal): a caixa de erro da linha 0x2f0 e nada mais. Senão +0x30 sobe um; ao chegar a 0 monta a série final
/// (0x412010) e a 1, a grande final (0x4121c0), as duas em call direto com o ecx da liga.
pub(crate) unsafe extern "thiscall" fn aus_nsl_advance_stage(this: usize) {
    let stage = read_usize(this + 0x30) as i32;
    if stage >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x987c20, 0x2f0);
    }
    let stage = stage + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, stage);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_AUSNSL_CREATE_FINALS_SERIES_STAGE);
        f(this);
    } else if stage == 1 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AUSNSL_CREATE_GRAND_FINAL_STAGE);
        f(this);
    }
}

/// The NSL finals series: the top six of the regular season play their own round-robin stage (with the league points
/// and tie-breakers) and its top two go to the grand final; sends the draw news. [inference] The real NSL used a
/// double-chance knockout; here it is a short league of the six
/// Original: `AusNsl::create_finals_series_stage` (0x412010, aus_nsl.cpp, thiscall, ret 0).
///
/// [pt] { 0x412010 } (thiscall, ret 0, aus_nsl.cpp; quadro SEH só para o new): `AusNsl::create_finals_series_stage()`, a
/// série final da NSL, do fim da temporada regular.
/// 1) Os seis primeiros da tabela da liga (as linhas 0 a 5, na ordem) num vetor local de 6.
/// 2) A tabela das rodadas pela virtual +0x3c (create_round_table: thiscall(this, 0, &rodadas, &extra, 0), ret 0x10).
/// 3) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, 6, o vetor, 1, +4, a tabela das rodadas, as rodadas (word), os pontos +0xc2 e +0xc3,
///    a categoria +0xc4, &+0xc5, &{2,0,0,0} (duas vagas de cima: os dois que vão à grande final), o ano +0x40 (word),
///    0, o extra (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
///    altos, que o construtor não lê: a cópia passa os valores limpos). Sem memória: 0.
/// 4) [+0xc][0] = a fase e a tabela das rodadas liberada (free 0x9452ca). Sem fase: a caixa da linha 0x328 e 0. Com
///    ela: a notícia do sorteio (0x684230 na fase, thiscall, ret 0), +0x30 = 0 e 1.
/// [inferência] A série final é uma liga curta dos seis; os dois primeiros dela vão à grande final.
pub(crate) unsafe extern "thiscall" fn create_finals_series_stage(this: usize) -> u32 {
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let slots: [u8; 4] = [2, 0, 0, 0];
    let mut clubs = [0usize; 6];
    for (i, c) in clubs.iter_mut().enumerate() {
        *c = aus_nsl_table_row_club(this, i); // cqr3: os seis primeiros da tabela
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, 0, &mut rodadas, &mut extra, 0);
    let p = new(0xee);
    let stage = if p != 0 {
        construct(p, this, 6, clubs.as_ptr(), 1, read_usize(this + 4), t, rodadas as u16 as u32, read_u8(this + 0xc2) as u32,
                  read_u8(this + 0xc3) as u32, read_u8(this + 0xc4) as u32, this + 0xc5, slots.as_ptr(), read_i16(this + 0x40) as u16 as u32, 0,
                  extra as u16 as u32, read_u8(this + 0x51) as u32, 1, 0, read_i16(this + 0xd9) as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned(read_usize(this + 0xc) as *mut usize, stage);
    soltar(t);
    let stage = read_usize(read_usize(this + 0xc));
    if stage == 0 {
        show_error_at(0x987c20, 0x328);
        return 0;
    }
    crate::comparacao::ramo("cqr_3 serie final");
    noticia(stage);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
    1
}

/// The NSL grand final: the top two of the finals series in a two-club cup stage [inference: a single game, the first
/// as home side]; the winner is the Australian champion
/// Original: `AusNsl::create_grand_final_stage` (0x4121c0, aus_nsl.cpp, thiscall, ret 0).
///
/// [pt] { 0x4121c0 } (thiscall, ret 0, aus_nsl.cpp; quadro SEH só para o new): `AusNsl::create_grand_final_stage()`, a grande
/// final da NSL, do fim da série final: os dois primeiros da tabela da série final ([+0xc][0], linhas 0 e 1, nessa
/// ordem) numa lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x348 e volta); a tabela das rodadas
/// pela virtual +0x3c (thiscall(this, 1, &rodadas, &extra, 0), ret 0x10); a fase (operator new 0x944cf1(0xb2) e o
/// construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano +0x40, 1, 1,
/// o extra, 0x14, 0, 0, 0 e sementes nulas; a cópia passa as words limpas) em [+0xc][1] e a tabela liberada (free
/// 0x9452ca). Sem fase: a caixa da linha 0x35c e volta (sem liberar a lista). Com ela: Cup::set_up_round (0x51c800,
/// thiscall(fase, 0), ret 4) e a lista liberada. A fase atual (+0x30 = 1) já foi gravada pela fase seguinte.
/// [inferência] A grande final é um jogo só, com o primeiro da série final como mandante.
pub(crate) unsafe extern "thiscall" fn create_grand_final_stage(this: usize) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let list = alloc(8);
    if list == 0 {
        return show_error_at(0x987c20, 0x348);
    }
    let serie = read_usize(read_usize(this + 0xc));
    core::ptr::write_unaligned(list as *mut usize, aus_nsl_table_row_club(serie, 0)); // cqr3: o primeiro da série final
    core::ptr::write_unaligned((list + 4) as *mut usize, aus_nsl_table_row_club(serie, 1)); // cqr3: o segundo da série final
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, 1, &mut rodadas, &mut extra, 0);
    let p = new(0xb2);
    let stage = if p != 0 {
        construct(p, this, 2, list, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 1, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 4) as *mut usize, stage);
    soltar(t);
    let stage = read_usize(read_usize(this + 0xc) + 4);
    if stage == 0 {
        return show_error_at(0x987c20, 0x35c);
    }
    crate::comparacao::ramo("cqr_3 grande final");
    build(stage, 0);
    soltar(list);
}







/// Writes the NSL rule fields (3 rounds, points, tiebreaks, 6 finalists...) before the schedule.
///
/// [pt] As regras da NSL (o que a 0x411ce0 grava antes do calendário).
pub(crate) unsafe fn set_nsl_rules(o: usize) {
    write_u16_at(o, 0xab, 0);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0x52, 2);
    write_u16_at(o, 0x3c, 3); // três turnos
    write_u8_at(o, 0xc2, 3); // cqj8: os pontos por vitória da NSL
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xbe, 6); // os seis da fase final
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u16_at(o, 0xd9, 0x28); // 0x8 fase seguinte, 0x20 eliminados marcados
    write_u8_at(o, 0x49, 4);
}

/// Writes the MLS rule fields (no schedule; 32 games, fixed round order...).
///
/// [pt] As regras da MLS (0x908660; sem calendário: os jogos da MLS são montados à mão pela 0x9086e0).
pub(crate) unsafe fn set_mls_rules(o: usize) {
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u8_at(o, 0xc3, 1); // cqj8: os pontos por empate da MLS
    write_u16_at(o, 0xab, 0xc2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // desempates: saldo, gols pró, confronto direto
    write_u8_at(o, 0xc7, 3);
    write_u8_at(o, 0xbe, 1);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u16_at(o, 0xd9, 0x48); // 0x8 fase seguinte, 0x40 rodadas em ordem fixa
    write_u16_at(o, 0x3c, 4);
    write_u16_at(o, 0xa7, 0x20); // 32 jogos por time
}

/// NSL: three round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; the top 6 go to
/// the finals series (rule word 0x28: 0x8 next stage, no champion band, and 0x20, clubs without a chance marked
/// eliminated); nobody goes up or down; 4 bench substitutes and 3 substitutions; fixed squad numbers. Then the schedule
/// Original: `AusNsl::set_up_rules` (0x411ce0, aus_nsl.cpp, thiscall).
///
/// [pt] { 0x411ce0 } (thiscall, aus_nsl.cpp): `AusNsl::set_up_rules` (slot 36), as regras da NSL e o calendário da fase
/// principal pelo slot 16 (`create_schedule` 0x4111a0, thiscall, ret 0x10: estágio -1, as saídas +0xa9 e +0x3a, sem
/// a marca; o ponteiro vai para +0xba); devolve 1.
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_up_rules(this: usize) -> u32 {
    set_nsl_rules(this);
    crate::comparacao::ramo("cqj_8 regras da nsl");
    let vt = *(this as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(this, -1, this + 0xa9, this + 0x3a, 0);
    write_u32_at(this, 0xba, r as u32);
    1
}




/// Calls record_season_honours (vtable +0x30) on the first stage [[this+0xc]].
///
/// [pt] O histórico do primeiro estágio: o slot 13 (+0x30, `record_season_honours`, thiscall sem argumentos) do estágio
/// [[this+0xc]].
pub(crate) unsafe fn record_first_stage_honours(this: usize) -> u32 {
    let e = *(*((this + 0xc) as *const usize) as *const usize);
    let vt = *(e as *const usize);
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(*((vt + 0x30) as *const usize));
    f(e)
}



/// end of the Argentine second division and Series A and B: the honours come from the first stage, not the overall
/// table
/// Original: `ArgSecond::record_season_honours` (0x408eb0, arg_second.cpp, thiscall).
///
/// [pt] { 0x408eb0 } (thiscall, arg_second.cpp): `ArgSecond::record_season_honours` (slot 13 da segunda divisão argentina e
/// das Séries A e B): só o histórico do primeiro estágio, em salto final; devolve o eax dele.
pub(crate) unsafe extern "thiscall" fn aus_nsl_record_season_honours(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_8 historico do primeiro estagio");
    record_first_stage_honours(this)
}


/// NSL: at the end of the regular season, the six clubs through to the finals series get the qualification record
/// (stage 0x435, weight 0x28); in the finals, those who go through get the record (round 0x96) and those who fall the
/// failed-to-qualify one; the champion (grand final) gets the champion record. No fate written to the table
/// Original: `AusNsl::update_staff_records_for_status` (0x411980, aus_nsl.cpp, thiscall).
///
/// [pt] { 0x411980 } (thiscall, ret 0x18, aus_nsl.cpp): `AusNsl::update_staff_records_for_status`: na temporada regular, os
/// seis classificados (6) ganham o registro de classificação para a fase final (fase 0x435); os de fora (-2) devolvem
/// -1; na fase final (fase 0), quem avança (6) ganha o registro (rodada 0x96) e quem cai (-2) o de não classificado
/// (fase 0x435) e -1; depois, o campeão (1) ganha o registro de campeão. Devolve 0 ou -1.
pub(crate) unsafe extern "thiscall" fn aus_nsl_update_staff_records_for_status(this: usize, clube: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    nsl_end_of_stage(this, clube, status, stage, true).0
}


/// the Australian NSL has 13 clubs, those of the division in database order; with fewer, the game shows the error box;
/// no prize fund nor TV money
/// Original: `AusNsl::create_table` (0x411b70, aus_nsl.cpp, thiscall, ret 0).
///
/// [pt] { 0x411b70 } (thiscall, ret 0, aus_nsl.cpp): `AusNsl::create_table()`: a NSL tem 13 clubes (+0x57 = [0x9cf89c]), na
/// ordem do banco, até encher (o contador num int, comparado no word); sem encher: a caixa da linha 0x252 e 0; cheia: 1,
/// sem fundo de prêmios nem TV. Fonte 0x987c20 (a caixa de memória na linha 0x241).
pub(crate) unsafe extern "thiscall" fn aus_nsl_create_table(this: usize) -> u32 {
    if alloc_league_table(this, 0xd, 0x987c20, 0x241) == 0 {
        return 0;
    }
    let mut k: u32 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if (k as i16) >= read_i16(this + 0x3e) {
            break;
        }
        if is_in_division(clube, KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE) {
            jap_j1_init_table_row(this, k as u8, read_i16(this + 0x3e), 0, clube); // cqm9: a linha da NSL
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i16 != read_i16(this + 0x3e) {
        show_error_at(0x987c20, 0x252);
        return 0;
    }
    crate::comparacao::ramo("cqm_9 nsl cheia");
    1
}

