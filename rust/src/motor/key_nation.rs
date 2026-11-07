use super::*;

// ---------- 257a leva: as linhas da nação-chave: a linha de uma nação, a linha do clube e o ano da temporada do clube (key_nation.cpp) ----------


/// the first row of the 35 key nation table holding the given nation, or 4
/// Original: `key_nation_index_of_nation` (0x669340, key_nation.cpp, cdecl).
///
/// [pt] { 0x669340 } (cdecl, key_nation.cpp): `key_nation_index_of_nation(nação)`: a primeira linha da tabela das 35
/// nações-chave (0xb63d60, 72 bytes, nação em +0) com a nação dada (sem conferir a nula), ou 4. Pela moral do jogo
/// 0x4cb250, pelas aposentadorias 0x7a7310, pelo amistoso livre 0x5c6b70, pelos prêmios do dia das nações e pelas vagas
/// da UEFA. Chamada a cada jogador: sem contador.
pub(crate) unsafe extern "C" fn linha_da_nacao(nacao: usize) -> u8 {
    for k in 0..0x23usize {
        if read_usize(0xb63d60 + 72 * k) == nacao {
            return k as u8; // cpu1: a linha da nacao dada
        }
    }
    4
}

pub(crate) unsafe extern "C" fn comparar_linha_da_nacao(nacao: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(1194)(nacao);
    let n = linha_da_nacao(nacao);
    PLACAR.registrar(r == n, || format!("linha_da_nacao(nação {:x}) original={} rust={}", nacao, r, n));
    r
}

/// the row of the club's chosen nation; national side, 1; nation not chosen, the row of its continent, or 3
/// Original: `key_nation_index_of_club` (0x669360, key_nation.cpp, cdecl).
///
/// [pt] { 0x669360 } (cdecl, key_nation.cpp): `key_nation_index_of_club(clube)`. Clube que não é seleção (0x53b900), com
/// nação (+0x53) escolhida para jogar (+0x11c da nação): a linha da nação na tabela, ou a caixa "Error" (linha 0x26d) e
/// 0. Senão (a original chama 0x53b900 de novo): seleção, 1; a primeira linha cujo continente (+4) é o da nação do clube
/// (+0x71), ou 3 (sem nação, 3). Pelas aposentadorias 0x7a7310, pelo adiamento pelo tempo 0x923630, pelos dias até a
/// temporada 0x59b4c0, pelo contrato desejado 0x85fc50, pelo ano da temporada 0x669950 e outras. Sem contador.
pub(crate) unsafe extern "C" fn linha_da_nacao_do_clube(clube: usize) -> u8 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) == 0 {
        let nacao = read_usize(clube + 0x53);
        if nacao != 0 && *((nacao + 0x11c) as *const u8) != 0 {
            for k in 0..0x23usize {
                if read_usize(0xb63d60 + 72 * k) == nacao {
                    return k as u8; // cpu1: a linha da nacao escolhida
                }
            }
            aviso_em(0x9eaff4, 0x26d);
            return 0;
        }
    }
    if selecao(clube) != 0 {
        return 1;
    }
    let nacao = read_usize(clube + 0x53); // cpu1: a nacao do continente
    for k in 0..0x23usize {
        if nacao != 0 && read_usize(0xb63d64 + 72 * k) == read_usize(nacao + 0x71) {
            return k as u8; // cpu1: a linha do continente
        }
    }
    3
}

/// Se o clube cai na caixa "Error" da 0x669360 (não é seleção, nação escolhida fora da tabela).
pub(crate) unsafe fn cpu1_caixa(clube: usize) -> bool {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let nacao = read_usize(clube + 0x53);
    selecao(clube) == 0
        && nacao != 0
        && *((nacao + 0x11c) as *const u8) != 0
        && !(0..0x23usize).any(|k| read_usize(0xb63d60 + 72 * k) == nacao)
}

