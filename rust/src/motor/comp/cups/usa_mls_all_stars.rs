use super::*;





/// Knockout cup start_new_season: frees entrants, rounds and ties, archives stats, bumps the year, rebuilds entrants,
/// rounds, tie tree and predictions.
///
/// [pt] A virada da temporada de uma copa (`start_new_season`, slot 3, +0x08; o molde das copas de mata-mata, conferido
/// instrução a instrução: só mudam o fonte das caixas, as linhas e a rotina dos inscritos): +0x4c = 0; a lista dos
/// inscritos (+0xa7) e a das rodadas (+0xa3) liberadas (0x9452ca, cdecl) e zeradas; os confrontos por rodada (+0xad: um
/// vetor por rodada, +0x3c rodadas, word com sinal) liberados e zerados um a um e o vetor liberado e zerado; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, push 1; mov ecx -> thiscall, ret 4); o ano (+0x40,
/// word) sobe `ano` (1); +0xab = 0 e +0xb1 = 0 (bytes); então, cada um com a sua caixa se devolver negativo: os inscritos
/// (`inscritos`, mov ecx, esi; call direto -> thiscall, ret 0), as rodadas (virtual +0x8c) e a árvore dos confrontos
/// (virtual +0x94); por fim, com `previstas`, as posições previstas (virtual +0x5c), cujo eax ela devolve (o Jogo das
/// Estrelas não as tem: devolve o eax da árvore).
pub(crate) unsafe fn usa_mls_all_stars_cup_start_new_season(this: usize, inscritos: usize, file: usize, rows: [u32; 3], year: i16, predicted: bool, ramo: &'static str) -> u32 {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    write_u32_from_usize(this + 0x4c, 0);
    for o in [0xa7usize, 0xa3] {
        let l = read_u32_as_usize(this + o);
        if l != 0 {
            soltar(l);
            write_u32_from_usize(this + o, 0);
        }
    }
    if read_u32_as_usize(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < read_i16(this + 0x3c) as i32 {
            let p = read_u32_as_usize(read_u32_as_usize(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
                write_u32_from_usize(read_u32_as_usize(this + 0xad) + i as usize * 4, 0);
            }
            i += 1;
        }
        soltar(read_u32_as_usize(this + 0xad));
        write_u32_from_usize(this + 0xad, 0);
    }
    let e = read_u32_as_usize(this + 8);
    if e != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
        arquivar(e, 1);
    }
    let a = read_i16(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(year));
    crate::comparacao::ramo(ramo);
    *((this + 0xab) as *mut u8) = 0;
    *((this + 0xb1) as *mut u8) = 0;
    let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(inscritos);
    // simplification: nas caixas a original sai com o eax da caixa (0x5e8290); quem chama a virada não lê a devolução
    if inscritos(this) < 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    if call_cup_virtual(this, 0x8c) < 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    let r = call_cup_virtual(this, 0x94);
    if r < 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if !predicted {
        return r as u32;
    }
    call_cup_virtual(this, 0x5c) as u32
}


/// rule: at the season turn the Japanese Super Cup is rebuilt: last edition's tables freed, history archived, year +1,
/// the two clubs (JapSuper::select_teams: the J1 champion and the Emperor's Cup winner [inference]), the rounds, the
/// tree and the predicted positions
/// Original: `JapSuper::start_new_season` (0x6664e0, jap_super.cpp, thiscall, ret 0).
///
/// [pt] { 0x6664e0 } (thiscall, ret 0, jap_super.cpp; a virtual +0x08 da Supercopa japonesa): `JapSuper::start_new_season()`,
/// a virada (`usa_mls_all_stars_cup_start_new_season`) com os dois clubes de JapSuper::select_teams (0x666160) e as caixas do jap_super.cpp nas
/// linhas 0x14b, 0x152 e 0x159.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(this, 0x666160, 0x9eaf4c, [0x14b, 0x152, 0x159], 1, true, "cqr_6 virada da supercopa japonesa") // cqr6: a Supercopa japonesa
}


