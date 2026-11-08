use super::*;

// ---------- 283a leva: a Copa América: as 12 seleções, as posições previstas e a temporada nova (goldcup.cpp) ----------




/// A seleção (o clube da seleção principal) da nação de id [`global`]: 0x53b3d0 database_get_national_team (push nação;
/// add esp -> cdecl) com a nação da tabela [0xae23a8] (0x122 bytes cada).
pub(crate) unsafe fn cqs12_selecao(id: usize) -> usize {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    selecao(read_u32_as_usize(G_NATION_TABLE) + id * 0x122)
}

/// As 12 da Copa América de 2001, na ordem da lista (os grupos intercalados: A nas vagas 0, 3, 6 e 9; B nas 1, 4, 7 e
/// 10; C nas 2, 5, 8 e 11): os globais dos ids das nações.
pub(crate) const CQS12_DE_2001: [usize; 12] = [
    KEY_NATION_COLOMBIA, KEY_NATION_BRAZIL, KEY_NATION_COSTA_RICA, KEY_NATION_CHILE, // cqs12: as vagas 0 a 3 (a Colômbia na 0)
    KEY_NATION_MEXICO, KEY_NATION_HONDURAS, KEY_NATION_ECUADOR, KEY_NATION_PERU, // cqs12: as vagas 4 a 7 (o México na 4, o Peru na 7)
    KEY_NATION_URUGUAY, KEY_NATION_VENEZUELA, KEY_NATION_PARAGUAY, KEY_NATION_BOLIVIA, // cqs12: as vagas 8 a 11 (a Bolívia na 11)
];

/// { 0x5e1cd0 } (thiscall, ret 0, goldcup.cpp): `GoldCup::set_up_team_list()`: +0xf2 = 12 e a lista +0xee (0x944e46(0x30):
/// push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x37b e devolve 0). Em 2001 (+0x40 = 0x7d1): as 12 fixas
/// (CQS12_DE_2001, cada uma pela 0x53b3d0) e devolve 1, sem ordenar. Nos outros anos: as sedes da Copa América
/// ([0x9cf7a8]) no ano (0x5fa730 HostCountry::find_hosts em [0xb5e134]: push 1; push &segunda; push &primeira; push ano;
/// push competição -> thiscall, ret 0x14; as duas começam em −1; devolve quantas, byte com sinal) na frente; depois,
/// pela ordem das nações ([0xae23a8], [0xae2350] delas, i como palavra) e enquanto a lista não enche (+0xf2), as do
/// continente [0x9cfa1c] (+0x71, o +0 dele) e as convidadas: em 1999 (0x7cf) a [0x9cf36c], nos outros anos a [0x9cf280];
/// e sempre o México ([0x9cf3d0]); cada uma entra se não é a nação de uma sede (+0x53 da seleção). Sem 12: a caixa da
/// linha 0x3bc e devolve 0; com 12, a lista ordenada pela reputação (qsort 0x945333 com 0x4b07d0: 4 pushes; add esp ->
/// cdecl) e devolve 1.
pub(crate) unsafe fn cqs12_selecoes(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    write_u32_from_usize(this + 0xf2, 0xc);
    let lista = alocar(0x30);
    write_u32_from_usize(this + 0xee, lista);
    if lista == 0 {
        aviso_em(0x9cd640, 0x37b);
        return 0;
    }
    let ano = read_i16(this + 0x40) as u16;
    if ano == 0x7d1 {
        crate::comparacao::ramo("cqs_12 selecoes de 2001");
        for (k, &g) in CQS12_DE_2001.iter().enumerate() {
            write_u32_from_usize(read_u32_as_usize(this + 0xee) + k * 4, cqs12_selecao(read_u32_as_usize(g)));
        }
        return 1;
    }
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_HOSTS);
    let mut ids = [-1i32; 2];
    let (a, b) = ids.split_at_mut(1);
    // ponytail: a original empurra o eax com o ano nos 16 bits baixos e sobra nos altos; a 0x5fa730 só lê o word
    let quantas = sedes(read_u32_as_usize(G_HOST_COUNTRY), read_u32_as_usize(KEY_CLUB_COMP_COPA_AMERICA), ano as u32, a.as_mut_ptr(), b.as_mut_ptr(), 1) as u8 as i8;
    let mut k = 0i16;
    while k < quantas as i16 {
        write_u32_from_usize(read_u32_as_usize(this + 0xee) + k as usize * 4, cqs12_selecao(ids[k as usize] as usize));
        k += 1;
    }
    let mut i = 0i16;
    let mut nacao = read_u32_as_usize(G_NATION_TABLE);
    while (i as i32) < read_u32_as_usize(G_NATION_COUNT) as i32 && (k as i32) < read_u32_as_usize(this + 0xf2) as i32 {
        let continente = read_u32_as_usize(nacao + nation::CONTINENT);
        let id = read_u32_as_usize(nacao);
        let entra = (continente != 0 && read_u32_as_usize(continente) == read_u32_as_usize(KEY_CONTINENT_SOUTH_AMERICA))
            || (ano == 0x7cf && id == read_u32_as_usize(KEY_NATION_JAPAN))
            || (ano != 0x7cf && id == read_u32_as_usize(KEY_NATION_CANADA))
            || id == read_u32_as_usize(KEY_NATION_MEXICO);
        if entra {
            let l = read_u32_as_usize(this + 0xee);
            let sede = (0..quantas.max(0) as usize).any(|j| read_u32_as_usize(read_u32_as_usize(l + j * 4) + 0x53) == nacao);
            if !sede {
                write_u32_from_usize(read_u32_as_usize(this + 0xee) + k as usize * 4, selecao(nacao));
                k += 1;
            }
        }
        i = i.wrapping_add(1);
        nacao += 0x122;
    }
    if k as i32 != read_u32_as_usize(this + 0xf2) as i32 {
        aviso_em(0x9cd640, 0x3bc);
        return 0;
    }
    ordenar(read_u32_as_usize(this + 0xee), k as i32 as u32, 4, 0x4b07d0);
    1
}

