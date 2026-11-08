use super::*;




/// True if the club's division (+0x57) has the competition id in global g.
///
/// [pt] O clube `c` está na competição de id [`g`] (+0x57 a divisão, ponteiro para o registro; +0 o id)?
pub(crate) unsafe fn eng_auto_cup_is_in_comp(c: usize, g: usize) -> bool {
    let l = read_u32_as_usize(c + 0x57);
    l != 0 && read_u32_as_usize(l) == read_u32_as_usize(g)
}

/// Puts club c in the next slot of the list at +0xb2 and increments the count.
///
/// [pt] Põe o clube `c` na vaga `k` da lista (+0xb2, relida a cada vez) e conta.
pub(crate) unsafe fn push_section_club(this: usize, k: &mut i32, c: usize) {
    *k += 1;
    write_u32_from_usize(read_u32_as_usize(this + 0xb2) + (*k as usize - 1) * 4, c);
}

/// EngAutoCup::select_section_clubs (0x56b730): 56 League Trophy clubs from the lower English divisions and Conference,
/// then sorts by stadium latitude and each half by reputation; returns 1 if full.
///
/// [pt] { 0x56b730 } (thiscall, ret 0, eng_auto_cup.cpp; da virada, n = 1 até 2002-06-01): `EngAutoCup::select_section_clubs()`:
/// os 56 clubes do Troféu da Liga (+0xb6 = 0x38) numa lista (0x944e46 de 0xe0 em +0xb2; sem memória, a caixa da linha
/// 0x24c e 0), em quatro passos, cada um até a lista encher (conta == +0xb6):
/// 1) pela ordem da tabela dos clubes ([0xae23b4], [0xae235c] clubes), os das divisões [0x9cf5c4] e [0x9cf5c8] (a segunda
///    e a terceira divisões inglesas [inferência]);
/// 2) o primeiro clube da divisão [0x9cf69c] (a Conference [inferência]) que veio da [0x9cf5c8] (+0x5b, a divisão da
///    temporada anterior [inferência]): o rebaixado; ele é lembrado (`achado`) para não entrar de novo;
/// 3) se a nação da copa ([+4] +0x5d) tem o bit 4 em +0x11c, os clubes da [0x9cf69c] que já estavam nela (+0x5b), pela
///    categoria +0x5f (com sinal): primeiro os de 6 ou menos, depois 7, 8... até encher; senão, até 30 clubes da
///    [0x9cf69c] (fora o achado) e, faltando, os clubes ingleses (+0x53 = [0x9cf2e4]) fora da divisão genérica [0x9cf764]
///    (entre os [0xae235c] − 2 × [0xae2350] primeiros), até 30, ordenados pela reputação com os 3 × faltam primeiros
///    embaralhados (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push faltam; push quantos; push lista; add esp, 0xc
///    -> cdecl; sorteia), e deles os primeiros que faltam.
/// Cheia, a lista é ordenada pela latitude da cidade do estádio (qsort 0x945333 com 0x53fd40) e cada metade (a do Norte e
/// a do Sul [inferência]) pela reputação, da menor para a maior (qsort com 0x4b0810); devolve 1. Sem encher, 0.
pub(crate) unsafe fn eng_auto_cup_select_section_clubs(this: usize) -> u32 {
    write_u32_from_usize(this + 0xb6, 0x38);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alloc(0xe0);
    write_u32_from_usize(this + 0xb2, l);
    if l == 0 {
        show_error_at(0x9c4180, 0x24c);
        return 0;
    }
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let n = read_u32_as_usize(G_CLUB_COUNT) as i32;
    let cheia = |k: i32| k == read_u32_as_usize(this + 0xb6) as i32;
    let mut k = 0i32;
    // 1) a segunda e a terceira divisões
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let c = base + i as u16 as usize * club::SIZE;
            if eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) || eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) {
                push_section_club(this, &mut k, c);
            }
            if cheia(k) {
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    // 2) o rebaixado da terceira divisão para a Conference
    // simplification: sem rebaixado a original compara depois com uma dword da pilha sem valor; a cópia usa 0 (nenhum clube)
    let mut found = 0usize;
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let c = base + i as u16 as usize * club::SIZE;
            let before = read_u32_as_usize(c + 0x5b);
            if eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) && before != 0 && read_u32_as_usize(before) == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) {
                crate::comparacao::ramo("cqs_5 rebaixado para a conference");
                push_section_club(this, &mut k, c); // cqs5: o rebaixado entra
                found = c;
                break;
            }
            if cheia(k) {
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    let nacao = read_u32_as_usize(read_u32_as_usize(this + 4) + 0x5d);
    if *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) & 4 != 0 {
        // 3a) a Conference pela categoria
        let mut cat: i8 = 6;
        if k < read_u32_as_usize(this + 0xb6) as i32 {
            loop {
                let mut i: i16 = 0;
                if n > 0 {
                    loop {
                        if k >= read_u32_as_usize(this + 0xb6) as i32 {
                            break;
                        }
                        let c = base + i as u16 as usize * club::SIZE;
                        let before = read_u32_as_usize(c + 0x5b);
                        if before != 0 && read_u32_as_usize(before) == read_u32_as_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) && eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
                            let v = *((c + 0x5f) as *const i8);
                            if ((cat == 6 && v <= cat) || v == cat) && c != 0 && c != found {
                                push_section_club(this, &mut k, c);
                            }
                        }
                        i = i.wrapping_add(1);
                        if i as i32 >= n {
                            break;
                        }
                    }
                }
                cat = cat.wrapping_add(1);
                if k >= read_u32_as_usize(this + 0xb6) as i32 {
                    break;
                }
            }
        }
    } else {
        // 3b) até 30 da Conference e, faltando, ingleses fora da divisão genérica; embaralhados
        let mut local = [0usize; 0x1e];
        let mut m: i8 = 0;
        let mut i: i16 = 0;
        if n > 0 {
            loop {
                let c = base + i as u16 as usize * club::SIZE;
                if eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) && found != c {
                    local[m as u8 as usize] = c;
                    m = m.wrapping_add(1);
                    if m == 0x1e {
                        break;
                    }
                }
                i = i.wrapping_add(1);
                if i as i32 >= n {
                    break;
                }
            }
        }
        if (m as i32) < read_u32_as_usize(this + 0xb6) as i32 - k {
            let lim = n.wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
            let mut j: i16 = 0;
            if lim > 0 {
                loop {
                    let c = base + j as u16 as usize * club::SIZE;
                    let nac = read_u32_as_usize(c + 0x53);
                    if nac != 0 && read_u32_as_usize(nac) == read_u32_as_usize(KEY_NATION_ENGLAND) && !eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_A_LOWER_DIVISION) {
                        local[m as u8 as usize] = c;
                        m = m.wrapping_add(1);
                        if m == 0x1e {
                            break;
                        }
                    }
                    j = j.wrapping_add(1);
                    if j as i32 >= lim {
                        break;
                    }
                }
            }
        }
        let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
        // simplification: a original empurra as duas palavras em dwords com lixo nos 16 bits altos; a 0x4afbd0 lê as palavras
        embaralhar(local.as_mut_ptr() as usize, m as i16 as u16 as u32, (read_i16(this + 0xb6).wrapping_sub(k as i16)) as u16 as u32);
        let mut a: i16 = 0;
        while k < read_u32_as_usize(this + 0xb6) as i32 {
            push_section_club(this, &mut k, local[a as u16 as usize]);
            a = a.wrapping_add(1);
        }
    }
    if k != read_u32_as_usize(this + 0xb6) as i32 {
        return 0;
    }
    crate::comparacao::ramo("cqs_5 chaves do trofeu da liga");
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    sort(read_u32_as_usize(this + 0xb2), read_u32_as_usize(this + 0xb6) as u32, 4, 0x53fd40); // cqs5: Norte e Sul pela latitude
    let half = (read_u32_as_usize(this + 0xb6) as i32) / 2;
    sort(read_u32_as_usize(this + 0xb2), half as u32, 4, 0x4b0810);
    let half = (read_u32_as_usize(this + 0xb6) as i32) / 2;
    sort(read_u32_as_usize(this + 0xb2) + half as usize * 4, half as u32, 4, 0x4b0810);
    1
}

