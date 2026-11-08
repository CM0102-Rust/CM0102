use super::*;

/// Uma rodada de grupo (registro de 0x41 bytes, como as das ligas): a data pela 0x68a160 (dia, mês com 0 = janeiro,
/// dia da semana, código), sem anos a somar e com valor 0; e o jogo 0 vazio (0x68a1c0 com −1, −1, −1 e valor 0).
pub(crate) type BraChampCupGroupRound = (u32, u32, u32, u32);

/// O que a virtual +0x3c monta numa fase: nada (devolve 0 sem gravar), uma fase de grupos (as rodadas, o extra, a linha
/// da caixa) ou o mata-mata (as rodadas, as palavras a mais (rodada, deslocamento, valor), a linha da caixa).
#[derive(Clone, Copy)]
pub(crate) enum BraChampCupStagePlan {
    Nada,
    Grupos(&'static [BraChampCupGroupRound], i16, u32, &'static str),
    Mata(&'static [EngCcCupCupRound], &'static [(usize, usize, u16)], u32, &'static str),
}

/// create_schedule for the chosen stage: group stages fill a 0x41-byte league round table, knockouts a 0x68-byte cup
/// round table with extra words; sets outputs; returns the table.
///
/// [pt] A virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`, thiscall, ret 0x10) pela fase `f` escolhida. Nos
/// grupos: é_liga (se pedido) = 1, rodadas, extra; a tabela de 0x41 bytes por rodada (0x944e46; sem memória, a caixa e
/// 0) com as datas, o ano +0x40 em 16 bits. No mata-mata: é_liga = 0, rodadas, extra = 0; a tabela de 0x68 bytes por
/// rodada (sem memória, a caixa e 0), as rodadas pela `eng_cc_cup_write_cup_round` com o ano +0x40 com sinal e as palavras a mais.
pub(crate) unsafe fn build_stage_schedule(this: usize, f: BraChampCupStagePlan, rounds: *mut i16, extra: *mut i16, league: *mut i32, file: usize) -> usize {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match f {
        BraChampCupStagePlan::Nada => 0,
        BraChampCupStagePlan::Grupos(grupos, id, linha, nome) => {
            let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
            let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
            if !league.is_null() {
                *league = 1;
            }
            *rounds = grupos.len() as i16;
            *extra = id;
            let t = alloc(grupos.len() as u32 * 0x41);
            if t == 0 {
                show_error_at(file, linha);
                return 0;
            }
            crate::comparacao::ramo(nome);
            // the year goes with garbage in the high bytes in the original (the previous register with the word +0x40 over it); the
            // 0x68a160 only uses the low 16 bits: the copy passes the clean year
            let year = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
            for (r, &(dia, mes, semana, codigo)) in grupos.iter().enumerate() {
                data(t, r as i32, dia as i32, mes as i32, 0, semana as i32, codigo as i32, year, 0);
                jogo(t, r as i32, 0, -1, -1, -1, 0);
            }
            t
        }
        BraChampCupStagePlan::Mata(mata, a_mais, linha, nome) => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = mata.len() as i16;
            *extra = 0;
            let t = alloc(mata.len() as u32 * 0x68);
            if t == 0 {
                show_error_at(file, linha);
                return 0;
            }
            crate::comparacao::ramo(nome);
            let year = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
            for (i, r) in mata.iter().enumerate() {
                eng_cc_cup_write_cup_round(t, i as u32, r, year);
            }
            for &(r, o, v) in a_mais {
                core::ptr::write_unaligned((t + r * 0x68 + o) as *mut u16, v);
            }
            t
        }
    }
}

/// Bytes the original writes in an n-round table for the stage: group rounds +0..+0xb and +0x3d..+0x40, knockout rounds
/// the cup bytes plus the extra words.
///
/// [pt] Os bytes que a original grava na tabela de `n` rodadas da fase `f`: nos grupos (0x41 bytes), +0..+0xb e
/// +0x3d..+0x40 de cada rodada; no mata-mata, os da eng_cc_cup_written_round_bytes e as palavras a mais.
pub(crate) unsafe fn written_schedule_bytes(f: BraChampCupStagePlan, t: usize, n: i16) -> Vec<u8> {
    match f {
        BraChampCupStagePlan::Nada => Vec::new(),
        BraChampCupStagePlan::Grupos(..) => {
            let mut v = Vec::new();
            for r in 0..n.max(0) as usize {
                let e = t + r * 0x41;
                v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
                v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
            }
            v
        }
        BraChampCupStagePlan::Mata(_, a_mais, ..) => {
            let mut v = eng_cc_cup_written_round_bytes(t, n);
            for &(r, o, _) in a_mais {
                v.extend_from_slice(core::slice::from_raw_parts((t + r * 0x68 + o) as *const u8, 2));
            }
            v
        }
    }
}

