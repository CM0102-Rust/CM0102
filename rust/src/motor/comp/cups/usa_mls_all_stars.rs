use super::*;

// ---------- 282a leva: a virada das copas: a Supercopa japonesa, a Copa dos EUA e o Jogo das Estrelas da MLS (com os dois times das estrelas e o estádio sorteado) (jap_super.cpp, usa_open_cup.cpp, usa_mls_all_stars.cpp) ----------





/// A virada da temporada de uma copa (`start_new_season`, slot 3, +0x08; o molde das copas de mata-mata, conferido
/// instrução a instrução: só mudam o fonte das caixas, as linhas e a rotina dos inscritos): +0x4c = 0; a lista dos
/// inscritos (+0xa7) e a das rodadas (+0xa3) liberadas (0x9452ca, cdecl) e zeradas; os confrontos por rodada (+0xad: um
/// vetor por rodada, +0x3c rodadas, word com sinal) liberados e zerados um a um e o vetor liberado e zerado; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, push 1; mov ecx -> thiscall, ret 4); o ano (+0x40,
/// word) sobe `ano` (1); +0xab = 0 e +0xb1 = 0 (bytes); então, cada um com a sua caixa se devolver negativo: os inscritos
/// (`inscritos`, mov ecx, esi; call direto -> thiscall, ret 0), as rodadas (virtual +0x8c) e a árvore dos confrontos
/// (virtual +0x94); por fim, com `previstas`, as posições previstas (virtual +0x5c), cujo eax ela devolve (o Jogo das
/// Estrelas não as tem: devolve o eax da árvore).
pub(crate) unsafe fn cqr6_virada(this: usize, inscritos: usize, arquivo: usize, linhas: [u32; 3], ano: i16, previstas: bool, ramo: &'static str) -> u32 {
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
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    *((this + 0xab) as *mut u8) = 0;
    *((this + 0xb1) as *mut u8) = 0;
    let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(inscritos);
    // ponytail: nas caixas a original sai com o eax da caixa (0x5e8290); quem chama a virada não lê a devolução
    if inscritos(this) < 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    if cqr5_virtual(this, 0x8c) < 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let r = cqr5_virtual(this, 0x94);
    if r < 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    if !previstas {
        return r as u32;
    }
    cqr5_virtual(this, 0x5c) as u32
}


/// rule: at the season turn the Japanese Super Cup is rebuilt: last edition's tables freed, history archived, year +1,
/// the two clubs (JapSuper::select_teams: the J1 champion and the Emperor's Cup winner [inference]), the rounds, the
/// tree and the predicted positions
/// Original: `JapSuper::start_new_season` (0x6664e0, jap_super.cpp, thiscall, ret 0).
///
/// [pt] { 0x6664e0 } (thiscall, ret 0, jap_super.cpp; a virtual +0x08 da Supercopa japonesa): `JapSuper::start_new_season()`,
/// a virada (`cqr6_virada`) com os dois clubes de JapSuper::select_teams (0x666160) e as caixas do jap_super.cpp nas
/// linhas 0x14b, 0x152 e 0x159.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season(this: usize) -> u32 {
    cqr6_virada(this, 0x666160, 0x9eaf4c, [0x14b, 0x152, 0x159], 1, true, "cqr_6 virada da supercopa japonesa") // cqr6: a Supercopa japonesa
}

pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_all_stars_start_new_season(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_ALL_STARS_START_NEW_SEASON);
    cqr5_comparar("usa_mls_all_stars_start_new_season", this, || o(this))
}

/// rule: the same turn in the US Open Cup: tables freed, history archived, year +1, the entrants
/// (UsaOpenCup::collect_entrants), the rounds, the tree and the predicted positions
/// Original: `UsaOpenCup::start_new_season` (0x90b790, usa_open_cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x90b790 } (thiscall, ret 0, usa_open_cup.cpp; a virtual +0x08 da Copa dos EUA): `UsaOpenCup::start_new_season()`,
/// a virada (`cqr6_virada`) com os inscritos de UsaOpenCup::collect_entrants (0x90b530) e as caixas do usa_open_cup.cpp
/// nas linhas 0x198, 0x19f e 0x1a6.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season_90b790(this: usize) -> u32 {
    cqr6_virada(this, 0x90b530, 0xaa20e0, [0x198, 0x19f, 0x1a6], 1, true, "cqr_6 virada da copa dos eua") // cqr6: a Copa dos EUA
}

pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_all_stars_start_new_season_90b790(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_ALL_STARS_START_NEW_SEASON_90B790);
    cqr5_comparar("usa_mls_all_stars_start_new_season_90b790", this, || o(this))
}

/// rule: the MLS All-Star Game turn: tables freed, history archived, year +1, the two all-star teams, the rounds and
/// the tree (no predicted positions)
/// Original: `UsaMlsAllStars::start_new_season` (0x90a760, usa_mls_all_stars.cpp, thiscall, ret 0).
///
/// [pt] { 0x90a760 } (thiscall, ret 0, usa_mls_all_stars.cpp; a virtual +0x08 do Jogo das Estrelas): `UsaMlsAllStars::
/// start_new_season()`, a virada (`cqr6_virada`) com os dois times de UsaMlsAllStars::set_up_teams (0x90a540) e as caixas
/// do usa_mls_all_stars.cpp nas linhas 0x133, 0x13a e 0x141, sem as posições previstas.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season_90a760(this: usize) -> u32 {
    cqr6_virada(this, 0x90a540, 0xaa20a0, [0x133, 0x13a, 0x141], 1, false, "cqr_6 virada do jogo das estrelas") // cqr6: o Jogo das Estrelas
}

pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_all_stars_start_new_season_90a760(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_ALL_STARS_START_NEW_SEASON_90A760);
    cqr5_comparar("usa_mls_all_stars_start_new_season_90a760", this, || o(this))
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
pub(crate) unsafe fn cqr6_clube(i: usize) -> usize {
    read_u32_as_usize(G_CLUB_TABLE).wrapping_add(i.wrapping_mul(0x245))
}

/// { 0x90a540 } (thiscall, ret 0, usa_mls_all_stars.cpp): `UsaMlsAllStars::set_up_teams()`, os dois times do Jogo das
/// Estrelas: +0x3e = 2; a lista (0x944e46 de 0xc, cdecl; sem memória, a caixa da linha 0xc6 e −1) com os dois clubes das
/// estrelas ([0x9d00a4] e [0x9d00a8], índices na tabela dos clubes): no ano ímpar (+0x40, com sinal) o de [0x9d00a4] é o
/// primeiro (o mandante) e a conferência dos estádios é 0x413; no ano par o de [0x9d00a8] é o primeiro e a conferência é
/// 0x414. Um sorteio de 0 a 5 (0x90cfc0(6), cdecl) escolhe o k-ésimo clube da MLS ([0x9cf590] na divisão do clube, +0x57)
/// dessa conferência (usa_mls_get_club_division 0x909ec0, push clube; add esp -> cdecl, palavra com sinal), pela ordem
/// da tabela: o primeiro time das estrelas fica com o estádio dele (+0x69) e +0x6d = 0 (sem o k-ésimo, nada muda). Os
/// dois ficam com a nação [0xae23a8] + 0x122 × [0x9cf4f8] (+0x53, os EUA [inferência]) e os quatro bytes da lista (+4,
/// +5, +0xa, +0xb) zerados. Devolve 1.
pub(crate) unsafe fn cqr6_times(this: usize) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0xc);
    write_u32_from_usize(this + 0xa7, l);
    if l == 0 {
        aviso_em(0xaa20a0, 0xc6);
        return -1;
    }
    let (a, b) = (read_u32_as_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST), read_u32_as_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST));
    let conferencia = if read_i16(this + 0x40) % 2 != 0 {
        write_u32_from_usize(l, cqr6_clube(a));
        write_u32_from_usize(read_u32_as_usize(this + 0xa7) + 6, cqr6_clube(b));
        0x413
    } else {
        write_u32_from_usize(l + 6, cqr6_clube(a));
        write_u32_from_usize(read_u32_as_usize(this + 0xa7), cqr6_clube(b));
        0x414
    };
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let mut k = sortear(6) as u8; // cqr6: o estádio sorteado entre 6
    let divisao: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_USA_MLS_GET_CLUB_DIVISION);
    let mut i = 0usize;
    while (i as i32) < read_u32_as_usize(G_CLUB_COUNT) as i32 {
        let c = cqr6_clube(i);
        let liga = read_u32_as_usize(c + 0x57);
        if liga != 0 && read_u32_as_usize(liga) == read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && divisao(c) as i32 == conferencia {
            if k == 0 {
                crate::comparacao::ramo("cqr_6 estadio do jogo das estrelas");
                let primeiro = read_u32_as_usize(read_u32_as_usize(this + 0xa7));
                write_u32_from_usize(primeiro + 0x69, read_u32_as_usize(c + 0x69)); // cqr6: o estádio do clube sorteado
                *((read_u32_as_usize(read_u32_as_usize(this + 0xa7)) + 0x6d) as *mut u8) = 0;
                break;
            }
            k = k.wrapping_sub(1);
        }
        i += 1;
    }
    let nacao = read_u32_as_usize(G_NATION_TABLE).wrapping_add(read_u32_as_usize(KEY_NATION_UNITED_STATES).wrapping_mul(0x122));
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
    cqr6_times(this)
}