/// rule: the League Trophy (LDV Vans) has 56 clubs: all of the second and third divisions, the club relegated from the
/// third to the Conference and, to complete it, Conference clubs (with the nation rule on, those already in it by
/// category, 6 or less first; otherwise up to 30 Conference clubs or other English clubs outside the generic division,
/// sorted by reputation with the best shuffled); the list is split into North and South by the latitude of the stadium
/// city and each half goes from lowest reputation to highest
/// Original: `EngAutoCup::select_section_clubs` (0x56b730, eng_auto_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: o Troféu da Liga (LDV Vans) tem 56 clubes: todos os da segunda e da terceira divisões, o rebaixado da
/// terceira para a Conference e, para completar, clubes da Conference (com a regra da nação ligada, os que já estavam
/// nela pela categoria, 6 ou menos primeiro; senão até 30 da Conference ou outros ingleses fora da divisão genérica,
/// ordenados pela reputação e com os melhores embaralhados); a lista é dividida em Norte e Sul pela latitude da cidade
/// do estádio e cada metade vai da menor reputação para a maior
pub(crate) unsafe extern "thiscall" fn select_section_clubs(this: usize) -> u32 {
    eng_auto_cup_select_section_clubs(this)
}


/// rule: the Trophy section stage is a knockout (Cup_Stage) with the 56 in list order (North, then South), with the
/// rounds of stage 0 of the cup table, no seeding
/// Original: `EngAutoCup::create_section_stage` (0x56bab0, eng_auto_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x56bab0 } (thiscall, ret 0, eng_auto_cup.cpp; quadro SEH só para o new, sem destrutor): `EngAutoCup::
/// create_section_stage()`: a fase das chaves (Norte e Sul) com os +0x3e clubes da lista +0xb2, na ordem: um vetor
/// (0x944e46 de 4 × +0x3e; sem memória, a caixa da linha 0x2e9) com eles; as rodadas pelo método +0x3c (push 0; push
/// &extra; push &rodadas; push 0; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2); Cup_Stage::Cup_Stage 0x522e00,
/// thiscall, ret 0x3c: a copa, +0x3e (word), o vetor, as rodadas (word), +4, a tabela, o ano +0x40 (word), 0, 1, o extra
/// (word), +0x51 (byte), 0, 0, 0, sem sementes; os words e bytes vão com lixo em cima, que o construtor não lê) em
/// [+0xc][0]; o vetor e a tabela liberados (0x9452ca). Sem fase, a caixa da linha 0x300; com ela, +0x30 = 0.
pub(crate) unsafe extern "thiscall" fn create_section_stage(this: usize) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let teams = alloc(((read_i16(this + 0x3e) as i32) << 2) as u32);
    if teams == 0 {
        return show_error_at(0x9c4180, 0x2e9);
    }
    let mut i = 0i32;
    while i < read_i16(this + 0x3e) as i32 {
        write_u32_from_usize(teams + i as usize * 4, read_u32_as_usize(read_u32_as_usize(this + 0xb2) + i as usize * 4)); // cqs5: a ordem das chaves
        i += 1;
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this, 0, &mut rodadas, &mut extra, 0);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = new(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, read_i16(this + 0x3e) as u16 as u32, teams, rodadas as u16 as u32, read_u32_as_usize(this + 4), table, read_i16(this + 0x40) as u16 as u32, 0, 1,
            extra as u16 as u32, *((this + 0x51) as *const u8) as u32, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc), f);
    soltar(teams);
    soltar(table);
    if read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        return show_error_at(0x9c4180, 0x300);
    }
    crate::comparacao::ramo("cqs_5 fase das chaves do trofeu");
    write_u32_from_usize(this + 0x30, 0);
}