/// rule: the same turn in the US Open Cup: tables freed, history archived, year +1, the entrants
/// (UsaOpenCup::collect_entrants), the rounds, the tree and the predicted positions
/// Original: `UsaOpenCup::start_new_season` (0x90b790, usa_open_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x90b790 } (thiscall, ret 0, usa_open_cup.cpp; a virtual +0x08 da Copa dos EUA): `UsaOpenCup::start_new_season()`,
/// a virada (`usa_mls_all_stars_cup_start_new_season`) com os inscritos de UsaOpenCup::collect_entrants (0x90b530) e as caixas do usa_open_cup.cpp
/// nas linhas 0x198, 0x19f e 0x1a6.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season_90b790(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(this, 0x90b530, 0xaa20e0, [0x198, 0x19f, 0x1a6], 1, true, "cqr_6 virada da copa dos eua") // cqr6: a Copa dos EUA
}


/// rule: the MLS All-Star Game turn: tables freed, history archived, year +1, the two all-star teams, the rounds and
/// the tree (no predicted positions)
/// Original: `UsaMlsAllStars::start_new_season` (0x90a760, usa_mls_all_stars.cpp, thiscall, ret 0).
///
/// [pt] { 0x90a760 } (thiscall, ret 0, usa_mls_all_stars.cpp; a virtual +0x08 do Jogo das Estrelas): `UsaMlsAllStars::
/// start_new_season()`, a virada (`usa_mls_all_stars_cup_start_new_season`) com os dois times de UsaMlsAllStars::set_up_teams (0x90a540) e as caixas
/// do usa_mls_all_stars.cpp nas linhas 0x133, 0x13a e 0x141, sem as posições previstas.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season_90a760(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(this, 0x90a540, 0xaa20a0, [0x133, 0x13a, 0x141], 1, false, "cqr_6 virada do jogo das estrelas") // cqr6: o Jogo das Estrelas
}


/// Club record i of G_CLUB_TABLE (0x245 bytes each).
///
/// [pt] O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
pub(crate) unsafe fn usa_mls_all_stars_club_by_index(i: usize) -> usize {
    read_u32_as_usize(G_CLUB_TABLE).wrapping_add(i.wrapping_mul(club::SIZE))
}