/// rule: the Copa América has 12 nations; in 2001 a fixed list (the groups interleaved in the places: A = Colombia,
/// Chile, Ecuador and Venezuela; B = Brazil, Mexico, Peru and Paraguay; C = Costa Rica, Honduras, Uruguay and Bolivia);
/// in other years the hosts first, then the South American nations and the guests (Japan in 1999 [inference: the nation
/// of 0x9cf36c], in other years that of 0x9cf280, and Mexico always), without repeating the hosts, up to 12, sorted by
/// reputation; without 12, an error
/// Original: `GoldCup::set_up_team_list` (0x5e1cd0, goldcup.cpp, thiscall, ret 0).
///
/// [pt] regra: a Copa América tem 12 seleções; em 2001 uma lista fixa (os grupos intercalados nas vagas: A = Colômbia,
/// Chile, Equador e Venezuela; B = Brasil, México, Peru e Paraguai; C = Costa Rica, Honduras, Uruguai e Bolívia); nos
/// outros anos as sedes na frente, depois as sul-americanas e as convidadas (o Japão em 1999 [inferência: a nação de
/// 0x9cf36c], nos outros anos a de 0x9cf280, e o México sempre), sem repetir as sedes, até 12, ordenadas pela
/// reputação; sem 12, erro
pub(crate) unsafe extern "thiscall" fn set_up_team_list(this: usize) -> u32 {
    cqs12_selecoes(this)
}