/// rule: the Trophy stage moves on by one; when the sections end (stage 1), it builds the final between the North and
/// South winners; at the last stage, the error box
/// Original: `EngAutoCup::advance_stage` (0x56bc50, eng_auto_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x56bc50 } (thiscall, ret 0, eng_auto_cup.cpp; a virtual +0x28): `EngAutoCup::advance_stage()`: na última fase
/// (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x314 e nada mais; senão +0x30 sobe um e, ao chegar a 1 (o fim das
/// chaves), monta a final (EngAutoCup::create_final_stage 0x56bce0, call direto com o ecx da copa -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_advance_stage(this: usize) {
    let stage = read_u32_as_usize(this + 0x30) as i32;
    if stage >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x9c4180, 0x314);
    }
    let stage = stage + 1;
    write_u32_from_usize(this + 0x30, stage as usize);
    if stage == 1 {
        crate::comparacao::ramo("cqs_5 final do trofeu");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_ENGAUTOCUP_CREATE_FINAL_STAGE);
        f(this);
    }
}





/// First club among the first n 6-byte entrants whose status (+5) is s, or None.
///
/// [pt] O primeiro clube da lista de inscritos `l` (entradas de 6 bytes: +0 o clube, +5 a situação) com a situação `s`, entre
/// as `n` primeiras (n = +0x3e da copa, word com sinal); sem ele, None.
pub(crate) unsafe fn first_entrant_with_status(l: usize, n: i32, s: u8) -> Option<usize> {
    (0..n.max(0) as usize).find(|&i| *((l + i * 6 + 5) as *const u8) == s).map(|i| read_u32_as_usize(l + i * 6))
}