/// 0x90a540 UsaMlsAllStars::set_up_teams: the two All-Star clubs (home side by year parity), a random MLS stadium of
/// that conference and the US nation; returns 1.
///
/// [pt] { 0x90a540 } (thiscall, ret 0, usa_mls_all_stars.cpp): `UsaMlsAllStars::set_up_teams()`, os dois times do Jogo das
/// Estrelas: +0x3e = 2; a lista (0x944e46 de 0xc, cdecl; sem memória, a caixa da linha 0xc6 e −1) com os dois clubes das
/// estrelas ([0x9d00a4] e [0x9d00a8], índices na tabela dos clubes): no ano ímpar (+0x40, com sinal) o de [0x9d00a4] é o
/// primeiro (o mandante) e a conferência dos estádios é 0x413; no ano par o de [0x9d00a8] é o primeiro e a conferência é
/// 0x414. Um sorteio de 0 a 5 (0x90cfc0(6), cdecl) escolhe o k-ésimo clube da MLS ([0x9cf590] na divisão do clube, +0x57)
/// dessa conferência (usa_mls_get_club_division 0x909ec0, push clube; add esp -> cdecl, palavra com sinal), pela ordem
/// da tabela: o primeiro time das estrelas fica com o estádio dele (+0x69) e +0x6d = 0 (sem o k-ésimo, nada muda). Os
/// dois ficam com a nação [0xae23a8] + 0x122 × [0x9cf4f8] (+0x53, os EUA [inferência]) e os quatro bytes da lista (+4,
/// +5, +0xa, +0xb) zerados. Devolve 1.
pub(crate) unsafe fn set_up_all_star_teams(this: usize) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alloc(0xc);
    write_u32_from_usize(this + 0xa7, l);
    if l == 0 {
        show_error_at(0xaa20a0, 0xc6);
        return -1;
    }
    let (a, b) = (read_u32_as_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST), read_u32_as_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST));
    let conferencia = if read_i16(this + 0x40) % 2 != 0 {
        write_u32_from_usize(l, usa_mls_all_stars_club_by_index(a));
        write_u32_from_usize(read_u32_as_usize(this + 0xa7) + 6, usa_mls_all_stars_club_by_index(b));
        0x413
    } else {
        write_u32_from_usize(l + 6, usa_mls_all_stars_club_by_index(a));
        write_u32_from_usize(read_u32_as_usize(this + 0xa7), usa_mls_all_stars_club_by_index(b));
        0x414
    };
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let mut k = sortear(6) as u8; // cqr6: o estádio sorteado entre 6
    let division: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_USA_MLS_GET_CLUB_DIVISION);
    let mut i = 0usize;
    while (i as i32) < read_u32_as_usize(G_CLUB_COUNT) as i32 {
        let c = usa_mls_all_stars_club_by_index(i);
        let league = read_u32_as_usize(c + 0x57);
        if league != 0 && read_u32_as_usize(league) == read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && division(c) as i32 == conferencia {
            if k == 0 {
                crate::comparacao::ramo("cqr_6 estadio do jogo das estrelas");
                let first = read_u32_as_usize(read_u32_as_usize(this + 0xa7));
                write_u32_from_usize(first + 0x69, read_u32_as_usize(c + 0x69)); // cqr6: o estádio do clube sorteado
                *((read_u32_as_usize(read_u32_as_usize(this + 0xa7)) + 0x6d) as *mut u8) = 0;
                break;
            }
            k = k.wrapping_sub(1);
        }
        i += 1;
    }
    let nacao = read_u32_as_usize(G_NATION_TABLE).wrapping_add(read_u32_as_usize(KEY_NATION_UNITED_STATES).wrapping_mul(nation::SIZE));
    let l = read_u32_as_usize(this + 0xa7);
    write_u32_from_usize(read_u32_as_usize(l + 6) + 0x53, nacao);
    write_u32_from_usize(read_u32_as_usize(l) + 0x53, read_u32_as_usize(read_u32_as_usize(l + 6) + 0x53));
    for o in [4usize, 5, 0xa, 0xb] {
        *((read_u32_as_usize(this + 0xa7) + o) as *mut u8) = 0;
    }
    1
}

/// rule: the MLS All-Star Game pits the two conference all-star teams ([0x9d00a4] and [0x9d00a8], clubs in the table);
/// in odd years the [0x9d00a4] team is at home, in even years the [0x9d00a8] one; the stadium is that of an MLS club
/// drawn (1 in 6, in table order) among the home conference's clubs (0x413 in odd years, 0x414 in even ones); both
/// teams get the USA nation [inference]
/// Original: `UsaMlsAllStars::set_up_teams` (0x90a540, usa_mls_all_stars.cpp, thiscall, ret 0).
///
/// [pt] regra: o Jogo das Estrelas da MLS opõe as duas seleções das conferências ([0x9d00a4] e [0x9d00a8], clubes da
/// tabela); no ano ímpar a de [0x9d00a4] manda o jogo, no par a de [0x9d00a8]; o estádio é o de um clube da MLS
/// sorteado (1 em 6, pela ordem da tabela) entre os da conferência do mandante (0x413 no ano ímpar, 0x414 no par); os
/// dois times ficam com a nação dos EUA [inferência]
pub(crate) unsafe extern "thiscall" fn set_up_teams(this: usize) -> i32 {
    set_up_all_star_teams(this)
}