/// Lado a lado: o +0xf2 e o [0xb67a34] a partir do mesmo estado (nada sorteia); a original deixa a lista dela em +0xee, a
/// cópia faz a sua, comparada nas 12 vagas e liberada.
pub(crate) unsafe extern "thiscall" fn comparar_set_up_team_list(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SET_UP_TEAM_LIST);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let r = cqs12_selecoes(this);
        lr.set(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        a == 0 || core::slice::from_raw_parts(lo as *const u8, 0x30) == core::slice::from_raw_parts(lr as *const u8, 0x30)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("set_up_team_list({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// As posições previstas da Copa América, pela ordem de reputação das 12.
pub(crate) const CQS12_PREVISTAS: [u32; 12] = [1, 2, 3, 3, 5, 5, 5, 5, 9, 0xa, 0xa, 0xa];

/// rule: the Copa América predicted positions by reputation: 1, 2, 3, 3, four 5ths, 9 and three 10ths
/// Original: `GoldCup::set_predicted_positions` (0x5e2340, goldcup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5e2340 } (thiscall, ret 0, goldcup.cpp; a virtual +0x5c): `GoldCup::set_predicted_positions()`: sem estatística
/// (+8), a caixa da linha 0x419 e volta; uma cópia da lista (0x944e46(+0xf2 × 4); sem memória, a caixa da linha 0x421 e
/// volta), ordenada pela reputação (qsort 0x945333 com 0x4b07d0, +0xf2 itens), e a posição prevista de cada uma na
/// estatística (0x4a2540 Comp_Stats::set_club_final_position: push posição; push clube; mov ecx, [+8] -> thiscall,
/// ret 8): 1, 2, 3, 3, quatro 5, 9 e três 10 (as 12 primeiras da cópia); a cópia liberada (0x9452ca).
pub(crate) unsafe extern "thiscall" fn goldcup_set_predicted_positions(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if read_u32_as_usize(this + 8) == 0 {
        return aviso_em(0x9cd640, 0x419);
    }
    let v = alocar((read_u32_as_usize(this + 0xf2) << 2) as u32);
    if v == 0 {
        return aviso_em(0x9cd640, 0x421);
    }
    let n = read_u32_as_usize(this + 0xf2) << 2;
    core::ptr::copy_nonoverlapping(read_u32_as_usize(this + 0xee) as *const u8, v as *mut u8, n);
    ordenar(v, read_u32_as_usize(this + 0xf2) as u32, 4, 0x4b07d0);
    crate::comparacao::ramo("cqs_12 previstas da copa america");
    for (k, &p) in CQS12_PREVISTAS.iter().enumerate() {
        posicao(read_u32_as_usize(this + 8), read_u32_as_usize(v + k * 4), p);
    }
    soltar(v);
}

// ponytail: a estatística acha ou cria a entrada de cada seleção (aloca): não roda duas vezes. Só a original; o plano:
// a lista ordenada pela reputação antes, e depois a posição (+0x72 da entrada, 0x4a4850 Comp_Stats::find_club_entry) de
// cada uma das 12 igual à da faixa.
pub(crate) unsafe extern "thiscall" fn comparar_goldcup_set_predicted_positions(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_GOLDCUP_SET_PREDICTED_POSITIONS);
    if read_u32_as_usize(this + 8) == 0 || read_u32_as_usize(this + 0xee) == 0 || read_u32_as_usize(this + 0xf2) != 0xc {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut v: Vec<usize> = (0..12).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0xee) + i * 4)).collect();
    ordenar(v.as_mut_ptr(), 12, 4, 0x4b07d0);
    o(this);
    let errado = (0..12).find(|&k| {
        let e = if v[k] != 0 { achar(read_u32_as_usize(this + 8), v[k]) } else { 0 };
        e == 0 || *((e + 0x72) as *const u8) as u32 != CQS12_PREVISTAS[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("goldcup_set_predicted_positions: a posição prevista da vaga {:?} não bate", errado));
}

/// rule: the Copa América is every 2 years: when it ends, the table and stages are undone, the season goes to the
/// statistics history, the year goes up 2 and the rules are set up again
/// Original: `GoldCup::start_new_season` (0x5e1530, goldcup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5e1530 } (thiscall, ret 0, goldcup.cpp; a virtual +8): `GoldCup::start_new_season()`, a Copa América seguinte:
/// +0x4c = 0, a tabela da copa liberada (0x687970 League::free_table: push 0; mov ecx, this -> thiscall, ret 4), a
/// tabela das rodadas +0xba liberada (0x9452ca) e zerada, a temporada arquivada na estatística (0x4a1c50
/// Comp_Stats::archive_season_and_reset: push 1; mov ecx, [+8] -> thiscall, ret 4), as fases 0..+0x30 destruídas (a
/// virtual 0 de cada uma com 1: o destrutor que libera), o ano +0x40 sobe 2, +0x45 = 0, +0x30 = −1 e as regras de novo
/// (a virtual +0x8c, thiscall, ret 0; 0: a caixa da linha 0x28b).
pub(crate) unsafe extern "thiscall" fn goldcup_start_new_season(this: usize) {
    let soltar_tabela: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
    write_u32_from_usize(this + 0x4c, 0);
    soltar_tabela(this, 0);
    if read_u32_as_usize(this + 0xba) != 0 {
        soltar(read_u32_as_usize(this + 0xba));
        write_u32_from_usize(this + 0xba, 0);
    }
    if read_u32_as_usize(this + 8) != 0 {
        arquivar(read_u32_as_usize(this + 8), 1);
    }
    if (read_u32_as_usize(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + i as usize * 4);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > read_u32_as_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    crate::comparacao::ramo("cqs_12 temporada nova da copa america");
    core::ptr::write_unaligned((this + 0x40) as *mut i16, read_i16(this + 0x40).wrapping_add(2)); // cqs12: a Copa América a cada 2 anos
    write_u32_from_usize(this + 0x45, 0);
    write_u32_from_usize(this + 0x30, usize::MAX);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(0x9cd640, 0x28b);
    }
}

// ponytail: libera as tabelas e destrói as fases: não roda duas vezes. Só a original; o plano: o ano sobe 2, +0x45 = 0,
// +0x30 = −1, +0xba = 0 e +0x4c = 0 (as regras gravam a tabela das rodadas de novo em +0xba: conferida só que existe
// quando havia antes).
pub(crate) unsafe extern "thiscall" fn comparar_goldcup_start_new_season(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_GOLDCUP_START_NEW_SEASON);
    let ano = read_i16(this + 0x40);
    o(this);
    let ok = read_i16(this + 0x40) == ano.wrapping_add(2) && read_u32_as_usize(this + 0x45) == 0 && read_u32_as_usize(this + 0x30) as i32 == -1 && read_u32_as_usize(this + 0x4c) == 0;
    PLACAR.registrar(ok, || format!("goldcup_start_new_season({:#x}): ano {} -> {}, +0x45 {}, +0x30 {}", this, ano, read_i16(this + 0x40), read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
}

// ---------- 282a leva: a Copa América: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros dois grupos (goldcup.cpp) ----------




/// A seleção k da lista dos sorteados (+0xee, ponteiros: os 12 em grupos intercalados, o grupo j nas vagas j, j + 3,
/// j + 6 e j + 9), a que o primeiro grupo e as fases dos outros leem.
pub(crate) unsafe fn cqr13_sorteado(this: usize, k: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(this + 0xee) + k * 4) // cqr13: o sorteado k
}

/// rule: the Copa América has 12 nations in 3 groups of 4 (single round, 3/1 points, two qualify from each group plus
/// the two best thirds to the quarter-finals); the draw is on the start day (20 March of the cup year, moved to weekday
/// 5) and only on the call without `completo`: the list of 12, the predicted positions, group A in the cup's own table,
/// the fixtures and groups B and C as stages; before that day the cup does not even run the league day
/// Original: `GoldCup::update_for_date` (0x5e2120, goldcup.cpp, thiscall, ret 8).
///
/// [pt] { 0x5e2120 } (thiscall, ret 8, goldcup.cpp; a virtual +0x14; quadro SEH só das datas locais, sem destrutor):
/// `GoldCup::update_for_date(hoje, completo)`, o dia da Copa América. A data do sorteio é a de 0x549ef0(+0xb6, +0xb8 +
/// +0x40) (lea ecx, local; push ano; push dia -> thiscall, ret 8, pura: o dia do ano e o ano). Antes dela (o ano de
/// hoje menor, sem sinal; ou o mesmo ano e o dia de hoje menor, com sinal) a rotina volta 0 sem o dia de liga. No dia
/// do sorteio, fora do sorteio feito (+0x45 = 0) e SEM `completo` (a original sorteia só na chamada com completo = 0):
/// a lista das 12 seleções (0x5e1cd0: mov ecx -> thiscall, ret 0), as posições previstas (a virtual +0x5c, thiscall,
/// ret 0), a tabela do primeiro grupo (0x5e11e0, thiscall, ret 0; 0: a caixa da linha 0x3e7), os jogos (0x6835c0
/// League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x3ed), a tabela ordenada (0x6827d0 push 0; mov ecx ->
/// thiscall, ret 4; 0: a caixa da linha 0x3f4) (um erro volta 0 sem o dia de liga), as fases dos grupos 0 e 1 (0x5e16b0:
/// push g; mov ecx -> thiscall, ret 4), a lista +0xee liberada (0x9452ca: push; add esp, 4 -> cdecl) e zerada, +0xf2 = 0
/// e +0x45 = 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx, this -> thiscall, ret 8), cuja
/// devolução ela repassa.
pub(crate) unsafe extern "thiscall" fn goldcup_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let selecoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_GOLDCUP_SET_UP_TEAM_LIST);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_GOLDCUP_CREATE_GROUP_TABLE);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(FN_GOLDCUP_CREATE_GROUP_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let dia = read_i16(this + 0xb6) as u16; // cqr13: o dia do sorteio
    let ano = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas duas voltas antes do sorteio e nos erros a original devolve só o al = 0 (o resto do eax é sobra das
    // chamadas de antes); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && read_u32_as_usize(this + 0x45) == 0 && completo == 0 {
        crate::comparacao::ramo("cqr_13 sorteio da copa america");
        selecoes(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
        previstas(this);
        if primeiro(this) == 0 {
            aviso_em(0x9cd640, 0x3e7);
            return 0;
        }
        if jogos(this) == 0 {
            aviso_em(0x9cd640, 0x3ed);
            return 0;
        }
        if ordenar(this, 0) == 0 {
            aviso_em(0x9cd640, 0x3f4);
            return 0;
        }
        for g in 0..2i32 {
            fase(this, g);
        }
        soltar(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, 0);
        write_u32_from_usize(this + 0xf2, 0);
        write_u32_from_usize(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: a lista, as previsões, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano no
// dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do primeiro grupo e as duas
// fases criadas e a fase atual (+0x30) a última delas (1).
pub(crate) unsafe extern "thiscall" fn comparar_goldcup_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_GOLDCUP_UPDATE_FOR_DATE);
    let sorteia = read_u32_as_usize(this + 0x45) == 0
        && completo == 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + 0xf2) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && fases != 0
            && (0..2).all(|g| read_u32_as_usize(fases + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 1;
        PLACAR.registrar(ok, || format!("goldcup_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// rule: group A of the Copa América has 4 nations, drawn entries 0, 3, 6 and 9 (the groups are interleaved in the
/// list), each in the row of its order; human managers get the group draw news
/// Original: `GoldCup::create_group_table` (0x5e11e0, goldcup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5e11e0 } (thiscall, ret 0, goldcup.cpp): `GoldCup::create_group_table()`, o grupo A da Copa América: 4 times
/// (+0x3e), a tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a
/// caixa da linha 0x1d4 e devolve 0), cada linha i com o sorteado 3i (+0xee: os grupos intercalados) (0x687430
/// League::init_table_row: push 0; push 0; push seleção; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do
/// sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0) e devolve 1.
pub(crate) unsafe extern "thiscall" fn goldcup_create_group_table(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9cd640, 0x1d4);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + 0x3e) {
        crate::comparacao::ramo("cqr_13 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr13_sorteado(this, i as usize * 3), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado 3i com a posição i (+4).
pub(crate) unsafe extern "thiscall" fn comparar_goldcup_create_group_table(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GOLDCUP_CREATE_GROUP_TABLE);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| read_u32_as_usize(lista + i * 12));
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (read_u32_as_usize(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && read_i16(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("goldcup_create_group_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// rule: groups B and C of the Copa América are league stages with drawn entries g + 1, g + 4, g + 7 and g + 10, the
/// cup's rules and year and the cup's round table for the group (6, 10 and 13 July); human managers get the news
/// Original: `GoldCup::create_group_stage` (0x5e16b0, goldcup.cpp, thiscall, ret 4).
///
/// [pt] { 0x5e16b0 } (thiscall, ret 4, goldcup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal):
/// `GoldCup::create_group_stage(g)`, a fase do grupo g + 1 (g = 0 e 1, do dia do sorteio). Os times num vetor de 4
/// ponteiros (0x944e46(0x10): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x2bf e 0): os sorteados g + 1,
/// g + 4, g + 7 e g + 10 (+0xee); a tabela das rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push g; mov
/// ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add esp, 4 -> cdecl; o construtor League_Stage
/// 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, 4, o vetor, 1, +4, a tabela das rodadas, as rodadas (word),
/// +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word), g, o id (word), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a
/// original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores limpos)
/// em [+0xc][g]; o vetor e a tabela das rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x2d6 e 0. Com ela:
/// +0x30 = g, a notícia do grupo (0x684230 na fase) e 1.
pub(crate) unsafe extern "thiscall" fn goldcup_create_group_stage(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let times = alocar(0x10);
    if times == 0 {
        aviso_em(0x9cd640, 0x2bf);
        return 0;
    }
    for n in 0..4usize {
        crate::comparacao::ramo("cqr_13 time do grupo");
        write_u32_from_usize(times + n * 4, cqr13_sorteado(this, (g + 1) as usize + n * 3)); // cqr13: o time na vaga do grupo
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            4,
            times,
            1,
            read_u32_as_usize(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            read_i16(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            4,
            0,
            read_i16(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(0x9cd640, 0x2d6);
        return 0;
    }
    write_u32_from_usize(this + 0x30, g as usize);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com 4 times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados g + 1, g + 4, g + 7 e g + 10 (em qualquer ordem: o construtor ordena a tabela).
pub(crate) unsafe extern "thiscall" fn comparar_goldcup_create_group_stage(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_GOLDCUP_CREATE_GROUP_STAGE);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 || !(0..2).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|n| read_u32_as_usize(lista + ((g + 1) as usize + n * 3) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + 0x30) as i32 == g && read_i16(f + 0x3e) == 4 && read_i16(f + 0x40) == read_i16(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("goldcup_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

// ---------- 281a leva: o calendário dos torneios de seleções: a Copa América, a Copa Africana de Nações e a Copa das Confederações (goldcup.cpp, african_nations.cpp, fifa_confed.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
pub(crate) enum Cqp12Op {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
pub(crate) use Cqp12Op::{B as Cqp12B, D as Cqp12D, E as Cqp12E, J as Cqp12J, L as Cqp12L, S as Cqp12S, W as Cqp12W, X as Cqp12X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações comuns e as da fase (os jogos de cada grupo) e o nome do contador. `None`: a fase devolve 0
/// sem gravar nada.
#[derive(Clone, Copy)]
pub(crate) struct Cqp12Fase {
    pub(crate) liga: i32,
    pub(crate) rodadas: i16,
    pub(crate) extra: i16,
    pub(crate) extra_checa: bool,
    pub(crate) tamanho: u32,
    pub(crate) linha: u32,
    pub(crate) ops: &'static [Cqp12Op],
    pub(crate) ops2: &'static [Cqp12Op],
    pub(crate) nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
pub(crate) unsafe fn cqp12_tabela(this: usize, f: Option<Cqp12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.liga;
    }
    *rodadas = f.rodadas;
    if !f.extra_checa || !extra.is_null() {
        *extra = f.extra;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(f.tamanho);
    if t == 0 {
        aviso_em(arquivo, f.linha);
        return 0;
    }
    crate::comparacao::ramo(f.nome);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let jogo_l: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo_c: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DAY);
    let ano16 = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for op in f.ops.iter().chain(f.ops2.iter()) {
        match *op {
            Cqp12D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqp12L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqp12S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqp12J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqp12E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqp12B(o, v) => *((t + o) as *mut u8) = v,
            Cqp12W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqp12X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
pub(crate) unsafe fn cqp12_gravados(ops: &[Cqp12Op], ops2: &[Cqp12Op], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqp12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqp12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqp12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqp12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqp12E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqp12B(o, _) => pega(o, 1),
            Cqp12W(o, _) => pega(o, 2),
            Cqp12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
pub(crate) unsafe fn cqp12_comparar(nome: &str, fase: u32, f: Option<Cqp12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rodadas, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && (extra.is_null() || *extra == e2) && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqp12_gravados(f.ops, f.ops2, a) == cqp12_gravados(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// A Copa América, as três rodadas de cada grupo (6, 10 e 13 de julho, código 1, valor 1) com o jogo 2 vazio.
pub(crate) const CQP12_COPA_AMERICA_GRUPOS: [Cqp12Op; 6] = [
    Cqp12D(0, 6, 6, 0, 3, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xa, 6, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xd, 6, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa América, os jogos do grupo da fase −1.
pub(crate) const CQP12_COPA_AMERICA_GRUPO_A: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 2, 2, 3, 4), Cqp12L(0, 1, 3, 2, 3, 0x80), Cqp12L(1, 0, 6, 1, 3, 0x100), Cqp12L(1, 1, 6, 2, 3, 0x40),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 2, 3, 0x80),
];
/// A Copa América, os jogos do grupo da fase 0.
pub(crate) const CQP12_COPA_AMERICA_GRUPO_B: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 4, 1, 3, 0x40), Cqp12L(0, 1, 4, 2, 3, 0x100), Cqp12L(1, 0, 0, 1, 3, 4), Cqp12L(1, 1, 0, 2, 3, 0x80),
    Cqp12L(2, 0, 3, 1, 3, 0x40), Cqp12L(2, 1, 3, 2, 3, 0x100),
];
/// A Copa América, os jogos do grupo da fase 1.
pub(crate) const CQP12_COPA_AMERICA_GRUPO_C: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 0x80), Cqp12L(0, 1, 5, 2, 3, 4), Cqp12L(1, 0, 1, 1, 3, 0x100), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x80),
];
/// A Copa América, o mata-mata em jogo único: quartas em 16/7 (com os confrontos fixos), semifinal em 19/7, final em 23/7 e terceiro lugar em 22/7.
pub(crate) const CQP12_COPA_AMERICA_MATA: [Cqp12Op; 80] = [
    Cqp12S(0, 0xf, 6, 0, 5), Cqp12J(0, 0x10, 6, 0, 6, 1, 1), Cqp12E(0, 0, 6, 2, 3, 4), Cqp12E(0, 1, 6, 1, 3, 0x40),
    Cqp12E(0, 2, 0, 1, 3, 0x100), Cqp12E(0, 3, 0, 2, 3, 0x80), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x103), Cqp12B(0x17, 0xa), Cqp12W(0x18, 8), Cqp12W(0x1a, 4),
    Cqp12W(0x1c, 8), Cqp12W(0xf, 0), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0),
    Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0), Cqp12S(1, 0x12, 6, 0, 1), Cqp12J(1, 0x13, 6, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 1, 3, 0x40), Cqp12E(1, 1, 2, 2, 3, 4), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, 0x103), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 4), Cqp12W(0x82, 2),
    Cqp12W(0x84, 0), Cqp12W(0x77, 0), Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0),
    Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0), Cqp12S(2, 0x14, 6, 0, 3), Cqp12J(2, 0x17, 6, 0, 6, 1, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, 3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2),
    Cqp12W(0xea, 1), Cqp12W(0xec, 0), Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1),
    Cqp12B(0xf2, 0), Cqp12X(0x12c, 0), Cqp12X(0x130, 0), Cqp12X(0x134, 0), Cqp12S(3, 0x14, 6, 0, 3),
    Cqp12J(3, 0x16, 6, 0, 5, 1, 0x80), Cqp12W(0x150, 2), Cqp12W(0x141, 3), Cqp12W(0x145, 3), Cqp12W(0x13f, 0x8c),
    Cqp12W(0x143, 0), Cqp12B(0x14f, 0xa), Cqp12W(0x152, 1), Cqp12W(0x154, 0), Cqp12W(0x147, 0), Cqp12W(0x156, 0),
    Cqp12B(0x158, 0), Cqp12B(0x159, 1), Cqp12B(0x15a, 0), Cqp12X(0x194, 0), Cqp12X(0x198, 0), Cqp12X(0x19c, 0),
];
/// A Copa Africana, as três rodadas de cada grupo (8, 12 e 16 de fevereiro) com o jogo 2 vazio.
pub(crate) const CQP12_AFRICANA_GRUPOS: [Cqp12Op; 6] = [
    Cqp12D(0, 8, 1, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xc, 1, 0, 3, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x10, 1, 0, 0, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa Africana, os jogos do grupo da fase −1.
pub(crate) const CQP12_AFRICANA_GRUPO_A: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 4), Cqp12L(0, 1, 6, 1, 3, 0x40), Cqp12L(1, 0, 2, 1, 3, 4), Cqp12L(1, 1, 2, 2, 3, 0x40),
    Cqp12L(2, 0, 6, 1, 3, 4), Cqp12L(2, 1, 6, 1, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 0.
pub(crate) const CQP12_AFRICANA_GRUPO_B: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 0, 1, 3, 4), Cqp12L(0, 1, 0, 2, 3, 0x40), Cqp12L(1, 0, 3, 1, 3, 4), Cqp12L(1, 1, 3, 2, 3, 0x40),
    Cqp12L(2, 0, 0, 1, 3, 4), Cqp12L(2, 1, 0, 1, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 1.
pub(crate) const CQP12_AFRICANA_GRUPO_C: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 6, 1, 3, 0x80), Cqp12L(0, 1, 6, 2, 3, 0x80), Cqp12L(1, 0, 2, 2, 3, 0x80),
    Cqp12L(1, 1, 3, 2, 3, 0x80), Cqp12L(2, 0, 0, 2, 3, 0x80), Cqp12L(2, 1, 0, 2, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 2.
pub(crate) const CQP12_AFRICANA_GRUPO_D: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 0, 2, 3, 0x80), Cqp12L(0, 1, 1, 2, 3, 0x80), Cqp12L(1, 0, 4, 1, 3, 0x80),
    Cqp12L(1, 1, 4, 2, 3, 0x80), Cqp12L(2, 0, 1, 1, 3, 0x40), Cqp12L(2, 1, 1, 1, 3, 0x80),
];
/// A Copa Africana, o mata-mata em jogo único: quartas em 21/2, semifinal em 25/2, final em 28/2 e terceiro lugar em 27/2.
pub(crate) const CQP12_AFRICANA_MATA: [Cqp12Op; 80] = [
    Cqp12S(0, 0x12, 1, 0, 2), Cqp12J(0, 0x15, 1, 0, 5, 1, 1), Cqp12E(0, 0, 4, 2, 3, 0x80), Cqp12E(0, 1, 5, 1, 3, 4),
    Cqp12E(0, 2, 5, 2, 3, 0x40), Cqp12E(0, 3, 6, 1, 3, 0x40), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 8), Cqp12W(0x1a, 4),
    Cqp12W(0x1c, 8), Cqp12W(0xf, 0), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0),
    Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0), Cqp12S(1, 0x17, 1, 0, 0), Cqp12J(1, 0x19, 1, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 1, 3, 4), Cqp12E(1, 1, 2, 2, 3, 0x80), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, 0x303), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 4), Cqp12W(0x82, 2),
    Cqp12W(0x84, 0), Cqp12W(0x77, 0), Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0),
    Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0), Cqp12S(2, 0x1a, 1, 0, 3), Cqp12J(2, 0x1c, 1, 0, 5, 1, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, 0x203), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2),
    Cqp12W(0xea, 1), Cqp12W(0xec, 0), Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1),
    Cqp12B(0xf2, 0), Cqp12X(0x12c, 0), Cqp12X(0x130, 0), Cqp12X(0x134, 0), Cqp12S(3, 0x1a, 1, 0, 3),
    Cqp12J(3, 0x1b, 1, 0, 4, 2, 0x40), Cqp12W(0x13f, 0x8c), Cqp12W(0x141, 3), Cqp12W(0x143, 0), Cqp12W(0x145, 0x203),
    Cqp12B(0x14f, 0xa), Cqp12W(0x150, 2), Cqp12W(0x152, 1), Cqp12W(0x154, 0), Cqp12W(0x147, 0), Cqp12W(0x156, 0),
    Cqp12B(0x158, 0), Cqp12B(0x159, 1), Cqp12B(0x15a, 0), Cqp12X(0x194, 0), Cqp12X(0x198, 0), Cqp12X(0x19c, 0),
];
/// A Copa das Confederações, as três rodadas de cada grupo (29 de julho, 1º e 4 de agosto) com o jogo 2 vazio.
pub(crate) const CQP12_CONFED_GRUPOS: [Cqp12Op; 6] = [
    Cqp12D(0, 0x1d, 6, 0, 5, 2, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 1, 7, 0, 1, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 4, 7, 0, 4, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa das Confederações, os jogos do grupo da fase −1.
pub(crate) const CQP12_CONFED_GRUPO_A: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 6, 1, 3, 4), Cqp12L(0, 1, 6, 2, 3, 0x40), Cqp12L(1, 0, 1, 1, 3, 4), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 3, 1, 3, 4), Cqp12L(2, 1, 3, 2, 3, 0x40),
];
/// A Copa das Confederações, os jogos do grupo da fase 0.
pub(crate) const CQP12_CONFED_GRUPO_B: [Cqp12Op; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 0x40), Cqp12L(0, 1, 5, 2, 3, 4), Cqp12L(1, 0, 2, 1, 3, 0x40), Cqp12L(1, 1, 2, 2, 3, 4),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x40),
];
/// A Copa das Confederações, o mata-mata em jogo único: semifinal em 6/8, final em 9/8 e terceiro lugar em 8/8.
pub(crate) const CQP12_CONFED_MATA: [Cqp12Op; 57] = [
    Cqp12S(0, 5, 7, 0, 5), Cqp12J(0, 6, 7, 0, 6, 1, 1), Cqp12E(0, 0, 6, 1, 3, 0x40), Cqp12E(0, 1, 6, 1, 3, 4),
    Cqp12E(0, 2, -1, -1, -1, 0), Cqp12W(0x7, 0x82), Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x383),
    Cqp12B(0x17, 0xa), Cqp12W(0x18, 4), Cqp12W(0x1a, 2), Cqp12W(0x1c, 4), Cqp12W(0xf, 0), Cqp12W(0x1e, 0),
    Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0), Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0),
    Cqp12S(1, 7, 7, 0, 0), Cqp12J(1, 9, 7, 0, 2, 1, 4), Cqp12W(0x6f, 0x96), Cqp12W(0x71, 1), Cqp12W(0x73, 0),
    Cqp12W(0x75, 0x83), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 2), Cqp12W(0x82, 1), Cqp12W(0x84, 0), Cqp12W(0x77, 0),
    Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0), Cqp12X(0xc4, 0), Cqp12X(0xc8, 0),
    Cqp12X(0xcc, 0), Cqp12S(2, 7, 7, 0, 0), Cqp12J(2, 8, 7, 0, 1, 1, 0x40), Cqp12W(0xd7, 0x8c), Cqp12W(0xd9, 2),
    Cqp12W(0xdb, 0), Cqp12W(0xdd, 0x83), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2), Cqp12W(0xea, 1), Cqp12W(0xec, 0),
    Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1), Cqp12B(0xf2, 0), Cqp12X(0x12c, 0),
    Cqp12X(0x130, 0), Cqp12X(0x134, 0),
];

