use super::*;

/// A Intertoto: cinco rodadas em ida e volta de junho a agosto (a 1ª com 40 times; 12 novos na 2ª, 8 na 3ª).
pub(crate) const INTERTOTO_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x19, 5, 0, 4), (0x1a, 5, 0, 5, 1, 0), 0xa, (0, 4, 7), 7, (0x28, 0x14, 0x28, 0), (0, 2, 4), (0, 0, 0)), // cqp13: the 1st round of the Intertoto
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

/// A Copa Interamericana: ida e volta a partir de 26 de janeiro do ano seguinte (sorteio em 10 de outubro), 7 dias.
pub(crate) const INTERAMERICAN_CUP: [EngCcCupCupRound; 1] = [
    cql10r((0xa, 9, 0, 5), (0x1a, 0, 1, 2, 2, 0), 0, (0, 0, 3), 8, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)),
];

/// A Copa dos Campeões da CONCACAF: quartas (28/8), semifinal (1/9) e final (4/9) em jogo único, 8 times.
pub(crate) const CONCACAF_CHAMPIONS_CUP_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((1, 7, 0, 5), (0x1c, 8, 0, 3, 2, 0), 0xa, (0, 3, 0), 8, (8, 4, 8, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqp13: the CONCACAF quarterfinals
    cql10r((0x1d, 8, 0, 4), (1, 9, 0, 6, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((2, 9, 0, 0), (4, 9, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// As rodadas fixas da Copa da UEFA (a preliminar e os campos que dependem de +0xb2 vêm à parte, em
/// `uefa_cup_rounds`): a 1ª rodada de qualificação em agosto, a 2ª no fim de agosto, a 1ª rodada em setembro, a 2ª
/// em novembro, a 3ª em dezembro, as oitavas em fevereiro, as quartas e a semifinal em março e abril e a final em
/// maio, todas em ida e volta menos a final.
pub(crate) const UEFA_CUP_ROUNDS: [EngCcCupCupRound; 8] = [
    cql10r((0x1c, 6, 0, 4), (0xa, 7, 0, 3, 2, 0), 0xdc, (4, 0, 3), 7, (0x52, 0x29, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqq10: the 1st UEFA qualifying round
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
    u[0].teams.2 = (0x52 - m) as u16;
    u[0].teams.3 = b as u16;
    u[1].teams.3 = (m + 0x52) as u16;
    u[3].teams.3 = (m + 0x89) as u16;
    let mut n = 0;
    if b > 0 {
        crate::comparison::branch("cqq_10 uefa preliminary");
        t[0].teams = (b as u16, m as u16, b as u16, 0);
        n = 1;
    }
    for r in u {
        t[n] = r;
        n += 1;
    }
    (t, n)
}

/// `IntertotoCup::collect_entrants()` (`intertoto_cup_collect_entrants`, thiscall, ret 0, intertoto_cup.cpp): the 60 clubs whose European cup
/// (+0x1db) is the Intertoto, put in the team list (+0xa7) with their coefficient as class and sorted.
///
/// +0x3e = 60; a local vector of 60 pointers (`exe::crt_malloc` of 0xf0; out of memory, the box for line 0x161 and 0) with the
/// clubs of the table ([0xae23b4], up to [0xae235c]) whose European cup (+0x1db) is the Intertoto ([0x9cf6f8]), up to
/// 60; if short, the box for line 0x172 and 0 (the vector stays allocated, as in the original).
///
/// The list (`exe::crt_malloc` of 6 × 60 at +0xa7; out of memory, the box 0x17c and −1) takes each club with class = its
/// coefficient (+0x1df) and status 0, goes to qsort (`cup_compare_entries_by_level_desc`) and the vector is freed. Returns 1 with the 60 (as the
/// result pair).
///
/// [pt] { 0x6336c0 } (thiscall, ret 0, intertoto_cup.cpp): `IntertotoCup::collect_entrants()`: +0x3e = 60; um vetor local de
/// 60 ponteiros (0x944e46 de 0xf0; sem memória, a caixa da linha 0x161 e 0) com os clubes da tabela ([0xae23b4], até
/// [0xae235c]) cuja copa europeia (+0x1db) é a Intertoto ([0x9cf6f8]), até 60; faltando, a caixa da linha 0x172 e 0 (o
/// vetor fica alocado, como na original). A lista (0x944e46 de 6 × 60 em +0xa7; sem memória, a caixa 0x17c e −1) leva
/// cada clube com a classe = o seu coeficiente (+0x1df) e a situação 0, vai ao qsort (0x522190) e o vetor é liberado.
/// Devolve 1 com os 60.
pub(crate) unsafe fn collect_intertoto_entrants(this: usize) -> (i32, i32) {
    const F: usize = 0x9e9e14;
    core::ptr::write_unaligned((this + cup::TEAM_COUNT) as *mut i16, 0x3c);
    let v = exe::crt_malloc(0xf0);
    if v == 0 {
        show_error_at(F, 0x161);
        return (0, 0);
    }
    let mut k = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if k >= read_i16(this + cup::TEAM_COUNT) as i32 {
            break;
        }
        let c = read_usize(G_CLUB_TABLE).wrapping_add((i as isize * 0x245) as usize);
        if read_usize(c + 0x1db) == read_usize(KEY_CLUB_COMP_INTER_TOTO_CUP) {
            core::ptr::write_unaligned((v + k as usize * 4) as *mut u32, c as u32);
            k += 1;
        }
        i = i.wrapping_add(1);
    }
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    if k < n {
        show_error_at(F, 0x172);
        return (0, 0);
    }
    let l = exe::crt_malloc((n * 6) as u32);
    core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(F, 0x17c);
        return (-1, 0);
    }
    crate::comparison::branch("cqq_10 intertoto entrants");
    let mut j: i16 = 0;
    while j < read_i16(this + cup::TEAM_COUNT) {
        let c = read_usize(v + j as usize * 4);
        let e = read_usize(this + cup::TEAM_TABLE) + j as usize * 6;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 4) as *mut u8) = *((c + 0x1df) as *const u8); // cqq10: the class by coefficient
        *((e + 5) as *mut u8) = 0;
        j = j.wrapping_add(1);
    }
    exe::crt_qsort(read_usize(this + cup::TEAM_TABLE), read_i16(this + cup::TEAM_COUNT) as i32 as u32, 6, 0x522190);
    exe::crt_free(v);
    ((k == read_i16(this + cup::TEAM_COUNT) as i32) as i32, read_i16(this + cup::TEAM_COUNT) as i32)
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

/// `IntertotoCup::collect_club_list(&saida, ativos)` (`0x633f70`, thiscall, ret 8, `intertoto_cup.cpp`; a virtual):
/// builds a pointer list of the clubs in list `+0xa7`.
///
/// Allocates the list (`0x944e46` of 4 × `+0x3e`; out of memory, the box of line `0x2fb`, `saida` = 0 and returns 0)
/// with the clubs of list `+0xa7` (non-null, up to `+0x3e`); with `ativos`, only those whose status (`+5`) is 0 or 1.
/// Stores the list in `saida` and returns the count (word).
///
/// [pt] { 0x633f70 } (thiscall, ret 8, intertoto_cup.cpp; uma virtual): `IntertotoCup::collect_club_list(&saida, ativos)`:
/// uma lista de ponteiros (0x944e46 de 4 × +0x3e; sem memória, a caixa da linha 0x2fb, saida = 0 e 0) com os clubes da
/// lista +0xa7 (não nulos, até +0x3e): com `ativos`, só os de situação (+5) 0 ou 1. Grava a lista em saida e devolve a
/// conta (palavra).
pub(crate) unsafe fn collect_club_list(this: usize, out: *mut usize, ativos: u32) -> u16 {
    let n = read_i16(this + cup::TEAM_COUNT);
    let l = exe::crt_malloc((n as i32 * 4) as u32);
    if l == 0 {
        show_error_at(0x9e9e14, 0x2fb);
        *out = 0;
        return 0;
    }
    crate::comparison::branch("cqq_10 intertoto list");
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    while i < read_i16(this + cup::TEAM_COUNT) {
        let e = read_usize(this + cup::TEAM_TABLE) + i as usize * 6;
        let s = *((e + 5) as *const u8);
        let c = read_usize(e);
        if (ativos as u8 == 0 || s == 1 || s == 0) && c != 0 && k < n { // cqq10: the clubs in the list
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
    read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_UEFA_CUP).wrapping_mul(4)))
}

/// The Intertoto winners (+5 = 1 in +0xa7) that fit the last three UEFA Cup slots, in the order `intertoto_cup_update_for_date` puts them →
/// (clubs, count).
///
/// [pt] Os vencedores do Intertoto (+5 = 1 no registro de 6 bytes de +0xa7) que cabem nas três últimas vagas da UEFA, na
/// ordem em que a 0x633d40 os põe: as vagas vão de +0x3e da UEFA − 27 até + 3 (palavras com sinal).
pub(crate) unsafe fn intertoto_winners_for_uefa(this: usize, uefa: usize) -> ([usize; 3], usize) {
    let mut v = [0usize; 3];
    let mut k = 0usize;
    let mut slot = read_i16(uefa + cup::TEAM_COUNT).wrapping_sub(0x1b);
    let limit = slot.wrapping_add(3);
    let mut i = 0i16;
    if read_i16(this + cup::TEAM_COUNT) > 0 {
        loop {
            if slot >= limit {
                break;
            }
            let e = read_usize(this + cup::TEAM_TABLE).wrapping_add((i as isize * 6) as usize);
            if read_u8(e + 5) == 1 {
                v[k] = read_usize(e);
                k += 1;
                slot = slot.wrapping_add(1);
            }
            i = i.wrapping_add(1);
            if i >= read_i16(this + cup::TEAM_COUNT) {
                break;
            }
        }
    }
    (v, k)
}

/// Daily Intertoto update: once a season, when the Intertoto ends, its final winners (up to three, in August) enter the
/// UEFA Cup; every day, the cup update.
///
/// `IntertotoCup::update_for_date(today, stage)`, virtual +0x14 of the Intertoto. Once (+0xb2 = 0), when the cup has
/// ended (virtual +0x10 with 0, 1: push 1; push 0; mov ecx, this -> thiscall, ret 8; the low byte): without the UEFA
/// Cup, the box for line 0x29e; with it, each Intertoto team with +5 = 1 (the winners of the finals) goes, in order,
/// into the places from +0x3e − 27 on of the UEFA Cup's table +0xa7 (the last three places; up to three: the club, +4
/// and +5 zeroed) and into its entry book (+0x14, 6 bytes: the club and status 4 at +4 [inference: entry through the
/// Intertoto]; +0x36 goes up); then the UEFA predicted positions (virtual +0x5c: mov ecx, uefa -> thiscall, ret 0),
/// +0xb2 = 1 and the predicted positions once more (the original re-reads the UEFA Cup). Always, at the end, the cup
/// update (`cup_update_for_date`: push stage; push today; mov ecx, this -> thiscall, ret 8), whose return value is returned.
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
    if read_usize(this + intertoto_cup::WINNERS_SENT_TO_UEFA) == 0 {
        let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x10));
        if acabou(this, 0, 1) as u8 != 0 {
            let uefa = uefa_cup_object();
            if uefa == 0 {
                show_error_at(0x9e9e14, 0x29e);
            } else {
                let mut slot = read_i16(uefa + cup::TEAM_COUNT).wrapping_sub(0x1b);
                let limit = slot.wrapping_add(3);
                let mut i = 0i16;
                if read_i16(this + cup::TEAM_COUNT) > 0 {
                    loop {
                        if slot >= limit {
                            break;
                        }
                        let e = read_usize(this + cup::TEAM_TABLE).wrapping_add((i as isize * 6) as usize);
                        if read_u8(e + 5) == 1 {
                            crate::comparison::branch("cqi_10 winner in uefa");
                            let s = read_usize(uefa + cup::TEAM_TABLE).wrapping_add((slot as isize * 6) as usize);
                            core::ptr::write_unaligned(s as *mut u32, read_usize(e) as u32);
                            *((s + 4) as *mut u8) = 0;
                            *((s + 5) as *mut u8) = 0;
                            let l = read_usize(uefa + cup::ENTRANT_BOOK).wrapping_add((read_i16(uefa + cup::ENTRANT_COUNT) as isize * 6) as usize);
                            core::ptr::write_unaligned(l as *mut u32, read_usize(s) as u32);
                            *((l + 4) as *mut u8) = 4; // cqi10: the situation of the Intertoto winner in the UEFA book
                            core::ptr::write_unaligned((uefa + cup::ENTRANT_COUNT) as *mut i16, read_i16(uefa + cup::ENTRANT_COUNT).wrapping_add(1));
                            slot = slot.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                        if i >= read_i16(this + cup::TEAM_COUNT) {
                            break;
                        }
                    }
                }
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(uefa) + 0x5c));
                prever(uefa);
                core::ptr::write_unaligned((this + intertoto_cup::WINNERS_SENT_TO_UEFA) as *mut u32, 1);
                let uefa = uefa_cup_object();
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(uefa) + 0x5c));
                prever(uefa);
                let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
                return base(this, today, stage);
            }
            core::ptr::write_unaligned((this + intertoto_cup::WINNERS_SENT_TO_UEFA) as *mut u32, 1);
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
    base(this, today, stage)
}