/// rule: the two MLS All-Star selections are called up on the day of Date(26, 6, year, 6): 26 July moved to weekday 6
/// [inference: the following Saturday]; the match is on 2 August (see cqm_12); then, every day, the cup update
/// Original: `UsaMlsAllStars::update_for_date` (0x90a900, usa_mls_all_stars.cpp, thiscall, ret 8).
///
/// [pt] { 0x90a900 } (thiscall, ret 8, usa_mls_all_stars.cpp; a virtual +0x14 do Jogo das Estrelas; quadro SEH pela Date
/// local, sem destrutor): `UsaMlsAllStars::update_for_date(hoje, fase)`: com a fase diferente de 0, no dia de Date(26, 6,
/// ano de hoje, dia da semana 6) (o sábado de 26 de julho em diante [inferência]), a convocação das duas seleções
/// (0x90a980, mov ecx -> thiscall, ret 0); depois a atualização da copa.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    // simplification: o ano vai num registrador com lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    let year = read_i16(today + 2) as u16 as u32;
    let d = make_date_dword(0x1a, 6, year, 6); // cqn10: a convocação do Jogo das Estrelas
    if stage != 0 && is_date_today(d, today) {
        crate::comparacao::ramo("cqn_10 convocacao do jogo das estrelas");
        let convocar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLSALLSTARS_SELECT_SQUADS);
        convocar(this);
    }
    eng_cc_cup_cup_update_for_date(this, today, stage)
}


/// rule: the MLS All-Star game is a single match between the two conference selections, drawn on 15/6 and played on 2/8
/// (moved to weekday 6) at the borrowed home ground [inference: venue 0], with no bracket draw (+0xd = 0x20
/// [inference]); category 8; no prize
/// Original: `UsaMlsAllStars::create_round_table` (0x90a400, usa_mls_all_stars.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x90a400 } (thiscall, ret 0x10, usa_mls_all_stars.cpp; a virtual +0x3c do Jogo das Estrelas): `UsaMlsAllStars::
/// create_round_table(fase, &rodadas, &extra, &saida)`: fora da fase −1 (o byte), 0. Na −1: saida (se pedida) = 0,
/// rodadas = 1, extra = 0; a tabela (0x944e46 de 0x68; sem memória, a caixa da linha 0x9a e 0) com a rodada única: o
/// sorteio em 15 do mês 5 (junho) levado ao dia da semana 5, o jogo em 2 do mês 7 (agosto) levado ao dia 6, turno 1,
/// mando 0 (0x521e60 e 0x521eb0, cdecl, com o ano +0x40); nome 0, +0xd = 0x20, categoria 8, 2 times, 1 jogo, 2 novos,
/// 1 jogo por confronto, sem prêmio.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 1;
    *extra = 0;
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alloc(0x68);
    if t == 0 {
        show_error_at(0xaa20a0, 0x9a);
        return 0;
    }
    crate::comparacao::ramo("cqm_12 tabela do jogo das estrelas");
    let year = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
    sorteio(t, 0, 0xf, 5, 0, 5, year);
    jogo(t, 0, 2, 7, 0, 6, 1, year, 0); // cqm12: o Jogo das Estrelas em 2 de agosto
    let w = |o: usize, v: u16| core::ptr::write_unaligned((t + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((t + o) as *mut u32, v);
    w(0x18, 2);
    w(0x1c, 2);
    w(7, 0);
    w(9, 0);
    w(0xb, 0);
    w(0xd, 0x20);
    *((t + 0x17) as *mut u8) = 8;
    w(0x1a, 1);
    w(0xf, 0);
    w(0x1e, 0);
    *((t + 0x20) as *mut u8) = 0;
    *((t + 0x21) as *mut u8) = 1;
    *((t + 0x22) as *mut u8) = 0;
    d(0x5c, 0);
    d(0x60, 0);
    d(0x64, 0);
    t
}