/// A fase da Copa América pelo byte com sinal: abaixo de 2 os grupos (id 0x420 + fase; os jogos de cada grupo para −1,
/// 0 e 1), 2 o mata-mata, acima nada.
pub(crate) fn cqp12_copa_america_fase(fase: u32) -> Option<Cqp12Fase> {
    let f = fase as u8 as i8;
    if f < 2 {
        let jogos: &'static [Cqp12Op] = match f {
            -1 => &CQP12_COPA_AMERICA_GRUPO_A,
            0 => &CQP12_COPA_AMERICA_GRUPO_B,
            1 => &CQP12_COPA_AMERICA_GRUPO_C,
            _ => &[],
        };
        Some(Cqp12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x420), extra_checa: false, tamanho: 0xc3, linha: 0x9c,
            ops: &CQP12_COPA_AMERICA_GRUPOS, ops2: jogos, nome: "cqp_12 grupos da copa america" })
    } else if f == 2 {
        Some(Cqp12Fase { liga: 0, rodadas: 4, extra: 0, extra_checa: false, tamanho: 0x1a0, linha: 0xd6,
            ops: &CQP12_COPA_AMERICA_MATA, ops2: &[], nome: "cqp_12 mata-mata da copa america" })
    } else {
        None
    }
}

/// rule: the Copa América has three groups (stages −1, 0 and 1, ids 0x41f to 0x421) of three rounds on 6, 10 and 13
/// July, each round with the group's two fixed matches (pairings and venue); the knockout (stage 2) in single matches:
/// quarter-finals on 16/7 with fixed pairings (the two best third-placed teams [inference]), semi-final on 19/7, third
/// place on 22/7 and final on 23/7 (venue 4, neutral ground [inference]); category 10; tie-break 0x103 up to the
/// semi-final and 3 in the final; no prizes
/// Original: `GoldCup::create_schedule` (0x5e0900, goldcup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5e0900 } (thiscall, ret 0x10, goldcup.cpp; a virtual +0x3c da Copa América): `GoldCup::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqp12_copa_america_fase`.
pub(crate) unsafe extern "thiscall" fn goldcup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp12_tabela(this, cqp12_copa_america_fase(fase), rodadas, extra, liga, 0x9cd640)
}