/// Side-by-side comparer of create_schedule: outputs and written table bytes must match; frees the port's table;
/// tableless stages only run the original.
///
/// [pt] Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
pub(crate) unsafe fn compare_stage_schedule(name: &str, stage: u32, f: BraChampCupStagePlan, rounds: *mut i16, extra: *mut i16, league: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    if let BraChampCupStagePlan::Nada = f {
        let a = o();
        PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", name, stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut l2 = if league.is_null() { 0 } else { *league };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, &mut e2, if league.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let outs = *rounds == r2 && *extra == e2 && (league.is_null() || *league == l2);
    let table = if a != 0 && b != 0 { written_schedule_bytes(f, a, *rounds) == written_schedule_bytes(f, b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && outs && table, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// A Copa dos Campeões, a fase preliminar (fase −1): três rodadas às segundas [inferência: dia da semana 1], 19 e 26 de
/// junho e 3 de julho.
pub(crate) const CHAMPIONS_CUP_PRELIMINARY_ROUNDS: [BraChampCupGroupRound; 3] = [(0x13, 5, 1, 2), (0x1a, 5, 1, 2), (3, 6, 1, 2)]; // cqp10: a preliminar da Copa dos Campeões

/// A Copa dos Campeões, os grupos (fases 0 e 1): três rodadas, 9, 16 e 20 de julho.
pub(crate) const CHAMPIONS_CUP_GROUP_ROUNDS: [BraChampCupGroupRound; 3] = [(9, 6, 0, 2), (0x10, 6, 0, 2), (0x14, 6, 4, 2)];

/// A Copa dos Campeões, o mata-mata (fase 2 em diante): semifinal e final em ida e volta no fim de julho.
pub(crate) const CHAMPIONS_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 2] = [
    cql10r((0x15, 6, 0, 5), (0x17, 6, 0, 0, 2, 0), 0x82, (0, 0x200, 1), 6, (4, 2, 4, 0), (0, 2, 2), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 6, 0, 3), (0x1b, 6, 0, 4, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 3), (0, 0xc3500, 0x61a80)),
];

/// Stage plan of the Copa dos Campeoes by signed stage: -1 preliminary, below 2 groups, 2 onward knockout.
///
/// [pt] A fase da Copa dos Campeões pelo byte com sinal: −1 a preliminar (extra 0xe6, caixa 0xaa), abaixo de 2 os grupos
/// (extra = fase + 0x3fd, caixa 0xcd), 2 em diante o mata-mata (caixa 0xed).
pub(crate) fn champions_cup_stage(stage: u32) -> BraChampCupStagePlan {
    let f = stage as u8 as i8;
    if f == -1 {
        BraChampCupStagePlan::Grupos(&CHAMPIONS_CUP_PRELIMINARY_ROUNDS, 0xe6, 0xaa, "cqp_10 preliminar da copa dos campeoes")
    } else if f < 2 {
        BraChampCupStagePlan::Grupos(&CHAMPIONS_CUP_GROUP_ROUNDS, (f as i16).wrapping_add(0x3fd), 0xcd, "cqp_10 grupos da copa dos campeoes")
    } else {
        BraChampCupStagePlan::Mata(&CHAMPIONS_CUP_KNOCKOUT_ROUNDS, &[], 0xed, "cqp_10 mata-mata da copa dos campeoes")
    }
}

/// rule: the Copa dos Campeões has a preliminary stage (stage −1, id 0xe6) of three rounds on Mondays [inference:
/// weekday 1], 19 and 26 June and 3 July; then two groups (stages 0 and 1, id 0x3fd + stage) of three rounds, 9, 16 and
/// 20 July; the knockout (stage 2 on): semi-final drawn on 21/7 and played on 23/7, return 2 days later, and final
/// drawn on 26/7, match on 27/7, return 3 days later; category 6; two-leg tie-break 1; final prize 800 thousand to the
/// winner and 400 thousand to the runner-up
/// Original: `BraChampCup::create_schedule` (0x425c10, bra_champ_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x425c10 } (thiscall, ret 0x10, bra_champ_cup.cpp; a virtual +0x3c da Copa dos Campeões): `BraChampCup::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a preliminar, os grupos ou o mata-mata (`champions_cup_stage`).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_stage_schedule(this, champions_cup_stage(stage), rounds, extra, league, 0x98a1e8)
}


/// A Merconorte, grupos (fases abaixo de 3): seis rodadas às terças de 15 em 15 dias, de 9 de setembro a 18 de novembro.
pub(crate) const MERCONORTE_GROUP_ROUNDS: [BraChampCupGroupRound; 6] = [(9, 8, 2, 2), (0x17, 8, 2, 2), (7, 9, 2, 2), (0x15, 9, 2, 2), (4, 0xa, 2, 2), (0x12, 0xa, 2, 2)]; // cqp10: os grupos da Merconorte

/// A Merconorte, o mata-mata (fase 3): semifinal em ida e volta (7 dias) e final em jogo único em dezembro.
pub(crate) const MERCONORTE_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 2] = [
    cql10r((0x13, 0xa, 0, 3), (0x19, 0xa, 0, 2, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 4, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xb, 0, 3), (9, 0xb, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Stage plan of the Merconorte: below 3 groups, 3 knockout, above nothing.
///
/// [pt] A fase da Merconorte: abaixo de 3 os grupos (extra = fase + 0x3e9, caixa 0x9b), 3 o mata-mata (caixa 0xc1), acima
/// nada.
pub(crate) fn merconorte_stage(stage: u32) -> BraChampCupStagePlan {
    let f = stage as u8 as i8;
    if f < 3 {
        BraChampCupStagePlan::Grupos(&MERCONORTE_GROUP_ROUNDS, (f as i16).wrapping_add(0x3e9), 0x9b, "cqp_10 grupos da merconorte")
    } else if f == 3 {
        BraChampCupStagePlan::Mata(&MERCONORTE_KNOCKOUT_ROUNDS, &[], 0xc1, "cqp_10 mata-mata da merconorte")
    } else {
        BraChampCupStagePlan::Nada
    }
}



/// A Supercopa Europeia: jogo único em 24 de agosto (sorteio em 4 de julho) em campo neutro (mando 0x8000), sem prêmios.
pub(crate) const EUROPEAN_SUPER_CUP_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((4, 6, 0, 5), (0x18, 7, 0, 4, 2, 0x8000), 0, (0, 0x83, 0), 8, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp10: a Supercopa Europeia
];



/// A Supercopa Asiática: ida e volta (14 dias entre os jogos) a partir de 4 de agosto, sorteio em 4 de julho, sem prêmios.
pub(crate) const ASIAN_SUPER_CUP: [EngCcCupCupRound; 1] = [
    cql10r((4, 6, 0, 5), (4, 7, 0, 5, 2, 0), 0, (0, 0, 0x83), 6, (2, 1, 2, 0), (0, 2, 0xe), (0, 0, 0)), // cqp10: a Supercopa Asiática
];

/// rule: the Asian Super Cup is two-legged, the first match on 4 August (drawn on 4 July) and the return 14 days later,
/// category 6, two-leg tie-break 0x83, no prizes
/// Original: `AsiaSuperCup::create_round_table` (0x410910, asia_super_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x410910 } (thiscall, ret 0x10, asia_super_cup.cpp; a virtual +0x3c da Supercopa Asiática): `AsiaSuperCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x97 sem memória.
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &ASIAN_SUPER_CUP, 0x987be0, 0x97, "cqp_10 tabela da supercopa asiatica")
}


/// rule: the Brazilian Champions' Cup group is single round robin, 3 points a win and 1 a draw; tiebreak by codes 3, 1,
/// 2 and 4 [inference: wins, goal difference, goals for, head-to-head]; two qualifiers; rule bits 8 (those who secure
/// the places go to the next stage); category 23 on each match (the champions' finals one); no prize fund
/// Original: `BraChampCup::set_up_rules` (0x426230, bra_champ_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x426230 } (thiscall, ret 0, bra_champ_cup.cpp; a virtual +0x8c): `BraChampCup::set_up_rules()`: o grupo da
/// Copa dos Campeões do Brasil em turno único, 3 pontos a vitória e 1 o empate, a categoria 0x17 (23, a das finais de
/// campeões), o desempate (3, 1, 2, 4), dois classificados e os bits de regra 8 (quem garante as vagas vai à fase
/// seguinte); sem fundo de prêmios. Depois a tabela das rodadas.
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa dos campeoes");
    set_cup_league_rules(this, &CHAMPIONS_CUP_RULES, 1)
}