/// Lado a lado a partir do mesmo estado (o sorteador, +0x3e, os dois clubes das estrelas e [0xb67a34]); a original deixa
/// a lista dela em +0xa7, a cópia faz a sua (sem contar o sorteio), comparada nos 12 bytes e liberada. A lista antiga
/// (+0xa7 na entrada) não é liberada por nenhuma das duas (a virada já a liberou).
pub(crate) unsafe extern "thiscall" fn comparar_set_up_teams(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SET_UP_TEAMS);
    let antes = read_u32_as_usize(this + 0xa7);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x3e, 2usize), (0xb67a34, 4), (cqr6_clube(read_u32_as_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST)), 0x245), (cqr6_clube(read_u32_as_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST)), 0x245)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xa7));
        write_u32_from_usize(this + 0xa7, antes);
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqr6_times(this));
        lr.set(read_u32_as_usize(this + 0xa7));
        write_u32_from_usize(this + 0xa7, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0xc) == core::slice::from_raw_parts(lr as *const u8, 0xc)
    } else {
        lo == lr
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("set_up_teams({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// rule: the two MLS All-Star selections are called up on the day of Date(26, 6, year, 6): 26 July moved to weekday 6
/// [inference: the following Saturday]; the match is on 2 August (see cqm_12); then, every day, the cup update
/// Original: `UsaMlsAllStars::update_for_date` (0x90a900, usa_mls_all_stars.cpp, thiscall, ret 8).
///
/// [pt] { 0x90a900 } (thiscall, ret 8, usa_mls_all_stars.cpp; a virtual +0x14 do Jogo das Estrelas; quadro SEH pela Date
/// local, sem destrutor): `UsaMlsAllStars::update_for_date(hoje, fase)`: com a fase diferente de 0, no dia de Date(26, 6,
/// ano de hoje, dia da semana 6) (o sábado de 26 de julho em diante [inferência]), a convocação das duas seleções
/// (0x90a980, mov ecx -> thiscall, ret 0); depois a atualização da copa.
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    let ano = read_i16(hoje + 2) as u16 as u32;
    let d = cqn10_data(0x1a, 6, ano, 6); // cqn10: a convocação do Jogo das Estrelas
    if fase != 0 && cqn10_e_hoje(d, hoje) {
        crate::comparacao::ramo("cqn_10 convocacao do jogo das estrelas");
        let convocar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_USAMLSALLSTARS_SELECT_SQUADS);
        convocar(this);
    }
    cqn10_base(this, hoje, fase)
}

pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_all_stars_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_USA_MLS_ALL_STARS_UPDATE_FOR_DATE)(this, hoje, fase)
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
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 1;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(0x68);
    if t == 0 {
        aviso_em(0xaa20a0, 0x9a);
        return 0;
    }
    crate::comparacao::ramo("cqm_12 tabela do jogo das estrelas");
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
    sorteio(t, 0, 0xf, 5, 0, 5, ano);
    jogo(t, 0, 2, 7, 0, 6, 1, ano, 0); // cqm12: o Jogo das Estrelas em 2 de agosto
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

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). Fora da
/// fase −1, só a original (nenhuma grava nada).
pub(crate) unsafe extern "thiscall" fn comparar_usa_mls_all_stars_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_USA_MLS_ALL_STARS_CREATE_ROUND_TABLE);
    if fase as u8 != 0xff {
        let a = o(this, fase, rodadas, extra, saida);
        PLACAR.registrar(a == 0, || format!("usa_mls_all_stars_create_round_table(fase {}) devolveu {:#x}", fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut s2 = if saida.is_null() { 0 } else { *saida };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, fase, rodadas, extra, saida),
        || usa_mls_all_stars_create_round_table(this, fase, &mut r2, &mut e2, if saida.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (saida.is_null() || *saida == s2);
    let tabela = if a != 0 && b != 0 { cqm12_gravados(a) == cqm12_gravados(b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("usa_mls_all_stars_create_round_table: saídas {} tabela {}", saidas, tabela));
    a
}