pub(crate) unsafe extern "thiscall" fn comparar_goldcup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GOLDCUP_CREATE_SCHEDULE);
    cqp12_comparar("goldcup_create_schedule", fase, cqp12_copa_america_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| goldcup_create_schedule(this, fase, r, e, l))
}

/// A fase da Copa Africana: abaixo de 3 os grupos (id 0x3e9 + fase; os jogos de cada grupo para −1, 0, 1 e 2), 3 o
/// mata-mata, acima nada.
pub(crate) fn cqp12_africana_fase(fase: u32) -> Option<Cqp12Fase> {
    let f = fase as u8 as i8;
    if f < 3 {
        let jogos: &'static [Cqp12Op] = match f {
            -1 => &CQP12_AFRICANA_GRUPO_A,
            0 => &CQP12_AFRICANA_GRUPO_B,
            1 => &CQP12_AFRICANA_GRUPO_C,
            2 => &CQP12_AFRICANA_GRUPO_D,
            _ => &[],
        };
        Some(Cqp12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x3e9), extra_checa: false, tamanho: 0xc3, linha: 0x9e,
            ops: &CQP12_AFRICANA_GRUPOS, ops2: jogos, nome: "cqp_12 grupos da copa africana" })
    } else if f == 3 {
        Some(Cqp12Fase { liga: 0, rodadas: 4, extra: 0, extra_checa: false, tamanho: 0x1a0, linha: 0xe0,
            ops: &CQP12_AFRICANA_MATA, ops2: &[], nome: "cqp_12 mata-mata da copa africana" })
    } else {
        None
    }
}



