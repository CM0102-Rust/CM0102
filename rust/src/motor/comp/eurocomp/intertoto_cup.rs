use super::*;

/// A Intertoto: cinco rodadas em ida e volta de junho a agosto (a 1ª com 40 times; 12 novos na 2ª, 8 na 3ª).
pub(crate) const INTERTOTO_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x19, 5, 0, 4), (0x1a, 5, 0, 5, 1, 0), 0xa, (0, 4, 7), 7, (0x28, 0x14, 0x28, 0), (0, 2, 4), (0, 0, 0)), // cqp13: a 1ª rodada da Intertoto
    cql10r((1, 6, 0, 3), (3, 6, 0, 5, 1, 0), 0x14, (0, 0, 3), 7, (0x20, 0x10, 0xc, 0x28), (0, 2, 7), (0, 0, 0)),
    cql10r((0xb, 6, 0, 6), (0x11, 6, 0, 5, 1, 0), 0x1e, (0, 0, 3), 7, (0x18, 0xc, 8, 0x34), (0, 2, 7), (0, 0, 0)),
    cql10r((0x19, 6, 0, 6), (0x1c, 6, 0, 2, 2, 0), 0x82, (0, 0, 3), 8, (0xc, 6, 0, 0), (0, 2, 7), (0, 0, 0)),
    cql10r((5, 7, 0, 3), (0xa, 7, 0, 1, 2, 0), 0x96, (0, 0, 3), 8, (6, 3, 0, 0), (0, 2, 0xe), (0, 0, 0)),
];

/// rule: the Intertoto Cup has 5 two-legged rounds: 1st drawn on 25/6 and played on 26/6 (40 teams, return 4 days
/// later), 2nd on 3/7 (32 teams: 20 through and 12 new), 3rd on 17/7 (24: 16 and 8 new), semi-final on 28/7 and final
/// on 10/8 (6 teams: the 3 winners go to the UEFA Cup [inference]); category 7 in the first three and 8 after; no
/// prizes
/// Original: `IntertotoCup::create_round_table` (0x633300, intertoto_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x633300 } (thiscall, ret 0x10, intertoto_cup.cpp; a virtual +0x3c): `IntertotoCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0xe4 sem memória.
pub(crate) unsafe extern "thiscall" fn intertoto_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &INTERTOTO_ROUNDS, 0x9e9e14, 0xe4, "cqp_13 tabela da intertoto")
}


/// A Copa Intercontinental: jogo único em 1º de dezembro (sorteio em 10 de outubro), mando 4 (Tóquio [inferência]).
pub(crate) const INTERCONTINENTAL_CUP_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((0xa, 9, 0, 5), (1, 0xb, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp13: a Copa Intercontinental
];

/// rule: the Intercontinental Cup (the European against the South American champion [inference]) is a single match on 1
/// December, drawn on 10 October, venue 4 (Tokyo [inference]), category 6, tie-break 3
/// Original: `WorldClubCup::create_round_table` (0x92b770, world_club_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x92b770 } (thiscall, ret 0x10, world_club_cup.cpp; a virtual +0x3c): `WorldClubCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
pub(crate) unsafe extern "thiscall" fn intertoto_cup_create_round_table_92b770(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &INTERCONTINENTAL_CUP_ROUNDS, 0xad5848, 0x9a, "cqp_13 tabela da intercontinental")
}


/// A Copa Interamericana: ida e volta a partir de 26 de janeiro do ano seguinte (sorteio em 10 de outubro), 7 dias.
pub(crate) const INTERAMERICAN_CUP: [EngCcCupCupRound; 1] = [
    cql10r((0xa, 9, 0, 5), (0x1a, 0, 1, 2, 2, 0), 0, (0, 0, 3), 8, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)),
];

/// rule: the Inter-American Cup is two-legged from 26 January of the following year (7 days), drawn on 10 October,
/// category 8, two-leg tie-break 3
/// Original: `InterAmerCup::create_round_table` (0x632340, inter_amer_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x632340 } (thiscall, ret 0x10, inter_amer_cup.cpp; a virtual +0x3c): `InterAmerCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
pub(crate) unsafe extern "thiscall" fn intertoto_cup_create_round_table_632340(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &INTERAMERICAN_CUP, 0x9e9d94, 0x9a, "cqp_13 tabela da interamericana")
}


