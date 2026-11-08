use super::*;

// ---------- 257a leva: as linhas da nação-chave: a linha de uma nação, a linha do clube e o ano da temporada do clube (key_nation.cpp) ----------


/// the first row of the 35 key nation table holding the given nation, or 4
/// Original: `key_nation_index_of_nation` (0x669340, key_nation.cpp, cdecl).
///
/// [pt] { 0x669340 } (cdecl, key_nation.cpp): `key_nation_index_of_nation(nação)`: a primeira linha da tabela das 35
/// nações-chave (0xb63d60, 72 bytes, nação em +0) com a nação dada (sem conferir a nula), ou 4. Pela moral do jogo
/// 0x4cb250, pelas aposentadorias 0x7a7310, pelo amistoso livre 0x5c6b70, pelos prêmios do dia das nações e pelas vagas
/// da UEFA. Chamada a cada jogador: sem contador.
pub(crate) unsafe extern "C" fn key_nation_index_of_nation(nacao: usize) -> u8 {
    for k in 0..0x23usize {
        if read_usize(G_LEAGUE_TABLE + 72 * k) == nacao {
            return k as u8; // cpu1: a linha da nacao dada
        }
    }
    4
}

pub(crate) unsafe extern "C" fn comparar_key_nation_index_of_nation(nacao: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_NATION)(nacao);
    let n = key_nation_index_of_nation(nacao);
    PLACAR.registrar(r == n, || format!("key_nation_index_of_nation(nação {:x}) original={} rust={}", nacao, r, n));
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
pub(crate) unsafe extern "C" fn key_nation_index_of_club(clube: usize) -> u8 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) == 0 {
        let nacao = read_usize(clube + club::NATION);
        if nacao != 0 && *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) != 0 {
            for k in 0..0x23usize {
                if read_usize(G_LEAGUE_TABLE + 72 * k) == nacao {
                    return k as u8; // cpu1: a linha da nacao escolhida
                }
            }
            show_error_at(0x9eaff4, 0x26d);
            return 0;
        }
    }
    if selecao(clube) != 0 {
        return 1;
    }
    let nacao = read_usize(clube + club::NATION); // cpu1: a nacao do continente
    for k in 0..0x23usize {
        if nacao != 0 && read_usize(G_LEAGUE_WORLD_CUP_CONTINENT + 72 * k) == read_usize(nacao + nation::CONTINENT) {
            return k as u8; // cpu1: a linha do continente
        }
    }
    3
}

/// Would the club hit the 0x669360 "Error" box (not a national team, nation selected but not in the league table)?
///
/// [pt] Se o clube cai na caixa "Error" da 0x669360 (não é seleção, nação escolhida fora da tabela).
pub(crate) unsafe fn hits_key_nation_error(clube: usize) -> bool {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let nacao = read_usize(clube + club::NATION);
    selecao(clube) == 0
        && nacao != 0
        && *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) != 0
        && !(0..0x23usize).any(|k| read_usize(G_LEAGUE_TABLE + 72 * k) == nacao)
}