/// A fase da Copa das Confederações: abaixo de 1 os grupos (id 0x3e9 + fase; os jogos de cada grupo para −1 e 0), 1 o
/// mata-mata, acima nada.
pub(crate) fn cqp12_confed_fase(fase: u32) -> Option<Cqp12Fase> {
    let f = fase as u8 as i8;
    if f < 1 {
        let jogos: &'static [Cqp12Op] = match f {
            -1 => &CQP12_CONFED_GRUPO_A,
            0 => &CQP12_CONFED_GRUPO_B,
            _ => &[],
        };
        Some(Cqp12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x3e9), extra_checa: false, tamanho: 0xc3, linha: 0x95,
            ops: &CQP12_CONFED_GRUPOS, ops2: jogos, nome: "cqp_12 grupos da copa das confederacoes" })
    } else if f == 1 {
        Some(Cqp12Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: false, tamanho: 0x138, linha: 0xc7,
            ops: &CQP12_CONFED_MATA, ops2: &[], nome: "cqp_12 mata-mata da copa das confederacoes" })
    } else {
        None
    }
}

/// rule: the Confederations Cup has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of three rounds on 29 July, 1 and
/// 4 August, with fixed matches; the knockout (stage 1) in single matches: semi-final on 6/8, third place on 8/8 and
/// final on 9/8 (venue 4); category 10; tie-break 0x383 in the semi-final and 0x83 after
/// Original: `FifaConfed::create_schedule` (0x58b560, fifa_confed.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x58b560 } (thiscall, ret 0x10, fifa_confed.cpp; a virtual +0x3c da Copa das Confederações): `FifaConfed::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase de `cqp12_confed_fase`.
pub(crate) unsafe extern "thiscall" fn goldcup_create_schedule_58b560(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqp12_tabela(this, cqp12_confed_fase(fase), rodadas, extra, liga, 0x9c7474)
}

