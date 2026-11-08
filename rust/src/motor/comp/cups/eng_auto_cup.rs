use super::*;

/// True if the club's division (+0x57) has the competition id in global g.
///
/// [pt] O clube `c` está na competição de id [`g`] (+0x57 a divisão, ponteiro para o registro; +0 o id)?
pub(crate) unsafe fn eng_auto_cup_is_in_comp(c: usize, g: usize) -> bool {
    let l = read_usize(c + 0x57);
    l != 0 && read_usize(l) == read_usize(g)
}

/// Puts club c in the next slot of the list at +0xb2 and increments the count.
///
/// [pt] Põe o clube `c` na vaga `k` da lista (+0xb2, relida a cada vez) e conta.
pub(crate) unsafe fn push_section_club(this: usize, k: &mut i32, c: usize) {
    *k += 1;
    write_usize(read_usize(this + eng_auto_cup::SELECTED_CLUBS) + (*k as usize - 1) * 4, c);
}

/// `EngAutoCup::select_section_clubs()` (0x56b730, thiscall, ret 0, eng_auto_cup.cpp; from the season rollover, n = 1
/// up to 2002-06-01): puts the 56 League Trophy clubs (+0xb6 = 0x38) in a list (0x944e46 of 0xe0 at +0xb2; out of
/// memory, the box for line 0x24c and 0), in four steps, each one until the list is full (count == +0xb6).
///
/// 1) In club table order ([0xae23b4], [0xae235c] clubs), those of divisions [0x9cf5c4] and [0x9cf5c8] (the English
/// second and third divisions [inference]).
///
/// 2) The first club of division [0x9cf69c] (the Conference [inference]) that came from [0x9cf5c8] (+0x5b, the previous
/// season's division [inference]): the relegated club; it is remembered (`achado`) so it does not enter again.
///
/// 3) If the cup's nation ([+4] +0x5d) has bit 4 in +0x11c, the clubs of [0x9cf69c] that were already in it (+0x5b), by
/// category +0x5f (signed): first those of 6 or less, then 7, 8... until full; otherwise, up to 30 clubs of [0x9cf69c]
/// (except the remembered one) and, if short, the English clubs (+0x53 = [0x9cf2e4]) outside the generic division
/// [0x9cf764] (among the first [0xae235c] − 2 × [0xae2350]), up to 30, sorted by reputation with the first 3 × missing
/// shuffled (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push missing; push how many; push list; add esp, 0xc ->
/// cdecl; uses random draws), and of those the first ones still missing.
///
/// Once full, the list is sorted by the latitude of the stadium's city (qsort 0x945333 with 0x53fd40) and each half
/// (the North and the South [inference]) by reputation, lowest to highest (qsort with 0x4b0810); returns 1. If it does
/// not fill, 0.
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
    write_usize(this + eng_auto_cup::SELECTED_COUNT, 0x38);
    let l = exe::crt_malloc(0xe0);
    write_usize(this + eng_auto_cup::SELECTED_CLUBS, l);
    if l == 0 {
        show_error_at(0x9c4180, 0x24c);
        return 0;
    }
    let base = read_usize(G_CLUB_TABLE);
    let n = read_usize(G_CLUB_COUNT) as i32;
    let cheia = |k: i32| k == read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32;
    let mut k = 0i32;
    // 1) the second and third divisions
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
    // 2) the club relegated from the third division to the Conference
    // simplification: without a relegated club the original later compares with a stack dword without a value; the copy uses 0 (no club)
    let mut found = 0usize;
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let c = base + i as u16 as usize * club::SIZE;
            let before = read_usize(c + 0x5b);
            if eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) && before != 0 && read_usize(before) == read_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) {
                crate::comparison::branch("cqs_5 rebaixado para a conference");
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
    let nacao = read_usize(read_usize(this + 4) + 0x5d);
    if *((nacao + nation::LEAGUE_SELECTED) as *const u8) & 4 != 0 {
        // 3a) the Conference by category
        let mut cat: i8 = 6;
        if k < read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
            loop {
                let mut i: i16 = 0;
                if n > 0 {
                    loop {
                        if k >= read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
                            break;
                        }
                        let c = base + i as u16 as usize * club::SIZE;
                        let before = read_usize(c + 0x5b);
                        if before != 0 && read_usize(before) == read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) && eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
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
                if k >= read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
                    break;
                }
            }
        }
    } else {
        // 3b) up to 30 from the Conference and, if short, English clubs outside the generic division; shuffled
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
        if (m as i32) < read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 - k {
            let lim = n.wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
            let mut j: i16 = 0;
            if lim > 0 {
                loop {
                    let c = base + j as u16 as usize * club::SIZE;
                    let nation = read_usize(c + 0x53);
                    if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_ENGLAND) && !eng_auto_cup_is_in_comp(c, KEY_CLUB_COMP_A_LOWER_DIVISION) {
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
        // simplification: the original pushes the two words in dwords with garbage in the high 16 bits; 0x4afbd0 reads the words
        exe::comp_util_sort_and_shuffle_top_clubs(local.as_mut_ptr() as usize, m as i16 as u16 as u32, (read_i16(this + eng_auto_cup::SELECTED_COUNT).wrapping_sub(k as i16)) as u16 as u32);
        let mut a: i16 = 0;
        while k < read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
            push_section_club(this, &mut k, local[a as u16 as usize]);
            a = a.wrapping_add(1);
        }
    }
    if k != read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32 {
        return 0;
    }
    crate::comparison::branch("cqs_5 chaves do trofeu da liga");
    exe::crt_qsort(read_usize(this + eng_auto_cup::SELECTED_CLUBS), read_usize(this + eng_auto_cup::SELECTED_COUNT) as u32, 4, 0x53fd40); // cqs5: North and South by latitude
    let half = (read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32) / 2;
    exe::crt_qsort(read_usize(this + eng_auto_cup::SELECTED_CLUBS), half as u32, 4, 0x4b0810);
    let half = (read_usize(this + eng_auto_cup::SELECTED_COUNT) as i32) / 2;
    exe::crt_qsort(read_usize(this + eng_auto_cup::SELECTED_CLUBS) + half as usize * 4, half as u32, 4, 0x4b0810);
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

/// Creates the Trophy section stage: a knockout (Cup_Stage) with the 56 clubs in list order (North, then South), using
/// the rounds of stage 0 of the cup table, no seeding.
///
/// `EngAutoCup::create_section_stage()`; SEH frame only for the new, no destructor. The section stage (North and South)
/// with the +0x3e clubs of list +0xb2, in order: a vector (0x944e46 of 4 × +0x3e; out of memory, the box for line
/// 0x2e9) holding them; the rounds through method +0x3c (push 0; push &extra; push &rounds; push 0; mov ecx ->
/// thiscall, ret 0x10); the stage (0x944cf1(0xb2); Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: the cup, +0x3e
/// (word), the vector, the rounds (word), +4, the table, the year +0x40 (word), 0, 1, the extra (word), +0x51 (byte),
/// 0, 0, 0, no seeds; the words and bytes are pushed with garbage in the upper bits, which the constructor does not
/// read) into [+0xc][0]; the vector and the table are freed (0x9452ca). Without a stage, the box for line 0x300; with
/// it, +0x30 = 0.
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
    let teams = exe::crt_malloc(((read_i16(this + cup::TEAM_COUNT) as i32) << 2) as u32);
    if teams == 0 {
        return show_error_at(0x9c4180, 0x2e9);
    }
    let mut i = 0i32;
    while i < read_i16(this + cup::TEAM_COUNT) as i32 {
        write_usize(teams + i as usize * 4, read_usize(read_usize(this + eng_auto_cup::SELECTED_CLUBS) + i as usize * 4)); // cqs5: the order of the brackets
        i += 1;
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this, 0, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, read_i16(this + cup::TEAM_COUNT) as u16 as u32, teams, rodadas as u16 as u32, read_usize(this + 4), table, read_i16(this + cup::YEAR) as u16 as u32, 0, 1,
            extra as u16 as u32, *((this + cup::IMPORTANCE_BIAS) as *const u8) as u32, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(read_usize(this + cup::STAGES), f);
    exe::crt_free(teams);
    exe::crt_free(table);
    if read_usize(read_usize(this + cup::STAGES)) == 0 {
        return show_error_at(0x9c4180, 0x300);
    }
    crate::comparison::branch("cqs_5 fase das chaves do trofeu");
    write_usize(this + cup::CURRENT_STAGE, 0);
}

/// Moves the Trophy on by one stage; when the sections end (stage 1) it builds the final between the North and South
/// winners; at the last stage, the error box.
///
/// `EngAutoCup::advance_stage()`, virtual +0x28. At the last stage (+0x30 ≥ +0x2c − 1, signed), the box for line 0x314
/// and nothing else; otherwise +0x30 goes up by one and, on reaching 1 (the end of the sections), it builds the final
/// (EngAutoCup::create_final_stage 0x56bce0, direct call with the cup's ecx -> thiscall, ret 0).
/// Original: `EngAutoCup::advance_stage` (0x56bc50, eng_auto_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x56bc50 } (thiscall, ret 0, eng_auto_cup.cpp; a virtual +0x28): `EngAutoCup::advance_stage()`: na última fase
/// (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x314 e nada mais; senão +0x30 sobe um e, ao chegar a 1 (o fim das
/// chaves), monta a final (EngAutoCup::create_final_stage 0x56bce0, call direto com o ecx da copa -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_auto_cup_advance_stage(this: usize) {
    let stage = read_usize(this + cup::CURRENT_STAGE) as i32;
    if stage >= (read_usize(this + cup::STAGE_COUNT) as i32).wrapping_sub(1) {
        return show_error_at(0x9c4180, 0x314);
    }
    let stage = stage + 1;
    write_usize(this + cup::CURRENT_STAGE, stage as usize);
    if stage == 1 {
        crate::comparison::branch("cqs_5 final do trofeu");
        exe::engautocup_create_final_stage(this);
    }
}

/// First club among the first n 6-byte entrants whose status (+5) is s, or None.
///
/// [pt] O primeiro clube da lista de inscritos `l` (entradas de 6 bytes: +0 o clube, +5 a situação) com a situação `s`, entre
/// as `n` primeiras (n = +0x3e da copa, word com sinal); sem ele, None.
pub(crate) unsafe fn first_entrant_with_status(l: usize, n: i32, s: u8) -> Option<usize> {
    (0..n.max(0) as usize).find(|&i| *((l + i * 6 + 5) as *const u8) == s).map(|i| read_usize(l + i * 6))
}

/// Builds the Trophy final when the sections end: a single tie (2-team Cup_Stage, stage 1) between the club still alive
/// in the cup list and the one still alive in the section stage, and draws the round at once (home side).
///
/// `EngAutoCup::create_final_stage()`; from the next stage, SEH frame only for the new, no destructor. The Trophy
/// final: a vector of 2 (0x944e46 of 8; out of memory, the box for line 0x337) with the first club of status 1 (the one
/// still alive [inference]) in the cup list (+0xa7) and the first of status 1 in the section stage list ([+0xc][0]
/// +0xa7), both among the first +0x3e (of the cup); the rounds through method +0x3c (push 0; push &extra; push &rounds;
/// push 1; mov ecx -> thiscall, ret 0x10); the stage (0x944cf1(0xb2); Cup_Stage::Cup_Stage 0x522e00, thiscall, ret
/// 0x3c: the cup, 2, the vector, the rounds (word), +4, the table, the year +0x40 (word), 1, 1, the extra (word), 0x14,
/// 0, 0, 0, no seeds) into [+0xc][1]; the round table is freed (0x9452ca). Without a stage, the box for line 0x35a (the
/// vector is left allocated, as in the original). With it, the round draw (Cup::draw_round 0x51c410: push 0; mov ecx,
/// stage -> thiscall, ret 4) and the vector is freed.
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
    let l = exe::crt_malloc(8);
    if l == 0 {
        return show_error_at(0x9c4180, 0x337);
    }
    if let Some(c) = first_entrant_with_status(read_usize(this + cup::TEAM_TABLE), read_i16(this + cup::TEAM_COUNT) as i32, 1) { // cqs6: the cup finalist
        write_usize(l, c);
    }
    if let Some(c) = first_entrant_with_status(read_usize(read_usize(read_usize(this + cup::STAGES)) + 0xa7), read_i16(this + cup::TEAM_COUNT) as i32, 1) { // cqs6: the brackets finalist
        write_usize(l + 4, c);
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this, 1, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 2, l, rodadas as u16 as u32, read_usize(this + 4), table, read_i16(this + cup::YEAR) as u16 as u32, 1, 1, // cqs6: the year of the final
             extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(read_usize(this + cup::STAGES) + 4, f);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + cup::STAGES) + 4);
    if f == 0 {
        return show_error_at(0x9c4180, 0x35a);
    }
    crate::comparison::branch("cqs_6 final do trofeu");
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_DRAW_ROUND);
    sortear(f, 0); // cqs6: the draw of the final
    exe::crt_free(l);
}