/// rule: when the sections end, the Trophy final is a single tie (2-team Cup_Stage, stage 1) between the one still
/// alive in the cup list (status 1) and the one still alive in the section stage; the round is drawn at once (home
/// side)
/// Original: `EngAutoCup::create_final_stage` (0x56bce0, eng_auto_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x56bce0 } (thiscall, ret 0, eng_auto_cup.cpp; da fase seguinte, quadro SEH só para o new, sem destrutor):
/// `EngAutoCup::create_final_stage()`: a final do Troféu: um vetor de 2 (0x944e46 de 8; sem memória, a caixa da linha
/// 0x337) com o primeiro clube de situação 1 (o que segue vivo [inferência]) da lista da copa (+0xa7) e o primeiro de
/// situação 1 da lista da fase das chaves ([+0xc][0] +0xa7), ambos entre os +0x3e primeiros (da copa); as rodadas pelo
/// método +0x3c (push 0; push &extra; push &rodadas; push 1; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2);
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 2, o vetor, as rodadas (word), +4, a tabela, o ano +0x40
/// (word), 1, 1, o extra (word), 0x14, 0, 0, 0, sem sementes) em [+0xc][1]; a tabela das rodadas liberada (0x9452ca). Sem
/// fase, a caixa da linha 0x35a (o vetor fica, como na original). Com ela, o sorteio da rodada (Cup::draw_round 0x51c410:
/// push 0; mov ecx, fase -> thiscall, ret 4) e o vetor liberado.
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_create_final_stage(this: usize) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let l = alloc(8);
    if l == 0 {
        return show_error_at(0x9c4180, 0x337);
    }
    if let Some(c) = first_entrant_with_status(read_u32_as_usize(this + 0xa7), read_i16(this + 0x3e) as i32, 1) { // cqs6: o finalista da copa
        write_u32_from_usize(l, c);
    }
    if let Some(c) = first_entrant_with_status(read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(this + 0xc)) + 0xa7), read_i16(this + 0x3e) as i32, 1) { // cqs6: o finalista das chaves
        write_u32_from_usize(l + 4, c);
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this, 1, &mut rodadas, &mut extra, 0);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = new(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 2, l, rodadas as u16 as u32, read_u32_as_usize(this + 4), table, read_i16(this + 0x40) as u16 as u32, 1, 1, // cqs6: o ano da final
             extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + 4, f);
    soltar(table);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4);
    if f == 0 {
        return show_error_at(0x9c4180, 0x35a);
    }
    crate::comparacao::ramo("cqs_6 final do trofeu");
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_DRAW_ROUND);
    sortear(f, 0); // cqs6: o sorteio da final
    soltar(l);
}