pub(crate) unsafe extern "thiscall" fn comparar_goldcup_create_schedule_58b560(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GOLDCUP_CREATE_SCHEDULE_58B560);
    cqp12_comparar("goldcup_create_schedule_58b560", fase, cqp12_confed_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| goldcup_create_schedule_58b560(this, fase, r, e, l))
}

// ---------- 274a leva: as regras dos torneios de seleções: a Copa América, a Copa Africana de Nações e a eliminatória da Oceania (goldcup.cpp, african_nations.cpp, wc_oceania_league.cpp) ----------

/// Os campos de byte e palavra que a virtual +0x8c (set_up_rules) de um torneio de seleções grava na liga do grupo:
/// (deslocamento, tamanho 1 ou 2, valor). Os pontos da vitória (+0xc2) vêm à parte, para as mutações.
pub(crate) unsafe fn cqk10_campos(this: usize, campos: &[(usize, u8, u32)], vitoria: u8) {
    for &(o, n, v) in campos {
        if n == 1 {
            *((this + o) as *mut u8) = if o == 0xc2 { vitoria } else { v as u8 };
        } else {
            core::ptr::write_unaligned((this + o) as *mut u16, v as u16);
        }
    }
}

/// A primeira palavra da Date(dia, mês, ano da competição +0x40 + `ano`, dia da semana) (0x549f70: push semana; push
/// ano; push mês; push dia; lea ecx, local -> thiscall, ret 0x10).
pub(crate) unsafe fn cqk10_data(this: usize, dia: u32, mes: u32, ano: i16, semana: u32) -> u16 {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16).wrapping_add(ano) as u16 as u32;
    data(d.as_mut_ptr(), dia, mes, a, semana);
    core::ptr::read_unaligned(d.as_ptr() as *const u16)
}


