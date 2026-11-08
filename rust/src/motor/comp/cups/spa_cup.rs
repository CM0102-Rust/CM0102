use super::*;

/// SpaCup::update_for_date(today, stage), the Copa del Rey's daily virtual: once per season (flag +0x45), when the UEFA
/// seeding year matches the cup's year (+0x40) and the stage is not 0, it frees the old entrant, round and fixture
/// lists and rebuilds the cup (entrants, rounds, bracket, predicted positions), returning 0 after an error box if a
/// step fails. Always ends by calling the generic cup update for the date and returns its result.
///
/// [pt] { 0x84d740 } (thiscall, ret 8, spa_cup.cpp; a virtual +0x14 da Copa do Rei, todo dia): `SpaCup::update_for_date(hoje,
/// fase)`. Uma vez (+0x45 = 0), quando o ano da classificação da UEFA ([0xde1f58] +0x439, palavra sem sinal) é o da copa
/// (+0x40, com sinal) e a fase não é 0: a lista dos inscritos (+0xa7), a das rodadas (+0xa3) e os confrontos de cada
/// rodada (+0xad, +0x3c rodadas) liberados (0x9452ca; os ponteiros ficam), +0x45 = 1, os inscritos (SpaCup::
/// collect_entrants 0x84d910, thiscall), as rodadas (virtual +0x8c) e a árvore (virtual +0x94) — uma devolução negativa
/// dá a caixa (linhas 0x161, 0x168, 0x16f) e 0 — e as posições previstas (virtual +0x5c). Sempre, no fim, a atualização da
/// copa (0x51f890: push fase; push hoje; mov ecx -> thiscall, ret 8), cuja devolução sai.
pub(crate) unsafe extern "thiscall" fn spa_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    if read_u32_as_usize(this + cup::SEASON_SET_UP) == 0 && read_i16(read_u32_as_usize(G_UEFA_SEEDING) + 0x439) as u16 as i32 == read_i16(this + cup::YEAR) as i32 && stage != 0 { // cqt5: the Copa del Rey year
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        for o in [0xa7usize, 0xa3] {
            if read_u32_as_usize(this + o) != 0 {
                soltar(read_u32_as_usize(this + o));
            }
        }
        if read_u32_as_usize(this + cup::TIE_TABLES) != 0 {
            let mut i = 0i32;
            while i < read_i16(this + cup::ROUND_COUNT) as i32 {
                let p = read_u32_as_usize(read_u32_as_usize(this + cup::TIE_TABLES) + i as usize * 4);
                if p != 0 {
                    soltar(p);
                }
                i += 1;
            }
            soltar(read_u32_as_usize(this + cup::TIE_TABLES));
        }
        crate::comparison::branch("cqt_5 copa do rei refeita");
        core::ptr::write_unaligned((this + cup::SEASON_SET_UP) as *mut u32, 1); // cqt5: the Copa del Rey redone once
        let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_SPACUP_COLLECT_ENTRANTS);
        // simplification: in the message boxes the original zeroes only al (the rest of eax comes from the box); the caller reads the byte
        if entrants(this) < 0 {
            show_error_at(0xa82810, 0x161);
            return 0;
        }
        if call_cup_virtual(this, 0x8c) < 0 {
            show_error_at(0xa82810, 0x168);
            return 0;
        }
        if call_cup_virtual(this, 0x94) < 0 {
            show_error_at(0xa82810, 0x16f);
            return 0;
        }
        call_cup_virtual(this, 0x5c);
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
    base(this, today, stage)
}

// simplification: the season turn frees and rebuilds the tables and the cup update plays and draws: they do not run twice. Only the
// original; the plan: +0x45 goes to 1 when the condition holds, otherwise it stays.