/// A Copa dos Campeões da CONCACAF: quartas (28/8), semifinal (1/9) e final (4/9) em jogo único, 8 times.
pub(crate) const CONCACAF_CHAMPIONS_CUP_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((1, 7, 0, 5), (0x1c, 8, 0, 3, 2, 0), 0xa, (0, 3, 0), 8, (8, 4, 8, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqp13: as quartas da CONCACAF
    cql10r((0x1d, 8, 0, 4), (1, 9, 0, 6, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((2, 9, 0, 0), (4, 9, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];



/// As rodadas fixas da Copa da UEFA (a preliminar e os campos que dependem de +0xb2 vêm à parte, em
/// `uefa_cup_rounds`): a 1ª rodada de qualificação em agosto, a 2ª no fim de agosto, a 1ª rodada em setembro, a 2ª
/// em novembro, a 3ª em dezembro, as oitavas em fevereiro, as quartas e a semifinal em março e abril e a final em
/// maio, todas em ida e volta menos a final.
pub(crate) const UEFA_CUP_ROUNDS: [EngCcCupCupRound; 8] = [
    cql10r((0x1c, 6, 0, 4), (0xa, 7, 0, 3, 2, 0), 0xdc, (4, 0, 3), 7, (0x52, 0x29, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqq10: a 1ª rodada de qualificação da UEFA
    cql10r((0x19, 7, 0, 4), (0xe, 8, 0, 3, 2, 0), 0xa, (4, 0x400, 3), 7, (0x60, 0x30, 0x37, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 8, 0, 4), (0x13, 9, 0, 3, 2, 0), 0x14, (4, 0, 3), 7, (0x30, 0x18, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((3, 0xa, 0, 4), (0x17, 0xa, 0, 3, 2, 0), 0x1e, (0, 0, 3), 8, (0x20, 0x10, 8, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((8, 0xb, 0, 4), (1, 2, 1, 3, 2, 0), 0x28, (0, 0, 3), 8, (0x10, 8, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((9, 2, 1, 4), (0xf, 2, 1, 3, 2, 0), 0x78, (0, 0, 3), 8, (8, 4, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x17, 2, 1, 4), (5, 3, 1, 3, 2, 0), 0x82, (0, 0, 3), 8, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x14, 3, 1, 4), (0x10, 4, 1, 2, 2, 4), 0x96, (0, 0x83, 0), 8, (2, 1, 0, 0), (0, 1, 0), (0, 0x16e360, 0)),
];

/// A preliminar (só com `b` = +0xb2 > 0): sorteio em 28/6, jogo em 6/7; b times, b/2 jogos, b novos.
pub(crate) const UEFA_CUP_PRELIMINARY_ROUND: EngCcCupCupRound =
    cql10r((0x1c, 5, 0, 2), (6, 6, 0, 3, 2, 0), 0xa0, (0, 0x200, 3), 7, (0, 0, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0));

/// UEFA Cup round descriptions for b = +0xb2 (preliminary round when b > 0, then the qualifying and main rounds with
/// new/entered team counts) → (rounds, count).
///
/// [pt] As rodadas da Copa da UEFA com `b` = +0xb2 (byte com sinal; as metades truncam como a original, cdq/sar): com b > 0
/// a preliminar (b times, b/2 jogos, b novos) e depois as oito; na 1ª rodada de qualificação 0x52 − b/2 novos e b já
/// inscritos, na 2ª b/2 + 0x52 inscritos, na 1ª rodada b/2 + 0x89 inscritos.
pub(crate) fn uefa_cup_rounds(b: i8) -> ([EngCcCupCupRound; 9], usize) {
    let mut t = [UEFA_CUP_PRELIMINARY_ROUND; 9];
    let (b, m) = (b as i32, b as i32 / 2);
    let mut u = UEFA_CUP_ROUNDS;
    u[0].times.2 = (0x52 - m) as u16;
    u[0].times.3 = b as u16;
    u[1].times.3 = (m + 0x52) as u16;
    u[3].times.3 = (m + 0x89) as u16;
    let mut n = 0;
    if b > 0 {
        crate::comparacao::ramo("cqq_10 preliminar da uefa");
        t[0].times = (b as u16, m as u16, b as u16, 0);
        n = 1;
    }
    for r in u {
        t[n] = r;
        n += 1;
    }
    (t, n)
}





/// IntertotoCup::collect_entrants: the 60 clubs whose European cup (+0x1db) is the Intertoto, put in the team list
/// (+0xa7) with their coefficient as class and sorted; returns the result pair.
///
/// [pt] { 0x6336c0 } (thiscall, ret 0, intertoto_cup.cpp): `IntertotoCup::collect_entrants()`: +0x3e = 60; um vetor local de
/// 60 ponteiros (0x944e46 de 0xf0; sem memória, a caixa da linha 0x161 e 0) com os clubes da tabela ([0xae23b4], até
/// [0xae235c]) cuja copa europeia (+0x1db) é a Intertoto ([0x9cf6f8]), até 60; faltando, a caixa da linha 0x172 e 0 (o
/// vetor fica alocado, como na original). A lista (0x944e46 de 6 × 60 em +0xa7; sem memória, a caixa 0x17c e −1) leva
/// cada clube com a classe = o seu coeficiente (+0x1df) e a situação 0, vai ao qsort (0x522190) e o vetor é liberado.
/// Devolve 1 com os 60.
pub(crate) unsafe fn collect_intertoto_entrants(this: usize) -> (i32, i32) {
    const F: usize = 0x9e9e14;
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x3c);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let v = alloc(0xf0);
    if v == 0 {
        show_error_at(F, 0x161);
        return (0, 0);
    }
    let mut k = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < read_u32_as_usize(G_CLUB_COUNT) as i32 {
        if k >= read_i16(this + 0x3e) as i32 {
            break;
        }
        let c = read_u32_as_usize(G_CLUB_TABLE).wrapping_add((i as isize * 0x245) as usize);
        if read_u32_as_usize(c + 0x1db) == read_u32_as_usize(KEY_CLUB_COMP_INTER_TOTO_CUP) {
            core::ptr::write_unaligned((v + k as usize * 4) as *mut u32, c as u32);
            k += 1;
        }
        i = i.wrapping_add(1);
    }
    let n = read_i16(this + 0x3e) as i32;
    if k < n {
        show_error_at(F, 0x172);
        return (0, 0);
    }
    let l = alloc((n * 6) as u32);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(F, 0x17c);
        return (-1, 0);
    }
    crate::comparacao::ramo("cqq_10 inscritos da intertoto");
    let mut j: i16 = 0;
    while j < read_i16(this + 0x3e) {
        let c = read_u32_as_usize(v + j as usize * 4);
        let e = read_u32_as_usize(this + 0xa7) + j as usize * 6;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 4) as *mut u8) = *((c + 0x1df) as *const u8); // cqq10: a classe pelo coeficiente
        *((e + 5) as *mut u8) = 0;
        j = j.wrapping_add(1);
    }
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    sort(read_u32_as_usize(this + 0xa7), read_i16(this + 0x3e) as i32 as u32, 6, 0x522190);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    soltar(v);
    ((k == read_i16(this + 0x3e) as i32) as i32, read_i16(this + 0x3e) as i32)
}

/// rule: the Intertoto Cup takes the 60 clubs the European rules put in it (European cup +0x1db = the Intertoto), in
/// club table order; each with class = its coefficient (+0x1df), and the list is sorted by class (the seeds); with
/// fewer than 60, the error box
/// Original: `IntertotoCup::collect_entrants` (0x6336c0, intertoto_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: entram na Intertoto os 60 clubes que as regras europeias puseram nela (copa europeia +0x1db = a Intertoto),
/// pela ordem da tabela de clubes; cada um com a classe = o seu coeficiente (+0x1df), e a lista é ordenada pela classe
/// (as sementes); com menos de 60, a caixa de erro
pub(crate) unsafe extern "thiscall" fn intertoto_cup_collect_entrants(this: usize) -> i32 {
    collect_intertoto_entrants(this).0
}


/// IntertotoCup::collect_club_list: allocates a pointer list of the clubs in +0xa7 (only status 0/1 when ativos),
/// stores it in saida and returns the count.
///
/// [pt] { 0x633f70 } (thiscall, ret 8, intertoto_cup.cpp; uma virtual): `IntertotoCup::collect_club_list(&saida, ativos)`:
/// uma lista de ponteiros (0x944e46 de 4 × +0x3e; sem memória, a caixa da linha 0x2fb, saida = 0 e 0) com os clubes da
/// lista +0xa7 (não nulos, até +0x3e): com `ativos`, só os de situação (+5) 0 ou 1. Grava a lista em saida e devolve a
/// conta (palavra).
pub(crate) unsafe fn collect_club_list(this: usize, out: *mut usize, ativos: u32) -> u16 {
    let n = read_i16(this + 0x3e);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alloc((n as i32 * 4) as u32);
    if l == 0 {
        show_error_at(0x9e9e14, 0x2fb);
        *out = 0;
        return 0;
    }
    crate::comparacao::ramo("cqq_10 lista da intertoto");
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    while i < read_i16(this + 0x3e) {
        let e = read_u32_as_usize(this + 0xa7) + i as usize * 6;
        let s = *((e + 5) as *const u8);
        let c = read_u32_as_usize(e);
        if (ativos as u8 == 0 || s == 1 || s == 0) && c != 0 && k < n { // cqq10: os clubes da lista
            core::ptr::write_unaligned((l + k as usize * 4) as *mut u32, c as u32);
            k = k.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    *out = l;
    k as u16
}

/// rule: the Intertoto club list (for the caller): all entrants, or only those still in (status 0 or 1) when asked
/// Original: `IntertotoCup::collect_club_list` (0x633f70, intertoto_cup.cpp, thiscall, ret 8).
///
/// [pt] regra: a lista dos clubes da Intertoto (para quem chama): todos os inscritos, ou só os ainda vivos (situação 0 ou 1)
/// quando pedido
pub(crate) unsafe extern "thiscall" fn intertoto_cup_collect_club_list(this: usize, out: *mut usize, ativos: u32) -> u16 {
    collect_club_list(this, out, ativos)
}





/// The UEFA Cup competition object ([G_COMP_OBJECT_TABLE + 4 × KEY_CLUB_COMP_UEFA_CUP]), or 0.
///
/// [pt] O objeto da Copa da UEFA ([[0xadadfc] + 4 × [0x9cf6f0]]), ou 0.
pub(crate) unsafe fn uefa_cup_object() -> usize {
    read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP).wrapping_mul(4)))
}

/// The Intertoto winners (+5 = 1 in +0xa7) that fit the last three UEFA Cup slots, in the order 0x633d40 puts them →
/// (clubs, count).
///
/// [pt] Os vencedores do Intertoto (+5 = 1 no registro de 6 bytes de +0xa7) que cabem nas três últimas vagas da UEFA, na
/// ordem em que a 0x633d40 os põe: as vagas vão de +0x3e da UEFA − 27 até + 3 (palavras com sinal).
pub(crate) unsafe fn intertoto_winners_for_uefa(this: usize, uefa: usize) -> ([usize; 3], usize) {
    let mut v = [0usize; 3];
    let mut k = 0usize;
    let mut slot = read_i16(uefa + 0x3e).wrapping_sub(0x1b);
    let limit = slot.wrapping_add(3);
    let mut i = 0i16;
    if read_i16(this + 0x3e) > 0 {
        loop {
            if slot >= limit {
                break;
            }
            let e = read_u32_as_usize(this + 0xa7).wrapping_add((i as isize * 6) as usize);
            if read_u8(e + 5) == 1 {
                v[k] = read_u32_as_usize(e);
                k += 1;
                slot = slot.wrapping_add(1);
            }
            i = i.wrapping_add(1);
            if i >= read_i16(this + 0x3e) {
                break;
            }
        }
    }
    (v, k)
}

/// rule: the winners of the Intertoto finals (up to three, in August) enter the UEFA Cup, in the last three places of
/// its table (from its size minus 27 on) and in its entry book with status 4 [inference: entry through the Intertoto];
/// then the UEFA predicted positions. Once a season, when the Intertoto ends; without the UEFA Cup, the error box.
/// Every day, the cup update
/// Original: `IntertotoCup::update_for_date` (0x633d40, intertoto_cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x633d40 } (thiscall, ret 8, intertoto_cup.cpp; a virtual +0x14 do Intertoto): `IntertotoCup::update_for_date(hoje,
/// fase)`. Uma vez (+0xb2 = 0), quando a copa acabou (virtual +0x10 com 0, 1: push 1; push 0; mov ecx, this ->
/// thiscall, ret 8; o byte de baixo): sem a Copa da UEFA, a caixa da linha 0x29e; com ela, cada time do Intertoto com
/// +5 = 1 (os vencedores das finais) vai, em ordem, para as vagas de +0x3e − 27 em diante da tabela +0xa7 da UEFA (até
/// três: o clube, +4 e +5 zerados) e para o livro de inscritos dela (+0x14, 6 bytes: o clube e a situação 4 em +4;
/// +0x36 sobe); depois as posições previstas da UEFA (virtual +0x5c: mov ecx, uefa -> thiscall, ret 0), +0xb2 = 1 e de
/// novo as posições previstas (a original relê a UEFA). Sempre, no fim, a atualização da copa (0x51f890: push fase;
/// push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução sai.
pub(crate) unsafe extern "thiscall" fn intertoto_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    if read_u32_as_usize(this + 0xb2) == 0 {
        let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x10));
        if acabou(this, 0, 1) as u8 != 0 {
            let uefa = uefa_cup_object();
            if uefa == 0 {
                show_error_at(0x9e9e14, 0x29e);
            } else {
                let mut slot = read_i16(uefa + 0x3e).wrapping_sub(0x1b);
                let limit = slot.wrapping_add(3);
                let mut i = 0i16;
                if read_i16(this + 0x3e) > 0 {
                    loop {
                        if slot >= limit {
                            break;
                        }
                        let e = read_u32_as_usize(this + 0xa7).wrapping_add((i as isize * 6) as usize);
                        if read_u8(e + 5) == 1 {
                            crate::comparacao::ramo("cqi_10 vencedor na uefa");
                            let s = read_u32_as_usize(uefa + 0xa7).wrapping_add((slot as isize * 6) as usize);
                            core::ptr::write_unaligned(s as *mut u32, read_u32_as_usize(e) as u32);
                            *((s + 4) as *mut u8) = 0;
                            *((s + 5) as *mut u8) = 0;
                            let l = read_u32_as_usize(uefa + 0x14).wrapping_add((read_i16(uefa + 0x36) as isize * 6) as usize);
                            core::ptr::write_unaligned(l as *mut u32, read_u32_as_usize(s) as u32);
                            *((l + 4) as *mut u8) = 4; // cqi10: a situação do vencedor do Intertoto no livro da UEFA
                            core::ptr::write_unaligned((uefa + 0x36) as *mut i16, read_i16(uefa + 0x36).wrapping_add(1));
                            slot = slot.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                        if i >= read_i16(this + 0x3e) {
                            break;
                        }
                    }
                }
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(uefa) + 0x5c));
                prever(uefa);
                core::ptr::write_unaligned((this + 0xb2) as *mut u32, 1);
                let uefa = uefa_cup_object();
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(uefa) + 0x5c));
                prever(uefa);
                let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
                return base(this, today, stage);
            }
            core::ptr::write_unaligned((this + 0xb2) as *mut u32, 1);
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
    base(this, today, stage)
}

// simplification: a atualização da copa (0x51f890) joga e sorteia: não roda duas vezes. Só a original roda; o plano, feito
// antes: se +0xb2 passa de 0 a 1 com a Copa da UEFA, o livro dela (+0x14) ganha os vencedores (intertoto_winners_for_uefa), em
// ordem e com a situação 4, e +0x36 sobe tanto quanto; senão o livro não cresce.

/// rule: the winner of an Intertoto final gets the qualification record for the UEFA Cup (round 0xa, weight 60 to 100
/// by round) and moves to the UEFA Cup; a club going through gets the next round record (weight round × 99 / (rounds −
/// 1) + 1); a club knocked out (or losing the final) gets the knocked-out record (weight 20 to 80) and leaves European
/// competition
/// Original: `IntertotoCup::update_staff_records_for_status` (0x633c10, intertoto_cup.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x633c10 } (thiscall, ret 0x18, intertoto_cup.cpp; a virtual +0x44 do Intertoto): `IntertotoCup::
/// update_staff_records_for_status(clube, desfecho, _, _, jogo, _)`; devolve 0. Peso = 0x4bf850(baixo, alto, rodada do
/// jogo +0x34, rodadas +0x3c) (cdecl, 4 pushes; add esp, 0x10).
/// - 1 (venceu a final): a classificação (0x86bdd0) para a Copa da UEFA (a competição [0xae23d0] + 0x6b × [0x9cf6f0]),
///   fase 0, rodada 0xa, peso(0x3c, 0x64), e o clube passa à Copa da UEFA (+0x1db = [0x9cf6f0]).
/// - 0 (passou de rodada): se a rodada seguinte existe (+0x34 + 1 < +0x3c), a classificação no Intertoto (+4) com a
///   fase +0x32 do jogo, o +7 da rodada seguinte (+0xa3, 0x68 bytes) e o peso rodada × 99 / (rodadas − 1) + 1 (só o
///   byte de baixo sobe: inc al).
/// - 0xff ou 2 (caiu): a eliminação (0x86c000) no Intertoto, fase 0, rodada +0x30 do jogo, peso(0x14, 0x50), e o clube
///   sai da competição europeia (+0x1db = −1).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_update_staff_records_for_status(this: usize, clube: usize, outcome: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    match outcome as u8 {
        1 => {
            let weight = scale_by_round(0x3c, 0x64, read_i16(jogo + 0x34), read_i16(this + 0x3c) as u16 as u32);
            let uefa = read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP).wrapping_mul(0x6b));
            european_cup_add_qualified_record(clube, uefa, 0, 0xa, weight);
            crate::comparacao::ramo("cqi_10 vencedor do intertoto");
            core::ptr::write_unaligned((clube + club::EURO_FLAG) as *mut u32, read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP) as u32); // cqi10: o vencedor passa à UEFA
        }
        0 => {
            let r = read_i16(jogo + 0x34);
            let n = read_i16(this + 0x3c);
            let next = r.wrapping_add(1);
            if next >= n {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_INTERTOTO_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, outcome, a3, a4, jogo, a6); // simplification: a original cai na divisão
            }
            let q = (r as i32 * 99) / div;
            let weight = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let round = core::ptr::read_unaligned((read_u32_as_usize(this + 0xa3).wrapping_add((next as isize * 0x68) as usize) + 7) as *const u16) as u32;
            // simplification: a fase vai num registrador cujos 16 bits altos são os do movsx da rodada seguinte (zero); limpa
            european_cup_add_qualified_record(clube, read_u32_as_usize(this + 4), core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32, round, weight);
        }
        0xff | 2 => {
            let weight = scale_by_round(0x14, 0x50, read_i16(jogo + 0x34), read_i16(this + 0x3c) as u16 as u32);
            // simplification: a rodada +0x30 vai num registrador com lixo nos 16 bits altos; a 0x86c000 só usa a palavra
            european_cup_add_knocked_out_record(clube, read_u32_as_usize(this + 4), 0, core::ptr::read_unaligned((jogo + 0x30) as *const u16) as u32, weight);
            crate::comparacao::ramo("cqi_10 caiu no intertoto");
            leave_euro_competition(clube); // cqi10: quem cai no Intertoto sai da competição europeia
        }
        _ => {}
    }
    0
}

// simplification: os registros vão para o histórico das pessoas (reputação, moral): não rodam duas vezes. Só a original roda;
// o plano confere o +0x1db do clube: [0x9cf6f0] para o vencedor, −1 para quem cai, intacto nos outros.


// simplification: a base grava os registros (histórico, reputação, moral) e pode dar a vaga da UEFA ao campeão: não roda duas
// vezes. Só a original roda; o plano confere o +0x1db do clube (−1 nos desfechos 1, 2 e 0xff, intacto nos outros).