/// O plano: os campos, +0xb6 (a data, calculada antes), +0xb8 e a devolução 1, lidos depois da original `o`.
pub(crate) unsafe fn cqk10_confere(nome: &str, this: usize, campos: &[(usize, u8, u32)], data: (u32, u32, i16, u32), b8: u16, o: impl FnOnce() -> u32) -> u32 {
    let palavra = cqk10_data(this, data.0, data.1, data.2, data.3);
    let r = o();
    let mut errados: [(usize, u32); 4] = [(0, 0); 4];
    let mut k = 0;
    for &(off, n, v) in campos {
        let (lido, v) = if n == 1 { (*((this + off) as *const u8) as u32, v & 0xff) } else { (core::ptr::read_unaligned((this + off) as *const u16) as u32, v & 0xffff) };
        if lido != v && k < 4 {
            errados[k] = (off, lido);
            k += 1;
        }
    }
    let b6 = core::ptr::read_unaligned((this + 0xb6) as *const u16);
    let lido8 = core::ptr::read_unaligned((this + 0xb8) as *const u16);
    PLACAR.registrar(r == 1 && k == 0 && b6 == palavra && lido8 == b8, || format!("{}({:#x}) devolveu {} campos errados {:x?} data {:#x} prevista {:#x} +0xb8 {:#x}", nome, this, r, &errados[..k], b6, palavra, lido8));
    r
}

pub(crate) const CQK10_COPA_AMERICA: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2),
    (0xc5, 1, 1), (0xc6, 1, 2), (0xc8, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// rule: the Copa America groups are single round robin, 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3
/// [inference: goal difference, goals for, wins]; two qualifiers per group; national teams; category 10 on each match;
/// 7 substitutes and 3 changes; the date of 20 of month 2 of the competition year on weekday 5 [inference: the draw
/// date]
/// Original: `GoldCup::set_up_rules` (0x5e12d0, goldcup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5e12d0 } (thiscall, ret 0, goldcup.cpp; a virtual +0x8c; quadro SEH só da data local, sem destrutor):
/// `GoldCup::set_up_rules()`: os grupos da Copa América em turno único, 3 pontos a vitória e 1 o empate, a categoria 0xa
/// (seleções), os desempates (1, 2, 3, 0), dois classificados por grupo, seleções (+0x43 = 2), 7 no banco e 3
/// substituições, começo em 20 do mês 2 do ano da competição num dia da semana 5; depois a tabela das rodadas.
pub(crate) unsafe extern "thiscall" fn goldcup_set_up_rules(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da copa america");
    let vitoria = 3; // cqk10: a vitória nos grupos da Copa América
    cqk10_campos(this, &CQK10_COPA_AMERICA, vitoria);
    let dia = 0x14; // cqk10: o dia da data da Copa América
    let palavra = cqk10_data(this, dia, 2, 0, 5);
    cqj12_fim(this, palavra, 0)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
pub(crate) unsafe extern "thiscall" fn comparar_goldcup_set_up_rules(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GOLDCUP_SET_UP_RULES);
    cqk10_confere("goldcup_set_up_rules", this, &CQK10_COPA_AMERICA, (0x14, 2, 0, 5), 0, || o(this))
}

pub(crate) const CQK10_AFRICANA: [(usize, u8, u32); 14] = [(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1),
    (0xc3, 1, 1), (0xc4, 1, 0xa), (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];


// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.

pub(crate) const CQK10_OCEANIA: [(usize, u8, u32); 14] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc6, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 1), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// rule: the Oceania qualifier groups are single round robin, 3/1 points, tiebreak 1, 2, 3, one qualifier per group (to
/// the Oceania final), category 10, 7 substitutes and 3 changes, starting on 15 of month 0 on weekday 5
/// Original: `WcOceaniaLeague::set_up_rules` (0x91ecf0, wc_oceania_league.cpp, thiscall, ret 0).
///
/// [pt] { 0x91ecf0 } (thiscall, ret 0, wc_oceania_league.cpp; a virtual +0x8c; quadro SEH só da data local):
/// `WcOceaniaLeague::set_up_rules()`: os grupos da eliminatória da Oceania em turno único, 3/1 pontos, a categoria 0xa,
/// os desempates (1, 2, 3), um classificado por grupo, seleções, 7 no banco e 3 substituições, começo em 15 do mês 0
/// do ano da competição num dia da semana 5.
pub(crate) unsafe extern "thiscall" fn goldcup_set_up_rules_91ecf0(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_10 regras da oceania");
    cqk10_campos(this, &CQK10_OCEANIA, 3);
    let dia = 0xf; // cqk10: o dia do começo na Oceania
    let palavra = cqk10_data(this, dia, 0, 0, 5);
    cqj12_fim(this, palavra, 0)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
pub(crate) unsafe extern "thiscall" fn comparar_goldcup_set_up_rules_91ecf0(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GOLDCUP_SET_UP_RULES_91ECF0);
    cqk10_confere("goldcup_set_up_rules_91ecf0", this, &CQK10_OCEANIA, (0xf, 0, 0, 5), 0, || o(this))
}

/// rule: the Copa América moves on one stage when the current one ends; after the 2 group stages (on reaching 2) it
/// builds the knockout; going past the last stage is an error
/// Original: `GoldCup::advance_stage` (0x5e1620, goldcup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5e1620 } (thiscall, ret 0, goldcup.cpp; a virtual +0x28): `GoldCup::advance_stage()`, a fase seguinte da Copa
/// América: a caixa da linha 0x29d na última; chegando a 2 (depois dos 2 grupos-fase), o mata-mata (0x5e18a0).
pub(crate) unsafe extern "thiscall" fn goldcup_advance_stage(this: usize) {
    cqs11_fase_seguinte(this, 0x9cd640, 0x29d, 2, 0x5e18a0, "cqs_11 mata-mata da copa america");
}

pub(crate) unsafe extern "thiscall" fn comparar_goldcup_advance_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_GOLDCUP_ADVANCE_STAGE);
    cqs11_comparar_fase_seguinte("goldcup_advance_stage", this, || o(this))
}

/// rule: the Copa America follows the African Nations Cup rule with two group stages (the knockout at stage 2)
/// Original: `GoldCup::update_staff_records_for_status` (0x5e13b0, goldcup.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x5e13b0 } (thiscall, ret 0x18, goldcup.cpp; a virtual +0x44 da Copa América): `GoldCup::
/// update_staff_records_for_status(seleção, desfecho, fase, _, jogo, a6)`: cqj10_selecao com 2 fases de grupos (o
/// mata-mata na fase 2).
pub(crate) unsafe extern "thiscall" fn goldcup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqj10_selecao(1, this, clube, desfecho, fase, jogo, a6, 2)
}

pub(crate) unsafe extern "thiscall" fn comparar_goldcup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_GOLDCUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("goldcup_update_staff_records_for_status({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}