/// Writes the Trophy final ranking into the cup statistics: the highest-reputation club of each half takes 1st and 2nd;
/// the others share one position per round, from the last round to the first.
///
/// `EngAutoCup::rank_teams()`, the virtual for the final ranking (n = 1 up to 2002-06-01). The final position of each
/// club in the cup statistics (+8, Comp_Stats; without them, the box for line 0x3bb). A vector of 2 × +0x3e pointers
/// (0x944e46; out of memory, the box for line 0x3c3): in the first half the clubs of the cup list (+0xa7, 6-byte
/// entries), in the second those of the section stage ([+0xc][0] +0xa7); each half sorted by reputation (+0x80, word),
/// highest first (qsort 0x945333 with 0x4b07d0). The first two (the highest-reputation club of each half): the one with
/// the higher reputation (signed, strictly) is 1st and the other 2nd (Comp_Stats::set_club_final_position 0x4a2540:
/// push position; push club; mov ecx, +8 -> thiscall, ret 8). Then, for each half, from the last round (+0x3c − 1) to
/// the first, as many clubs (the next ones of the half, from the 2nd on) as the round has in +0x1a of the round table
/// (+0xa3, 0x68 bytes per round), with positions 3, 5, 7... (3, then 3 + 2 × those already placed). The original pushes
/// the dword of the position byte with stack garbage in the upper bits; 0x4a2540 only reads the byte. The vector is
/// freed.
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
    if read_usize(this + 8) == 0 {
        return show_error_at(0x9c4180, 0x3bb);
    }
    let v = exe::crt_malloc(((read_i16(this + cup::TEAM_COUNT) as i32) << 3) as u32);
    if v == 0 {
        return show_error_at(0x9c4180, 0x3c3);
    }
    let mut d: i16 = 0;
    while d < read_i16(this + cup::TEAM_COUNT) {
        let e = d as i32 as usize * 6;
        write_usize(v + d as i32 as usize * 4, read_usize(read_usize(this + cup::TEAM_TABLE) + e));
        let n = read_i16(this + cup::TEAM_COUNT) as i32;
        write_usize(v + (n + d as i32) as usize * 4, read_usize(read_usize(read_usize(read_usize(this + cup::STAGES)) + 0xa7) + e));
        d = d.wrapping_add(1);
    }
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0);
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    exe::crt_qsort(v + n as usize * 4, n as u32, 4, 0x4b07d0);
    let position: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    let a = read_usize(v);
    let b = read_usize(v + read_i16(this + cup::TEAM_COUNT) as i32 as usize * 4);
    crate::comparison::branch("cqs_6 classificacao do trofeu");
    if read_i16(a + 0x80) > read_i16(b + 0x80) {
        position(read_usize(this + 8), a, 1);
        position(read_usize(this + 8), read_usize(v + read_i16(this + cup::TEAM_COUNT) as i32 as usize * 4), 2);
    } else {
        position(read_usize(this + 8), a, 2);
        position(read_usize(this + 8), read_usize(v + read_i16(this + cup::TEAM_COUNT) as i32 as usize * 4), 1);
    }
    for lado in 0..2u8 {
        let mut k: i16 = if lado == 0 { 1 } else { read_i16(this + cup::TEAM_COUNT).wrapping_add(1) };
        let mut pos: u8 = 3;
        let last = read_i16(this + cup::ROUND_COUNT).wrapping_sub(1);
        if last < 0 {
            continue;
        }
        let mut r = last as i32;
        while r >= 0 {
            let p = pos;
            let mut j: i16 = 0;
            while j < read_i16(read_usize(this + cup::ROUND_TABLE) + r as usize * 0x68 + 0x1a) {
                let c = read_usize(v + k as i32 as usize * 4);
                k = k.wrapping_add(1);
                position(read_usize(this + 8), c, p as u32);
                pos = pos.wrapping_add(2);
                j = j.wrapping_add(1);
            }
            r -= 1;
        }
    }
    exe::crt_free(v);
}

// simplification: writes to the cup statistics (Comp_Stats, which finds or appends the club’s entry: allocates); original only

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