// simplification: the cup update (0x51f890) plays and rolls: it does not run twice. Only the original runs; the plan, done
// before: if +0xb2 goes from 0 to 1 with the UEFA Cup, its book (+0x14) gets the winners (intertoto_winners_for_uefa), in
// order and with status 4, and +0x36 goes up as much; otherwise the book does not grow.

/// Records a club's Intertoto outcome in the staff records: the final winner qualifies for the UEFA Cup, a club going
/// through gets the next round record, a club knocked out leaves European competition.
///
/// `IntertotoCup::update_staff_records_for_status(club, outcome, _, _, match, _)`, virtual +0x44 of the Intertoto;
/// returns 0. Weight = `comp_util_scale_by_round`(low, high, match round +0x34, rounds +0x3c) (cdecl, 4 pushes; add esp, 0x10).
///
/// - 1 (won the final): the qualification record (`add_qualified_for_round_record`) for the UEFA Cup (the competition [0xae23d0] + 0x6b ×
/// [0x9cf6f0]), stage 0, round 0xa, weight(0x3c, 0x64), and the club moves to the UEFA Cup (+0x1db = [0x9cf6f0]).
///
/// - 0 (went through a round): if the next round exists (+0x34 + 1 < +0x3c), the qualification record in the Intertoto
/// (+4) with the match's stage +0x32, the +7 of the next round (+0xa3, 0x68 bytes) and the weight round × 99 / (rounds
/// − 1) + 1 (only the low byte is incremented: inc al).
///
/// - 0xff or 2 (knocked out, or lost the final): the elimination record (`add_knocked_out_record`) in the Intertoto, stage 0, round
/// +0x30 of the match, weight(0x14, 0x50), and the club leaves European competition (+0x1db = −1).
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
pub(crate) unsafe extern "thiscall" fn intertoto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, a3: u32, a4: u32, game: usize, a6: u32) -> u32 {
    match outcome as u8 {
        1 => {
            let weight = scale_by_round(0x3c, 0x64, read_i16(game + 0x34), read_i16(this + cup::ROUND_COUNT) as u16 as u32);
            let uefa = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_UEFA_CUP).wrapping_mul(0x6b));
            european_cup_add_qualified_record(club, uefa, 0, 0xa, weight);
            crate::comparison::branch("cqi_10 intertoto winner");
            core::ptr::write_unaligned((club + club::CONTINENTAL_COMP_ID) as *mut u32, read_usize(KEY_CLUB_COMP_UEFA_CUP) as u32); // cqi10: o vencedor passa à UEFA
        }
        0 => {
            let r = read_i16(game + 0x34);
            let n = read_i16(this + cup::ROUND_COUNT);
            let next = r.wrapping_add(1);
            if next >= n {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_INTERTOTO_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, a3, a4, game, a6); // simplification: the original falls into the division
            }
            let q = (r as i32 * 99) / div;
            let weight = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let round = core::ptr::read_unaligned((read_usize(this + cup::ROUND_TABLE).wrapping_add((next as isize * 0x68) as usize) + 7) as *const u16) as u32;
            // simplification: the stage goes in a register whose high 16 bits are those of the next round’s movsx (zero); clears
            european_cup_add_qualified_record(club, read_usize(this + 4), core::ptr::read_unaligned((game + 0x32) as *const u16) as u32, round, weight);
        }
        0xff | 2 => {
            let weight = scale_by_round(0x14, 0x50, read_i16(game + 0x34), read_i16(this + cup::ROUND_COUNT) as u16 as u32);
            // simplification: round +0x30 goes in a register with garbage in the high 16 bits; 0x86c000 only uses the word
            european_cup_add_knocked_out_record(club, read_usize(this + 4), 0, core::ptr::read_unaligned((game + 0x30) as *const u16) as u32, weight);
            crate::comparison::branch("cqi_10 dropped into intertoto");
            leave_euro_competition(club); // cqi10: whoever is knocked out in the Intertoto leaves the European competition
        }
        _ => {}
    }
    0
}

// simplification: the records go to the people’s history (reputation, morale): they do not run twice. Only the original runs;
// the plan checks the club’s +0x1db: [0x9cf6f0] for the winner, −1 for those knocked out, untouched for the others.

// simplification: the base writes the records (history, reputation, morale) and may give the UEFA place to the champion: does not run
// times. Only the original runs; the plan checks the club’s +0x1db (−1 on outcomes 1, 2 and 0xff, untouched on the others).