/// record_season_honours of a cup: calls virtual +0x30 of its final stage ([+0xc][o/4]) and returns its eax.
///
/// [pt] O histórico da temporada de uma copa (`record_season_honours`, 11 bytes: mov eax, [ecx+0xc]; mov ecx, [eax+o]; mov
/// edx, [ecx]; jmp [edx+0x30]): a virtual +0x30 da fase [+0xc][o/4] (a última fase da copa: a final), com o eax dela.
pub(crate) unsafe fn record_season_honours(this: usize, o: usize, ramo: &'static str) -> i32 {
    crate::comparison::branch(ramo);
    let stage = read_u32_as_usize(read_u32_as_usize(this + cup::STAGES) + o);
    call_cup_virtual(stage, 0x30)
}

/// EuropeanCup::record_season_honours(): records the season's honours by calling the honours routine of the cup's final
/// stage (stage slot +0x30, the Champions League final). Returns that routine's result.
///
/// [pt] { 0x584ad0 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::record_season_honours()`: o histórico da fase 12
/// (+0x30: a final da Liga dos Campeões).
pub(crate) unsafe extern "thiscall" fn spa_cup_record_season_honours(this: usize) -> i32 {
    record_season_honours(this, 0x30, "cqt_5 historico da liga dos campeoes")
}

/// GreCup::record_season_honours (the Greek Cup, sharing code with the Spanish Cup): records the season's winners in
/// the history from stage 10 (+0x28). Returns the shared routine's result.
///
/// [pt] { 0x5e93e0 } (thiscall, ret 0, gre_cup.cpp): `GreCup::record_season_honours()`: o histórico da fase 10 (+0x28).
pub(crate) unsafe extern "thiscall" fn spa_cup_record_season_honours_5e93e0(this: usize) -> i32 {
    record_season_honours(this, 0x28, "cqt_5 historico da copa da grecia")
}

/// IreLgeCup::record_season_honours() (the Irish league cup, despite the file name): records the season's honours by
/// calling virtual +0x30 of the final stage (stage 7, cup +0xc table entry +0x1c); returns its result.
///
/// [pt] { 0x637750 } (thiscall, ret 0, ire_lge_cup.cpp): `IreLgeCup::record_season_honours()`: o histórico da fase 7 (+0x1c).
pub(crate) unsafe extern "thiscall" fn spa_cup_record_season_honours_637750(this: usize) -> i32 {
    record_season_honours(this, 0x1c, "cqt_5 historico da copa da liga irlandesa")
}

/// ItaCCup::record_season_honours (Italian Serie C cup, despite the file): records the season honours from stage 15
/// (+0x3c, the cup's knockout) by calling its virtual +0x30; returns that call's result.
///
/// [pt] { 0x63d770 } (thiscall, ret 0, ita_c_cup.cpp): `ItaCCup::record_season_honours()`: o histórico da fase 15 (+0x3c: o
/// mata-mata da Copa da Série C).
pub(crate) unsafe extern "thiscall" fn spa_cup_record_season_honours_63d770(this: usize) -> i32 {
    record_season_honours(this, 0x3c, "cqt_5 historico da copa da serie c")
}

// simplification: the history writes to the competition statistics (finds or appends entries: allocates); original only

/// rule: the Copa del Rey has 7 rounds: preliminary (code 0xe6, 38 teams, 7 days between legs) drawn 18/8, played 30/8;
/// 1st round 8/9 and 13/12 (64 teams, 45 new); 2nd 15/12 and 3/1; 3rd (last 16) 5/1 and 10/1, quarter-finals 19/1 and
/// 31/1 and semi-final 23/2 and 11/4 over two legs (7, 7 and 14 days apart); final drawn 26/4, played 26/5 at a neutral
/// ground [inference: venue 4]; up to the semi-final the lower-division club plays at home and clubs of the same nation
/// are kept apart in the draw (+0xb = 3); no prize per round (−1); category 4 up to the last 16 and 6 after
/// Original: `SpaCup::create_round_table` (0x84d220, spa_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x84d220 } (thiscall, ret 0x10, spa_cup.cpp; a virtual +0x3c da Copa do Rei): `SpaCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa8 sem memória.
pub(crate) unsafe extern "thiscall" fn spa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(this, stage, rounds, extra, out, &COPA_DEL_REY_ROUNDS, 0xa82810, 0xa8, "cql_11 tabela da copa do rei")
}