pub(crate) unsafe extern "C" fn comparar_key_nation_index_of_club(clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_CLUB);
    if hits_key_nation_error(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = key_nation_index_of_club(clube);
    PLACAR.registrar(r == n, || format!("key_nation_index_of_club(clube {:x}) original={} rust={}", clube, r, n));
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
pub(crate) unsafe extern "C" fn key_nation_get_club_season_year(clube: usize) -> u16 {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let ano = *(G_CURRENT_DATE_YEAR as *const u16);
    if clube != 0 && selecao(clube) == 0 {
        let k = linha(clube) as i8 as isize;
        let e = (0xb63d60isize + 72 * k) as usize;
        let inicio = core::ptr::read_unaligned((e + 0x15) as *const u16);
        if (read_usize(e + 0x17) != 0 || inicio < 0x3c) && (*(G_CURRENT_DATE as *const i16) as i32) < inicio as i32 {
            return ano.wrapping_sub(1); // cpu1: o ano anterior
        }
    }
    ano
}

pub(crate) unsafe extern "C" fn comparar_key_nation_get_club_season_year(clube: usize) -> u16 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_KEY_NATION_GET_CLUB_SEASON_YEAR);
    if clube != 0 && hits_key_nation_error(clube) {
        return o(clube);
    }
    let r = o(clube);
    let n = key_nation_get_club_season_year(clube);
    PLACAR.registrar(r == n, || format!("key_nation_get_club_season_year(clube {:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 257a leva: a temporada do clube e a nação principal: se o clube está na temporada e as datas dela, o continente principal e a nação principal (key_nation.cpp) ----------



/// Writes to dst (if not null) the date built by 0x549ef0 from day and year (with leap flag).
///
/// [pt] Grava em `dst` (se não nulo) a data (dia, ano) montada pela 0x549ef0 (com o bissexto), como a cópia 0x417c10.
pub(crate) unsafe fn write_date(dst: usize, dia: i16, ano: u16) {
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
pub(crate) unsafe extern "C" fn key_nation_club_in_season(clube: usize, inicio: usize, fim: usize) -> u32 {
    let mut linha = 0xb63e38usize;
    let mut c: i16 = 0x28;
    let nacao = read_usize(clube + club::NATION);
    if nacao != 0 {
        if let Some(k) = (0..0x23usize).find(|&k| read_usize(G_LEAGUE_TABLE + 72 * k) == nacao) {
            linha = 0xb63d60 + 72 * k;
            c = read_i16(linha + 0x39);
        }
    }
    let a = read_i16(linha + 0x2a);
    let t = *(G_CURRENT_DATE as *const i16);
    let y = *(G_CURRENT_DATE_YEAR as *const u16);
    if a > c {
        if t > a {
            write_date(inicio, a, y);
            if fim != 0 {
                write_date(fim, c, y.wrapping_add(1)); // cpu2: o fim no ano seguinte
            }
            return 1;
        }
        if t < c {
            write_date(inicio, a, y.wrapping_sub(1));
            write_date(fim, c, y);
            return 1; // cpu2: antes do fim, na temporada
        }
        write_date(inicio, a, y);
        write_date(fim, c, y);
        return 0; // cpu2: entre o fim e o inicio, fora
    }
    if t < a {
        write_date(inicio, a, y);
        write_date(fim, c, y);
        return 0;
    }
    if t > c {
        write_date(inicio, a, y.wrapping_add(1));
        write_date(fim, c, y.wrapping_add(1));
        return 1;
    }
    write_date(inicio, a, y);
    write_date(fim, c, y);
    1 // cpu2: entre o inicio e o fim do ano
}

pub(crate) unsafe extern "C" fn comparar_key_nation_club_in_season(clube: usize, inicio: usize, fim: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_KEY_NATION_CLUB_IN_SEASON)(clube, inicio, fim);
    let (mut i, mut f) = ([0u8; 8], [0u8; 8]);
    let n = key_nation_club_in_season(
        clube,
        if inicio != 0 { i.as_mut_ptr() as usize } else { 0 },
        if fim != 0 { f.as_mut_ptr() as usize } else { 0 },
    );
    let ler = |p: usize| if p != 0 { *(p as *const [u8; 8]) } else { [0u8; 8] };
    let (oi, of) = (ler(inicio), ler(fim));
    let ok = r == n && (inicio == 0 || oi == i) && (fim == 0 || of == f);
    PLACAR.registrar(ok, || {
        format!("key_nation_club_in_season(clube {:x}) original={} {:02x?} {:02x?} rust={} {:02x?} {:02x?}", clube, r, oi, of, n, i, f)
    });
    r
}

/// the continent of the human's nation, or that of the first main nation of the table
/// Original: `key_nation_get_main_continent` (0x6695c0, key_nation.cpp, cdecl).
///
/// [pt] { 0x6695c0 } (cdecl, key_nation.cpp): `key_nation_get_main_continent()`: o continente (+0x71) da nação do humano
/// ([0xae2d08]); sem ela ou sem continente, o da primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
pub(crate) unsafe extern "C" fn key_nation_get_main_continent() -> usize {
    let h = read_usize(G_MAIN_NATION);
    if h != 0 && read_usize(h + 0x71) != 0 {
        return read_usize(h + 0x71);
    }
    for k in 0..0x23usize {
        let n = read_usize(G_LEAGUE_TABLE + 72 * k);
        if n != 0 && *((n + 0x11c) as *const u8) & 2 != 0 {
            return read_usize(n + 0x71);
        }
    }
    0
}

pub(crate) unsafe extern "C" fn comparar_key_nation_get_main_continent() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(IDX_KEY_NATION_GET_MAIN_CONTINENT)();
    let n = key_nation_get_main_continent();
    PLACAR.registrar(r == n, || format!("key_nation_get_main_continent() original={:x} rust={:x}", r, n));
    r
}

/// the human's nation, or the first main nation of the table
/// Original: `key_nation_get_main_nation` (0x669600, key_nation.cpp, cdecl).
///
/// [pt] { 0x669600 } (cdecl, key_nation.cpp): `key_nation_get_main_nation()`: a nação do humano ([0xae2d08]); sem ela, a
/// primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
pub(crate) unsafe extern "C" fn key_nation_get_main_nation() -> usize {
    let h = read_usize(G_MAIN_NATION);
    if h != 0 {
        return h;
    }
    for k in 0..0x23usize {
        let n = read_usize(G_LEAGUE_TABLE + 72 * k);
        if n != 0 && *((n + 0x11c) as *const u8) & 2 != 0 {
            return n;
        }
    }
    0
}

pub(crate) unsafe extern "C" fn comparar_key_nation_get_main_nation() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(IDX_KEY_NATION_GET_MAIN_NATION)();
    let n = key_nation_get_main_nation();
    PLACAR.registrar(r == n, || format!("key_nation_get_main_nation() original={:x} rust={:x}", r, n));
    r
}

