use super::*;

/// rule: the FA Cup has 9 single-leg rounds with replays (+0x20 = 1, 10 days later) up to the quarter-finals:
/// preliminary (code 0xdc) drawn 9/10, played 16/10, 56 teams; 1st round 17/10 and 18/11; 2nd 19/11 and 9/12; 3rd 10/12
/// and 6/1; 4th 7/1 and 27/1; 5th 28/1 and 17/2; quarter-finals 18/2 and 10/3; semi-final 11/3 and 8/4 at its own
/// ground (venue 0x2a, no replay); final 9/4 and 12/5 at a neutral ground; the team counts from the 2nd to the 5th
/// round depend on the exemptions (+0xb3) and on the choice of entering at the 5th round (+0xb4 = 1); prize for winning
/// a round from 10 thousand to 400 thousand, and the final 2 million to the winner and 1 million to the runner-up;
/// category 4 up to the 5th round and 6 from the quarter-finals
/// Original: `EngFaCup::create_round_table` (0x56f320, eng_fa_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x56f320 } (thiscall, ret 0x10, eng_fa_cup.cpp; a virtual +0x3c da Copa da Inglaterra): `EngFaCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, as 9 rodadas (`fa_cup_rounds`);
/// a caixa da linha 0xb6 sem memória.
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = fa_cup_rounds(*((this + eng_fa_cup::EXEMPTION_COUNT) as *const i8) as i16, *((this + eng_fa_cup::FIFTH_ROUND_ENTRY) as *const u8), core::ptr::read_unaligned((this + cup::TEAM_COUNT) as *const i16));
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &t, 0x9c42cc, 0xb6, "cql_10 tabela da copa da inglaterra")
}

/// Daily update of the FA Cup. Rule: the FA Cup gathers its entrants on 3 October each year (Date(3, 9): month 9 is
/// October; the community map says September), rebuilds the rounds and the tree and draws the preliminary round, once
/// per season (+0x45); on 1 October 1999 it asked the human manager whether the club enters at the 5th round (+0xb2
/// marks the question); then, every day, the cup update.
///
/// Ret 8; virtual +0x14 of the English cup; SEH frame for the news of the 1999 path, no destructor on the normal path:
/// `EngFaCup::update_for_date(today, stage)`. With +0x45 = 0 and the stage different from 0: on Date(1, 9, 1999) (month
/// 9 is October) with +0xb2 = 0, the question to the human manager about entering at the 5th round (only in 1999: the
/// original routine through the trampoline); on Date(3, 9, today's year) (3 October), frees the tables, +0x45 = 1, the
/// entrants (0x56fab0, thiscall), the rounds (virtual +0x8c), the tree (virtual +0x94) (a negative return gives the
/// box, lines 0x3d6, 0x3dd, 0x3e4, and 1), the predicted order (virtual +0x5c) and the draw of the 1st round (0x51c800:
/// push 0; mov ecx, cup -> thiscall, ret 4). Always, at the end, the cup update.
/// Original: `EngFaCup::update_for_date` (0x570550, eng_fa_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x570550 } (thiscall, ret 8, eng_fa_cup.cpp; a virtual +0x14 da Copa da Inglaterra; quadro SEH pela notícia do
/// caminho de 1999, sem destrutor no caminho normal): `EngFaCup::update_for_date(hoje, fase)`. Com +0x45 = 0 e a fase
/// diferente de 0: em Date(1, 9, 1999) (o mês 9 é outubro) com +0xb2 = 0, a pergunta ao técnico humano sobre a entrada na
/// 5ª rodada (só em 1999: a original pelo trampolim); em Date(3, 9, ano de hoje) (3 de outubro), libera as tabelas, +0x45
/// = 1, os inscritos (0x56fab0, thiscall), as rodadas (virtual +0x8c), a árvore (virtual +0x94) — uma devolução negativa
/// dá a caixa (linhas 0x3d6, 0x3dd, 0x3e4) e 1 —, a ordem prevista (virtual +0x5c) e o sorteio da 1ª rodada (0x51c800:
/// push 0; mov ecx, copa -> thiscall, ret 4). Sempre, no fim, a atualização da copa.
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    if read_u32_as_usize(this + cup::SEASON_SET_UP) == 0 && stage != 0 {
        if is_date_today(make_date_dword(1, 9, 0x7cf, 0xffff_ffff), today) && *((this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) as *const u8) == 0 {
            // simplification: the 1999 question builds and delivers a news item (only on that day, outside any new career)
            return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ENG_FA_CUP_UPDATE_FOR_DATE)(this, today, stage);
        }
        // simplification: the year goes in ax with the rest of eax in the high bytes; 0x549f70 reads it as a word
        if is_date_today(make_date_dword(3, 9, read_i16(today + 2) as u16 as u32, 0xffff_ffff), today) { // cqn10: the FA Cup entries on 3/10
            free_season_tables(this);
            crate::comparison::branch("cqn_10 inscricoes da copa da inglaterra");
            core::ptr::write_unaligned((this + cup::SEASON_SET_UP) as *mut u32, 1);
            let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_ENGFACUP_COLLECT_ENTRANTS);
            if entrants(this) < 0 {
                show_error_at(0x9c42cc, 0x3d6);
                return 1;
            }
            if call_cup_virtual(this, 0x8c) < 0 {
                show_error_at(0x9c42cc, 0x3dd);
                return 1;
            }
            if call_cup_virtual(this, 0x94) < 0 {
                show_error_at(0x9c42cc, 0x3e4);
                return 1;
            }
            call_cup_virtual(this, 0x5c);
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
            sortear(this, 0);
        }
    }
    eng_cc_cup_cup_update_for_date(this, today, stage)
}