pub(crate) unsafe extern "C" fn comparar_linha_da_nacao_do_clube(clube: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize) -> u8>(1195);
    if cpu1_caixa(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = linha_da_nacao_do_clube(clube);
    PLACAR.registrar(r == n, || format!("linha_da_nacao_do_clube(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// the club's season year: the previous year before the season start day of its row
/// Original: `key_nation_get_club_season_year` (0x669950, key_nation.cpp, cdecl).
///
/// [pt] { 0x669950 } (cdecl, key_nation.cpp): `key_nation_get_club_season_year(clube)`: o ano de hoje ([0xae2c92]); para
/// clube que não é seleção, pela linha do clube (0x669360): com o dword +0x17 da linha não zero ou o dia de início
/// (+0x15, sem sinal) abaixo de 60, e o dia de hoje ([0xae2c90], com sinal) antes do dia de início, o ano anterior. A
/// original devolve só os 16 bits baixos com sentido (os altos são a sobra da vaga empurrada). Pelos registros do clube
/// 0x444930/0x447430/0x448ad0 e pela história do jogador 0x7c3e40/0x7c5fe0.
pub(crate) unsafe extern "C" fn ano_da_temporada_do_clube(clube: usize) -> u16 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let ano = *(0xae2c92 as *const u16);
    if clube != 0 && selecao(clube) == 0 {
        let k = linha(clube) as i8 as isize;
        let e = (0xb63d60isize + 72 * k) as usize;
        let inicio = core::ptr::read_unaligned((e + 0x15) as *const u16);
        if (read_usize(e + 0x17) != 0 || inicio < 0x3c) && (*(0xae2c90 as *const i16) as i32) < inicio as i32 {
            return ano.wrapping_sub(1); // cpu1: o ano anterior
        }
    }
    ano
}

pub(crate) unsafe extern "C" fn comparar_ano_da_temporada_do_clube(clube: usize) -> u16 {
    let o = orig::<unsafe extern "C" fn(usize) -> u16>(1196);
    if clube != 0 && cpu1_caixa(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = ano_da_temporada_do_clube(clube);
    PLACAR.registrar(r == n, || format!("ano_da_temporada_do_clube(clube {:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 257a leva: a temporada do clube e a nação principal: se o clube está na temporada e as datas dela, o continente principal e a nação principal (key_nation.cpp) ----------



/// Grava em `dst` (se não nulo) a data (dia, ano) montada pela 0x549ef0 (com o bissexto), como a cópia 0x417c10.
pub(crate) unsafe fn cpu2_data(dst: usize, dia: i16, ano: u16) {
    if dst == 0 {
        return;
    }
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mut b = [0u8; 8];
    data(b.as_mut_ptr(), dia as u16 as u32, ano as u32);
    core::ptr::copy_nonoverlapping(b.as_ptr(), dst as *mut u8, 8);
}

/// by the start and end days of the club nation row and today: whether it is in season and the start and end dates with
/// the right year
/// Original: `key_nation_club_in_season` (0x669bd0, key_nation.cpp, cdecl).
///
/// [pt] { 0x669bd0 } (cdecl, key_nation.cpp, quadro SEH só das datas locais): `key_nation_club_in_season(clube, &início,
/// &fim)`. A linha da nação do clube (+0x53) na tabela das 35 nações-chave (0xb63d60, 72 bytes; sem nação ou fora da
/// tabela, a linha 3 0xb63e38 e o dia de fim da data padrão 0x54a110, 40). `a` = o dia +0x2a da linha (início), `c` = o
/// dia da data +0x39 (fim; a original também copia os 7 bytes seguintes e, com o clube +0x1db ≠ −1 e o byte +0x43 3, 4
/// ou 5, troca dois deles, mas não os usa), `t` o dia de hoje ([0xae2c90]) e `y` o ano ([0xae2c92]); comparações de 16
/// bits com sinal. Com a > c (temporada que vira o ano): t > a, (a, y) e (c, y+1), 1; t < c, (a, y−1) e (c, y), 1; senão
/// (a, y) e (c, y), 0. Com a ≤ c: t < a, (a, y) e (c, y), 0; t > c, (a, y+1) e (c, y+1), 1; senão (a, y) e (c, y), 1. As
/// datas vão para `início` e `fim` quando não nulos. Pelo clube na temporada 0x547770, pela forma física 0x62a920,
/// 0x62c680 e 0x62ce30, pelo mês do fim da temporada 0x4d45f0 e pela data da transferência 0x8cee70. Chamada a cada
/// jogador: sem contador.
pub(crate) unsafe extern "C" fn clube_na_temporada(clube: usize, inicio: usize, fim: usize) -> u32 {
    let mut linha = 0xb63e38usize;
    let mut c: i16 = 0x28;
    let nacao = read_usize(clube + 0x53);
    if nacao != 0 {
        if let Some(k) = (0..0x23usize).find(|&k| read_usize(0xb63d60 + 72 * k) == nacao) {
            linha = 0xb63d60 + 72 * k;
            c = read_i16(linha + 0x39);
        }
    }
    let a = read_i16(linha + 0x2a);
    let t = *(0xae2c90 as *const i16);
    let y = *(0xae2c92 as *const u16);
    if a > c {
        if t > a {
            cpu2_data(inicio, a, y);
            if fim != 0 {
                cpu2_data(fim, c, y.wrapping_add(1)); // cpu2: o fim no ano seguinte
            }
            return 1;
        }
        if t < c {
            cpu2_data(inicio, a, y.wrapping_sub(1));
            cpu2_data(fim, c, y);
            return 1; // cpu2: antes do fim, na temporada
        }
        cpu2_data(inicio, a, y);
        cpu2_data(fim, c, y);
        return 0; // cpu2: entre o fim e o inicio, fora
    }
    if t < a {
        cpu2_data(inicio, a, y);
        cpu2_data(fim, c, y);
        return 0;
    }
    if t > c {
        cpu2_data(inicio, a, y.wrapping_add(1));
        cpu2_data(fim, c, y.wrapping_add(1));
        return 1;
    }
    cpu2_data(inicio, a, y);
    cpu2_data(fim, c, y);
    1 // cpu2: entre o inicio e o fim do ano
}

pub(crate) unsafe extern "C" fn comparar_clube_na_temporada(clube: usize, inicio: usize, fim: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(1197)(clube, inicio, fim);
    let (mut i, mut f) = ([0u8; 8], [0u8; 8]);
    let n = clube_na_temporada(
        clube,
        if inicio != 0 { i.as_mut_ptr() as usize } else { 0 },
        if fim != 0 { f.as_mut_ptr() as usize } else { 0 },
    );
    let ler = |p: usize| if p != 0 { *(p as *const [u8; 8]) } else { [0u8; 8] };
    let (oi, of) = (ler(inicio), ler(fim));
    let ok = r == n && (inicio == 0 || oi == i) && (fim == 0 || of == f);
    PLACAR.registrar(ok, || {
        format!("clube_na_temporada(clube {:x}) original={} {:02x?} {:02x?} rust={} {:02x?} {:02x?}", clube, r, oi, of, n, i, f)
    });
    r
}

/// the continent of the human's nation, or that of the first main nation of the table
/// Original: `key_nation_get_main_continent` (0x6695c0, key_nation.cpp, cdecl).
///
/// [pt] { 0x6695c0 } (cdecl, key_nation.cpp): `key_nation_get_main_continent()`: o continente (+0x71) da nação do humano
/// ([0xae2d08]); sem ela ou sem continente, o da primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
pub(crate) unsafe extern "C" fn continente_principal() -> usize {
    let h = read_usize(0xae2d08);
    if h != 0 && read_usize(h + 0x71) != 0 {
        return read_usize(h + 0x71);
    }
    for k in 0..0x23usize {
        let n = read_usize(0xb63d60 + 72 * k);
        if n != 0 && *((n + 0x11c) as *const u8) & 2 != 0 {
            return read_usize(n + 0x71);
        }
    }
    0
}

pub(crate) unsafe extern "C" fn comparar_continente_principal() -> usize {
    let r = orig::<unsafe extern "C" fn() -> usize>(1198)();
    let n = continente_principal();
    PLACAR.registrar(r == n, || format!("continente_principal() original={:x} rust={:x}", r, n));
    r
}

/// the human's nation, or the first main nation of the table
/// Original: `key_nation_get_main_nation` (0x669600, key_nation.cpp, cdecl).
///
/// [pt] { 0x669600 } (cdecl, key_nation.cpp): `key_nation_get_main_nation()`: a nação do humano ([0xae2d08]); sem ela, a
/// primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
pub(crate) unsafe extern "C" fn nacao_principal() -> usize {
    let h = read_usize(0xae2d08);
    if h != 0 {
        return h;
    }
    for k in 0..0x23usize {
        let n = read_usize(0xb63d60 + 72 * k);
        if n != 0 && *((n + 0x11c) as *const u8) & 2 != 0 {
            return n;
        }
    }
    0
}

pub(crate) unsafe extern "C" fn comparar_nacao_principal() -> usize {
    let r = orig::<unsafe extern "C" fn() -> usize>(1199)();
    let n = nacao_principal();
    PLACAR.registrar(r == n, || format!("nacao_principal() original={:x} rust={:x}", r, n));
    r
}

// ---------- 256a leva: a nação-chave do jogo: a nação-chave ativa de uma competição ou jogo, se a competição é a Copa do Mundo e se é um torneio internacional (comp.cpp, key_nation.cpp) ----------


/// the active key of the key nation that owns the fixture: by the competition nation, by the nations of the friendly
/// clubs, 0 for the World Cup, 1 for national side tournaments
/// Original: `comp_active_key_nation_of_fixture` (0x48dd70, comp.cpp, cdecl).
///
/// [pt] { 0x48dd70 } (cdecl, comp.cpp): `comp_active_key_nation_of_fixture(competição, partida)`, a chave das listas de jogos
/// do FIXTURE_MANAGER (add_fixture 0x5aa680, find_fixtures 0x5acbe0, ...), da simulação rápida (0x6b66a0) e do peso do
/// ranking da FIFA (0x58cf70). Amistoso (0x5c6710 sobre [0xae2c74]) ou reservas ([0x9cf684]) com a partida dada e a
/// competição dela amistosa ([0xae2c74]+4) ou de reservas: com uma das duas seleções (0x53b900 dos clubes +0x1c/+0x20),
/// 1; nações (+0x53) diferentes ou fora da escolha (+0x11c), 3; a nação na tabela das 35 nações-chave (0xb63d60, 72
/// bytes cada, escolhida em +0x11c), o byte +0x14 da linha; senão 0 com o mandante seleção e 3 sem. Senão, competição de
/// nação (+0x5d): o byte +0x14 da linha dela, ou 3. Sem nação: Copa do Mundo e eliminatórias (0x669a10), 0; torneio
/// internacional (0x669ae0), 1; senão o índice da nação-chave (0x669470). Chamada a cada jogo procurado: sem contador.
pub(crate) unsafe extern "C" fn nacao_chave_do_jogo(comp: usize, partida: usize) -> u8 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let linha = |nacao: usize| (0..0x23usize).find(|&k| {
        let n = read_usize(0xb63d60 + 72 * k);
        n != 0 && *((n + 0x11c) as *const u8) != 0 && n == nacao
    });
    let nacao = read_usize(comp + 0x5d);
    let reservas = read_usize(KEY_CLUB_COMP_RESERVE);
    if (amistoso(read_usize(0xae2c74), comp) != 0 || read_usize(comp) == reservas) && partida != 0 {
        let pc = read_usize(partida + 0x14);
        if pc == read_usize(read_usize(0xae2c74) + 4) || read_usize(pc) == reservas {
            if selecao(read_usize(partida + 0x1c)) != 0 || selecao(read_usize(partida + 0x20)) != 0 {
                return 1;
            }
            let casa = read_usize(partida + 0x1c);
            let a = read_usize(casa + 0x53);
            if a != read_usize(read_usize(partida + 0x20) + 0x53) || *((a + 0x11c) as *const u8) == 0 {
                return 3;
            }
            if let Some(k) = linha(a) {
                return *((0xb63d74 + 72 * k) as *const u8); // cpt3: a nacao do amistoso
            }
            return if selecao(casa) != 0 { 0 } else { 3 };
        }
    }
    if read_usize(comp + 0x5d) != 0 {
        return match linha(nacao) {
            Some(k) => *((0xb63d74 + 72 * k) as *const u8), // cpt3: a nacao-chave da competicao
            None => 3,
        };
    }
    let copa: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_IS_WORLD_CUP_COMP);
    let internacional: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_IS_INTERNATIONAL_COMP);
    let indice: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_COMP);
    if copa(comp) != 0 {
        return 0; // cpt3: Copa do Mundo e eliminatorias
    }
    if internacional(comp) != 0 {
        return 1; // cpt3: torneio internacional
    }
    indice(comp)
}

pub(crate) unsafe extern "C" fn comparar_nacao_chave_do_jogo(comp: usize, partida: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(1161)(comp, partida);
    let n = nacao_chave_do_jogo(comp, partida);
    PLACAR.registrar(r == n, || format!("nacao_chave_do_jogo(competição {}, partida {:x}) original={} rust={}", read_usize(comp) as i32, partida, r, n));
    r
}

/// Os ids das competições da Copa do Mundo: a final e as seis eliminatórias (Europa [0x9cf780], África, Ásia,
/// CONCACAF, Oceania, América do Sul).
pub(crate) const CPT3_COPA: [usize; 7] = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT];

/// Os ids dos torneios de seleções fora da Copa do Mundo: Olimpíada, Eurocopa, eliminatórias da Eurocopa, Copa
/// Africana [0x9cf7ac], Copa América, seleção B, sub-21, Copa da Oceania, Copa das Confederações e Copa da Ásia.
pub(crate) const CPT3_INTERNACIONAIS: [usize; 10] = [KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_CONFEDERATIONS_CUP, KEY_CLUB_COMP_ASIAN_CUP];

/// 1 when the competition id is the World Cup or one of its six qualifying sections
/// Original: `key_nation_is_world_cup_comp` (0x669a10, key_nation.cpp, cdecl).
///
/// [pt] { 0x669a10 } (cdecl, key_nation.cpp): `key_nation_is_world_cup_comp(competição)`: o id (+0) é a Copa do Mundo ou uma
/// das eliminatórias, 1; senão 0. Competição nula: a caixa "Error" (linha 0x3d7) e 0.
pub(crate) unsafe extern "C" fn e_copa_do_mundo(comp: usize) -> u32 {
    if comp == 0 {
        aviso_em(0x9eaff4, 0x3d7);
        return 0;
    }
    let id = read_usize(comp);
    CPT3_COPA.iter().any(|&g| read_usize(g) == id) as u32
}

pub(crate) unsafe extern "C" fn comparar_e_copa_do_mundo(comp: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(1162);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = e_copa_do_mundo(comp);
    PLACAR.registrar(r == n, || format!("e_copa_do_mundo(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

/// 1 when the competition id is one of the ten national side tournaments outside the World Cup
/// Original: `key_nation_is_international_comp` (0x669ae0, key_nation.cpp, cdecl).
///
/// [pt] { 0x669ae0 } (cdecl, key_nation.cpp): `key_nation_is_international_comp(competição)`: o id é um dos dez torneios de
/// seleções fora da Copa do Mundo, 1; senão 0. Competição nula: a caixa (linha 0x3ef) e 0.
pub(crate) unsafe extern "C" fn e_torneio_internacional(comp: usize) -> u32 {
    if comp == 0 {
        aviso_em(0x9eaff4, 0x3ef);
        return 0;
    }
    let id = read_usize(comp);
    CPT3_INTERNACIONAIS.iter().any(|&g| read_usize(g) == id) as u32
}

pub(crate) unsafe extern "C" fn comparar_e_torneio_internacional(comp: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(1163);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = e_torneio_internacional(comp);
    PLACAR.registrar(r == n, || format!("e_torneio_internacional(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