/// rule: the Trophy final ranking in the statistics: the highest-reputation club of each half takes 1st and 2nd (the
/// one with the higher reputation is 1st); the others, round by round from the last to the first, share one position
/// per round (3, then 3 + 2 × those already placed...)
/// Original: `EngAutoCup::rank_teams` (0x56c010, eng_auto_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x56c010 } (thiscall, ret 0, eng_auto_cup.cpp; a virtual da classificação final, n = 1 até 2002-06-01): `EngAutoCup::
/// rank_teams()`: a posição final de cada clube nas estatísticas da copa (+8, Comp_Stats; sem elas, a caixa da linha 0x3bb).
/// Um vetor de 2 × +0x3e ponteiros (0x944e46; sem memória, a caixa da linha 0x3c3): na primeira metade os clubes da lista
/// da copa (+0xa7, entradas de 6 bytes), na segunda os da fase das chaves ([+0xc][0] +0xa7); cada metade ordenada pela
/// reputação (+0x80, word), da maior para a menor (qsort 0x945333 com 0x4b07d0). Os dois primeiros (o de maior reputação
/// de cada metade): o de reputação maior (com sinal, estritamente) fica em 1º e o outro em 2º (Comp_Stats::
/// set_club_final_position 0x4a2540: push posição; push clube; mov ecx, +8 -> thiscall, ret 8). Depois, para cada metade,
/// da última rodada (+0x3c − 1) para a primeira, tantos clubes (os seguintes da metade, a partir do 2º) quantos a rodada
/// tem em +0x1a da tabela das rodadas (+0xa3, 0x68 bytes por rodada), com as posições 3, 5, 7... (a original empurra o
/// dword do byte da posição com lixo da pilha em cima; a 0x4a2540 só lê o byte). O vetor é liberado.
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_rank_teams(this: usize) {
    if read_u32_as_usize(this + 8) == 0 {
        return show_error_at(0x9c4180, 0x3bb);
    }
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let v = alloc(((read_i16(this + 0x3e) as i32) << 3) as u32);
    if v == 0 {
        return show_error_at(0x9c4180, 0x3c3);
    }
    let mut d: i16 = 0;
    while d < read_i16(this + 0x3e) {
        let e = d as i32 as usize * 6;
        write_u32_from_usize(v + d as i32 as usize * 4, read_u32_as_usize(read_u32_as_usize(this + 0xa7) + e));
        let n = read_i16(this + 0x3e) as i32;
        write_u32_from_usize(v + (n + d as i32) as usize * 4, read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(this + 0xc)) + 0xa7) + e));
        d = d.wrapping_add(1);
    }
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let n = read_i16(this + 0x3e) as i32;
    sort(v, n as u32, 4, 0x4b07d0);
    let n = read_i16(this + 0x3e) as i32;
    sort(v + n as usize * 4, n as u32, 4, 0x4b07d0);
    let position: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    let a = read_u32_as_usize(v);
    let b = read_u32_as_usize(v + read_i16(this + 0x3e) as i32 as usize * 4);
    crate::comparacao::ramo("cqs_6 classificacao do trofeu");
    if read_i16(a + 0x80) > read_i16(b + 0x80) {
        position(read_u32_as_usize(this + 8), a, 1);
        position(read_u32_as_usize(this + 8), read_u32_as_usize(v + read_i16(this + 0x3e) as i32 as usize * 4), 2);
    } else {
        position(read_u32_as_usize(this + 8), a, 2);
        position(read_u32_as_usize(this + 8), read_u32_as_usize(v + read_i16(this + 0x3e) as i32 as usize * 4), 1);
    }
    for lado in 0..2u8 {
        let mut k: i16 = if lado == 0 { 1 } else { read_i16(this + 0x3e).wrapping_add(1) };
        let mut pos: u8 = 3;
        let last = read_i16(this + 0x3c).wrapping_sub(1);
        if last < 0 {
            continue;
        }
        let mut r = last as i32;
        while r >= 0 {
            let p = pos;
            let mut j: i16 = 0;
            while j < read_i16(read_u32_as_usize(this + 0xa3) + r as usize * 0x68 + 0x1a) {
                let c = read_u32_as_usize(v + k as i32 as usize * 4);
                k = k.wrapping_add(1);
                position(read_u32_as_usize(this + 8), c, p as u32);
                pos = pos.wrapping_add(2);
                j = j.wrapping_add(1);
            }
            r -= 1;
        }
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    soltar(v);
}

// simplification: grava nas estatísticas da copa (Comp_Stats, que acha ou acrescenta a entrada do clube: aloca); só a original

/// rule: the League Trophy (Auto Windscreens) has two 28-club sections; the second takes the last 28 of the 56 chosen
/// clubs (the array at +0xb2, already sorted by latitude and reputation: the southern section [inference]), in order,
/// class and status 0
/// Original: `EngAutoCup::collect_entrants` (0x56b3c0, eng_auto_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: o Troféu da Liga (Auto Windscreens) tem duas chaves de 28 clubes; a segunda leva os 28 últimos dos 56
/// escolhidos (o vetor em +0xb2, já ordenado pela latitude e pela reputação: a chave do sul [inferência]), na ordem,
/// classe e situação 0
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_collect_entrants(this: usize) -> i32 {
    collect_auto_cup_entrants(this).0
}


/// rule: the LDV Vans Trophy has two sections (stages −1 and 0, North and South [inference], ids 0x41d and 0x41e) of
/// five single-match rounds: 1st on 10/12 (24 teams), 2nd on 14/1 (16 teams, 4 new), quarter-finals on 4/2, semi-final
/// on 18/2 and the section final over two legs from 4/3 (7 days); the final (stage 1) drawn on 12/3 and played on 20/4
/// of the following year with venue 4 (neutral ground [inference]); category 4 in the sections and 6 in the finals; no
/// per-round prizes
/// Original: `EngAutoCup::create_round_table` (0x56ade0, eng_auto_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x56ade0 } (thiscall, ret 0x10, eng_auto_cup.cpp; a virtual +0x3c do LDV Vans Trophy): `EngAutoCup::
/// create_round_table(fase, &rodadas, &extra, &é_liga)`: a fase de `ldv_trophy_stage`.
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(this, ldv_trophy_stage(stage), rounds, extra, league, 0x9c4180)
}