// ---------- 256a leva: a nação-chave do jogo: a nação-chave ativa de uma competição ou jogo, se a competição é a Copa do Mundo e se é um torneio internacional (comp.cpp, key_nation.cpp) ----------




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
pub(crate) unsafe extern "C" fn key_nation_is_world_cup_comp(comp: usize) -> u32 {
    if comp == 0 {
        show_error_at(0x9eaff4, 0x3d7);
        return 0;
    }
    let id = read_usize(comp);
    CPT3_COPA.iter().any(|&g| read_usize(g) == id) as u32
}

pub(crate) unsafe extern "C" fn comparar_key_nation_is_world_cup_comp(comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_KEY_NATION_IS_WORLD_CUP_COMP);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = key_nation_is_world_cup_comp(comp);
    PLACAR.registrar(r == n, || format!("key_nation_is_world_cup_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

/// 1 when the competition id is one of the ten national side tournaments outside the World Cup
/// Original: `key_nation_is_international_comp` (0x669ae0, key_nation.cpp, cdecl).
///
/// [pt] { 0x669ae0 } (cdecl, key_nation.cpp): `key_nation_is_international_comp(competição)`: o id é um dos dez torneios de
/// seleções fora da Copa do Mundo, 1; senão 0. Competição nula: a caixa (linha 0x3ef) e 0.
pub(crate) unsafe extern "C" fn key_nation_is_international_comp(comp: usize) -> u32 {
    if comp == 0 {
        show_error_at(0x9eaff4, 0x3ef);
        return 0;
    }
    let id = read_usize(comp);
    CPT3_INTERNACIONAIS.iter().any(|&g| read_usize(g) == id) as u32
}

pub(crate) unsafe extern "C" fn comparar_key_nation_is_international_comp(comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_KEY_NATION_IS_INTERNATIONAL_COMP);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = key_nation_is_international_comp(comp);
    PLACAR.registrar(r == n, || format!("key_nation_is_international_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

/// the competition's key nation row: the nation's, 0 for the World Cup, 1 for national side tournaments, 2 for the club
/// world cups, the continent's or 3
/// Original: `key_nation_index_of_comp` (0x669470, key_nation.cpp, cdecl).
///
/// [pt] { 0x669470 } (cdecl, key_nation.cpp): `key_nation_index_of_comp(competição)` (pelo alcance das regras da partida
/// 0x8d2fa0, pela liga do dia 0x6847c0, pelos registros da copa 0x519890 e pela nação-chave do jogo 0x48dd70 da cpt_3):
/// competição de nação (+0x5d): a linha dela na tabela das 35 nações-chave (0xb63d60, 72 bytes), ou 4. Sem nação: Copa
/// do Mundo e eliminatórias, 0; torneio de seleções (0x669ae0), 1; Copa do Mundo de Clubes e Mundial de Clubes da FIFA,
/// 2; senão a linha de continente (nação 0) com o continente da competição (+0x59), ou 3. A original lê +0x5d antes de
/// conferir a competição nula (o caso de erro nunca chega à caixa). Chamada a cada partida: sem contador.
pub(crate) unsafe extern "C" fn key_nation_index_of_comp(comp: usize) -> u8 {
    let nacao = read_usize(comp + competition::CLUB_COMP_NATION);
    if nacao != 0 {
        let linha = (0..0x23usize).find(|&k| read_usize(G_LEAGUE_TABLE + 72 * k) == nacao); // cpt4: a busca da nacao
        return match linha {
            Some(k) => k as u8, // cpt4: a linha da nacao
            None => 4,
        };
    }
    let id = read_usize(comp);
    let copa = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT]; // cpt4: a Copa e as eliminatorias
    if copa.iter().any(|&g| read_usize(g) == id) {
        return 0;
    }
    let internacional: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_IS_INTERNATIONAL_COMP);
    if internacional(comp) != 0 {
        return 1;
    }
    if id == read_usize(KEY_CLUB_COMP_WORLD_CLUB_CUP) || id == read_usize(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) {
        return 2;
    }
    for k in 0..0x23usize {
        let c = read_usize(comp + competition::CLUB_COMP_CONTINENT);
        if read_usize(G_LEAGUE_TABLE + 72 * k) == 0 && c != 0 && c == read_usize(G_LEAGUE_WORLD_CUP_CONTINENT + 72 * k) {
            return k as u8; // cpt4: a linha do continente
        }
    }
    3
}

pub(crate) unsafe extern "C" fn comparar_key_nation_index_of_comp(comp: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_COMP)(comp);
    let n = key_nation_index_of_comp(comp);
    PLACAR.registrar(r == n, || format!("key_nation_index_of_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}
